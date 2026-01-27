use crate::{
    functions::build_upsert_plan::build_upsert_plan,
    sql::{select::select_schools, upsert::upsert_schools_by_plans},
    structs::{
        plan::UpsertSchoolPlan,
        school::{School, SchoolRaw},
    },
};

const URL_SCHOOL_DATA: &str = "https://www.edb.gov.hk/attachment/en/student-parents/sch-info/sch-search/sch-location-info/SCH_LOC_EDB.json";

/// Fetch schools from EDB and upsert to MongoDB database.
pub async fn fetch_schools() -> anyhow::Result<()> {
    println!("Fetching schools...");

    let raw: Vec<SchoolRaw> =
        reqwest::get(URL_SCHOOL_DATA).await?.json().await?;

    let schools: Vec<School> = select_schools().await?;

    let mut plans: Vec<UpsertSchoolPlan> = Vec::new();

    println!("Building school upsert plans...");

    for data in raw {
        let school_id: String = data.school_no.to_string();

        let en = School {
            school_id: school_id.clone(),
            lang: "en".to_string(),
            category: data.category,
            name: data.name,
            address: data.address,
            longitude: data.longitude,
            latitude: data.latitude,
            easting: data.easting,
            northing: data.northing,
            students_gender: data.students_gender,
            session: data.session,
            district: data.district,
            finance_type: data.finance_type,
            level: data.level,
            telephone: data.telephone,
            fax: data.fax,
        };

        if let Some(plan) = build_upsert_plan(&schools, en) {
            plans.push(plan);
        }

        let zh = School {
            school_id,
            lang: "zh-hant".to_string(),
            category: data.中文類別,
            name: data.中文名稱,
            address: data.中文地址,
            longitude: data.經度,
            latitude: data.緯度,
            easting: data.坐標東,
            northing: data.坐標北,
            students_gender: data.就讀學生性別,
            session: data.學校授課時間,
            district: data.分區,
            finance_type: data.資助種類,
            level: data.學校類型,
            telephone: data.聯絡電話,
            fax: data.傳真號碼,
        };

        if let Some(plan) = build_upsert_plan(&schools, zh) {
            plans.push(plan);
        }
    }

    upsert_schools_by_plans(plans).await?;

    Ok(())
}
