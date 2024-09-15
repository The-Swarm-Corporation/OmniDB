use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

// Define an enum for all supported data types in OmniDb
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataType {
    Integer(i64),             // Supports i32, i64 types
    UnsignedInteger(u64),      // Supports u32, u64 types
    Float(f64),                // Supports f32, f64 types
    Boolean(bool),             // Boolean type
    Text(String),              // Textual data
    Json(Value),               // Flexible JSON-like data
    Binary(Vec<u8>),           // Binary or blob data
    Vector(Vec<DataType>),     // Vector of any DataType
    Timestamp(NaiveDateTime),  // Date and time values
    File(PathBuf),             // File paths for file storage
    Null,
    Undefined,
    None,
}