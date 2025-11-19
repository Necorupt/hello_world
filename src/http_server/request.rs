pub struct Request {
    pub path: String,
    pub method: String,
    pub proto: String,
}

impl Request {
    pub fn new() -> Self {
        return {
            Request {
                path: String::from(""),
                method: String::from(""),
                proto: String::from("HTTP/1.1"),
            }
        };
    }

    pub fn parse(&mut self, _data: Vec<String>) -> bool {
        let head: Vec<&str> = _data[0].split(' ').collect::<Vec<&str>>();

        if head.len() < 3 {
            return false;
        }

        self.method = String::from(head[0]);
        self.path = String::from(head[1]);
        self.proto = String::from(head[2]);

        return true;
    }
}
