use std::collections::HashMap;

pub enum StatusCode {
    Ok200,
    NotFound404,
    ServerError500,
}

pub struct Response {
    pub status: StatusCode,
    pub content: String,
    pub proto_version_maj: u32,
    pub proto_version_min: u32,
    response_str: String,
    headers: HashMap<String, String>,
}

impl Response {
    pub fn new() -> Self {
        return Response {
            status: StatusCode::Ok200,
            content: String::from(""),
            proto_version_maj: 1,
            proto_version_min: 1,
            response_str: String::from(""),
            headers: HashMap::new(),
        };
    }

    pub fn add_header(&mut self, _key: String, _val: String) {
        self.headers.insert(_key, _val);
    }

    pub fn to_string(&self) -> String {
        let length = self.content.len();
        let head = format!(
            "{0} {1}\r\n",
            self.generate_proto_str(),
            self.generate_status_string()
        );

        let mut response = head;

        for header in self.headers.clone() {
            let header_str = format!("{0}:{1}\r\n", header.0, header.1);

            response.push_str(&header_str);
        }

        response.push_str(&format!("Content-Length:{length}\r\n\r\n"));
        response.push_str(&self.content);

        return response;
    }

    pub fn get_response_string(&self) -> String {
        return self.response_str.clone();
    }

    pub fn generate_proto_str(&self) -> String {
        return format!(
            "HTTP/{0}.{1}",
            self.proto_version_maj, self.proto_version_min
        );
    }

    pub fn generate_status_string(&self) -> String {
        match self.status {
            StatusCode::Ok200 => String::from("200 OK"),
            StatusCode::NotFound404 => String::from("404 NOT FOUND"),
            StatusCode::ServerError500 => String::from("500 SERVER ERROR"),
        }
    }

    pub fn generate(&self) -> bool {
        return true;
    }
}
