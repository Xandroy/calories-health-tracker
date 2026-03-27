use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;

pub fn load_from_file<T>(path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned + Default,
{
    //If file doesn't exist, return empty vector instead of an error.
    //This is only important for the first start, when no JSON files exist yet
    if !Path::new(path).exists() {
        return Ok(T::default());
    }

    let content = fs::read_to_string(path)?;

    //If File is empty or only includes whitespace and indentations,
    //then return empty vector, not an error (like above)
    if content.trim().is_empty() {
        return Ok(T::default());
    }

    let data = serde_json::from_str(&content)?;

    Ok(data)
}