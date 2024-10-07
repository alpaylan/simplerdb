use std::path::PathBuf;

use crate::page::Page;


pub(crate) struct BlockId {
    filename: String,
    block_number: u64,
}


pub(crate) struct FileManager {
    dir: PathBuf,
    blocksize: u64
}

impl FileManager {
    pub(crate) fn read(&mut self, block: &BlockId, page: &Page) {
        unimplemented!()
    }

    pub(crate) fn write(&mut self, block: &BlockId, page: &Page) {
        unimplemented!()
    }

    pub(crate) fn append(&mut self, filename: &str) -> BlockId {
        unimplemented!()
    }

    pub(crate) fn is_new(&self) -> bool {
        unimplemented!();
    }

    pub(crate) fn length(&self, filename: &str) -> u64 {
        unimplemented!()
    }
}

