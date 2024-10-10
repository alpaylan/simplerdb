use crate::{file_manager::{self, BlockId, FileManager}, log_manager::LogManager, page::Page};


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

    pub(crate) fn set_modified(&mut self, lsn: Option<u64>, txnum: u64) {
        self.transaction_number = Some(txnum);
        self.log_sequence_number = lsn;
    }

    pub(crate) fn assign_to_block(&mut self, block: BlockId, file_manager: &FileManager, log_manager: &mut LogManager) -> Result<(), std::io::Error> {
        self.flush(file_manager, log_manager)?;
        file_manager.read(&block, &mut self.contents).unwrap();
        self.block = Some(block);
        self.pins = 0;

        Ok(())
    }

    pub(crate) fn flush(&mut self, file_manager: &FileManager, log_manager: &mut LogManager) -> Result<(), std::io::Error> {
        if self.transaction_number.is_some() && self.log_sequence_number.is_some() && self.block.is_some() {
            log_manager.flush_with_lsn(file_manager, self.log_sequence_number.unwrap())?;
            file_manager.write(self.block.as_ref().unwrap(), &mut self.contents)?;
            self.transaction_number = None;
        }

        Ok(())
    }

    pub(crate) fn pin(&mut self) {
        self.pins += 1;
    }

    pub(crate) fn unpin(&mut self) {
        self.pins -= 1;
    }
}


#[cfg(test)]
mod tests {
    use crate::{file_manager::BlockId, log_manager::LogManager, page::Page, simpledb::SimpleDB};
    use expect_test::{expect, Expect};
    use pretty_hex::PrettyHex;

    #[test]
    fn test_buffer() {
        let db = SimpleDB::new("buffertest", 400, 3).unwrap();
        let buff1 = db.buffer_manager.pin(&BlockId::new("testfile".to_string(), 1));
        let p = buff1.contents();
        let n = p.get_int(80);
        p.set_int(80, n + 1);
        buff1.set_modified(1, 0);
        db.buffer_manager.unpin(buff1);

        let buff2 = db.buffer_manager.pin(&BlockId::new("testfile".to_string(), 2));
        let buff3 = db.buffer_manager.pin(&BlockId::new("testfile".to_string(), 3));
        let buff4 = db.buffer_manager.pin(&BlockId::new("testfile".to_string(), 4));
        let p = buff2.contents();
        p.set_int(80, 9999);
        buff2.set_modified(1, 0);
        db.buffer_manager.unpin(buff2);
    }

}

