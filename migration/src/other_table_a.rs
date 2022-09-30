use sea_orm_migration::prelude::*;
use crate::table_a::TableA;

#[derive(Iden)]
pub enum OtherTableA{
    Table,
    Key,
    TableA
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "other_table_a"
    }
}

pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {


        manager
            .create_table(
                Table::create()
                    .table(OtherTableA::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OtherTableA::Key)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OtherTableA::TableA)
                            .integer()
                            .not_null()
                    ).foreign_key(
                    ForeignKey::create()
                        .name("table_a")
                        .to(TableA::Table, TableA::Key)
                        .from(OtherTableA::Table, OtherTableA::TableA)
                ).to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(OtherTableA::Table).to_owned())
            .await
    }
}


