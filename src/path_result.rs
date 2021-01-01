use serde::{Deserialize, Serialize};
use crate::parser::Line;
use std::collections::HashMap;


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Result {
    path_list: HashMap<String, PathResult>,
    total_access: u16,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct PathResult {
    total_access: u16,
    method: String,
    path: String,
    status_code: String,
}

impl Result {
    pub fn new() -> Result {
        Result{
            path_list: HashMap::new(),
            total_access: 0,
        }
    }

    pub fn add_result(&mut self, l: Line) {
        let path_key = get_path_key(l.to_owned());
        self.total_access += 1;
        if !self.path_list.contains_key(&path_key) {
            self.path_list.insert(path_key, PathResult::new_from_line(l.clone()));
        } else {
            let path = self.path_list.get_mut(&path_key).unwrap();
            path.from_line(l);
        }
    }
}

impl PathResult {
    fn new_from_line(l: Line) -> PathResult {
        PathResult {
            total_access: 1,
            method: l.method,
            path: l.path,
            status_code: l.status_code,
        }
    }

    fn from_line(&mut self, l: Line) {
        self.total_access += 1; 
    }
}

fn get_path_key(l: Line) -> String {
    format!("{}::{}", l.method, l.path)
}
