use std::fs::File;
use std::io::Read;
use regex::Regex;

#[derive(Debug)]
struct SysKey {
    #[allow(unused)]
    name: String,
    #[allow(unused)]
    value: Vec<u8>
}
impl SysKey {
    pub fn new(key: String, hash: Vec<u8>) -> Self {
        Self {
            name : key,
            value : hash.into()
        }
    }
}
pub struct KeysDb {
    prod: Vec<SysKey>,
    title: Vec<SysKey>
}

fn ascii_to_utf(buffer: &Vec<u8>) -> Result<&str, usize> {
    for (pos, byte) in buffer.into_iter().enumerate() {
        if byte >= &0x80 {
            return Err(pos);
        }
    }
    Ok(unsafe {
        // This is safe because we verified above that it's a valid ASCII
        // string, and all ASCII strings are also UTF8 strings
        core::str::from_utf8_unchecked(buffer)
    })
}

impl KeysDb {
    pub fn new() -> Self {
        Self {
            prod: vec![],
            title: vec![]
        }
    }
    #[allow(unused_variables)]
    pub fn populate_chain(&mut self, key_files: &Vec<String>) {
        let mut has_prod = false;
        let mut has_title = false;

        for key_path in key_files.into_iter().skip(1) {
            let mut keys = Vec::new();

            File::open(key_path).unwrap().read_to_end(&mut keys).expect("TODO: panic message");
            dbg!("{:?}", keys[0..10].iter().as_slice());

            let is_prod = |path: &String| {
                path.contains("prod.keys")
            };
            let is_title = |path: &String| {
                path.contains("title.keys")
            };

            let mut add_key = |key: SysKey| {
                if is_prod(key_path) {
                    has_prod = true;
                    self.prod.push(key);
                } else if is_title(key_path) {
                    has_title = true;
                    self.title.push(key);
                }
            };

            let prod_regex = Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*\s*=\s*[0-9a-fA-F]{32}").unwrap();
            let title_regex = Regex::new(r"^[0-9a-fA-F]+\s*=\s*[0-9a-fA-F]{32}").unwrap();

            let verify_pair = |line| {
                let mut is_ok = false;

                if is_prod(key_path) && prod_regex.is_match(line) {
                    is_ok = true;
                } else if is_title(key_path) && title_regex.is_match(line) {
                    is_ok = true;
                }
                if !is_ok {
                    panic!("(Crypto:Error): {line} in file {key_path} was not recognized")
                }
            };

            for line in ascii_to_utf(&keys).unwrap().lines() {
                verify_pair(line);
                dbg!(line.len());
                let collected: Vec<&str> = line.trim().split(" = ").collect();

                add_key(SysKey::new(
                    collected[0].into(),
                    collected[1].as_bytes().to_vec()));
            }
        }
        if !has_prod || !has_title {
            panic!("No key found for the key manager, {has_prod}, {has_title}")
        }
        //dbg!("{}", &self.prod);
        dbg!("{}", &self.title);
    }
}
