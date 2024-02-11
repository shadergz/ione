struct SysKey {
    #[allow(unused)]
    name: String,
    #[allow(unused)]
    value: [i8; 20]
}

pub struct KeysDb {
    _prod: Vec<SysKey>,
    _title: Vec<SysKey>
}

impl KeysDb {
    pub fn new() -> Self {
        Self {
            _prod: vec![],
            _title: vec![]
        }
    }
    #[allow(unused_variables)]
    pub fn populate_chain(&mut self, key_files: &Vec<String>) {
        let mut has_prod = false;
        let mut has_title = false;

        for key_path in key_files {
            let title = String::from("");
            let key: [i8; 20] = [0; 20];

            if key_path.contains("prod.keys") {
                has_prod = true;
                self._prod.push(SysKey{ name : title, value : key });
            } else if key_path.contains("title.keys") {
                has_title = true;
                self._title.push(SysKey{ name : title, value : key });
            }
        }
        if !has_prod {
            panic!("")
        }
        if !has_title {
            panic!("")
        }
    }
}

