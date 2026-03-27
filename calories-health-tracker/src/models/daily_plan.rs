use {super::food_category::FoodCategory, serde::{Serialize, Deserialize}};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyPlan {
    pub id: String,
    pub date: String,
    pub entries: Vec<DailyPlanEntry>,
    pub categories: FoodCategory,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DailyPlanEntry {
    Ingredient (DailyPlanIngredient),
    RecipePortion(DailyPlanRecipePortion),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyPlanIngredient {
    pub ingredient_id: String,
    pub grams: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DailyPlanRecipePortion {
    pub recipe_id: String,
    pub portions: f32,
}