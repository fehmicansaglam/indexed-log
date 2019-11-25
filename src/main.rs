use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::Path;

fn append(file: &mut File, key: &str, value: &str) {
    let key_len = key.len() as u8;
    let value_len = value.len() as u16;
    file.write_all(&key_len.to_le_bytes());
    file.write_all(key.as_bytes());
    file.write_all(&value_len.to_le_bytes());
    file.write_all(value.as_bytes());
}

fn read_key(file: &mut File) -> String {
    let mut buf = [0u8; 1];
    String::from_utf8(read(file, &mut buf)).unwrap()
}

fn read_value(file: &mut File) -> String {
    let mut buf = [0u8; 2];
    String::from_utf8(read(file, &mut buf)).unwrap()
}

fn read(file: &mut File, len_buf: &mut [u8]) -> Vec<u8> {
    file.read_exact(len_buf);
    let len = len_buf[0];
    let mut buf = vec![0u8; len as usize];
    file.read_exact(&mut buf);
    return buf;
}

fn main() {
    let path = Path::new("./.indexed-log.db");
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();

    println!("{} => {}", read_key(&mut file), read_value(&mut file));

    append(&mut file, "test_key", "test_value");
}
