use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

use crate::menu::models::Duserreq;


pub fn create_d_user(user: &Duserreq) -> Duserreq  {
    use crate::schema::destino_users;

    let mut conn = establish_connection();

    diesel::insert_into(destino_users::table)
        .values(user)
        .get_result(&mut conn)
        .expect("Error saving new post")
}