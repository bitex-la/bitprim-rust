use std::fs;
use std::fs::ReadDir;
use std::process::Command;
use std::env;
use walkdir::WalkDir;
use std::io::prelude::*;

pub struct InstallVendor {
    pub bitprim_version: &'static str,
    pub currency_target: &'static str
}

impl InstallVendor {

    pub fn new() -> InstallVendor {
        #[cfg(feature = "v0_9_0")]
        let version = "0.9";
        #[cfg(feature = "v0_9_1")]
        let version = "0.9.1";

        #[cfg(feature = "bch")]
        let target = "bch";
        #[cfg(feature = "btc")]
        let target = "btc";
        #[cfg(feature = "ltc")]
        let target = "ltc";

        InstallVendor {
            bitprim_version: version,
            currency_target: target
        }
    }

    pub fn load_path(&self, lib: &str) -> ReadDir {
        let dir_creation = fs::create_dir(lib);
        if let Err(_) = dir_creation {
            println!("Directory already exists");
        }
        fs::read_dir(lib).expect("Path not found")
    }

    pub fn install(&self) {
        let folders: [&str; 11] = ["bitprim-blockchain", "bitprim-consensus", "bitprim-core", "bitprim-database", 
                                   "bitprim-network", "bitprim-node-cint", "bitprim-node", "boost", "gmp", "icu", "secp256k1"];
        let files: [&str; 16] = ["libbitprim-blockchain.a", "libbitprim-consensus.a", "libbitprim-core.a", "libbitprim-database.a",
                                 "libbitprim-network.a", "libbitprim-node-cint.a", "libbitprim-node.a", "libboost_filesystem.a",
                                 "libboost_iostreams.a", "libboost_log.a", "libboost_program_options.a", "libboost_regex.a",
                                 "libboost_system.a", "libboost_thread.a", "libgmp.a", "libsecp256k1.a"];

        if cfg!(target_os = "linux") {
            Command::new(format!("conan install bitprim-node-exe/{bitprim_version}@bitprim/stable -o currency={currency_target}",
                                 bitprim_version = self.bitprim_version, currency_target = self.currency_target));
            Command::new(format!("conan install bitprim-node-cint/{bitprim_version}@bitprim/stable -o currency={currency_target}",
                                 bitprim_version = self.bitprim_version, currency_target = self.currency_target));
            if let Err(_) = fs::remove_file("bn") {};
            if let Err(_) = fs::remove_file("deploy_manifest.txt") {};
        }

        let home = match env::home_dir() {
            Some(path) => format!("{}", path.display()),
            None => panic!("Impossible to get your home dir!")
        };

        for folder in &folders {
            let package_path = format!("{home}/.conan/data/{folder}", home = home, folder = folder);
            let package_path_with_version = format!("{package_path}/{bitprim_version}",
                                                    package_path = package_path, bitprim_version = self.bitprim_version);

            if folder.contains("bitprim") {
                for entry in WalkDir::new(package_path_with_version) {

                    let raw_entry = entry.unwrap();
                    if raw_entry.path().to_str().unwrap().contains("conaninfo") {
                        let mut file = fs::File::open(raw_entry.path().to_str().unwrap()).unwrap();
                        let mut contents = String::new();
                        file.read_to_string(&mut contents).unwrap();
                        if contents.contains(&format!("currency={}", self.currency_target.to_uppercase())) {
                            for sub_entry in WalkDir::new(raw_entry.path().to_str().unwrap().replace("conaninfo.txt", "")).into_iter() {
                                let path_str = sub_entry.unwrap().path().to_str().unwrap().to_string();
                                for file in &files {
                                    if path_str.contains(file) {
                                        fs::copy(path_str.clone(), format!("vendor/bitprim_{}/{}", self.currency_target, file)).unwrap();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
