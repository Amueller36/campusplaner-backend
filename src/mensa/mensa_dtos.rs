use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct MenuDto {
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "date")]
    pub dates: Vec<Date>,
}

#[derive(Debug, Deserialize)]
pub struct Date {
    #[serde(rename = "timestamp")]
    pub timestamp: i64,
    #[serde(rename = "item")]
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "meal")]
    pub meal: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "foodicons")]
    pub foodicons: String,
    #[serde(rename = "price1")]
    pub price_students: String,
    #[serde(rename = "price2")]
    pub price_employees: String,
    #[serde(rename = "price3")]
    pub price_internal_guests: String,
    #[serde(rename = "price4")]
    pub price_external_guests: String,
    #[serde(rename = "weight_unit")]
    pub weight_unit: Option<String>,
    #[serde(rename = "foto")]
    pub foto: Option<String>,
    #[serde(rename = "uhrzeit")]
    pub uhrzeit: Option<String>,
}

impl Item {
    pub fn parse_float(value: &str) -> Result<f32, Box<dyn Error>> {
        value.replace(',', ".").parse::<f32>().map_err(|e| e.into())
    }
}