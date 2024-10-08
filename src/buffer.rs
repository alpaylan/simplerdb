use crate::{file_manager::{BlockId, FileManager}, page::Page};


pub(crate) struct Buffer {
    pub(crate) contents: Page,
    pub(crate) block: Option<BlockId>,
    pub(crate) pins: u64,
    pub(crate) log_sequence_number: Option<u64>,
    pub(crate) transaction_number: Option<u64>,

}

impl Buffer {
    pub(crate) fn new(file_manager: &FileManager) -> Self {
        let contents = Page::new(file_manager.blocksize);
        Self {
            contents,
            block: None,
            pins: 0,
            log_sequence_number: None,
            transaction_number: None,
        }
    }
}