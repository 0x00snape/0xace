#![allow(unused)]

mod xorlock;
mod offspace;
mod p4th;

use std::{fs, io::{self, prelude::*}, path::{self, Path}, time::{SystemTime, UNIX_EPOCH}};

fn main() {
    
banner();

print!("Option:> ");
let _ = io::stdout().flush();

let mut option = String::new();
io::stdin().read_line(&mut option);
let option: i32 = option.trim().parse().unwrap();

match option {
    1 => { 
            let path = p4th::check_path(p4th::get_path());
            let key: u8 = get_key();
            println!("Your KEY: {}", key);
            
            for line in path.lines() {
                println!("{}", line);
               let path = Path::new(&line);
               xorlock::xor(path, key);
            }
        },
    2 => {},
    3 => {},
    4 => {},
    _ => panic!(),
}


}

fn banner(){
        println!("
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
        ");
    println!("
    [*] OPTIONS [*]\t\t\t\t\t[-] INFO [-]\n    
    1) Obfuscator  \t\t\t\t\tobfuscate single/multiple files. 
    2) Deobfuscator\t\t\t\t\tdeobfuscate single/multiple file.
    3) Checkmate   \t\t\t\t\tobfuscate the whole disk. 
    4) Stalemate   \t\t\t\t\tdeobfuscate the whole disk.
    ");

}


/*
fn get_path() -> String {
    
print!("Path:> ");
let _ = io::stdout().flush();

let mut _path = String::new();
io::stdin().read_line(&mut _path);
let _path: String = _path.trim().parse().unwrap();

_path
}

fn check_path(path: String) -> String {

 let path = Path::new(&path);
        
        // Checking directory or file
        if path.is_dir() == true { 

            let path = std::fs::read_dir(&path).unwrap();
            let mut _path = "".to_string();   
            
            for files in path {

                    let mut files = files.unwrap().path().display().to_string();
                    let files = Path::new(&files);
                   
                    _path.push_str(files.display().to_string().as_str());
                    _path.push_str("\n");
                    
                }
             _path
        
        } else { 
            
            // file/Directory exists or not 
            match fs::metadata(path) { 
                Ok(_) => { 
                           path.display().to_string()
                         },
                Err(_) => panic!()// println!("[*] File/Directory doesn't exist !!!\n[*] Ensure that you have chosen the correct path..."),
            }
        }

}
*/

fn get_key() -> u8 {

    let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
    
    nanos as u8

}
