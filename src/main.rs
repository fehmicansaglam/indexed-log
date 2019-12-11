use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};
use std::path::Path;

fn append(file: &mut File, key: &str, value: &str) -> Result<()> {
    let key_bytes: [u8; 1] = (key.len() as u8).to_be_bytes();
    let value_bytes: [u8; 2] = (value.len() as u16).to_be_bytes();

    let mut buf: Vec<u8> = vec![key_bytes[0], value_bytes[0], value_bytes[1]];
    buf.append(&mut [key.as_bytes(), value.as_bytes()].concat());

    file.write_all(&buf)
}

fn read_key_value(file: &mut File) -> Result<(String, String)> {
    let mut len_buf = [0u8; 3];
    file.read_exact(&mut len_buf)?;

    let key_len = len_buf[0] as usize;
    let value_len = u16::from_be_bytes([len_buf[1], len_buf[2]]) as usize;
    let len = key_len + value_len;

    println!("{} + {} = {}", key_len, value_len, len);

    let mut buf = vec![0u8; len as usize];
    file.read_exact(&mut buf)?;

    Ok((
        String::from_utf8_lossy(&buf[0..key_len]).into_owned(),
        String::from_utf8_lossy(&buf[key_len..(len)]).into_owned(),
    ))
}

fn main() {
    let path = Path::new("./.indexed-log.db");
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)
        .unwrap();

    match read_key_value(&mut file) {
        Err(why) => println!("{}", why),
        Ok((key, value)) => println!("{} => {}", key, value),
    }

    append(&mut file, "test_key", "test_value");
}
