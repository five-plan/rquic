// MIT License
//
// Copyright (c) 2020 five-plan
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

/// codec mod contain the encoder and encoder

#[allow(unused)]
pub struct Encoder {
    buf: Vec<u8>,
}

#[allow(unused)]
impl Encoder {
    /// Encoder encode the num using variable length encoding
    pub fn new() -> Encoder {
        Encoder { buf: Vec::new() }
    }
    pub fn encode_data(&mut self, data: &[u8]) {
        self.buf.extend_from_slice(data)
    }
    pub fn encode_byte(&mut self, data: u8) {
        self.buf.push(data)
    }
    pub fn encode_uint<T: Into<u64>>(&mut self, n: usize, v: T) {
        let v = v.into();
        for i in 0..n {
            self.encode_byte(((v >> ((n - i - 1) * 8)) & 0xff_u64) as u8)
        }
    }
    pub fn encode_varint<T: Into<u64>>(&mut self, v: T) {
        let v = v.into();
        match () {
            _ if v < (1 << 6) => self.encode_uint(1, v),
            _ if v < (1 << 14) => self.encode_uint(2, v),
            _ if v < (1 << 30) => self.encode_uint(4, v),
            _ if v < (1 << 62) => self.encode_uint(8, v),
            _ => panic!("variable length value is too large"),
        }
    }
    pub fn raw(&self) -> Vec<u8> {
        self.buf.clone()
    }
}

#[allow(unused)]
pub struct Decoder<'a> {
    buf: &'a [u8],
    offset: usize,
}

#[allow(unused)]
impl<'a> Decoder<'a> {
    pub fn new(buf: &[u8]) -> Decoder {
        Decoder { buf, offset: 0 }
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
            0 => Some(self.decode_uint(1)? | 0x3f_u64),
            1 => Some(self.decode_uint(2)? & 0x3fff_u64),
            2 => Some(self.decode_uint(4)? & 0x3fff_ffff_u64),
            3 => Some(self.decode_uint(8)? & 0x3fff_ffff_ffff_ffff_u64),
            _ => panic!("can not reach"),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::cmp::min;

    #[test]
    fn test_encode_data() {
        let mut origin: [u8; 0x100] = [0; 0x100];
        for i in 0..0xff {
            origin[i] = i as u8;
        }
        let mut encoder = Encoder::new();
        encoder.encode_data(&origin);
        assert_eq!(&origin[..], &encoder.raw()[..]);
    }

    #[test]
    fn test_encode_byte() {
        let mut encoder = Encoder::new();
        for i in 0..0xff {
            encoder.encode_byte(i)
        }
        let buf = encoder.raw();
        for i in 0..0xff {
            assert_eq!(i, buf[i] as usize)
        }
    }

    #[test]
    fn test_encode_uint() {
        let small = 0x1_u8;
        let mid = 0x0203_u16;
        let big = 0x0405_0607_u32;
        let bigger = 0x0809_0a0b_0c0d_0e0f_u64;
        let mut encoder = Encoder::new();
        encoder.encode_uint(1, small);
        encoder.encode_uint(2, mid);
        encoder.encode_uint(4, big);
        encoder.encode_uint(8, bigger);
        let res = encoder.raw();
        for i in 1..(min(0x10, res.len() + 1)) {
            assert_eq!(i, res[i - 1] as usize)
        }
    }

    #[test]
    fn test_encode_varint() {}
}
