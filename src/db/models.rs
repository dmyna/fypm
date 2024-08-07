use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

use crate::db::schema::{worktimes, filters};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = worktimes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Worktime {
    pub id: String,
    pub name: String,
    pub description: String,
    pub style: String,
    pub start_time: String,
    pub end_time: String,
    pub polybar_background: String,
    pub polybar_foreground: String,
}

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = filters)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Filter {
    pub id: String,
    pub name: String,
    pub filter: String,
}
