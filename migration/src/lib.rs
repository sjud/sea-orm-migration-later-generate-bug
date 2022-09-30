pub use sea_orm_migration::prelude::*;

mod table_a;
mod other_table_a;
mod table_b;
mod other_table_b;
mod alter_bug;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(table_a::Migration),
            Box::new(table_b::Migration),
            Box::new(other_table_a::Migration),
            Box::new(other_table_b::Migration),
            Box::new(alter_bug::Migration),
        ]
    }
}
