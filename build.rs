extern crate regex;
extern crate walkdir;
extern crate dirs;

mod install_vendor;

use std::path::Path;
use regex::Regex;
use install_vendor::InstallVendor;

fn main(){
    println!(r"cargo:rustc-link-search=/usr/lib/gcc/x86_64-linux-gnu/7");
    println!(r"cargo:rustc-link-lib=static=stdc++");

    let install_vendor = InstallVendor::new();

    let libs = ["./vendor/bitprim_", install_vendor.currency_target].join("");
    let previous_exists = Path::new(&libs).exists();

    println!(r"cargo:rustc-link-search={}", libs);

    let re = Regex::new(r"lib([0-9A-Za-z_-]+)\.a").unwrap();
    let paths = install_vendor.load_path(&libs);

    for entry in paths {
        let path = entry.unwrap().path();
        let filename = path.to_str().unwrap();
        let captures = re.captures(filename).expect("A non-library found");
        println!(r"cargo:rustc-link-lib=static={}", &captures[1]);
    }

    if !previous_exists {
        install_vendor.install();
    }
}
