use std::fs::File;
use std::io::{Read, Result, Seek, SeekFrom, Write};

pub fn append(file: &mut File, key: &str, value: &str) -> Result<()> {
    let key_bytes: [u8; 1] = (key.len() as u8).to_be_bytes();
    let value_bytes: [u8; 2] = (value.len() as u16).to_be_bytes();

    let mut buf: Vec<u8> = vec![key_bytes[0], value_bytes[0], value_bytes[1]];
    buf.append(&mut [key.as_bytes(), value.as_bytes()].concat());

    file.write_all(&buf)
}

pub fn read_key_value(file: &mut File) -> Result<(String, String)> {
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

pub fn read_all(file: &mut File) {
    file.seek(SeekFrom::Start(0));

    loop {
        match read_key_value(file) {
            Err(why) => {
                println!("Reached EOF: {}", why);
                break;
            }
            Ok((key, value)) => println!("{} => {}", key, value),
        }
    }
}
