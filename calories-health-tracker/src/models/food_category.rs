use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum FoodCategory {
    Meat,
    Fish,
    Vegetarian,
    Vegan
}