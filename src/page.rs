use crate::byte_buffer::ByteBuffer;

#[derive(Debug)]
pub(crate) struct Page {
    pub(crate) block_size: u64,
    pub(crate) bb: ByteBuffer,
}

impl Page {
    pub(crate) fn new(block_size: u64) -> Self {
        Self {
            block_size,
            bb: ByteBuffer::new(vec![0; block_size as usize]),
        }
    }

    pub(crate) fn from_bytes(bytes: Vec<u8>) -> Self {
        let block_size = bytes.len() as u64;
        Self {
            block_size,
            bb: ByteBuffer::new(bytes),
        }
    }
}

impl Page {
    pub(crate) fn get_int(&mut self, offset: u64) -> u64 {
        self.bb.get_int_with_offset(offset)
    }

    pub(crate) fn get_bytes(&mut self, offset: u64) -> &[u8] {
        let len = self.bb.get_int_with_offset(offset);
        self.bb.get_bytes(len)
    }

    pub(crate) fn get_string(&mut self, offset: u64) -> &str {
        let bytes = self.get_bytes(offset);
        std::str::from_utf8(bytes).unwrap()
    }

    pub(crate) fn set_int(&mut self, offset: u64, value: u64) {
        self.bb.set_int_with_offset(offset, value);
    }

    pub(crate) fn set_bytes(&mut self, offset: u64, value: &[u8]) {
        self.bb.position = offset;
        self.bb.set_int(value.len() as u64);
        self.bb.set_bytes(value);
    }

    pub(crate) fn set_string(&mut self, offset: u64, value: &str) {
        self.set_bytes(offset, value.as_bytes());
    }

    pub(crate) fn max_length(strlen: u64) -> u64 {
        strlen + 8
    }
}