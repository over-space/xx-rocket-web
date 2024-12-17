use rocket::State;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, PaginatorTrait};

use crate::employee;

pub async fn get_employee(db: &State<DatabaseConnection>, id: u32) -> Result<Option<employee::Model>, DbErr> {
    employee::Entity::find_by_id(id).one(db.inner()).await
}

pub async fn list_employee(db: &State<DatabaseConnection>, page:u64, size:u64) -> Result<Vec<employee::Model>, DbErr> {
    employee::Entity::find().paginate(db.inner(), size).fetch_page(page).await
}