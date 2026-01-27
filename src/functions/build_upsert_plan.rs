use crate::{
    functions::is_same_data::is_same_data,
    structs::{plan::UpsertSchoolPlan, school::School},
};

/// Build upsert plan for upserting schools into database.
pub fn build_upsert_plan(
    schools: &[School],
    data: School,
) -> Option<UpsertSchoolPlan> {
    let current: Option<&School> = schools
        .iter()
        .find(|s| s.school_id == data.school_id && s.lang == data.lang);

    match current {
        | Some(existing) if is_same_data(existing, &data) => None,
        | Some(_) => Some(UpsertSchoolPlan::Update { data }),
        | None => Some(UpsertSchoolPlan::Insert { data }),
    }
}
