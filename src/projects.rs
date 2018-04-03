use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use schema::heroes;
use serde_json::{value,Error};

#[table_name = "projects"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
struct Project {
	name: String,
	description: String,
	link: String,
	file_link: String,
}

impl Project {
	pub fn create(project: Project, connection: &MysqlConnection) -> Project {
	        diesel::insert_into(projects::table)
	            .values(&project)
	            .execute(connection)
	            .expect("Error creating new project");
	
	        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
	    }
	
	    pub fn read(connection: &MysqlConnection) -> Vec<Project> {
	        project::table.order(project::id.asc()).load::<Project>(connection).unwrap()
	    }
	
	    pub fn update(name: String, project: Project, connection: &MysqlConnection) -> bool {
	        let query = diesel::update(project::table.find(id)).set(&project).execute(connection);
	        !query.is_err()
	    }
	
	    pub fn delete(name: String, connection: &MysqlConnection) -> bool {
	        let query = diesel::delete(projects::table.find(name)).execute(connection);
	        !query.is_err()
	    }
}
