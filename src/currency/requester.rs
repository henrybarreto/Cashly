use crate::currency::Currency;
use std::borrow::{Borrow, BorrowMut};

extern crate hyper;

use hyper::client::{Client, HttpConnector};
use hyper::{Response, Body};
use hyper::client::*;

pub struct Requester {
    api_key: String,
    api_link: String,
    api_link_path: String
}

impl Requester {
    pub fn request_currency(currency_source: [char; 3]) -> Currency {
        todo!()
    }
}

#[cfg(test)]
mod requester_test {
    use crate::currency::requester::Requester;

    #[tokio::test]
    async fn requet_information_tester() {
        todo!()
    }
}