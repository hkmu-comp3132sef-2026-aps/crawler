use dotenv_plus::get_var;

/// Get MongoDB URI.
pub fn get_mongodb_uri() -> String {
    match get_var("MONGODB_URI") {
        | Ok(v) => v,
        | Err(_) => "mongodb://localhost:27017".to_string(),
    }
}

/// Get MongoDB Database Name.
pub fn get_mongodb_db_name() -> String {
    match get_var("MONGODB_DB_NAME") {
        | Ok(v) => v,
        | Err(_) => "sch".to_string(),
    }
}
