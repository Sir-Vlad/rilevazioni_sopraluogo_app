use diesel_migrations::{embed_migrations, EmbeddedMigrations};

mod models;
mod schema;

pub const MIGRATIONS_POSTGRES: EmbeddedMigrations = embed_migrations!("./migrations/postgres/");