use std::{fs::File, io::Read};

pub fn readfile(path: String) -> Vec<u8> {
    let mut f = File::open(path).expect("Failed to read file.");
    let mut content = vec![];
    f.read_to_end(&mut content)
        .expect("Something went wrong reading the file.");
    content
}
