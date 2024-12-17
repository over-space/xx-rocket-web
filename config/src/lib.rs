use std::sync::Arc;

use sea_orm::DbConn;

pub mod init;
pub type MySQLPool = Arc<DbConn>;