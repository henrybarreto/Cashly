use crate::currency::Currency;
use std::borrow::{Borrow, BorrowMut};
use reqwest;
use reqwest::{Response, Client, Method, RequestBuilder};
use std::error::Error;

/***
    Structure returned from the API request
 */
#[derive(Clone)]
pub struct Requester {
    api_key: String,
    api_link: String,
    api_link_path: String
}

impl Requester {
    /***
        Making a request to the API with the `currency_source` as an argument
     */
    pub async fn request_currency(&mut self, currency_source: String) -> Result<Response, reqwest::Error> {
        let requester = self.clone();
        let currency = currency_source.clone();
        let client = Client::new();
        let request = client.get(
            format!("{link}{path}{currency}",
                    link=requester.api_link.clone(),
                    path=requester.api_link_path,
                    currency=currency))
            //.uri("v1/exchangerate/BTC")
            //.header(hyper::header::CONTENT_TYPE, "application/json")
            .header("X-CoinAPI-Key", requester.api_key.clone())
            .body("")
            .send().await;

        request
    }
}

#[cfg(test)]
mod requester_test {
    use crate::requester::Requester;

    #[tokio::test]
    async fn request_currency_tester() {
        let mut requester = Requester {
            api_key: "00A6F8AE-7797-4214-80F0-8D46F7CF5796".to_string(),
            api_link: "http://rest-sandbox.coinapi.io/".to_string(),
            api_link_path: "v1/exchangerate/".to_string()
        };

        let req = requester.request_currency("BTC".to_string()).await.unwrap();
        println!("{:#?}", req.text().await.unwrap());
    }
}