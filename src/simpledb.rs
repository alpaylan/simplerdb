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