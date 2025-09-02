use diesel_migrations::{EmbeddedMigrations, embed_migrations};

pub mod schema;
pub mod models;

pub const MIGRATIONS_POSTGRES: EmbeddedMigrations = embed_migrations!("./migrations/");
