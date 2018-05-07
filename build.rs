extern crate regex;
use std::fs;
use regex::Regex;

fn main(){
    println!(r"cargo:rustc-link-search=/usr/lib/gcc/x86_64-linux-gnu/7");
    println!(r"cargo:rustc-link-lib=static=stdc++");

    #[allow(unused_variables)]
    let currency_target = "bch";

    #[cfg(feature = "btc")]
    let currency_target = "btc";
    #[cfg(feature = "ltc")]
    let currency_target = "ltc";

    let libs = ["./vendor/bitprim_", currency_target].join("");
    println!(r"cargo:rustc-link-search={}", libs);

    let re = Regex::new(r"lib([0-9A-Za-z_-]+)\.a").unwrap();
    let paths = fs::read_dir(libs).expect("Path not found");

    for entry in paths {
        let path = entry.unwrap().path();
        let filename = path.to_str().unwrap();
        let captures = re.captures(filename).expect("A non-library found");
        println!(r"cargo:rustc-link-lib=static={}", &captures[1]);
    }
}
