use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Result, Seek, SeekFrom, Write};
use std::path::Path;

pub struct IndexedLog<'a> {
    index: HashMap<&'a str, u64>,
    file: File,
}

impl<'a> IndexedLog<'a> {
    pub fn new() -> Self {
        let path = Path::new("./.indexed-log.db");
        let file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)
            .unwrap();

        IndexedLog {
            index: HashMap::new(),
            file,
        }
    }

    pub fn append(&mut self, key: &'a str, value: &'a str) {
        let position = append_to_file(&mut self.file, key, value).unwrap();
        self.index.insert(key, position);
    }

    pub fn print_all(&self) {
        for (key, &offset) in self.index.iter() {
            println!("{}: {}", key, offset);
        }
    }

    pub fn read_all(&mut self) {
        self.file.seek(SeekFrom::Start(0));

        println!("Current db file state");
        println!("---------------------");
        loop {
            match read_key_value(&mut self.file) {
                Err(why) => {
                    println!("Reached EOF: {}", why);
                    break;
                }
                Ok((key, value)) => println!("{} => {}", key, value),
            }
        }
        println!("---------------------");
    }
}

fn append_to_file(file: &mut File, key: &str, value: &str) -> Result<u64> {
    let key_bytes: [u8; 1] = (key.len() as u8).to_be_bytes();
    let value_bytes: [u8; 2] = (value.len() as u16).to_be_bytes();

    let mut buf: Vec<u8> = vec![key_bytes[0], value_bytes[0], value_bytes[1]];
    buf.append(&mut [key.as_bytes(), value.as_bytes()].concat());

    let position = file.seek(SeekFrom::Current(0));
    file.write_all(&buf);
    position
}

fn read_key_value(file: &mut File) -> Result<(String, String)> {
    let mut len_buf = [0u8; 3];
    file.read_exact(&mut len_buf)?;

    let key_len = len_buf[0] as usize;
    let value_len = u16::from_be_bytes([len_buf[1], len_buf[2]]) as usize;
    let len = key_len + value_len;

    let mut buf = vec![0u8; len as usize];
    file.read_exact(&mut buf)?;

    Ok((
        String::from_utf8_lossy(&buf[0..key_len]).into_owned(),
        String::from_utf8_lossy(&buf[key_len..(len)]).into_owned(),
    ))
}
