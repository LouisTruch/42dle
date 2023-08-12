use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Users::Login)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Users::ProfilePic).string().not_null())
                    .col(ColumnDef::new(Users::Score).integer().default(0).not_null())
                    .col(ColumnDef::new(Users::Try).array(ColumnType::String(Some(12))).not_null())
                    .col(ColumnDef::new(Users::Win).boolean().not_null().default(false))
                    .col(ColumnDef::new(Users::Student).boolean().not_null().default(true))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                .table(Game::Table)
                .if_not_exists()
                .col(ColumnDef::new(Game::Id).integer().not_null().primary_key().default(1))
                .col(ColumnDef::new(Game::LoginToFind).string().not_null())
                .col(ColumnDef::new(Game::FirstName).string().not_null())
                .col(ColumnDef::new(Game::LastName).string().not_null())
                .col(ColumnDef::new(Game::ProfilePic).string().not_null())
                .to_owned()
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                .table(StudentUsers::Table)
                .if_not_exists()
                .col(ColumnDef::new(StudentUsers::Login).string().not_null().primary_key())
                .col(ColumnDef::new(StudentUsers::FirstName).string().not_null())
                .col(ColumnDef::new(StudentUsers::LastName).string().not_null())
                .col(ColumnDef::new(StudentUsers::ProfilePic).string().not_null())
                .to_owned()
            )
            .await
            .unwrap();

        manager
            .create_table(
                Table::create()
                .table(PoolUsers::Table)
                .if_not_exists()
                .col(ColumnDef::new(PoolUsers::Login).string().not_null().primary_key())
                .col(ColumnDef::new(PoolUsers::FirstName).string().not_null())
                .col(ColumnDef::new(PoolUsers::LastName).string().not_null())
                .col(ColumnDef::new(PoolUsers::ProfilePic).string().not_null())
                .to_owned()
            )
            .await

 
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(StudentUsers::Table).to_owned())
            .await
            .unwrap();

        manager
            .drop_table(Table::drop().table(PoolUsers::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden

#[derive(Iden)]
enum Users {
    Table,
    Login,
    ProfilePic,
    Score,
    Try,
    Win,
    Student
}

#[derive(Iden)]
enum Game {
    Table,
    Id,
    LoginToFind,
    FirstName,
    LastName,
    ProfilePic,
}

#[derive(Iden)]
enum StudentUsers {
    Table,
    Login,
    FirstName,
    LastName,
    ProfilePic,
}

#[derive(Iden)]
enum PoolUsers {
    Table,
    Login,
    FirstName,
    LastName,
    ProfilePic,
}