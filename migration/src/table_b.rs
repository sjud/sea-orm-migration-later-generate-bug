use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum TableB{
    Table,
    Key,
    OtherTable,
}

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "table_b"
    }
}

pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(TableB::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TableB::Key)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(TableB::OtherTable)
                            .integer()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .drop_table(Table::drop().table(TableB::Table).to_owned())
            .await
    }
}


