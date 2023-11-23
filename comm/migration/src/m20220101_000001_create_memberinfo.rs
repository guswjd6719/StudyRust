use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(MemberInfo::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(MemberInfo::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MemberInfo::Active)
                        .boolean()
                        .not_null()
                        .default(true))
                    .col(ColumnDef::new(MemberInfo::CreatedDate).timestamp_with_time_zone())
                    .col(ColumnDef::new(MemberInfo::Name).string())
                    .col(ColumnDef::new(MemberInfo::Password).string().not_null())
                    .col(ColumnDef::new(MemberInfo::Email).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MemberInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum MemberInfo {
    Table,
    Id,
    Active,
    CreatedDate,
    Name,
    Password,
    Email
}
