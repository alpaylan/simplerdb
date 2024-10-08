use crate::{buffer_manager::BufferManager, file_manager::FileManager, log_manager::LogManager};

pub(crate) struct SimpleDB {
    pub(crate) file_manager: FileManager,
    pub(crate) log_manager: LogManager,
    pub(crate) buffer_manager: BufferManager,
}

impl SimpleDB {
    pub(crate) fn new(
        dirname: &str,
        blocksize: u64,
        buffersize: u64,
    ) -> Result<Self, std::io::Error> {
        let file_manager = FileManager::new(dirname, blocksize)?;
        // lm = new LogMgr(fm, LOG_FILE);
        let log_manager = LogManager::new(&file_manager, "logfile");
        // bm = new BufferMgr(fm, lm, buffsize);
        let buffer_manager = BufferManager::new(&file_manager, buffersize);

        Ok(Self {
            file_manager,
            log_manager,
            buffer_manager,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{file_manager::BlockId, page::Page, simpledb::SimpleDB};

    #[test]
    fn test_file_manager() {
        let db = SimpleDB::new("filetest", 400, 8).unwrap();

        let fm = db.file_manager;
        let block = BlockId::new("testfile".to_string(), 2);
        let mut p1 = Page::new(fm.blocksize);
        let pos1 = 88;
        p1.set_string(pos1, "abcdefghijklm");
        let size = Page::max_length("abcdefghijklm".len() as u64);
        let pos2 = pos1 + size;
        p1.set_int(pos2, 345);
        fm.write(&block, &mut p1).unwrap();

        let mut p2 = Page::new(fm.blocksize);
        println!("Reading block: {:?}", block);
        fm.read(&block, &mut p2).unwrap();
        println!("Read to page {:?}", p2);
        
        assert_eq!(p2.get_string(pos1), "abcdefghijklm");
        assert_eq!(p2.get_int(pos2), 345);
    }
}
