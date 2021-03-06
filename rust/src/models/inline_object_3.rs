/*
 * spoonacular API
 *
 * The spoonacular Nutrition, Recipe, and Food API allows you to access over 380,000 recipes, thousands of ingredients, 80,000 food products, and 100,000 menu items. Our food ontology and semantic recipe search engine makes it possible to search for recipes using natural language queries, such as \"gluten free brownies without sugar\" or \"low fat vegan cupcakes.\" You can automatically calculate the nutritional information for any recipe, analyze recipe costs, visualize ingredient lists, find recipes for what's in your fridge, find recipes based on special diets, nutritional requirements, or favorite ingredients, classify recipes into types and cuisines, convert ingredient amounts, or even compute an entire meal plan. With our powerful API, you can create many kinds of food and especially nutrition apps.  Special diets/dietary requirements currently available include: vegan, vegetarian, pescetarian, gluten free, grain free, dairy free, high protein, whole 30, low sodium, low carb, Paleo, ketogenic, FODMAP, and Primal.
 *
 * The version of the OpenAPI document: 1.0
 * Contact: david@spoonacular.com
 * Generated by: https://openapi-generator.tech
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct InlineObject3 {
    /// The title of the recipe.
    #[serde(rename = "title")]
    pub title: String,
    /// The binary image of the recipe as jpg.
    #[serde(rename = "image")]
    pub image: &std::path::Path,
    /// The ingredient list of the recipe, one ingredient per line (separate lines with \\n).
    #[serde(rename = "ingredients")]
    pub ingredients: String,
    /// The instructions to make the recipe. One step per line (separate lines with \\n).
    #[serde(rename = "instructions")]
    pub instructions: String,
    /// The number of minutes it takes to get the recipe on the table.
    #[serde(rename = "readyInMinutes")]
    pub ready_in_minutes: f32,
    /// The number of servings the recipe makes.
    #[serde(rename = "servings")]
    pub servings: f32,
    /// The mask to put over the recipe image (\"ellipseMask\", \"diamondMask\", \"starMask\", \"heartMask\", \"potMask\", \"fishMask\").
    #[serde(rename = "mask")]
    pub mask: String,
    /// The background image (\"none\",\"background1\", or \"background2\").
    #[serde(rename = "backgroundImage")]
    pub background_image: String,
    /// The author of the recipe.
    #[serde(rename = "author")]
    pub author: Option<String>,
    /// The background color for the recipe card as a hex-string.
    #[serde(rename = "backgroundColor")]
    pub background_color: Option<String>,
    /// The font color for the recipe card as a hex-string.
    #[serde(rename = "fontColor")]
    pub font_color: Option<String>,
    /// The source of the recipe.
    #[serde(rename = "source")]
    pub source: Option<String>,
}

impl InlineObject3 {
    pub fn new(title: String, image: &std::path::Path, ingredients: String, instructions: String, ready_in_minutes: f32, servings: f32, mask: String, background_image: String) -> InlineObject3 {
        InlineObject3 {
            title: title,
            image: image,
            ingredients: ingredients,
            instructions: instructions,
            ready_in_minutes: ready_in_minutes,
            servings: servings,
            mask: mask,
            background_image: background_image,
            author: None,
            background_color: None,
            font_color: None,
            source: None,
        }
    }
}
