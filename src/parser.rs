#[derive(Default, Debug, Clone)]
pub struct Line {
    pub date: String,
    pub ip: String,
    pub method: String,
    pub path: String,
    pub status_code: String,
}

pub fn parse_line(l: String) -> Option<Line> {
    let mut line: Line = Default::default();

    // Split the whole text into two separate part
    let s = l.split("] ").collect::<Vec<&str>>();
    if s.len() != 2 {
        return None;
    }

    // Deal with the first part, fetch the date
    let data = s[0].split(" [").collect::<Vec<&str>>();
    if data.len() != 2 {
        return None;
    }
    line.date = String::from(data[1]);

    // Deal with the ip related information from the beginning
    let data = data[0].split(" ").collect::<Vec<&str>>();
    if data.len() != 4 {
        return None;
    }
    line.ip = String::from(data[1]);

    // Deal with the second part
    let data = s[1].split("\"").collect::<Vec<&str>>();
    if data.len() < 4 {
        return None;
    }
    let path_related = data[1];
    let status_related = data[2];

    // Parse path and method
    let data = path_related.split(" ").collect::<Vec<&str>>();
    if data.len() != 3 {
        return None;
    }
    line.path = String::from(data[1]);
    line.method = String::from(data[0]);

    // Parse status code
    let data = status_related.split(" ").collect::<Vec<&str>>();
    if data.len() < 3 {
        return None;
    }
    line.status_code = String::from(data[1]);
    Some(line)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "localhost:80 100.10.10.10 - - [09/Dec/2020:15:08:17 +0000] \"GET /gpack/travian_default/lang/en/lang.css?e21d2 HTTP/1.1\" 200 1574 \"http://100.69.67.12/install/\" \"Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36\"";
        let result = parse_line(String::from(line));

        assert_eq!(result.unwrap().ip, String::from("100.10.10.10"))
    }
}
