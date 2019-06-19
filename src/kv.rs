extern crate csv;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::string::String;

pub struct KvStore {
    map: HashMap<String, String>,
    file: String,
}

impl KvStore {
    pub fn new(file: Option<&str>) -> KvStore {
        match file {
            Some(file) => KvStore::read(file.to_string()),
            _ => KvStore {
                map: HashMap::new(),
                file: String::from("default"),
            },
        }
    }

    pub fn read(file: String) -> KvStore {
        let mut kvstore = KvStore {
            map: HashMap::new(),
            file: (&file).clone(),
        };
        if Path::new(&file).exists() {
            let mut f = File::open(&file).unwrap();
            let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(f);
            for result in rdr.records() {
                let record = result.unwrap();
                kvstore.set(record[0].to_string(), record[1].to_string());
            }
        }
        kvstore
    }

    pub fn sync(&self) {
        let path = Path::new(&self.file);

        let mut wtr = csv::Writer::from_path(path).unwrap();
        for (k, v) in &self.map {
            wtr.write_record(&[k, v]);
        }
    }

    pub fn get(&self, v: String) -> Option<String> {
        self.map.get(&v).cloned()
    }

    pub fn set(&mut self, k: String, v: String) {
        self.map.insert(k, v);
        self.sync();
    }

    pub fn remove(&mut self, k: String) -> Option<String> {
        let v = self.map.remove(&k);
        self.sync();
        v
    }
}
