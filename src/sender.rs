use reqwest::blocking::{multipart, Client};

pub fn send_file_request(path: String, url: String) {
    let client = Client::new();
    let form = multipart::Form::new().file("file", path).unwrap();
    let resp = client.post(url).multipart(form).send().unwrap();
}
