use crate::currency::Currency;

/**
    Structure used to convert currencies
 */
pub trait Converter {
    /**
        This method convert a currency's quote based on another.
     */
    fn convert(currency_base: Currency, currency_to: Currency) -> f32 {
        let value_converted: f32 = currency_base.quote * currency_to.quote;
        value_converted
    }
}

#[cfg(test)]
mod converter_test {
    #[tokio::test]
    async fn currency_conversion_test() {
        todo!()
    }
}
