use serde::{Deserialize, Serialize};
use crate::parser::Line;
use std::collections::HashMap;
use std::time::Duration;
use std::thread;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Result {
    ip_list: HashMap<String, IpResult>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct IpResult {
    total_access: u16,
    info: Info,
}

#[derive(Default, Debug, Serialize, Deserialize)]
struct Info {
    country: String,
    city: String,
    //lat: String,
    //lon: String,
    isp: String,
    message: String,
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
        let blank: IpResult = Default::default();
        let url_result = reqwest::Url::parse(format!("http://ip-api.com/json/{}?fields=message,country,city,isp", l.ip).as_str());
        let url = match url_result {
            Ok(u) => u,
            Err(err) => {
                println!("[url] {}", err);
                return blank;
            },
        };

        let get_data_result = reqwest::blocking::get(url);
        let get_data = match get_data_result {
            Ok(g) => g,
            Err(err) => {
                println!("[get_req] {}", err);
                return blank;
            },
        };
        let info_map_result = get_data.json::<HashMap<String, String>>();
        let info_map = match info_map_result {
            Ok(i) => i,
            Err(err) => {
                println!("[serialization] {}", err);
                return blank;
            },
        };
        let mut info: Info = Default::default();
        if info_map.contains_key(&String::from("country")) {
            info.country = info_map.get(&String::from("country")).unwrap().to_string();
        } 
        if info_map.contains_key(&String::from("city")) {
            info.city = info_map.get(&String::from("city")).unwrap().to_string();
        }
        // if infoMap.contains_key(&String::from("lat")) {
            // info.country = infoMap.get(&String::from("lat")).unwrap().to_string();
        // }
        // if infoMap.contains_key(&String::from("lon")) {
            // info.country = infoMap.get(&String::from("lon")).unwrap().to_string();
        // }
        if info_map.contains_key(&String::from("isp")) {
            info.isp = info_map.get(&String::from("isp")).unwrap().to_string();
        }
        if info_map.contains_key(&String::from("message")) {
            info.message = info_map.get(&String::from("message")).unwrap().to_string();
        } 
        thread::sleep(Duration::from_millis(1400));
        IpResult {
            total_access: 1,
            info: info,
        }
    }

    fn from_line(&mut self, l: Line) {
        self.total_access += 1; 
    }
}
