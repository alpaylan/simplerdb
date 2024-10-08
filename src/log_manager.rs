use std::path::PathBuf;

use crate::resultset::Record;
use crate::{
    file_manager::{BlockId, FileManager},
    page::Page,
};

#[derive(Debug)]
pub(crate) struct LogManager {
    pub(crate) log_file: PathBuf,
    pub(crate) log_page: Page,
    pub(crate) current_block: BlockId,
    pub(crate) latest_lsn: u64,
    pub(crate) last_saved_lsn: u64,
}

impl LogManager {
    pub(crate) fn new(file_manager: &FileManager, log_file: &str) -> Self {
        let mut log_page = Page::new(file_manager.blocksize);

        let current_block = if let Ok(log_size) = file_manager.length(log_file) {
            let block = BlockId::new(log_file.into(), log_size - 1);
            file_manager.read(&block, &mut log_page).unwrap();
            block
        } else {
            let block = file_manager.append(log_file).unwrap();
            log_page.set_int(0, file_manager.blocksize);
            file_manager.write(&block, &mut log_page).unwrap();
            block
        };

        Self {
            log_file: log_file.into(),
            log_page,
            current_block,
            latest_lsn: 0,
            last_saved_lsn: 0,
        }
    }
}
impl LogManager {
    pub(crate) fn append(
        &mut self,
        file_manager: &FileManager,
        record: &[u8],
    ) -> Result<u64, std::io::Error> {
        let mut boundary = self.log_page.get_int(0);
        let rec_size = record.len() as u64;
        let bytes_needed = rec_size + 8;
        println!("[before] Boundary: {:?}", boundary);
        println!("[before] Bytes needed: {:?}", bytes_needed);
        if boundary < bytes_needed + 8 {
            self.flush(file_manager)?;
            self.current_block = file_manager.append(&self.log_file.to_string_lossy())?;
            self.log_page.set_int(0, file_manager.blocksize);
            file_manager.write(&self.current_block, &mut self.log_page)?;
            boundary = self.log_page.get_int(0);
        }
        
        println!("[after] Boundary: {:?}", boundary);
        println!("[after] Bytes needed: {:?}", bytes_needed);
        let rec_pos = boundary - bytes_needed;
        
        self.log_page.set_bytes(rec_pos, record);
        self.log_page.set_int(0, rec_pos);

        self.latest_lsn += 1;
        Ok(self.latest_lsn)
    }

    pub(crate) fn flush(&mut self, file_manager: &FileManager) -> Result<(), std::io::Error> {
        file_manager.write(&self.current_block, &mut self.log_page)?;
        self.last_saved_lsn = self.latest_lsn;
        Ok(())
    }

    pub(crate) fn flush_with_lsn(
        &mut self,
        file_manager: &FileManager,
        lsn: u64,
    ) -> Result<(), std::io::Error> {
        if lsn > self.last_saved_lsn {
            self.flush(file_manager)?;
        }

        Ok(())
    }
}

impl Iterator for LogManager {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
