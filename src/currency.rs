/**
    Currency representation
 */
#[derive(Debug, Clone)]
pub struct Currency {
    pub name: String,
    pub source: String,
    pub quote: f32,
}
