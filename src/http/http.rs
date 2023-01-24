use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
pub struct RequestResult {
    pub success: bool,
    pub error: Option<String>,
    pub response: Option<String>,
}

pub struct Http {
    client: Client,
}

impl Http {
    pub fn new() -> Http {
        Http {
            client: Client::new(),
        }
    }

    pub async fn request(
        &self,
        method: &str,
        url: &str,
        body: &str,
        content_type: &str,
        timeout: std::time::Duration,
    ) -> RequestResult {
        let result;

        match method {
            "POST" => {
                result = self
                    .client
                    .post(url)
                    .body(body.to_owned())
                    .header("content-type", content_type)
                    .timeout(timeout)
                    .send()
                    .await;
            }
            "GET" => {
                result = self.client.get(url).timeout(timeout).send().await;
            }
            _ => {
                return RequestResult {
                    success: false,
                    response: Option::None,
                    error: Option::Some(String::from("Method not supported")),
                };
            }
        };

        match result {
            Ok(response) => {
                return RequestResult {
                    success: true,
                    error: Option::None,
                    response: Option::Some(match response.text().await {
                        Ok(text) => text,
                        Err(_) => String::new(),
                    }),
                };
            }

            Err(err) => {
                return RequestResult {
                    success: false,
                    error: Option::Some(err.to_string()),
                    response: Option::None,
                }
            }
        }
    }
}
