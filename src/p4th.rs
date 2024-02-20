use std::{env::{self, current_dir}, fmt::format, fs::{self, File}, io::{self,prelude::*}, path::Path, process::exit};


pub fn get_path() -> String {
    
    print!("Path:> ");
    let _ = io::stdout().flush();

    let mut _path = String::new();
    io::stdin().read_line(&mut _path);
    let _path: String = _path.trim().parse().unwrap();

    _path
}



pub fn recursive(path: String) -> String {

    let path = Path::new(&path);  
            
    //Directory exists or not 
    match fs::metadata(path) {

        Ok(_) => { 
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
                                path.display().to_string()
                    }
                 },

        Err(_) => {
                    println!("[*] File/Path {} doesn't exist !!!\nExample:\nPath:> /src/test.txt\nPath:> /src/test/", path.display().to_string());
                    exit(1)
                },
    }

}

