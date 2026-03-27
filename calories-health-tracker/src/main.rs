mod app;
mod models;
mod storage;
mod service;
mod commands;
mod utils;

use crate::models::ingredient::Ingredient;
use crate::storage::json_store::load_from_file;

//Test for JSON read function
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ingredients: Vec<Ingredient> = load_from_file("data/ingredients.json")?;

    println!("{:#?}", ingredients);

    Ok(())
}
