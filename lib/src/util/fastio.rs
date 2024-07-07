
use std::io::prelude::*;
use std::io::StdinLock;
const BUFFER_SIZE: usize = 1 << 20;
const CH_0: u8 = b'0' as u8;
const CH_9: u8 = b'9' as u8;

pub trait Reader<T> {
    fn read(&mut self) -> T;
}
pub struct IO<'a> {
    reader: StdinLock<'a>,
    buffer: [u8; BUFFER_SIZE],
    index: usize,
    writer: std::io::BufWriter<std::io::StdoutLock<'a>>,
}
impl<'a> IO<'a> {
    pub fn new() -> Self {
        let stdin = std::io::stdin();
        let stdout = std::io::stdout();
        let writer = std::io::BufWriter::new(stdout.lock());
        let io = IO {
            reader: stdin.lock(),
            buffer: [0; BUFFER_SIZE],
            index: BUFFER_SIZE,
            writer,
        };
        return io;
    }
    fn get_any_char(&mut self) -> u8 {
        if self.index == self.buffer.len() {
            self.reader.read(&mut self.buffer).unwrap();
            self.index = 0;
        }
        let ch = self.buffer[self.index];
        self.index += 1;
        return ch;
    }
    fn get_notempty_char(&mut self) -> u8 {
        let mut ch = self.get_any_char();
        while ch == b' ' || ch == b'\n' || ch == b'\r' || ch == b'\t' {
            ch = self.get_any_char();
        }
        return ch;
    }
    fn get_integer_first_char(&mut self) -> (bool, u8) {
        let mut ch = self.get_notempty_char();
        let is_negative = ch == b'-';
        if is_negative {
            ch = self.get_any_char();
        }
        return (is_negative, ch);
    }
    pub fn write<T: std::fmt::Display>(&mut self, x: T) {
        writeln!(self.writer, "{}", x).unwrap();
    }
}
impl Reader<usize> for IO<'_> {
    fn read(&mut self) -> usize {
        let mut ch = self.get_notempty_char();
        let mut x = 0;
        while CH_0 <= ch && ch <= CH_9 {
            x = x * 10 + (ch - CH_0) as usize;
            ch = self.get_any_char();
        }
        return x;
    }
}
impl Reader<i32> for IO<'_> {
    fn read(&mut self) -> i32 {
        let (is_negative, mut ch) = self.get_integer_first_char();
        let mut x: i32 = 0;
        while CH_0 <= ch && ch <= CH_9 {
            x = x * 10 + (ch - CH_0) as i32;
            ch = self.get_any_char();
        }
        return if is_negative { -x } else { x };
    }
}
impl Reader<i128> for IO<'_> {
    fn read(&mut self) -> i128 {
        let (is_negative, mut ch) = self.get_integer_first_char();
        let mut x: i128 = 0;
        while CH_0 <= ch && ch <= CH_9 {
            x = x * 10 + (ch - CH_0) as i128;
            ch = self.get_any_char();
        }
        return if is_negative { -x } else { x };
    }
}
