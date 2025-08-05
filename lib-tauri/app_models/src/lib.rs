use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub mod models;
pub mod schema;

pub const MIGRATIONS_POSTGRES: EmbeddedMigrations = embed_migrations!("./migrations/postgres/");
