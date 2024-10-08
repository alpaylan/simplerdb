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
    use crate::{file_manager::BlockId, log_manager::LogManager, page::Page, simpledb::SimpleDB};
    use expect_test::{expect, Expect};
    use pretty_hex::PrettyHex;

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
        fm.read(&block, &mut p2).unwrap();

        assert_eq!(p2.get_string(pos1), "abcdefghijklm");
        assert_eq!(p2.get_int(pos2), 345);
    }


    fn check_logs(actual: &LogManager, expect: Expect) {
        let s = pretty_hex::pretty_hex(&actual.log_page.bb.bytes());
        expect.assert_eq(&s);
    }
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
                p.set_int(npos, i);
                let lsn = lm.append(&db.file_manager, p.bb.bytes());
                println!("LM: {:?}", lm);
            });
        };

        create_records(1, 10);
        create_records(11, 20);
        lm.flush_with_lsn(&db.file_manager, 20).unwrap();


        let logfile = std::fs::read("logtest/logfile").unwrap();
        let s = pretty_hex::pretty_hex(&logfile);
        let expected = expect![[r#"
            Length: 800 (0x320) bytes
            0000:   00 00 00 00  00 00 00 19  00 00 00 00  00 00 00 00   ................
            0010:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
            0020:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0030:   33 00 00 00  00 00 00 00  0d 00 00 00  00 00 00 00   3...............
            0040:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0050:   32 00 00 00  00 00 00 00  0c 00 00 00  00 00 00 00   2...............
            0060:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0070:   31 00 00 00  00 00 00 00  0b 00 00 00  00 00 00 00   1...............
            0080:   17 00 00 00  00 00 00 00  07 72 65 63  6f 72 64 39   .........record9
            0090:   00 00 00 00  00 00 00 09  00 00 00 00  00 00 00 17   ................
            00a0:   00 00 00 00  00 00 00 07  72 65 63 6f  72 64 38 00   ........record8.
            00b0:   00 00 00 00  00 00 08 00  00 00 00 00  00 00 17 00   ................
            00c0:   00 00 00 00  00 00 07 72  65 63 6f 72  64 37 00 00   .......record7..
            00d0:   00 00 00 00  00 07 00 00  00 00 00 00  00 17 00 00   ................
            00e0:   00 00 00 00  00 07 72 65  63 6f 72 64  36 00 00 00   ......record6...
            00f0:   00 00 00 00  06 00 00 00  00 00 00 00  17 00 00 00   ................
            0100:   00 00 00 00  07 72 65 63  6f 72 64 35  00 00 00 00   .....record5....
            0110:   00 00 00 05  00 00 00 00  00 00 00 17  00 00 00 00   ................
            0120:   00 00 00 07  72 65 63 6f  72 64 34 00  00 00 00 00   ....record4.....
            0130:   00 00 04 00  00 00 00 00  00 00 17 00  00 00 00 00   ................
            0140:   00 00 07 72  65 63 6f 72  64 33 00 00  00 00 00 00   ...record3......
            0150:   00 03 00 00  00 00 00 00  00 17 00 00  00 00 00 00   ................
            0160:   00 07 72 65  63 6f 72 64  32 00 00 00  00 00 00 00   ..record2.......
            0170:   02 00 00 00  00 00 00 00  17 00 00 00  00 00 00 00   ................
            0180:   07 72 65 63  6f 72 64 31  00 00 00 00  00 00 00 01   .record1........
            0190:   00 00 00 00  00 00 00 d0  00 00 00 00  00 00 00 00   ................
            01a0:   00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00   ................
            01b0:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            01c0:   33 00 00 00  00 00 00 00  0d 00 00 00  00 00 00 00   3...............
            01d0:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            01e0:   32 00 00 00  00 00 00 00  0c 00 00 00  00 00 00 00   2...............
            01f0:   18 00 00 00  00 00 00 00  08 72 65 63  6f 72 64 31   .........record1
            0200:   31 00 00 00  00 00 00 00  0b 00 00 00  00 00 00 00   1...............
            0210:   17 00 00 00  00 00 00 00  07 72 65 63  6f 72 64 39   .........record9
            0220:   00 00 00 00  00 00 00 09  00 00 00 00  00 00 00 17   ................
            0230:   00 00 00 00  00 00 00 07  72 65 63 6f  72 64 38 00   ........record8.
            0240:   00 00 00 00  00 00 08 00  00 00 00 00  00 00 17 00   ................
            0250:   00 00 00 00  00 00 07 72  65 63 6f 72  64 37 00 00   .......record7..
            0260:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            0270:   72 65 63 6f  72 64 31 39  00 00 00 00  00 00 00 13   record19........
            0280:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            0290:   72 65 63 6f  72 64 31 38  00 00 00 00  00 00 00 12   record18........
            02a0:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            02b0:   72 65 63 6f  72 64 31 37  00 00 00 00  00 00 00 11   record17........
            02c0:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            02d0:   72 65 63 6f  72 64 31 36  00 00 00 00  00 00 00 10   record16........
            02e0:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            02f0:   72 65 63 6f  72 64 31 35  00 00 00 00  00 00 00 0f   record15........
            0300:   00 00 00 00  00 00 00 18  00 00 00 00  00 00 00 08   ................
            0310:   72 65 63 6f  72 64 31 34  00 00 00 00  00 00 00 0e   record14........"#]];

        expected.assert_eq(&s);
    }
}
