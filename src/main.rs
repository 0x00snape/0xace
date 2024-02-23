#![allow(unused)]
#![allow(non_snake_case)]

mod modules;

use clap::{App, Arg, ArgMatches};
use std::path::Path;


fn main() {

env_logger::init().unwrap();

    let matches = App::new(modules::banner())       

        .arg(Arg::with_name("key")
             .help("Key is used for encryption/decryption purpose. Value of key must be (0-255)")
             .long("key")
             .short("k")
             .required(true)
             .value_name("KEY"))
        
               
        .arg(Arg::with_name("encrypt")
             .help("Data present on File/Directory will be encrypted, also it encrypt all the child directories.")
             .long("encrypting")
             .short("e")
             .required(false)
             .value_name("FILE/DIRECTORY"))

        
        .arg(Arg::with_name("decrypt")
             .help("Data present on File/Directory will be decrypted.")
             .long("decrypting")
             .short("d")
             .required(false)
             .value_name("FILE/DIRECTORY"))


        .get_matches();


    if matches.is_present("key") && matches.is_present("encrypt") {
       
        println!("{}", modules::banner());

        let key: String  = format!("{}", matches.value_of("key").unwrap());
        let key: u8      = key.trim().parse().unwrap(); 

        let path = format!("{}",matches.value_of("encrypt").unwrap());                      
        let path = modules::check(path);
            
        println!("[*] Please wait... [*]");
        for line in path.lines() {
            
                modules::encode(Path::new(&line));
                modules::xor(Path::new(&line), key);
        }            
        
        println!("[*] Data present in ({}) has been encoded with key:{} [*]\n", matches.value_of("encrypt").unwrap(), key);

   } else {

        println!("{}", modules::banner());

        let key: String  = format!("{}", matches.value_of("key").unwrap());
        let key: u8      = key.trim().parse().unwrap(); 

        let path = format!("{}",matches.value_of("decrypt").unwrap());                      
        let path = modules::check(path);

        println!("[*] Please wait... [*]");
        for line in path.lines() {
            
                modules::xor(Path::new(&line), key);
                modules::decode(Path::new(&line));
        }            

        println!("[*] Data present in ({}) has been decoded with key:{} [*]\n", matches.value_of("decrypt").unwrap(), key);

   }


}

