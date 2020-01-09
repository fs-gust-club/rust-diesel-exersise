pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use std::env;

use crate::schema::rooms::dsl::*;
use models::{NewRoom, Room};

fn establish_connection() -> SqliteConnection {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect("UNABLE TO CONNECT TO DB")
}

fn main() {
    let connection = establish_connection();
    create_room(&connection);

    let rooms_vec = get_rooms(&connection);

    for room in rooms_vec {
        println!("Room: {}, Description: {:?}, ID: {}", room.name, room.description, room.id);
    }
}

pub fn create_room(conn: &SqliteConnection) -> usize {
    use schema::rooms;

    let new_room = NewRoom {
        name: "hello",
        description: "World",
    };

    diesel::insert_into(rooms::table)
        .values(&new_room)
        .execute(conn)
        .expect("Error saving new post")

}

pub fn get_rooms(conn: &SqliteConnection) -> Vec<Room> {
    // use schema::rooms;

    rooms.load::<Room>(conn).expect("Failed to retrieve data")
}
