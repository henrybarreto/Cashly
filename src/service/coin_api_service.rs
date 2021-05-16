use crate::service::Service;
use crate::currency::Currency;


/**
    Minimum information needed to use the CoinAPI
 */
#[derive(Debug, Clone)]
pub struct CoinAPIService {
    pub api_key: String,
    pub api_link: String,
    pub api_link_path: String
}

/**
    Implementation of Service methods to CoinAPI
 */
impl Service for CoinAPIService {
    fn build_uri(&self, currency: String)-> String {
        let uri_builded = format!("{link}{path}{currency}",
                                  link = &self.api_link.clone(),
                                  path = &self.api_link_path,
                                  currency = currency);
        uri_builded
    }

    fn get_api_key(&self) -> String {
        self.api_key.clone()
    }
}
