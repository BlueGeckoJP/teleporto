use std::{
    fs::File,
    io::{BufReader, Read},
};

pub fn readfile(path: String) -> Vec<u8> {
    let f = File::open(path).expect("Failed to read file.");
    let mut reader = BufReader::new(f);
    let mut content = vec![];
    reader
        .read_to_end(&mut content)
        .expect("Something went wrong reading the file.");
    content
}
