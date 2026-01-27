use serde::{Deserialize, Serialize};

/// Raw school data from EDB.
#[derive(Debug, Deserialize)]
pub struct SchoolRaw {
    #[serde(rename = "SCHOOL NO.")]
    pub school_no: i64,

    #[serde(rename = "ENGLISH CATEGORY")]
    pub category: String,
    #[serde(rename = "ENGLISH NAME")]
    pub name: String,
    #[serde(rename = "ENGLISH ADDRESS")]
    pub address: String,
    #[serde(rename = "LONGITUDE")]
    pub longitude: f64,
    #[serde(rename = "LATITUDE")]
    pub latitude: f64,
    #[serde(rename = "EASTING")]
    pub easting: f64,
    #[serde(rename = "NORTHING")]
    pub northing: f64,
    #[serde(rename = "STUDENTS GENDER")]
    pub students_gender: String,
    #[serde(rename = "SESSION")]
    pub session: String,
    #[serde(rename = "DISTRICT")]
    pub district: String,
    #[serde(rename = "FINANCE TYPE")]
    pub finance_type: String,
    #[serde(rename = "SCHOOL LEVEL")]
    pub level: String,
    #[serde(rename = "TELEPHONE")]
    pub telephone: String,
    #[serde(rename = "FAX NUMBER")]
    pub fax: String,

    // zh-hant
    pub 中文類別: String,
    pub 中文名稱: String,
    pub 中文地址: String,
    pub 經度: f64,
    pub 緯度: f64,
    pub 坐標東: f64,
    pub 坐標北: f64,
    pub 就讀學生性別: String,
    pub 學校授課時間: String,
    pub 分區: String,
    pub 資助種類: String,
    pub 學校類型: String,
    pub 聯絡電話: String,
    pub 傳真號碼: String,
}

/// Structured school data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct School {
    pub school_id: i64,
    pub lang: String,
    pub category: String,
    pub name: String,
    pub address: String,
    pub longitude: f64,
    pub latitude: f64,
    pub easting: f64,
    pub northing: f64,
    pub students_gender: String,
    pub session: String,
    pub district: String,
    pub finance_type: String,
    pub level: String,
    pub telephone: String,
    pub fax: String,
}
