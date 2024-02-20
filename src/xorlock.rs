     
use std::{io::{self, prelude::*}, path::Path, fs::File};
    // XoRing the data present in file
pub fn xor(path: &Path, key: u8) {
    let mut file = File::open(path).expect("Cannot open the file");

    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).unwrap();

    for bytes in &mut file_content {
        *bytes ^= key;
    } 

    let mut file = File::create(path).expect("Cannot create a file");
    file.write_all(&file_content).unwrap();

}
