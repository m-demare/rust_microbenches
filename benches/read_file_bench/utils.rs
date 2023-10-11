use core::str::from_utf8;
use criterion::black_box;
use std::{fs::File, io::{self, Read}, str::Chars};

macro_rules! read_with_buf {
    ($reader: expr, $size: expr, $by_chars: expr) => ({
        const MAX_PENDING_BYTES: usize = 4;
        let mut reader = $reader;
        let mut pending_bytes = 0;  // In case the buf is trimming a utf8 char
                                    // (I'm not sure this is 100% right, needs some testing)
        let mut u = [0u8; $size + MAX_PENDING_BYTES];
        
        loop {
            for i in 0..pending_bytes {
                u[MAX_PENDING_BYTES - i]  = u[u.len() - i];
            }
            let n = reader.read(&mut u[MAX_PENDING_BYTES .. ])?;
            if n==0 {break;}
            let mut res = from_utf8(&u[MAX_PENDING_BYTES - pending_bytes .. n + MAX_PENDING_BYTES]);
            if let Err(err) = res {
                let new_pending_bytes = u.len() - err.valid_up_to();
                if new_pending_bytes <= MAX_PENDING_BYTES {
                    res = from_utf8(&u[MAX_PENDING_BYTES - pending_bytes .. err.valid_up_to() + MAX_PENDING_BYTES]);
                    pending_bytes = new_pending_bytes;
                }
            }

            if($by_chars)
                { consume(res.unwrap().chars()); }
            else
                { black_box(&res.unwrap()); }
        }
        Ok(())
    });
    ($reader: expr, $size: expr) => ({
        read_with_buf!($reader, $size, false)
    })
}

const MAX_PENDING_BYTES: usize = 4;

pub struct CharReader1 {
    reader: File,
    buf: Box<[u8]>,
    chars: Chars<'static>,
    pending_bytes: usize,
}

// Converts to &str, then iterates over it
impl CharReader1 {
    pub fn new(reader: File) -> Self {
        Self::with_capacity(2usize.pow(14), reader)
    }

    pub fn with_capacity(capacity: usize, reader: File) -> Self {
        Self { reader, buf: vec![0u8; capacity].into_boxed_slice(), chars: "".chars(), pending_bytes: 0 }
    }
}


impl Iterator for CharReader1 {
    type Item = Result<char, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.chars.next();
        if next.is_some() { return Ok(next).transpose(); }

        let n = match self.reader.read(&mut self.buf[MAX_PENDING_BYTES .. ]) {
            Ok(n) => n,
            Err(_) => return Some(Err(())),
        };
        if n==0 { return None; }
        
        let mut res = from_utf8(&self.buf[MAX_PENDING_BYTES - self.pending_bytes .. n + MAX_PENDING_BYTES]);
        if let Err(err) = res {
            let new_pending_bytes = self.buf.len() - err.valid_up_to();
            if new_pending_bytes <= MAX_PENDING_BYTES {
                res = from_utf8(&self.buf[MAX_PENDING_BYTES - self.pending_bytes .. err.valid_up_to() + MAX_PENDING_BYTES]);
                self.pending_bytes = new_pending_bytes;
            }
        }
        match res {
            Ok(s) => self.chars = unsafe { std::mem::transmute(s.chars()) },
            Err(_) => return Some(Err(())),
        }
        Ok(self.chars.next()).transpose()
    }
}

// Iterates over u8, creates chars from them
pub struct CharReader2 {
    reader: File,
    buf: Box<[u8]>,
    idx: usize,
    n: usize,
}

impl CharReader2 {
    pub fn new(reader: File) -> Self {
        Self::with_capacity(2usize.pow(14), reader)
    }

    pub fn with_capacity(capacity: usize, reader: File) -> Self {
        Self { reader, buf: vec![0u8; capacity].into_boxed_slice(), idx: 0, n: 0 }
    }

    fn read_into_buf(&mut self) -> Result<(), io::Error> {
        self.n = self.reader.read(&mut self.buf)?;
        Ok(())
    }
}

impl Iterator for CharReader2 {
    type Item = Result<char, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n {
            match self.read_into_buf() {
                Ok(..) => if self.n == 0 { return None; },
                Err(..) => return Some(Err(())),
            }
            self.idx = 0;
        }
        if self.buf[self.idx] < 128 {
            self.idx += 1;
            // Didn't notice too much of a speed difference between char::from_u32_unchecked and char::from_u32
            return unsafe { Some(Ok(char::from_u32_unchecked(self.buf[self.idx-1].into()))) }
        }
        todo!("Chars longer than 1 byte");
    }
}

pub struct ByteReader {
    reader: File,
    buf: Box<[u8]>,
    idx: usize,
    n: usize,
}

impl ByteReader {
    pub fn new(reader: File) -> Self {
        Self::with_capacity(2usize.pow(14), reader)
    }

    pub fn with_capacity(capacity: usize, reader: File) -> Self {
        Self { reader, buf: vec![0u8; capacity].into_boxed_slice(), idx: 0, n: 0 }
    }

    fn read_into_buf(&mut self) -> Result<(), io::Error> {
        self.n = self.reader.read(&mut self.buf)?;
        Ok(())
    }
}

impl Iterator for ByteReader {
    type Item = Result<u8, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.n {
            match self.read_into_buf() {
                Ok(..) => if self.n == 0 { return None; },
                Err(..) => return Some(Err(())),
            }
            self.idx = 0;
        }
        self.idx += 1;
        Some(Ok(self.buf[self.idx - 1]))
    }
}

pub fn consume<T: Iterator> (it: T) {
    for ch in it {
        black_box(ch);
    }
}

pub(super) use read_with_buf;

