use std::fmt::Display;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Cursor, Read, Seek, SeekFrom, Write};
use std::sync::{Arc, RwLock};

use super::filehandle::FileHandle;
use crate::utils::crc32::{extend, hash};
use anyhow::{anyhow, Result};

pub struct DataFile {
    fd: FileHandle,
    readonly: bool,
}

impl DataFile {
    pub fn new(file_path: &str, readonly: bool) -> Result<Self> {
        let fd = FileHandle::new(file_path)?;
        Ok(Self { fd, readonly })
    }

    pub fn set_read_only(&mut self, readonly: bool) {
        self.readonly = readonly;
    }

    pub fn write(&mut self, key: &[u8], value: &[u8], timestamp: u128) -> Result<usize> {
        if self.readonly {
            return Err(anyhow!("Write to a read only datafile."));
        }
        let entry = Entry::new(key, value, timestamp);
        self.fd.write(&entry.to_bytes())
    }

    pub fn read(&mut self, offset: u64, len: usize) -> Result<Entry> {
        let mut buf = Vec::with_capacity(len);
        if self.fd.read(offset, &mut buf)? != len {
            return Err(anyhow!("Wrong data length."));
        }
        Entry::from_bytes(&mut buf)
    }
}

#[derive(Debug)]
pub struct Entry {
    pub crc: u32,
    pub timestamp: u128,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
}

impl Entry {
    pub fn new(key: &[u8], value: &[u8], timestamp: u128) -> Self {
        let mut crc = hash(&timestamp.to_be_bytes());
        crc = extend(crc, key);
        crc = extend(crc, value);

        Self {
            crc,
            timestamp,
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let key_len = self.key.len() as u64;
        let value_len = self.value.len() as u64;
        bytes.extend_from_slice(&self.crc.to_be_bytes());
        bytes.extend_from_slice(&self.timestamp.to_be_bytes());
        bytes.extend_from_slice(&key_len.to_be_bytes());
        bytes.extend_from_slice(&value_len.to_be_bytes());
        bytes.extend_from_slice(&self.key);
        bytes.extend_from_slice(&self.value);
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);
        let mut crc_buf = [0u8; 4];
        cursor.read_exact(&mut crc_buf)?;
        let crc = u32::from_be_bytes(crc_buf);
        if crc != hash(&bytes[4..]) {
            return Err(anyhow!("crc check sum error"));
        }
        let mut timestamp_buf = [0u8; 16];
        cursor.read_exact(&mut timestamp_buf)?;
        let timestamp = u128::from_be_bytes(timestamp_buf);
        let mut key_len_buf = [0u8; 8];
        cursor.read_exact(&mut key_len_buf)?;
        let key_len = u64::from_be_bytes(key_len_buf);
        let mut value_len_buf = [0u8; 8];
        cursor.read_exact(&mut value_len_buf)?;
        let value_len = u64::from_be_bytes(value_len_buf);
        let mut key_buf = Vec::with_capacity(key_len as usize);
        let mut value_buf = Vec::with_capacity(value_len as usize);
        cursor.read_exact(&mut key_buf)?;
        cursor.read_exact(&mut value_buf)?;

        Ok(Self {
            crc,
            timestamp,
            key: key_buf,
            value: value_buf,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_datafile() -> Result<()> {
        let mut datafile = DataFile::new("test.txt", false)?;

        Ok(())
    }
}
