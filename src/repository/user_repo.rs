use crate::config::db::DbPool;
use crate::models::User;
use crate::schema::users::dsl::*;
use diesel::prelude::*;

pub struct UserRepository {
    pool: DbPool,
}

impl UserRepository {
    // สร้าง instance ใหม่ของ UserRepository โดยรับ DbPool
    pub fn new(pool: DbPool) -> Self {
        UserRepository { pool }
    }

    // Method get_all ไม่ต้องรับ pool อีกต่อไป
    pub fn get_all(&self) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("ไม่สามารถเชื่อมต่อฐานข้อมูล");
        users.load::<User>(&mut conn)
    }
}
