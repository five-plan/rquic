/// codec mod contain the encoder and encoder
pub struct Encoder {
    buf: Vec<u8>,
}

impl Encoder {
    /// Encoder encode the num using variable length encoding
    pub fn encode_data(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data)
    }
    pub fn encode_byte(&mut self, data: u8) {
        self.buf.push(data)
    }
    pub fn encode_uint<T: Into<u64>>(&mut self, n: usize, v: T) {
        let v = v.into();
        for i in 0..n {
            self.encode_byte(((v >> (n - i - 1) * 8) & 0xff) as u8)
        }
    }
    pub fn encode_variable<T: Into<u64>>(&mut self, v: T) {
        let v = v.into();
        match () {
            _ if v < (1 << 6) => self.encode_uint(1, v),
            _ if v < (1 << 14) => self.encode_uint(2, v),
            _ if v < (1 << 30) => self.encode_uint(4, v),
            _ if v < (1 << 62) => self.encode_uint(8, v),
            _ => panic!("variable length value is too large"),
        }
    }
}

pub struct Decoder<'a> {
    buf: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(buf: &[u8]) -> Decoder {
        return Decoder { buf, offset: 0 };
    }
    pub fn remain(&self) -> usize {
        self.buf.len() - self.offset
    }
    pub fn empty(&self) -> bool {
        self.remain() == 0
    }
    pub fn peek_byte(&self) -> Option<u8> {
        if self.empty() {
            return None;
        }
        let b = self.buf[self.offset];
        Some(b)
    }
    pub fn decode(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.remain() < n {
            return None;
        }
        let b = &self.buf[self.offset..self.offset + n];
        self.offset += 1;
        Some(b)
    }
    pub fn decode_uint(&mut self, n: usize) -> Option<u64> {
        if self.remain() < n {
            return None;
        }
        let mut res = 0_u64;
        for i in 0..n {
            res = res << 8 | u64::from(self.buf[self.offset + i])
        }
        Some(res)
    }
    pub fn decode_varint(&mut self) -> Option<u64> {
        let peek = match self.peek_byte() {
            Some(b) => b,
            None => return None,
        };

        match peek >> 6 {
            0 => return Some(self.decode_uint(1)? | 0x3f_u64),
            1 => return Some(self.decode_uint(2)? & 0x3fff_u64),
            2 => return Some(self.decode_uint(4)? & 0x3fffffff_u64),
            3 => return Some(self.decode_uint(8)? & 0x3fffffffffffffff_u64),
            _ => panic!("can not reach"),
        }
    }
}
