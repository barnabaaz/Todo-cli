use std::{collections::HashMap, io::Read, str::FromStr};
pub struct Config {
    pub command: String,
    pub item: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();
        let command = match args.next() {
            Some(args) => args,
            None => return Err("Didn't get any parameter"),
        };

        let item = match args.next() {
            Some(i) => i,
            None => String::new(),
        };
        Ok(Config { command, item })
    }
}

pub struct Todo {
    pub map: HashMap<String, bool>,
}
impl Todo {
    pub fn new() -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open("db.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let map: HashMap<String, bool> = contents
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        // for line in contents.lines () {
        // let mut values = line.split(2, '\t');
        // let key = values.next().expect("invalid Key");
        // let value = values.next().expect("invalid value");
        // map.insert(String::from(key), bool::frm_str(value).unwrap())
        // }
        Ok(Todo { map })
    }
    pub fn add(&mut self, item: String) {
        self.map.insert(item, true);
    }
    pub fn list(&self) {
        for (i, (k, v)) in self.map.iter().enumerate() {
            println!("{}. {} : {}", i + 1, k, v)
        }
    }
    pub fn complete(&mut self, key: &String) -> Option<()> {
        let key = self.map.get_mut(key);
        match key {
            Some(item) => Some(*item = false),
            None => None,
        }
    }
    pub fn save(self) -> Result<(), std::io::Error> {
        let mut contents = String::new();
        for (k, v) in self.map {
            let record = format!("{}\t{}\n", k, v);
            contents.push_str(&record);
        }
        std::fs::write("db.txt", contents)
    }
}

pub fn run(config: Config, mut todo: Todo) -> Result<(), &'static str> {
    match &config.command[..] {
        "add" => {
            todo.add(config.item);
            match todo.save() {
                Ok(_) => {
                    println!("Item Saved Succesfully");
                    Ok(())
                }
                Err(e) => {
                    println!("Item cannot be saved {}", e);
                    Err("can't print this ")
                }
            }
        }
        "list" => {
            todo.list();
            Ok(())
        }
        _ => Err("Invalid command, please use 'add' "),
    }
}
