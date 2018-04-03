#[macro_use]
extern crate serde_json;
extern crate chrono;

use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::posts;

use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
use serde_json::{Value, Error};

[table_name = "posts"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
struct Post {
	id: Option<i32>
	name: String,
	body: String,
	link: String,
	date: chrono::DateTime::Date,
}

impl Post {
	pub fn create(post: Post, connection: &MysqlConnection) -> Post {
	        diesel::insert_into(posts::table)
	            .values(&post)
	            .execute(connection)
	            .expect("Error creating new hero");
	
	        posts::table.order(posts::id.desc()).first(connection).unwrap()
	    }
	
	    pub fn read(connection: &MysqlConnection) -> Vec<Hero> {
	        posts::table.order(posts::id.asc()).load::<Post>(connection).unwrap()
	    }
	
	    pub fn update(id: i32, post: Post, connection: &MysqlConnection) -> bool {
	        let query = diesel::update(posts::table.find(id)).set(&post).execute(connection);
	        !query.is_err()
	    }
	
	    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
	        let query = diesel::delete(posts::table.find(id)).execute(connection);
	        !query.is_err()
	    }
}
