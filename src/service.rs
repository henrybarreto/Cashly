pub mod coin_api_service;

use crate::service::coin_api_service::CoinAPIService;
use crate::currency::Currency;

/**
    Generic methods to manager an API service.
 */
pub trait Service {
    fn build_uri(&self, currency: String) -> String;
    fn get_api_key(&self) -> String;
}
