use serde::{Deserialize, Serialize};
use crate::parser::Line;
use std::collections::HashMap;


#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Result {
    ip_list: HashMap<String, IpResult>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct IpResult {
    total_access: u16,
    path_list: HashMap<String, PathResult>,
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
            ip_list: HashMap::new(),
        }
    }

    pub fn add_result(&mut self, l: Line) {
        if !self.ip_list.contains_key(&l.ip) {
            self.ip_list.insert(l.ip.to_owned(), IpResult::new_from_line(l.clone()));
        } else {
            let ip = self.ip_list.get_mut(&l.ip).unwrap();
            ip.from_line(l);
        }
    }
}

impl IpResult {
    fn new_from_line(l: Line) -> IpResult {
        let mut res = IpResult {
            total_access: 1,
            path_list: HashMap::new(),
        };

        res.path_list.insert(
            get_path_key(l.clone()),
            PathResult {
                total_access: 1,
                method: l.method,
                path: l.path,
                status_code: l.status_code,
            },
        );
        res
    }

    fn from_line(&mut self, l: Line) {
        self.total_access += 1;
        let path_key = get_path_key(l.clone());
        if !self.path_list.contains_key(&path_key) {
            let path = PathResult{
                total_access: 1,
                method: l.method,
                path: l.path,
                status_code: l.status_code,
            };
            self.path_list.insert(path_key, path);
        } else {
            let path = self.path_list.get_mut(&path_key).unwrap();
            path.total_access += 1;
        }
    }
}

fn get_path_key(l: Line) -> String {
    format!("{}::{}", l.method, l.path)
}
