#![feature(iter_intersperse)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::path::Path;

use serde::{Deserialize, Serialize};

fn main() {
    const RU_PATH: &str = "./lang/ru.lang";

    if !Path::new(RU_PATH).exists() {
        let file = File::create(RU_PATH).unwrap();
        serde_json::to_writer(BufWriter::new(file), &RuLang::default()).unwrap();
    }

    if let Ok(file) = File::open(RU_PATH) {
        if let Ok(ru_lang) = serde_json::from_reader::<_, RuLang>(BufReader::new(file)) {
            let text = String::from_iter(
                std::env::args()
                    .skip(1)
                    .intersperse(String::from(" "))
                    .flat_map(|s| s.chars().collect::<Vec<_>>())
                    .map(|s| ru_lang.get(&s).unwrap_or(&vec![String::from("$")])[0].clone()),
            );
            println!("{}", text);
        }
        else {
            println!("Config is invalid at {}", RU_PATH);
        }
    }
    else {
        println!("Unable to find config at {}", RU_PATH);
    }
}

#[derive(Serialize, Deserialize)]
pub struct RuLang(HashMap<char, Vec<String>>);

impl Default for RuLang {
    fn default() -> Self {
        let mut map = HashMap::new();
        for c in "абвгдеёжзийклмнопрстуфхцчшщъыьэюя ".chars() {
            map.insert(c, vec![]);
        }
        RuLang(map)
    }
}

impl Deref for RuLang {
    type Target = HashMap<char, Vec<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
