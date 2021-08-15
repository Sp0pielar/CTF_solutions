use std::process::Command;

fn main() {


    if cfg!(target_os = "linux") {

        Command::new("make")
                .args(&["-C", "./tiny-AES-c-master/", "lib"])
                .output()
                .expect("failed to compile tinyAES");

        Command::new("cp")
                .arg("./tiny-AES-c-master/aes.a")
                .arg("./tiny-AES-c-master/libaes.a")
                .output()
                .expect("failed to copy libaes.a");


        println!("cargo:rustc-link-search=native=./tiny-AES-c-master");


    } else {
        panic!("ONLY LINUX IS supported !!!");
    }

}

