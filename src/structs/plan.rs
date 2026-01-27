use crate::structs::school::School;

#[derive(Debug, Clone)]
pub enum UpsertSchoolPlan {
    Insert { data: School },
    Update { data: School },
}
