use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Export function
pub fn establish_connection_pool() -> DbPool {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("ต้องกำหนด DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("ไม่สามารถสร้าง connection pool")
}
