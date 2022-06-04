table! {
    application (application_id) {
        application_id -> Int8,
        jobboard_id -> Int8,
        vacancy_id -> Int8,
        first_name -> Nullable<Varchar>,
        last_name -> Varchar,
        email -> Nullable<Varchar>,
        url_resume -> Nullable<Varchar>,
        url_extra_1 -> Nullable<Varchar>,
        url_extra_2 -> Nullable<Varchar>,
        url_extra_3 -> Nullable<Varchar>,
        verified -> Bool,
        status -> Varchar,
    }
}

table! {
    company (company_id) {
        company_id -> Int8,
        jobboard_id -> Int8,
        company_name -> Varchar,
        logo -> Nullable<Varchar>,
        website -> Varchar,
        description -> Nullable<Varchar>,
        region -> Nullable<Varchar>,
        timestamp -> Nullable<Timestamptz>,
        verified -> Bool,
        active -> Bool,
    }
}

table! {
    jobboard (jobboard_id) {
        jobboard_id -> Int8,
        jobboard_name -> Varchar,
        url -> Nullable<Varchar>,
        account -> Varchar,
        key -> Nullable<Varchar>,
        timestamp -> Nullable<Timestamptz>,
        verified -> Bool,
        active -> Bool,
    }
}

table! {
    vacancy (vacancy_id) {
        vacancy_id -> Int8,
        jobboard_id -> Int8,
        company_id -> Int8,
        job_title -> Varchar,
        location -> Nullable<Varchar>,
        start_date -> Nullable<Timestamptz>,
        directly -> Nullable<Bool>,
        hours -> Int4range,
        positions -> Nullable<Int2>,
        responsibilities -> Nullable<Varchar>,
        skills -> Nullable<Varchar>,
        conditions -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        url -> Nullable<Varchar>,
        commission -> Nullable<Int2>,
        status -> Varchar,
        verified -> Bool,
        active -> Bool,
    }
}

joinable!(application -> jobboard (jobboard_id));
joinable!(application -> vacancy (vacancy_id));
joinable!(company -> jobboard (jobboard_id));
joinable!(vacancy -> company (company_id));
joinable!(vacancy -> jobboard (jobboard_id));

allow_tables_to_appear_in_same_query!(application, company, jobboard, vacancy,);
