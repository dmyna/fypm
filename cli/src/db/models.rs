////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

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
