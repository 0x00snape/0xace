use std::{ fs::{self, File}, path::Path, io::prelude::* ,process::exit };
use walkdir::WalkDir;


// Banner
pub fn banner() -> String {
    format!("
         ▒█████  ▒██   ██▒ ▄▄▄       ▄████▄  ▓█████
        ▒██▒  ██▒▒▒ █ █ ▒░▒████▄    ▒██▀ ▀█  ▓█   ▀
        ▒██░  ██▒░░  █   ░▒██  ▀█▄  ▒▓█    ▄ ▒███  
        ▒██   ██░ ░ █ █ ▒ ░██▄▄▄▄██▒▒▓▓▄ ▄██ ▒▓█  ▄
        ░ ████▓▒░▒██▒ ▒██▒▒▓█   ▓██░▒ ▓███▀ ▒░▒████
        ░ ▒░▒░▒░ ▒▒ ░ ░▓ ░░▒▒   ▓▒█░░ ░▒ ▒  ░░░ ▒░ 
          ░ ▒ ▒░ ░░   ░▒ ░░ ░   ▒▒    ░  ▒  ░ ░ ░  
        ░ ░ ░ ▒   ░    ░    ░   ▒   ░           ░  
            ░ ░   ░    ░        ░   ░ ░     ░   ░      
                                                v(0.1) -> snape
        ")       
}



// Checking the File/Directories and returning the value
pub fn check(path: String) -> String {

    let path = Path::new(&path);
    let mut recursive = "".to_string();

    match fs::metadata(path) {

        Ok(_) => { 
                    if path.is_file() == true {
                        path.display().to_string()
                    }else{
                            for file in WalkDir::new(&path).into_iter().filter_map(|file| file.ok()) {
                                if file.metadata().unwrap().is_file() {
                                    recursive.push_str(file.path().display().to_string().as_str());
                                    recursive.push_str("\n");
                                }
                            }
                            recursive
                        }

                 },

        Err(_) => {
                    println!("[*] File/Path {} doesn't exist !!!\nExample:\nPath:> /src/test.txt\nPath:> /src/test/", path.display().to_string());
                    exit(1)
                },
    }
}



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



// Encoding the data with spaces(\x20) and newlines(\x0A)
pub fn encode(path: &Path) { 
    
    let mut data = "".to_string();
    
    let mut file = File::open(path).expect("Can't open a file");
    let mut file_content: Vec<_> = Vec::new();
    file.read_to_end(&mut file_content).unwrap();

    for bytes in &mut file_content {
        let length: u32  = bytes.to_string().parse().unwrap();
        let mut i = 0;
        
        loop { 
                if i != length {
                    data.push_str(" ");
                    i += 1;
                
                } else {
                    data.push_str("\n");
                break;
                }

            }
    } 
    
    let data = data.as_bytes();
    let mut file = fs::File::create(path).unwrap();
    file.write_all(&data).expect("Cannot write the data");

}



// Decoding the data of spaces(\x20) and newlines(\x0A)
pub fn decode(path: &Path) {
   
    let mut data  = "".to_string();

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

                data.push_str(&value);
                
                length = 0;
                continue;
            }
    }

    let data = data.as_bytes();
    let mut file = fs::File::create(path).unwrap();
    file.write_all(&data).expect("Cannot write the data");

}



