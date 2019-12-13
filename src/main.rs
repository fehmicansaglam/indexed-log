extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::lib::IndexedLog;

mod lib;

fn main() {
    let mut indexed_log = IndexedLog::new();

    indexed_log.read_all();

    let (rand_key, rand_value) = random_key_value();
    indexed_log.append(&rand_key, &rand_value);

    let (rand_key, rand_value) = random_key_value();
    indexed_log.append(&rand_key, &rand_value);

    let (rand_key, rand_value) = random_key_value();
    indexed_log.append(&rand_key, &rand_value);

    indexed_log.print_all();
    indexed_log.read_all();
}

fn random_key_value() -> (String, String) {
    let rand_key: String = thread_rng().sample_iter(&Alphanumeric).take(11).collect();
    let rand_value: String = thread_rng().sample_iter(&Alphanumeric).take(19).collect();
    (rand_key, rand_value)
}
