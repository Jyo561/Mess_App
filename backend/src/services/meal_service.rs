use common::MealType;
use chrono::{Datelike, Local};

pub fn get_available_meals() -> Vec<MealType> {
    let now = Local::now();
    let is_weekday = now.weekday().number_from_monday() <= 5;

    if is_weekday {
        vec![MealType::Breakfast, MealType::Dinner]
    } else {
        vec![MealType::Breakfast, MealType::Lunch, MealType::Dinner]
    }
}
