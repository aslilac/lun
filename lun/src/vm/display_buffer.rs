use std::{io, io::Write};

#[derive(Debug)]
pub struct DisplayBuffer(pub(super) Vec<u8>);

impl Default for DisplayBuffer {
    fn default() -> Self {
        Self(Vec::with_capacity(1024))
    }
}

impl DisplayBuffer {
    pub fn flush(&mut self) {
        let DisplayBuffer(buf) = self;
        let _ = io::stdout().lock().write(buf.as_slice());
        buf.clear();
    }

    pub fn clear(&mut self) {
        let DisplayBuffer(buf) = self;
        buf.clear();
    }

    pub fn push(&mut self, byte: u8) {
        {
            let should_flush = {
                let DisplayBuffer(buf) = self;
                buf.len() == buf.capacity()
            };

            if should_flush {
                self.flush();
            }
        }

        let DisplayBuffer(buf) = self;
        buf.push(byte);
    }
}
