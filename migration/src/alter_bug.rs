
use sea_orm_migration::prelude::*;
use crate::other_table_a::OtherTableA;
use crate::other_table_b::OtherTableB;
use crate::table_a::TableA;
use crate::table_b::TableB;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        file!()
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table_a_key = TableForeignKey::new()
            .name("other_table")
            .from_tbl(TableA::Table)
            .from_col(TableA::OtherTable)
            .to_tbl(OtherTableA::Table)
            .to_col(OtherTableA::Key)
            .to_owned();
        let table_b_key = TableForeignKey::new()
            .name("other_table")
            .from_tbl(TableB::Table)
            .from_col(TableB::OtherTable)
            .to_tbl(OtherTableB::Table)
            .to_col(OtherTableB::Key)
            .to_owned();
        let table_a_alter = Table::alter()
            .table(TableA::Table)
            .add_foreign_key(&table_a_key)
            .to_owned();
        let table_b_alter = Table::alter()
            .table(TableB::Table)
            .add_foreign_key(&table_b_key)
            .to_owned();

        manager.alter_table(table_a_alter).await?;
        manager.alter_table(table_b_alter).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let table_a = Table::alter()
            .table(TableA::Table)
            .drop_foreign_key(TableA::OtherTable)
            .to_owned();

        manager.alter_table(table_a).await?;

        let table_b = Table::alter()
            .table(TableB::Table)
            .drop_foreign_key(TableB::OtherTable)
            .to_owned();
        manager.alter_table(table_b).await?;


        Ok(())
    }
}
