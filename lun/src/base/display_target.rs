use std::{
    io,
    io::{BufWriter, Write},
};

#[derive(Debug)]
pub enum DisplayTarget {
    Stdout(io::Stdout),
    Sink(io::Sink),
}

impl Default for DisplayTarget {
    #[cfg(not(test))]
    fn default() -> Self {
        Self::stdout()
    }

    #[cfg(test)]
    fn default() -> Self {
        Self::sink()
    }
}

impl Write for DisplayTarget {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Stdout(stdout) => stdout.write(buf),
            Self::Sink(sink) => sink.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Stdout(stdout) => stdout.flush(),
            Self::Sink(sink) => sink.flush(),
        }
    }
}

impl DisplayTarget {
    pub fn sink() -> Self {
        Self::Sink(io::sink())
    }

    pub fn sink_buffer() -> BufWriter<Self> {
        io::BufWriter::new(Self::sink())
    }

    pub fn sink_buffer_with_capacity(capacity: usize) -> BufWriter<Self> {
        io::BufWriter::with_capacity(capacity, Self::sink())
    }

    pub fn stdout() -> Self {
        Self::Stdout(io::stdout())
    }

    pub fn stdout_buffer() -> BufWriter<Self> {
        io::BufWriter::new(Self::stdout())
    }

    pub fn stdout_buffer_with_capacity(capacity: usize) -> BufWriter<Self> {
        io::BufWriter::with_capacity(capacity, Self::stdout())
    }

    pub fn buffer_with_capacity(capacity: usize) -> BufWriter<Self> {
        io::BufWriter::with_capacity(capacity, Self::default())
    }
}

pub type DisplayBuffer = BufWriter<DisplayTarget>;
