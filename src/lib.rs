pub mod structures;
use reqwest::{Client as RClient, RequestBuilder, Response};
use serde::Serialize;

pub struct Client {
    pub api_key: String,
    pub api_secret: String,
    pub host: String,
}

impl Client {
    fn apply_authentication_headers(&self, request: RequestBuilder) -> RequestBuilder {
        let r = request
            .header("Authentication-Key", &self.api_key)
            .header("Authentication-Secret", &self.api_secret);
        r
    }

    fn format_endpoint(&self, endpoint: &str) -> String {
        format!("https://{}.simplicate.nl/api/v2/{}", &self.host, endpoint)
    }
}

pub trait QueryMany<O> {
    const ENDPOINT: &'static str;

    fn unwrap_response(response: Response) -> Vec<O>;

    fn get_response(cli: Client, params: Option<Vec<(String, String)>>) -> Option<Response> {
        let url = cli.format_endpoint(Self::ENDPOINT);
        let builder = RClient::new().get(&url);
        let builder = cli.apply_authentication_headers(builder);
        let response = match params {
            Some(p) => builder.query(&p).send(),
            None => builder.send(),
        };
        match response {
            Ok(x) => Some(x),
            _ => None,
        }
    }

    fn fetch_many(cli: Client, params: Option<Vec<(String, String)>>) -> Option<Vec<O>> {
        let response = Self::get_response(cli, params);
        match response {
            Some(resp) => Some(Self::unwrap_response(resp)),
            None => None,
        }
    }
}

pub trait Post<T>: Serialize {
    const ENDPOINT: &'static str;
    fn post(&self, cli: Client) -> Option<Response> {
        let url = cli.format_endpoint(Self::ENDPOINT);
        let builder = RClient::new().post(&url).json(self);
        let builder = cli.apply_authentication_headers(builder);
        match builder.send() {
            Ok(x) => Some(x),
            _ => None,
        }
    }
}
