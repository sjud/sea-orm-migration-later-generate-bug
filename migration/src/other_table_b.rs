use sea_orm_migration::prelude::*;
use crate::table_b::TableB;

#[derive(Iden)]
pub enum OtherTableB{
    Table,
    Key,
    TableB
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "other_table_b"
    }
}
pub struct Migration;


#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(OtherTableB::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OtherTableB::Key)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OtherTableB::TableB)
                            .integer()
                            .not_null()
                    ).foreign_key(
                    ForeignKey::create()
                        .name("table_a")
                        .to(TableB::Table, TableB::Key)
                        .from(OtherTableB::Table, OtherTableB::TableB)
                ).to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(OtherTableB::Table).to_owned())
            .await
    }
}


