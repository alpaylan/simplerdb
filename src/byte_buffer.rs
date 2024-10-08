use std::fmt::Debug;

pub(crate) struct ByteBuffer {
    bytes: Vec<u8>,
    pub(crate) position: u64,
}

impl Debug for ByteBuffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(
            format_args!(
                "ByteBuffer[{}]\n{}\n\n",
                self.position, 
                // Group bits in 8 bytes per line, write offsets to the left
                self.bytes
                    .chunks(8)
                    .map(|chunk| chunk.iter().map(|b| format!("{:03}", b)).collect::<Vec<String>>().join(" "))
                    .enumerate()
                    .map(|(i, line)| format!("{:5} {}", i * 8, line))
                    .collect::<Vec<String>>()
                    .join("\n")
            )
        )
    }
}

impl ByteBuffer {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        Self { bytes, position: 0 }
    }

    pub(crate) fn contents(&mut self) -> &mut [u8] {
        self.position = 0;
        &mut self.bytes
    }

    pub(crate) fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl ByteBuffer {
    pub(crate) fn get_int(&mut self) -> u64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.bytes[self.position as usize..self.position as usize + 8]);
        self.position += 8;
        u64::from_be_bytes(bytes)
    }

    pub(crate) fn get_int_with_offset(&mut self, offset: u64) -> u64 {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(&self.bytes[offset as usize..offset as usize + 8]);
        self.position = offset + 8;
        u64::from_be_bytes(bytes)
    }

    pub(crate) fn get_bytes(&mut self, len: u64) -> &[u8] {
        let result = &self.bytes[self.position as usize..(self.position + len) as usize];
        self.position += len;
        result
    }

    pub(crate) fn get_bytes_with_offset(&mut self, offset: u64, len: u64) -> &[u8] {
        self.position = offset + len;
        &self.bytes[offset as usize..(offset + len) as usize]
    }

    pub(crate) fn set_int_with_offset(&mut self, offset: u64, value: u64) {
        self.bytes[offset as usize..offset as usize + 8].copy_from_slice(&value.to_be_bytes());
        self.position = offset + 8;
    }

    pub(crate) fn set_bytes_with_offset(&mut self, offset: u64, value: &[u8]) {
        self.bytes[offset as usize..(offset + value.len() as u64) as usize].copy_from_slice(value);
        self.position = offset + value.len() as u64;
    }

    pub(crate) fn set_int(&mut self, value: u64) {
        self.bytes[self.position as usize..self.position as usize + 8]
            .copy_from_slice(&value.to_be_bytes());
        self.position = self.position + 8;
    }

    pub(crate) fn set_bytes(&mut self, value: &[u8]) {
        self.bytes[self.position as usize..(self.position + value.len() as u64) as usize]
            .copy_from_slice(value);
        println!("Setting bytes between {:?} and {:?}", self.position, self.position + value.len() as u64);
        self.position = self.position + value.len() as u64;
    }
}
