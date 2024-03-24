use sea_orm_migration::{prelude::*, sea_orm::Set};

use dokiray_entity::user;
use sea_orm_migration::sea_orm::ActiveModelTrait;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::CreateTime)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::UpdateTime).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::Email).string())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .col(ColumnDef::new(User::Role).string_len(20).default("admin"))
                    .to_owned(),
            )
            .await?;

        let db = manager.get_connection();
        let admin = user::ActiveModel {
            name: Set("admin".to_owned()),
            password: Set("fdasdfa".to_owned()),
            ..Default::default()
        };
        admin.insert(db).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    CreateTime,
    UpdateTime,
    Email,
    #[iden = "password"]
    Password,
    Role,
}

#[derive(Iden)]
enum Link {
    Table,
    Id,
    Url,
    CreateTime,
    UpdateTime,
    Ios,
    #[iden = "password"]
    Password,
    Role,
}
