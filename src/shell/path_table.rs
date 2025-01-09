use std::collections::HashMap;
use std::env;
use std::fs::read_dir;

use string_cache::DefaultAtom as Atom;

#[derive(Debug)]
pub struct PathTable {
    pub paths: HashMap<Atom, Atom>,
}

impl Default for PathTable {
    fn default() -> Self {
        let mut pt = Self { paths: HashMap::new() };
        pt.cache();
        pt
    }
}

impl PathTable {
    pub fn cache(&mut self) {
        let path = env::var("PATH");
        if let Err(err) = path {
            panic!("{err}");
        }
        self.paths.clear();
        for bind in path.unwrap().split(":") {
            if let Ok(files) = read_dir(bind) {
                for entry in files {
                    let file = entry.unwrap();
                    let file_name = file.file_name().into_string().unwrap();
                    let path_string = file.path().display().to_string();
                    self.paths.insert(Atom::from(file_name), Atom::from(path_string));
                }
            }
        }
    }
}
