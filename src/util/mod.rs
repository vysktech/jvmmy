pub struct ByteReader {
    pub index: usize,
    pub contents: Vec<u8>,
}

impl ByteReader {
    pub const fn new(contents: Vec<u8>) -> Self {
        ByteReader { index: 0, contents }
    }

    pub fn read_u8(&mut self) -> u8 {
        let result: u8 = self.contents[self.index];
        self.index += 1;
        result
    }

    pub fn read_u16(&mut self) -> u16 {
        let result: u16 = (self.contents[self.index] as u16) << 8
            | (self.contents[self.index + 1] as u16) << 0;
        self.index += 2;
        result
    }

    pub fn read_u32(&mut self) -> u32 {
        let result: u32 = (self.contents[self.index] as u32) << 24
            | (self.contents[self.index + 1] as u32) << 16
            | (self.contents[self.index + 2] as u32) << 8
            | (self.contents[self.index + 3] as u32) << 0;
        self.index += 4;
        result
    }

    pub fn read_str(&mut self, length: usize) -> &str {
        let range = self.index..self.index + length;
        let result: &str = std::str::from_utf8(&self.contents[range])
            .expect("Invalid UTF-8 value");
        self.index += length;
        result
    }

    pub fn read_vec_u8(&mut self, length: usize) -> Vec<u8> {
        let range = self.index..self.index + length;
        let result: Vec<u8> = Vec::from(&self.contents[range]);
        self.index += length;
        result
    }
}
