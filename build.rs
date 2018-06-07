extern crate regex;
use std::fs;
use regex::Regex;

fn main() {
    println!(r"cargo:rustc-link-search=/usr/lib/gcc/x86_64-linux-gnu/7");
    println!(r"cargo:rustc-link-lib=static=stdc++");

    let re = Regex::new(r"lib([0-9A-Za-z_-]+)\.a").unwrap();
    for lib in ["./vendor/bitprim_btc", "./vendor/bitprim_bch"].iter() {
        println!(r"cargo:rustc-link-search={}", lib);
        let path = fs::read_dir(lib).expect("Path not found");
        for entry in path {
            let path = entry.unwrap().path();
            let filename = path.to_str().unwrap();
            let captures = re.captures(filename).expect("A non-library found");
            println!(r"cargo:rustc-link-lib=static={}", &captures[1]);
        }
    }
}
