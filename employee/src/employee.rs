use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

pub type EmployeeEntity  = crate::employee::Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)] // 添加 DeriveEntityModel
#[sea_orm(table_name = "t_employee")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub age: u8,
    pub email: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}