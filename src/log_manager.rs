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
        if boundary < bytes_needed + 8 {
            self.flush(file_manager)?;
            self.current_block = file_manager.append(&self.log_file.to_string_lossy())?;
            self.log_page.set_int(0, file_manager.blocksize);
            file_manager.write(&self.current_block, &mut self.log_page)?;
            boundary = self.log_page.get_int(0);
        }
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


#[cfg(test)]
mod tests {
    use crate::{file_manager::BlockId, log_manager::LogManager, page::Page, simpledb::SimpleDB};
    use expect_test::{expect, Expect};
    use pretty_hex::PrettyHex;

    #[test]
    fn test_log_manager() {
        let db = SimpleDB::new("logtest", 400, 8).unwrap();
        let mut lm = db.log_manager;
        let mut create_records = |start: u64, end: u64| {
            (start..end).for_each(|i| {
                let s = format!("record{}", i);
                let npos = Page::max_length(s.len() as u64);
                let mut p = Page::new(npos + 8);
                p.set_string(0, &s);
                p.set_int(npos, i + 100);
                let lsn = lm.append(&db.file_manager, p.bb.bytes());
            });
        };

        create_records(1, 14);
        // create_records(11, 21);
        lm.flush_with_lsn(&db.file_manager, 20).unwrap();


        let logfile = std::fs::read("logtest/logfile").unwrap();
        let s = pretty_hex::pretty_hex(&logfile);
        let expected = expect![[r#"
            Length: 800 (0x320) bytes
            0000:   00 00 00 00  00 00 00 19  00 00 00 00  00 00 00 00   ................
            0010:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
            0020:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0030:   32 00 00 00  00 00 00 00  70 00 00 00  00 00 00 00   2.......p.......
            0040:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0050:   31 00 00 00  00 00 00 00  6f 00 00 00  00 00 00 00   1.......o.......
            0060:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0070:   30 00 00 00  00 00 00 00  6e 00 00 00  00 00 00 00   0.......n.......
            0080:   17 00 00 00  00 00 00 00  07 72 65 63  6f 72 64 39   .........record9
            0090:   00 00 00 00  00 00 00 6d  00 00 00 00  00 00 00 17   .......m........
            00a0:   00 00 00 00  00 00 00 07  72 65 63 6f  72 64 38 00   ........record8.
            00b0:   00 00 00 00  00 00 6c 00  00 00 00 00  00 00 17 00   ......l.........
            00c0:   00 00 00 00  00 00 07 72  65 63 6f 72  64 37 00 00   .......record7..
            00d0:   00 00 00 00  00 6b 00 00  00 00 00 00  00 17 00 00   .....k..........
            00e0:   00 00 00 00  00 07 72 65  63 6f 72 64  36 00 00 00   ......record6...
            00f0:   00 00 00 00  6a 00 00 00  00 00 00 00  17 00 00 00   ....j...........
            0100:   00 00 00 00  07 72 65 63  6f 72 64 35  00 00 00 00   .....record5....
            0110:   00 00 00 69  00 00 00 00  00 00 00 17  00 00 00 00   ...i............
            0120:   00 00 00 07  72 65 63 6f  72 64 34 00  00 00 00 00   ....record4.....
            0130:   00 00 68 00  00 00 00 00  00 00 17 00  00 00 00 00   ..h.............
            0140:   00 00 07 72  65 63 6f 72  64 33 00 00  00 00 00 00   ...record3......
            0150:   00 67 00 00  00 00 00 00  00 17 00 00  00 00 00 00   .g..............
            0160:   00 07 72 65  63 6f 72 64  32 00 00 00  00 00 00 00   ..record2.......
            0170:   66 00 00 00  00 00 00 00  17 00 00 00  00 00 00 00   f...............
            0180:   07 72 65 63  6f 72 64 31  00 00 00 00  00 00 00 65   .record1.......e
            0190:   00 00 00 00  00 00 01 70  00 00 00 00  00 00 00 00   .......p........
            01a0:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
            01b0:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            01c0:   32 00 00 00  00 00 00 00  70 00 00 00  00 00 00 00   2.......p.......
            01d0:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            01e0:   31 00 00 00  00 00 00 00  6f 00 00 00  00 00 00 00   1.......o.......
            01f0:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0200:   30 00 00 00  00 00 00 00  6e 00 00 00  00 00 00 00   0.......n.......
            0210:   17 00 00 00  00 00 00 00  07 72 65 63  6f 72 64 39   .........record9
            0220:   00 00 00 00  00 00 00 6d  00 00 00 00  00 00 00 17   .......m........
            0230:   00 00 00 00  00 00 00 07  72 65 63 6f  72 64 38 00   ........record8.
            0240:   00 00 00 00  00 00 6c 00  00 00 00 00  00 00 17 00   ......l.........
            0250:   00 00 00 00  00 00 07 72  65 63 6f 72  64 37 00 00   .......record7..
            0260:   00 00 00 00  00 6b 00 00  00 00 00 00  00 17 00 00   .....k..........
            0270:   00 00 00 00  00 07 72 65  63 6f 72 64  36 00 00 00   ......record6...
            0280:   00 00 00 00  6a 00 00 00  00 00 00 00  17 00 00 00   ....j...........
            0290:   00 00 00 00  07 72 65 63  6f 72 64 35  00 00 00 00   .....record5....
            02a0:   00 00 00 69  00 00 00 00  00 00 00 17  00 00 00 00   ...i............
            02b0:   00 00 00 07  72 65 63 6f  72 64 34 00  00 00 00 00   ....record4.....
            02c0:   00 00 68 00  00 00 00 00  00 00 17 00  00 00 00 00   ..h.............
            02d0:   00 00 07 72  65 63 6f 72  64 33 00 00  00 00 00 00   ...record3......
            02e0:   00 67 00 00  00 00 00 00  00 17 00 00  00 00 00 00   .g..............
            02f0:   00 07 72 65  63 6f 72 64  32 00 00 00  00 00 00 00   ..record2.......
            0300:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            0310:   72 65 63 6f  72 64 31 33  00 00 00 00  00 00 00 71   record13.......q"#]];

        expected.assert_eq(&s);
    }
}

