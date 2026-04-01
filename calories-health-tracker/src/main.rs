mod app;
mod models;
mod storage;
mod service;
mod commands;
mod utils;

use crate::models::food_category::FoodCategory;
use crate::models::ingredient::Ingredient;
use crate::storage::json_store::{load_from_file, write_to_file};

// Test for JSON write + read function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_ingredients = vec![Ingredient {
        id: "ing_test_001".to_string(),
        name: "Test Rice".to_string(),
        kcal_per_100g: 130.0,
        protein_per_100g: 2.7,
        fat_per_100g: 0.3,
        carbs_per_100g: 28.0,
        category: FoodCategory::Vegan,
    }];

    write_to_file("data/test_ingredients.json", &test_ingredients)?;

    let loaded_ingredients: Vec<Ingredient> = load_from_file("data/test_ingredients.json")?;

    println!("Written data:");
    println!("{:#?}", test_ingredients);

    println!("Loaded data:");
    println!("{:#?}", loaded_ingredients);

    Ok(())
}