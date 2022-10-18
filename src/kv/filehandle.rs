use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

use anyhow::Result;

pub struct FileHandle {
    rhandle: BufReaderWithPos<File>,
    whandle: BufWriterWithPos<File>,
}

impl FileHandle {
    pub fn new<P: AsRef<Path>>(file_path: P) -> Result<Self> {
        let rhandle = BufReaderWithPos::new(File::open(&file_path)?)?;
        let whandle = BufWriterWithPos::new(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&file_path)?,
        )?;
        Ok(Self { rhandle, whandle })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<usize> {
        Ok(self.whandle.write(data)?)
    }

    pub fn read(&mut self, pos: u64, buf: &mut [u8]) -> Result<usize> {
        self.rhandle.seek(SeekFrom::Start(pos));
        let len = self.rhandle.read(buf)?;
        Ok(len)
    }

    pub fn whandle_pos(&self) -> u64 {
        self.whandle.pos
    }

    pub fn rhandle_pos(&self) -> u64 {
        self.rhandle.pos
    }
}

pub struct BufReaderWithPos<R: Read + Seek> {
    reader: BufReader<R>,
    pos: u64,
}

impl<R: Read + Seek> BufReaderWithPos<R> {
    fn new(mut inner: R) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufReaderWithPos {
            reader: BufReader::new(inner),
            pos,
        })
    }
}

impl<R: Read + Seek> Read for BufReaderWithPos<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(buf)?;
        self.pos += len as u64;
        Ok(len)
    }
}

impl<R: Read + Seek> Seek for BufReaderWithPos<R> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.reader.seek(pos)?;
        Ok(self.pos)
    }
}

pub struct BufWriterWithPos<W: Write + Seek> {
    writer: BufWriter<W>,
    pos: u64,
}

impl<W: Write + Seek> BufWriterWithPos<W> {
    fn new(mut inner: W) -> Result<Self> {
        let pos = inner.seek(SeekFrom::Current(0))?;
        Ok(BufWriterWithPos {
            writer: BufWriter::new(inner),
            pos,
        })
    }
}

impl<W: Write + Seek> Write for BufWriterWithPos<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let len = self.writer.write(buf)?;
        self.pos += len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write + Seek> Seek for BufWriterWithPos<W> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.pos = self.writer.seek(pos)?;
        Ok(self.pos)
    }
}
