

    use std::{env};
    
    fn main() {
        
        let library_dir = "/opt/rsbash/builtins";
        let libsh_dir = "./lib/sh/";
        let exe_dir = "/opt/rsbash/";
        
        println!("cargo:rustc-link-search=native={}", env::join_paths(&[library_dir  ]).unwrap().to_str().unwrap());
        println!("cargo:rustc-link-search=native={}", env::join_paths(&[  exe_dir]).unwrap().to_str().unwrap());
        println!("cargo:rustc-link-search=native={}", env::join_paths(&[  libsh_dir]).unwrap().to_str().unwrap());
     
        println!("cargo:rustc-link-args=-Wl,--copy-dt-needed-entries -fpic");
        
        println!("cargo:rustc-flags=-l static=sh");
        println!("cargo:rustc-flags=-l dylib=rt");

     
    }
