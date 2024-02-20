#![allow(unused)]
#![allow(non_snake_case)]


mod xorlock;
mod offspace;
mod p4th;


use std::{io::{self, prelude::*}, path::{self, Path}, time::{SystemTime, UNIX_EPOCH}};

fn main() {
    
banner();

print!("Option:> ");
let _ = io::stdout().flush();

let mut option = String::new();
io::stdin().read_line(&mut option);
let option: i32 = option.trim().parse().unwrap();

match option {
    1 => { 
            let path = p4th::recursive(p4th::get_path());
            let key: u8 = get_key();
            println!("Your KEY: {}\n", key);
           
            for line in path.lines() {
                println!("[*] File [{}] has been encoded [*]", line);
                let path = Path::new(&line);
                
                offspace::encode(path);
                xorlock::xor(path, key);
            }            
        },

    2 => {
            let path = p4th::recursive(p4th::get_path());
            print!("KEY:> ");
            let _ = io::stdout().flush();
            
            let mut key = String::new();
            io::stdin().read_line(&mut key);
            let key: u8 = key.trim().parse().unwrap();
            println!();

            for line in path.lines() {
                println!("[*] File [{}] has been decoded [*]", line);
                let path = Path::new(&line);
                
                xorlock::xor(path, key);
                offspace::decode(path);                    
            }
        },

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
    [-] OPTIONS [-]\t\t\t[-] INFO [-]\n    
    1) Obfuscator  \t\t\tEncrypt a file/directory. 
    2) Deobfuscator\t\t\tDecrypt a file/directory.
    ");

}


fn get_key() -> u8 {

    let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .subsec_nanos();
    
    nanos as u8

} 

