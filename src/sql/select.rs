use mongodb::{Collection, Cursor, bson::Document, error::Error as MongoError};

use crate::{configs::database::get_schools, structs::school::School};

/// Select all schools from database.
pub async fn select_schools() -> Result<Vec<School>, MongoError> {
    let schools: Collection<School> = get_schools().await;

    let mut cursor: Cursor<School> = schools.find(Document::default()).await?;

    let mut result: Vec<School> = Vec::new();

    while cursor.advance().await? {
        let school: School = cursor.deserialize_current()?;

        result.push(school);
    }

    Ok(result)
}
