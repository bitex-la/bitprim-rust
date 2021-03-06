use std::fs;
use std::fs::ReadDir;
use walkdir::WalkDir;
use std::io::prelude::*;
use dirs;

pub struct InstallVendor {
    pub bitprim_version: &'static str,
    pub currency_target: &'static str
}

impl InstallVendor {

    pub fn new() -> InstallVendor {
        #[cfg(feature = "v0_18_0")]
        let version = "0.18.0";

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
        let folders: [&str; 12] = ["bitprim-blockchain", "bitprim-consensus", "bitprim-core", "bitprim-database", "lmdb",
                                   "bitprim-network", "bitprim-node-cint", "bitprim-node", "boost", "gmp", "icu", "secp256k1"];
        let files: [&str; 18] = ["libbitprim-blockchain.a", "libbitprim-consensus.a", "libbitprim-core.a", "libbitprim-database.a",
                                 "libbitprim-network.a", "libbitprim-node-cint.a", "libbitprim-node.a", "libboost_filesystem.a",
                                 "libboost_iostreams.a", "libboost_log.a", "libboost_program_options.a", "libboost_regex.a", "liblmdb.a",
                                 "libboost_system.a", "libboost_thread.a", "libgmp.a", "libsecp256k1.a", "libbitprim-node-cint-version.a"];

        let home = match dirs::home_dir() {
            Some(path) => format!("{}", path.display()),
            None => panic!("Impossible to get your home dir!")
        };

        for folder in &folders {
            let package_path = format!("{home}/.conan/data/{folder}", home = home, folder = folder);
            let package_path_with_version = format!("{package_path}/{bitprim_version}",
                                                    package_path = package_path, bitprim_version = self.bitprim_version);

            if folder.contains("bitprim") {
                self.find_files(&package_path_with_version, files);
            } else {
                self.find_files(&package_path, files);
            }
        }
    }

    fn find_files(&self, path: &str, files: [&str; 18]) {
        for entry in WalkDir::new(path) {

            let raw_entry = entry.unwrap();
            if raw_entry.path().to_str().unwrap().contains("conaninfo") {
                let mut file = fs::File::open(raw_entry.path().to_str().unwrap()).unwrap();
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();
                if path.contains("bitprim") && contents.contains(&format!("currency={}", self.currency_target.to_uppercase())) {
                    self.copy_files(&raw_entry.path().to_str().unwrap().replace("conaninfo.txt", ""), files);
                } else {
                    self.copy_files(path, files);
                }
            }
        }
    }

    fn copy_files(&self, path: &str, files: [&str; 18]) {
        for sub_entry in WalkDir::new(path).into_iter() {
            let dir_entry = sub_entry.unwrap();
            let path_str = dir_entry.path().to_str().unwrap();
            for file in &files {
                if path_str.contains(file) {
                    fs::copy(path_str, format!("vendor/bitprim_{}/{}", self.currency_target, file)).unwrap();
                }
            }
        }
    }
}

