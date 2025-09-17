use super::Rating;

pub struct Product {
    id: u32,
    title: String,
    price: f32,
    description: String,
    category: String,
    image: String,
    rating: Rating,
}
