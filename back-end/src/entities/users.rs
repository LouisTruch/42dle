//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3



use sea_orm :: entity :: prelude :: * ;


// Model for the users table: src --> https://www.youtube.com/watch?v=OW3paoAqm1U&t=1510s
# [derive (Clone , Debug , PartialEq , DeriveEntityModel , Eq)] 
# [sea_orm (table_name = "users")] 
pub struct Model { 
    # [sea_orm (primary_key , auto_increment = false)] 
    pub login : String , 
    pub profile_pic : String , 
    pub score : Option < i32 > , 
}

# [derive (Copy , Clone , Debug , EnumIter , DeriveRelation)] pub enum Relation { }

impl ActiveModelBehavior for ActiveModel { }