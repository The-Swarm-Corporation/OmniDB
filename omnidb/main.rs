use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

// Define a type alias for NoSQL-like flexible JSON data
type Document = Value;

// A Record struct for SQL-like tables with typed data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    id: u32,
    data: Document, // Can hold any type of data, stored as JSON
}

// A table structure that holds records for SQL-like storage
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub records: HashMap<u32, Record>,
}

impl Table {
    // Create a new table with the given name and columns
    pub fn new(name: &str, columns: Vec<String>) -> Self {
        Self {
            name: name.to_string(),
            columns,
            records: HashMap::new(),
        }
    }

    // Insert a record into the table
    pub fn insert(&mut self, id: u32, data: Document) {
        let record = Record { id, data };
        self.records.insert(id, record);
    }

    // Read a record by ID
    pub fn read(&self, id: u32) -> Option<&Record> {
        self.records.get(&id)
    }

    // Update a record by ID
    pub fn update(&mut self, id: u32, data: Document) -> Result<(), String> {
        if let Some(record) = self.records.get_mut(&id) {
            record.data = data;
            Ok(())
        } else {
            Err(format!("Record with id {} not found", id))
        }
    }

    // Delete a record by ID
    pub fn delete(&mut self, id: u32) -> bool {
        self.records.remove(&id).is_some()
    }
}

// Database struct to hold tables (SQL-like) and documents (NoSQL-like)
pub struct Database {
    pub tables: HashMap<String, Table>,
    pub documents: HashMap<String, Document>, // NoSQL-like document store
}

impl Database {
    // Create a new empty database
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
            documents: HashMap::new(),
        }
    }

    // Create a new table in the database
    pub fn create_table(&mut self, name: &str, columns: Vec<String>) {
        let table = Table::new(name, columns);
        self.tables.insert(name.to_string(), table);
    }

    // Insert a record into a table
    pub fn insert_into_table(&mut self, table_name: &str, id: u32, data: Document) -> Result<(), String> {
        if let Some(table) = self.tables.get_mut(table_name) {
            table.insert(id, data);
            Ok(())
        } else {
            Err(format!("Table {} not found", table_name))
        }
    }

    // Create a new document in the NoSQL-like document store
    pub fn create_document(&mut self, name: &str, doc: Document) {
        self.documents.insert(name.to_string(), doc);
    }

    // Read a document from the NoSQL-like document store
    pub fn read_document(&self, name: &str) -> Option<&Document> {
        self.documents.get(name)
    }

    // Update a document in the NoSQL-like document store
    pub fn update_document(&mut self, name: &str, doc: Document) -> Result<(), String> {
        if self.documents.contains_key(name) {
            self.documents.insert(name.to_string(), doc);
            Ok(())
        } else {
            Err(format!("Document {} not found", name))
        }
    }

    // Delete a document from the NoSQL-like document store
    pub fn delete_document(&mut self, name: &str) -> bool {
        self.documents.remove(name).is_some()
    }
}

fn main() {
    // Create a new database instance
    let mut db = Database::new();

    // Create a SQL-like table
    db.create_table("users", vec!["id".to_string(), "name".to_string(), "email".to_string()]);

    // Insert a record into the users table (SQL-like)
    let user_data = serde_json::json!({
        "name": "John Doe",
        "email": "john.doe@example.com"
    });
    db.insert_into_table("users", 1, user_data).expect("Failed to insert record");

    // Read the record
    if let Some(record) = db.tables["users"].read(1) {
        println!("User: {:?}", record);
    }

    // Update the record
    let updated_user_data = serde_json::json!({
        "name": "John Smith",
        "email": "john.smith@example.com"
    });
    db.tables.get_mut("users").unwrap().update(1, updated_user_data).expect("Failed to update record");

    // Delete the record
    db.tables.get_mut("users").unwrap().delete(1);

    // Insert a NoSQL-like document
    let doc_data = serde_json::json!({
        "title": "NoSQL Example",
        "content": "This is a NoSQL document."
    });
    db.create_document("doc1", doc_data);

    // Read the NoSQL document
    if let Some(document) = db.read_document("doc1") {
        println!("Document: {:?}", document);
    }
}
