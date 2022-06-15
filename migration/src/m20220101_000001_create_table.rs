use sea_orm_migration::{prelude::*, sea_orm::{Statement, ConnectionTrait}};

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let users = "create table users (
                            id int not null primary key
                            );";
        let categories  = "
                create table categories (
                    id serial not null primary key,
                    name varchar not null,
                    user_id int references users (id),
                    unique (name, user_id)
                );";
        let tracks = "
                create table tracks (
                  id serial not null,
                  category_id int not null references categories (id),
                  saved_at timestamp not null,
                  time interval not null,
                  primary key (id, saved_at)
                );";
        let hypertable = "select create_hypertable('tracks', 'saved_at');";
        let db = manager.get_connection();
        for query in [users, categories, tracks, hypertable] {
            let stmt = Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, query, []);
            ConnectionTrait::execute(db, stmt).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let drop_tracks = "drop table tracks";
        let drop_categories = "drop table categories";
        let drop_users = "drop table users";
        let db = manager.get_connection();
        for query in [drop_tracks, drop_categories, drop_users] {
            let stmt = Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, query, []);
            ConnectionTrait::execute(db, stmt).await?;
        }
        Ok(())
    }
}
