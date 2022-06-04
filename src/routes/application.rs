use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::response::IntoResponse;
use crate::schema::application;
use crate::schema::application::dsl::application as application_table;
use crate::{Database, Response};

#[derive(JsonSchema, Queryable, Serialize)]
pub struct Application {
    application_id: i64,
    jobboard_id: i64,
    vacancy_id: i64,
    first_name: Option<String>,
    last_name: String,
    email: Option<String>,
    url_resume: Option<String>,
    url_extra_1: Option<String>,
    url_extra_2: Option<String>,
    url_extra_3: Option<String>,
    verified: bool,
    status: String,
}

#[derive(JsonSchema, Deserialize, Insertable)]
#[serde(deny_unknown_fields)]
#[table_name = "application"]
pub struct NewApplication {
    jobboard_id: i64,
    vacancy_id: i64,
    first_name: Option<String>,
    last_name: String,
    email: Option<String>,
    url_resume: Option<String>,
    url_extra_1: Option<String>,
    url_extra_2: Option<String>,
    url_extra_3: Option<String>,
}

#[derive(JsonSchema, Deserialize, AsChangeset)]
#[serde(deny_unknown_fields)]
#[table_name = "application"]
pub struct ApplicationChangeset {
    verified: bool,
    status: String,
}

#[openapi(tag = "Application")]
#[get("/application")]
pub async fn get_all_applications(database: Database) -> Response<Vec<Application>> {
    database.get_all(application_table).await.into_response(Status::Ok)
}

#[openapi(tag = "Application")]
#[post("/application", data = "<new_application>")]
pub async fn add_new_application(new_application: Json<NewApplication>, database: Database) -> Response<Application> {
    database
        .create(application_table, new_application.into_inner())
        .await
        .into_response(Status::Created)
}

#[openapi(tag = "Application")]
#[get("/application/<application_id>")]
pub async fn get_application(application_id: i64, database: Database) -> Response<Application> {
    database
        .get(application_table, application_id)
        .await
        .into_response(Status::Ok)
}

#[openapi(tag = "Application")]
#[put("/application/<application_id>", data = "<application_changeset>")]
pub async fn update_application(
    application_id: i64,
    application_changeset: Json<ApplicationChangeset>,
    database: Database,
) -> Response<Application> {
    database
        .update(application_table, application_id, application_changeset.into_inner())
        .await
        .into_response(Status::Ok)
}

#[openapi(tag = "Application")]
#[delete("/application/<application_id>")]
pub async fn delete_application(application_id: i64, database: Database) -> Response<()> {
    database
        .delete(application_table, application_id)
        .await
        .into_response(Status::NoContent)
}
