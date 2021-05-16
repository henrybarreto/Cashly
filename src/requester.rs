use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

use reqwest;
use reqwest::{Client, Method, RequestBuilder, Response, StatusCode};

use crate::currency::Currency;
use crate::service::Service;

/**
    Structure methods to perform actions to the API
 */
pub struct Requester {
    pub service: Box<dyn Service>
}

impl Requester {
    /**
        Structure constructor
     */
    pub fn new(service: Box<dyn Service>) -> Self {
        Requester {
            service
        }
    }
    /**
        Making a request to the API to get information about a currency
     */
    pub async fn request_currency(&self, currency_source: String) -> Result<Response, reqwest::Error> {
        let requester = self.clone();
        let service = &requester.service;
        let currency = currency_source.clone();
        let client = Client::new();
        let request = client.get(
            service.build_uri(currency))
            //.uri("v1/exchangerate/BTC")
            //.header(hyper::header::CONTENT_TYPE, "application/json")
            .header("X-CoinAPI-Key", service.get_api_key())
            .body("")
            .send().await;

        request
    }
}

#[cfg(test)]
mod requester_test {
    use reqwest::StatusCode;

    use crate::requester::Requester;
    use crate::service::coin_api_service::CoinAPIService;

    #[tokio::test]
    async fn request_currency_test() {
        let service = CoinAPIService {
            api_key: "00A6F8AE-7797-4214-80F0-8D46F7CF5796".to_string(),
            api_link: "http://rest-sandbox.coinapi.io/".to_string(),
            api_link_path: "v1/exchangerate/".to_string()
        };
        let mut requester = Requester {
            service: Box::new(service)
        };

        let req = requester.request_currency("BTC".to_string()).await.unwrap();

        assert!(req.status().is_success());
    }
}