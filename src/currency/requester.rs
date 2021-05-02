use crate::currency::Currency;
use std::borrow::{Borrow, BorrowMut};
use reqwest;
use reqwest::Response;

#[derive(Clone)]
pub struct Requester {
    api_key: String,
    api_link: String,
    api_link_path: String
}

impl Requester {
    pub async fn request_currency(&mut self, currency_source: [char; 3]) -> Response {
        /*let requester = self.clone();
        let client: Client<HttpConnector> = Client::new();
        let request = Request::builder()
            .method(Method::GET)
            .uri("https://api.github.com/henrybarreto")
            //.uri("v1/exchangerate/BTC")
            .header(hyper::header::CONTENT_TYPE, "application/json")
            //.header("X-CoinAPI-Key", "00A6F8AE-7797-4214-80F0-8D46F7CF5796")
            .body(Body::empty())
            .expect("Não pode fazer o request para API");
        
        client.request(request).await.unwrap()*/
    }
}

#[cfg(test)]
mod requester_test {
    use crate::currency::requester::Requester;

    #[tokio::test]
    async fn request_currency_tester() {
        /*let mut requester = Requester {
            api_key: "00A6F8AE-7797-4214-80F0-8D46F7CF5796".to_string(),
            api_link: "http://rest-sandbox.coinapi.io/".to_string(),
            api_link_path: "v1/exchangerate/BTC/".to_string()
        };

        let req = requester.request_currency(['b', 't', 'c']).await;
        println!("{:#?}", req.text().await.unwrap());*/
        todo!()
    }
}