use std::collections::HashMap;
use reqwest::Url;
use reqwest::{header::USER_AGENT};

pub fn find_url() -> (String, String, String, String) {
    let client = reqwest::blocking::Client::new();
    let request = client.get("http://access.olleh.com")
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
        .send().unwrap();

    let url = request.url();

    let scheme = url.scheme().to_string();
    let host = url.host().unwrap().to_string();
    let port = url.port().unwrap().to_string();
    let query = url.query().unwrap().to_string();

    return (scheme, host, port, query);
}

pub fn enter_admin_url(data: (String, String, String, String)) -> String {
    format!("{}://{}:{}/enterAdminId.html?{}", data.0, data.1, data.2, data.3)
}

pub fn admin_url(data: (String, String, String, String)) -> String {
    format!("{}://{}:{}/reauth_said.html", data.0, data.1, data.2)
}

pub fn reset() -> String {
    let find_url = find_url();

    let url = Url::parse(&enter_admin_url(find_url.clone())).unwrap();
    let query: HashMap<String, String> = url.query_pairs().into_owned().collect();

    let mut form = HashMap::new();
    form.insert("userID", "reset");
    form.insert("userPW", "reset1");
    form.insert("sso", query.get("sso").unwrap());
    form.insert("no", query.get("no").unwrap());

    let client = reqwest::blocking::Client::new();
    let response = client.post(admin_url(find_url.clone()))
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36")
        .header("Referer", enter_admin_url(find_url))
        .form(&form)
        .send().unwrap();

    return response.status().to_string();
}
