use std::{
    io::{Read, Seek, SeekFrom, Write},
    path::PathBuf,
};

use crate::page::Page;

#[derive(Debug)]
pub(crate) struct BlockId {
    filename: String,
    block_number: u64,
}

impl BlockId {
    pub(crate) fn new(filename: String, block_number: u64) -> Self {
        Self {
            filename,
            block_number,
        }
    }
}

pub(crate) struct FileManager {
    pub(crate) dir: PathBuf,
    pub(crate) blocksize: u64,
}

impl FileManager {
    pub(crate) fn new(dirname: &str, blocksize: u64) -> Result<Self, std::io::Error> {
        // Create the directory if it doesn't exist
        std::fs::create_dir_all(dirname)?;

        // Remove any leftover temp files
        std::fs::read_dir(dirname)?
            .map(|entry| entry.unwrap().path())
            .filter(|path| path.is_file() && path.starts_with("temp"))
            .try_for_each(|path| std::fs::remove_file(path))?;

        Ok(Self {
            dir: PathBuf::from(dirname),
            blocksize,
        })
    }
}

impl FileManager {
    pub(crate) fn read(&self, block: &BlockId, page: &mut Page) -> Result<(), std::io::Error> {
        let filename = self.dir.join(&block.filename);
        let offset = block.block_number * self.blocksize;
        let mut file = std::fs::File::open(filename)?;
        file.seek(SeekFrom::Start(offset))?;
        file.read(page.bb.contents())?;
        Ok(())
    }

    pub(crate) fn write(&self, block: &BlockId, page: &mut Page) -> Result<(), std::io::Error> {
        let filename = self.dir.join(&block.filename);
        let offset = block.block_number * self.blocksize;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(filename)?;
        file.seek(SeekFrom::Start(offset))?;
        file.write_all(page.bb.contents())
    }

    pub(crate) fn append(&self, filename: &str) -> Result<BlockId, std::io::Error> {
        let filepath = self.dir.join(filename);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&filepath)?;

        let block_number = self.length(filename)? / self.blocksize;
        let block = BlockId::new(filename.to_string(), block_number);
        let offset = block.block_number * self.blocksize;

        file.seek(SeekFrom::Start(offset))?;
        file.write_all(&vec![0; self.blocksize as usize])?;

        Ok(block)
    }

    pub(crate) fn length(&self, filename: &str) -> Result<u64, std::io::Error> {
        let filename = self.dir.join(filename);
        let metadata = std::fs::metadata(filename)?;
        Ok(metadata.len())
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
}

