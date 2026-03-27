use {super::food_category::FoodCategory, serde::{Serialize, Deserialize}};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ingredient {
    pub id: String,
    pub name: String,
    pub kcal_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub category: FoodCategory,
}