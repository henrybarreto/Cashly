//! # Cryptoly
//!
//! ## Currency converter crate
//!
//! This is a simple lib to convert cryptocurrency values
//!

pub mod currency;
pub mod converter;
pub mod requester;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
