use std::path::PathBuf;

use crate::resultset::Record;
use crate::{
    file_manager::{BlockId, FileManager},
    page::Page,
};

pub(crate) struct LogManager {
    log_file: PathBuf,
    log_page: Page,
    current_block: BlockId,
    latest_lsn: u64,
    last_saved_lsn: u64,
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
    pub(crate) fn append(&mut self, record: &[u8]) -> u64 {
        unimplemented!()
    }

    pub(crate) fn flush(&mut self, file_manager: &FileManager) -> Result<(), std::io::Error> {
        file_manager.write(&self.current_block, &mut self.log_page)?;
        self.last_saved_lsn = self.latest_lsn;
        Ok(())
    }
}

impl Iterator for LogManager {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
