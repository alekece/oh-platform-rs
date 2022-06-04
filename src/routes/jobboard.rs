use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{delete, get, post, put};
use rocket_okapi::openapi;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::response::IntoResponse;
use crate::schema::jobboard;
use crate::schema::jobboard::dsl::jobboard as jobboard_table;
use crate::{Database, Response};

#[derive(JsonSchema, Queryable, Serialize)]
pub struct Jobboard {
    jobboard_id: i64,
    jobboard_name: String,
    url: Option<String>,
    account: String,
    key: Option<String>,
    timestamp: Option<DateTime<Utc>>,
    verified: bool,
    active: bool,
}

#[derive(JsonSchema, Deserialize, Insertable)]
#[serde(deny_unknown_fields)]
#[table_name = "jobboard"]
pub struct NewJobboard {
    jobboard_name: String,
    url: Option<String>,
    account: String,
    key: Option<String>,
}

#[derive(JsonSchema, Deserialize, AsChangeset)]
#[serde(deny_unknown_fields)]
#[table_name = "jobboard"]
pub struct JobboardChangeset {
    verified: bool,
    active: bool,
}

#[openapi(tag = "Jobboard")]
#[get("/jobboard")]
pub async fn get_all_jobboards(database: Database) -> Response<Vec<Jobboard>> {
    database.get_all(jobboard_table).await.into_response(Status::Ok)
}

#[openapi(tag = "Jobboard")]
#[post("/jobboard", data = "<new_jobboard>")]
pub async fn add_new_jobboard(new_jobboard: Json<NewJobboard>, database: Database) -> Response<Jobboard> {
    database
        .create(jobboard_table, new_jobboard.into_inner())
        .await
        .into_response(Status::Created)
}

#[openapi(tag = "Jobboard")]
#[get("/jobboard/<jobboard_id>")]
pub async fn get_jobboard(jobboard_id: i64, database: Database) -> Response<Jobboard> {
    database
        .get(jobboard_table, jobboard_id)
        .await
        .into_response(Status::Ok)
}

#[openapi(tag = "Jobboard")]
#[put("/jobboard/<jobboard_id>", data = "<jobboard_changeset>")]
pub async fn update_jobboard(
    jobboard_id: i64,
    jobboard_changeset: Json<JobboardChangeset>,
    database: Database,
) -> Response<Jobboard> {
    database
        .update(jobboard_table, jobboard_id, jobboard_changeset.into_inner())
        .await
        .into_response(Status::Ok)
}

#[openapi(tag = "Jobboard")]
#[delete("/jobboard/<jobboard_id>")]
pub async fn delete_jobboard(jobboard_id: i64, database: Database) -> Response<()> {
    database
        .delete(jobboard_table, jobboard_id)
        .await
        .into_response(Status::NoContent)
}
