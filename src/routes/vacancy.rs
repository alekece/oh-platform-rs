use std::ops::Bound;

use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::response::IntoResponse;
use crate::schema::vacancy;
use crate::schema::vacancy::dsl::vacancy as vacancy_table;
use crate::{Database, Response};

#[derive(JsonSchema, Queryable, Serialize)]
pub struct Vacancy {
    vacancy_id: i64,
    jobboard_id: i64,
    company_id: i64,
    job_title: String,
    location: Option<String>,
    start_date: Option<DateTime<Utc>>,
    directly: Option<bool>,
    hours: (Bound<i32>, Bound<i32>),
    positions: Option<i16>,
    responsibilities: Option<String>,
    skills: Option<String>,
    conditions: Option<String>,
    description: Option<String>,
    url: Option<String>,
    commission: Option<i16>,
    status: String,
    verified: bool,
    active: bool,
}

#[derive(JsonSchema, Deserialize, Insertable)]
#[serde(deny_unknown_fields)]
#[table_name = "vacancy"]
pub struct NewVacancy {
    jobboard_id: i64,
    vacancy_id: i64,
    job_title: String,
    location: Option<String>,
    start_date: Option<DateTime<Utc>>,
    directly: Option<bool>,
    hours: (Bound<i32>, Bound<i32>),
    positions: Option<i16>,
    responsibilities: Option<String>,
    skills: Option<String>,
    conditions: Option<String>,
    description: Option<String>,
    url: Option<String>,
    commission: Option<i16>,
}

#[derive(JsonSchema, Deserialize, AsChangeset)]
#[serde(deny_unknown_fields)]
#[table_name = "vacancy"]
pub struct VacancyChangeset {
    status: String,
    verified: bool,
    active: bool,
}

#[openapi(tag = "Vacancy")]
#[get("/vacancy")]
pub async fn get_all_vacancies(database: Database) -> Response<Vec<Vacancy>> {
    database.get_all(vacancy_table).await.into_response(Status::Ok)
}

#[openapi(tag = "Vacancy")]
#[post("/vacancy", data = "<new_vacancy>")]
pub async fn add_new_vacancy(new_vacancy: Json<NewVacancy>, database: Database) -> Response<Vacancy> {
    database
        .create(vacancy_table, new_vacancy.into_inner())
        .await
        .into_response(Status::Created)
}

#[openapi(tag = "Vacancy")]
#[get("/vacancy/<vacancy_id>")]
pub async fn get_vacancy(vacancy_id: i64, database: Database) -> Response<Vacancy> {
    database.get(vacancy_table, vacancy_id).await.into_response(Status::Ok)
}

#[openapi(tag = "Vacancy")]
#[put("/vacancy/<vacancy_id>", data = "<vacancy_changeset>")]
pub async fn update_vacancy(
    vacancy_id: i64,
    vacancy_changeset: Json<VacancyChangeset>,
    database: Database,
) -> Response<Vacancy> {
    database
        .update(vacancy_table, vacancy_id, vacancy_changeset.into_inner())
        .await
        .into_response(Status::Ok)
}

#[openapi(tag = "Vacancy")]
#[delete("/vacancy/<vacancy_id>")]
pub async fn delete_vacancy(vacancy_id: i64, database: Database) -> Response<()> {
    database
        .delete(vacancy_table, vacancy_id)
        .await
        .into_response(Status::NoContent)
}
