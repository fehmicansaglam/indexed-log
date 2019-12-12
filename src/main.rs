extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::OpenOptions;
use std::io::Result;
use std::path::Path;

mod lib;

fn main() -> Result<()> {
    let path = Path::new("./.indexed-log.db");
    let mut file = OpenOptions::new()
        .create(true)
        .read(true)
        .append(true)
        .open(path)?;

    lib::read_all(&mut file);

    let rand_key: String = thread_rng().sample_iter(&Alphanumeric).take(11).collect();

    let rand_value: String = thread_rng().sample_iter(&Alphanumeric).take(19).collect();

    lib::append(&mut file, &rand_key, &rand_value)
}
