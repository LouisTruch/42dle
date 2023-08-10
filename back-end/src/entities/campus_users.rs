//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3



use sea_orm :: entity :: prelude :: * ;

# [derive (Clone , Debug , PartialEq , DeriveEntityModel , Eq)] # [sea_orm (table_name = "campus_users")] pub struct Model { # [sea_orm (primary_key , auto_increment = false)] pub login : String , pub first_name : String , pub last_name : String , pub profile_pic : String , }

# [derive (Copy , Clone , Debug , EnumIter , DeriveRelation)] pub enum Relation { }

impl ActiveModelBehavior for ActiveModel { }