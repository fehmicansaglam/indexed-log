use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::Path;

fn append(file: &mut File, key: &str, value: &str) {
    let key_len = key.len();
    let key_bytes = (key_len as u8).to_be_bytes();
    let value_len = value.len();
    let value_bytes = (value_len as u16).to_be_bytes();

    let mut buf = vec![key_bytes[0], value_bytes[0], value_bytes[1]];
    buf.append(&mut [key.as_bytes(), value.as_bytes()].concat());

    file.write_all(&buf);
}

fn read_key_value(file: &mut File) -> String {
    let mut len_buf = [0u8; 3];
    file.read_exact(&mut len_buf);

    let key_len = len_buf[0] as usize;
    let value_len = u16::from_be_bytes([len_buf[1], len_buf[2]]) as usize;
    let len = key_len + value_len;

    println!("{} + {} = {}", key_len, value_len, len);

    let mut buf = vec![0u8; len as usize];
    file.read_exact(&mut buf);

    String::from_utf8(buf).unwrap()
}

fn main() {
    let path = Path::new("./.indexed-log.db");
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();

    println!("{}", read_key_value(&mut file));

    append(&mut file, "test_key", "test_value");
}
