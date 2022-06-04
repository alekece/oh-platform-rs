use diesel::associations::HasTable;
use diesel::query_builder::{AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget, UpdateStatement};
use diesel::query_dsl::methods::{ExecuteDsl, FindDsl};
use diesel::query_dsl::LoadQuery;
use diesel::result::Error;
use diesel::{Insertable, RunQueryDsl};
use rocket_okapi::request::OpenApiFromRequest;
use rocket_sync_db_pools::{database, diesel::PgConnection};

type Connection = PgConnection;

#[derive(OpenApiFromRequest)]
#[database("main")]
pub struct Database(Connection);

impl Database {
    pub async fn get<T, I, R>(&self, table: T, id: I) -> Result<R, Error>
    where
        T: FindDsl<I> + Send + 'static,
        T::Output: RunQueryDsl<Connection> + LoadQuery<Connection, R> + Send + 'static,
        I: Send + 'static,
        R: Send + 'static,
    {
        self.run(move |connection| {
            table.find(id).load(connection).and_then(|mut values| {
                if values.is_empty() {
                    Err(Error::NotFound)
                } else {
                    Ok(values.remove(0))
                }
            })
        })
        .await
    }

    pub async fn get_all<T, R>(&self, table: T) -> Result<Vec<R>, Error>
    where
        T: RunQueryDsl<Connection> + LoadQuery<Connection, R> + Send + 'static,
        R: Send + 'static,
    {
        self.run(|connection| table.load(connection)).await
    }

    pub async fn create<T, U, R>(&self, table: T, new_resource: U) -> Result<R, Error>
    where
        U: Insertable<T> + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
        InsertStatement<T, U::Values>: LoadQuery<Connection, R>,
    {
        self.run(move |connection| diesel::insert_into(table).values(new_resource).get_result(connection))
            .await
    }

    pub async fn update<T, I, U, R>(&self, table: T, id: I, resource_changeset: U) -> Result<R, Error>
    where
        T: FindDsl<I> + Send + 'static,
        T::Output: IntoUpdateTarget + HasTable,
        U: AsChangeset<Target = <T::Output as HasTable>::Table> + Send + 'static,
        I: Send + 'static,
        R: Send + 'static,
        UpdateStatement<<T::Output as HasTable>::Table, <T::Output as IntoUpdateTarget>::WhereClause, U::Changeset>:
            LoadQuery<Connection, R>,
    {
        self.run(|connection| {
            diesel::update(table.find(id))
                .set(resource_changeset)
                .get_result(connection)
        })
        .await
    }

    pub async fn delete<T, I>(&self, table: T, id: I) -> Result<(), Error>
    where
        T: FindDsl<I> + Send + 'static,
        T::Output: IntoUpdateTarget + HasTable,
        I: Send + 'static,
        DeleteStatement<<T::Output as HasTable>::Table, <T::Output as IntoUpdateTarget>::WhereClause>:
            ExecuteDsl<Connection>,
    {
        self.run(move |connection| {
            diesel::delete(table.find(id))
                .execute(connection)
                .and_then(|count| match count {
                    0 => Err(Error::NotFound),
                    _ => Ok(()),
                })
        })
        .await
    }
}
