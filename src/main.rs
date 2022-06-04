use eyre::Report;
use oh_platform::{catchers, routes, Database};
use rocket::catchers;
use rocket_okapi::swagger_ui::{self as swagger, SwaggerUIConfig};

#[rocket::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    rocket::build()
        .attach(Database::fairing())
        .mount(
            "/v1/",
            rocket_okapi::openapi_get_routes![
                routes::get_all_jobboards,
                routes::add_new_jobboard,
                routes::get_jobboard,
                routes::update_jobboard,
                routes::delete_jobboard,
                routes::get_all_companies,
                routes::add_new_company,
                routes::get_company,
                routes::update_company,
                routes::delete_company,
                routes::get_all_vacancies,
                routes::add_new_vacancy,
                routes::get_vacancy,
                routes::update_vacancy,
                routes::delete_vacancy,
                routes::get_all_applications,
                routes::add_new_application,
                routes::get_application,
                routes::update_application,
                routes::delete_application
            ],
        )
        .mount(
            "/swagger/",
            swagger::make_swagger_ui(&SwaggerUIConfig {
                url: "../v1/openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .register(
            "/",
            catchers![
                catchers::not_found,
                catchers::bad_request,
                catchers::unprocessable_entity,
            ],
        )
        .launch()
        .await?;

    Ok(())
}
