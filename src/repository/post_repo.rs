use crate::config::db::DbPool;
use crate::models::Post;
use crate::schema::post::dsl::*;
use diesel::prelude::*;

pub struct PostRepository {
    pool: DbPool,
}

impl PostRepository {
    pub fn new(pool: DbPool) -> Self {
        PostRepository { pool }
    }
    pub fn get_all(&self) -> Result<Vec<Post>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("ไม่สามารถเชื่อมต่อฐานข้อมูล");
        post.load::<Post>(&mut conn)
    }
}
