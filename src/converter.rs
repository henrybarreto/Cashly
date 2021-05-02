use crate::currency::Currency;

pub trait Converter {
    fn convert(currency_form: Currency, currency_to: Currency) -> f32 {
        let value_converted: f32 = currency_form.quote * currency_to.quote;
        return value_converted;
    }
}

#[cfg(test)]
mod converter_test {
    #[test]
    fn currency_conversion_test() {
        todo!()
    }
}
