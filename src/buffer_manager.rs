use crate::{buffer::Buffer, file_manager::FileManager, log_manager::LogManager};

pub(crate) struct BufferManager {
    pub(crate) bufferpool: Vec<Buffer>,
    pub(crate) available: u64,
}

impl BufferManager {
    pub(crate) fn new(file_manager: &FileManager, numbuffs: u64) -> Self {
        let mut bufferpool = Vec::with_capacity(numbuffs as usize);

        for _ in 0..numbuffs {
            bufferpool.push(Buffer::new(&file_manager));
        }

        Self {
            bufferpool,
            available: numbuffs,
        }
    }

    pub(crate) fn flush_all(
        &mut self,
        file_manager: &FileManager,
        log_manager: &mut LogManager,
        txnum: u64,
    ) -> Result<(), std::io::Error> {
        self.bufferpool
            .iter_mut()
            .filter(|b| b.transaction_number == Some(txnum))
            .try_for_each(|b| b.flush(file_manager, log_manager))
    }

    pub(crate) fn unpin(&mut self, buff: &mut Buffer) {
        buff.unpin();
        if buff.pins == 0 {
            self.available += 1;

        }

    }
}

#[cfg(test)]
mod tests {
    use crate::{file_manager::BlockId, log_manager::LogManager, page::Page, simpledb::SimpleDB};
    use expect_test::{expect, Expect};
    use pretty_hex::PrettyHex;

    #[test]
    fn test_buffer() {
        let db = SimpleDB::new("buffermgrtest", 400, 3).unwrap();
        let bm = &db.buffer_manager;
        let mut buff = Vec::with_capacity(6);
        buff.push(bm.pin(&BlockId::new("testfile".to_string(), 0)));
        buff.push(bm.pin(&BlockId::new("testfile".to_string(), 1)));
        buff.push(bm.pin(&BlockId::new("testfile".to_string(), 2)));

        bm.unpin(buff[1]);
        buff[1] = None;
        buff[3] = bm.pin(BlockId::new("testfile".to_string(), 0)); // block 0 pinned twice
        buff[4] = bm.pin(BlockId::new("testfile".to_string(), 1)); // block 1 repinned

        assert!(bm.pin(BlockId::new("testfile".to_string(), 3)).is_err()); // no available buffers

        bm.unpin(buff[2]);
        buff[2] = None;
        buff[5] = bm.pin(BlockId::new("testfile".to_string(), 3)); // now this works

        println!("Final Buffer Allocation:");
        for (i, b) in buff.iter().enumerate() {
            if let Some(b) = b {
                println!("buff[{}] pinned to block {}", i, b.block());
            }
        }
    }
}
