use std::{fs::{self, File}, path::Path, io::{self, prelude::*} };

pub fn encode(path: &Path) { 
    
    let mut _data = "".to_string();
    
    let mut file = File::open(path).expect("Can't open a file");
    let mut file_content: Vec<_> = Vec::new();
    file.read_to_end(&mut file_content).unwrap();

    for bytes in &mut file_content {
        let length: u32  = bytes.to_string().parse().unwrap();
        let mut i = 0;
        
        loop { 
                if i != length {
                _data.push_str(" ");
                i += 1;
                
                } else {
                _data.push_str("\n");
                break;
                }

            }
    } 
    save(path, _data);
}



pub fn decode(path: &Path) {
   
    let mut _data  = "".to_string();

    let mut file = File::open(path).expect("cannot open the file");
    let mut file_content: Vec<_> = Vec::new();
    file.read_to_end(&mut file_content).unwrap();

    let mut length: u8 = 0;
    for bytes in &mut file_content {

        let encoded_string = *bytes as char; 

            if encoded_string.is_ascii_whitespace() && encoded_string != '\n' {
            length += 1;
            } else {
                
                let value = length as char;
                let value = value.to_string();

                _data.push_str(&value);
                
                length = 0;
                continue;
            }
    }

    save(path, _data);
}

pub fn save(path: &Path, data: String) {

    let data = data.as_bytes();
    let mut file = fs::File::create(path).unwrap();
    file.write_all(&data).expect("Cannot write the data");

}


