use std::path::PathBuf;

use crate::{file_manager::{BlockId, FileManager}, page::Page};
use crate::resultset::Record;

pub(crate) struct LogManager {
    file_manager: FileManager,
    log_file: PathBuf,
    log_page: Page,
    current_block: BlockId,
    latest_lsn: u64,
    last_saved_lsn: u64

}

impl LogManager {
    pub(crate) fn append(&mut self, record: &[u8]) -> u64 {
        unimplemented!()
    }

    pub(crate) fn flush(&self, log_sequence_number: u64) {
        unimplemented!()
    }
}

impl Iterator for LogManager {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
