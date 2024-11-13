


use serde_json;
use super::types::Skill;


/// Serializes a slice of `Skill` structs into a JSON string.
///
/// # Arguments
///
/// * `skills` - A slice of `Skill` structs to serialize.
///
/// # Returns
///
/// * `Ok(String)` containing the JSON representation if successful.
/// * `Err(serde_json::Error)` if serialization fails.
pub fn serialize_skills(skills: &[Skill]) -> Result<String, serde_json::Error> {
    serde_json::to_string(skills)
}

/// Deserializes a JSON string into a vector of `Skill` structs.
///
/// # Arguments
///
/// * `json_data` - A JSON string representing an array of skills.
///
/// # Returns
///
/// * `Ok(Vec<Skill>)` containing the deserialized skills if successful.
/// * `Err(serde_json::Error)` if deserialization fails.
pub fn deserialize_skills(json_data: &str) -> Result<Vec<Skill>, serde_json::Error> {
    serde_json::from_str(json_data)
}
