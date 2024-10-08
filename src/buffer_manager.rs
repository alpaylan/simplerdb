use crate::{buffer::Buffer, file_manager::FileManager, log_manager::LogManager};

pub(crate) struct BufferManager {
    pub(crate) bufferpool: Vec<Buffer>,
}

impl BufferManager {
    pub(crate) fn new(
        file_manager: &FileManager,
        numbuffs: u64,
    ) -> Self {
        let mut bufferpool = Vec::with_capacity(numbuffs as usize);
        for _ in 0..numbuffs {
            bufferpool.push(Buffer::new(&file_manager));
        }
        Self {
            bufferpool,
        }
    }
}
