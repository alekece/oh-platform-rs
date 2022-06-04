use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::response::IntoResponse;
use crate::schema::company;
use crate::schema::company::dsl::company as company_table;
use crate::{Database, Response};

#[derive(JsonSchema, Queryable, Serialize)]
pub struct Company {
    company_id: i64,
    jobboard_id: i64,
    company_name: String,
    logo: Option<String>,
    website: String,
    description: Option<String>,
    region: Option<String>,
    timestamp: Option<DateTime<Utc>>,
    verified: bool,
    active: bool,
}

#[derive(JsonSchema, Deserialize, Insertable)]
#[serde(deny_unknown_fields)]
#[table_name = "company"]
pub struct NewCompany {
    jobboard_id: i64,
    company_name: String,
    logo: Option<String>,
    website: String,
    description: Option<String>,
    region: Option<String>,
}

#[derive(JsonSchema, Deserialize, AsChangeset)]
#[serde(deny_unknown_fields)]
#[table_name = "company"]
pub struct CompanyChangeset {
    verified: bool,
    active: bool,
}

#[openapi(tag = "Company")]
#[get("/company")]
pub async fn get_all_companies(database: Database) -> Response<Vec<Company>> {
    database.get_all(company_table).await.into_response(Status::Ok)
}

#[openapi(tag = "Company")]
#[post("/company", data = "<new_company>")]
pub async fn add_new_company(new_company: Json<NewCompany>, database: Database) -> Response<Company> {
    database
        .create(company_table, new_company.into_inner())
        .await
        .into_response(Status::Created)
}

#[openapi(tag = "Company")]
#[get("/company/<company_id>")]
pub async fn get_company(company_id: i64, database: Database) -> Response<Company> {
    database.get(company_table, company_id).await.into_response(Status::Ok)
}

#[openapi(tag = "Company")]
#[put("/company/<company_id>", data = "<company_changeset>")]
pub async fn update_company(
    company_id: i64,
    company_changeset: Json<CompanyChangeset>,
    database: Database,
) -> Response<Company> {
    database
        .update(company_table, company_id, company_changeset.into_inner())
        .await
        .into_response(Status::Ok)
}

#[openapi(tag = "Company")]
#[delete("/company/<company_id>")]
pub async fn delete_company(company_id: i64, database: Database) -> Response<()> {
    database
        .delete(company_table, company_id)
        .await
        .into_response(Status::NoContent)
}
