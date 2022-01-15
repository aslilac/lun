use std::{io, io::Write};

#[derive(Debug)]
pub struct WriteBuffer(pub(super) Vec<u8>);

impl Default for WriteBuffer {
    fn default() -> Self {
        Self(Vec::with_capacity(1024))
    }
}

impl WriteBuffer {
    pub fn buf(&self) -> &Vec<u8> {
        let WriteBuffer(buf) = self;
        buf
    }

    pub fn mbuf(&mut self) -> &mut Vec<u8> {
        let WriteBuffer(buf) = self;
        buf
    }

    pub fn clear(&mut self) {
        self.mbuf().clear();
    }

    pub fn flush(&mut self) {
        let _ = io::stdout().lock().write(self.buf().as_slice());
        self.clear();
    }

    pub fn push(&mut self, byte: u8) {
        {
            let should_flush = {
                let buf = self.buf();
                buf.len() == buf.capacity()
            };

            if should_flush {
                self.flush();
            }
        }

        self.mbuf().push(byte);
    }
}
