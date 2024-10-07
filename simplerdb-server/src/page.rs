
pub(crate) struct Page {
    block_size: u64,
    bytes: Vec<u8>
}

impl Page {
    pub(crate) fn get_int(&self, offset: u64) -> i64 {
        unimplemented!()
    }

    pub(crate) fn get_bytes(&self, offset: u64) -> &[u8] {
        unimplemented!()
    }

    pub(crate) fn get_string(&self, offset: u64) -> &str {
        unimplemented!()
    }

    pub(crate) fn set_int(&mut self, offset: u64, value: i64) {
        unimplemented!()
    }

    pub(crate) fn set_bytes(&mut self, offset: u64, value: &[u8]) {
        unimplemented!()
    }

    pub(crate) fn set_string(&mut self, offset: u64, value: &str) {
        unimplemented!()
    }

    pub(crate) fn max_length(&self, strlen: u64) -> u64 {
        unimplemented!()
    }
}