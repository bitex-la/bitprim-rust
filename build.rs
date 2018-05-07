extern crate regex;
use std::fs;
use regex::Regex;
use std::env;

fn main(){
  println!(r"cargo:rustc-link-search=/usr/lib/gcc/x86_64-linux-gnu/7");
  println!(r"cargo:rustc-link-lib=static=stdc++");

  let currency_target = env::var("CURRENCY_TARGET").unwrap_or("BCH".to_string());
  let libs = ["./vendor/bitprim_", &currency_target.to_lowercase()].join("");
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
