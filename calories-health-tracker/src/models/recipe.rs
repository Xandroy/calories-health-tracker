use {super::food_category::FoodCategory, serde::{Serialize, Deserialize}};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct RecipeIngredient {
    pub ingredient_id: String,
    pub grams: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub servings: f32,
}
