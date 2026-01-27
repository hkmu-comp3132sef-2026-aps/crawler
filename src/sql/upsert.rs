use mongodb::{
    Client, Collection,
    bson::doc,
    error::Error as MongoError,
    options::{InsertOneModel, UpdateOneModel},
};

use crate::{
    configs::database::{get_client, get_schools},
    structs::{plan::UpsertSchoolPlan, school::School},
};

/// Upsert schools by plans.
pub async fn upsert_schools_by_plans(
    plans: Vec<UpsertSchoolPlan>
) -> Result<(), MongoError> {
    println!("Preparing to upsert schools...");

    let client: Client = get_client().await;

    let schools: Collection<School> = get_schools().await;

    let mut models_insert: Vec<InsertOneModel> = Vec::new();

    let mut models_update: Vec<UpdateOneModel> = Vec::new();

    for plan in plans {
        match plan {
            | UpsertSchoolPlan::Insert { data, .. } => {
                let model: InsertOneModel = InsertOneModel::builder()
                    .namespace(schools.namespace())
                    .document(doc! {
                        "school_id": data.school_id,
                        "lang": data.lang,
                        "category": data.category,
                        "name": data.name,
                        "address": data.address,
                        "longitude": data.longitude,
                        "latitude": data.latitude,
                        "easting": data.easting,
                        "northing": data.northing,
                        "students_gender": data.students_gender,
                        "session": data.session,
                        "district": data.district,
                        "finance_type": data.finance_type,
                        "level": data.level,
                        "telephone": data.telephone,
                        "fax": data.fax,
                    })
                    .build();

                models_insert.push(model);
            },
            | UpsertSchoolPlan::Update { data, .. } => {
                let model: UpdateOneModel = UpdateOneModel::builder()
                    .namespace(schools.namespace())
                    .filter(doc! {
                        "school_id": data.school_id,
                        "lang": data.lang,
                    })
                    .update(doc! {
                        "$set": {
                            "category": data.category,
                        "address": data.address,
                        "longitude": data.longitude,
                        "latitude": data.latitude,
                        "easting": data.easting,
                        "northing": data.northing,
                        "students_gender": data.students_gender,
                        "session": data.session,
                        "district": data.district,
                        "finance_type": data.finance_type,
                        "level": data.level,
                        "telephone": data.telephone,
                        "fax": data.fax,
                        }
                    })
                    .build();

                models_update.push(model);
            },
        }
    }

    println!("Upserting schools...");

    if !models_insert.is_empty() {
        client.bulk_write(models_insert).await?;
    }

    if !models_update.is_empty() {
        client.bulk_write(models_update).await?;
    }

    Ok(())
}
