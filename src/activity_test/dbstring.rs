use rusqlite::Connection;
use rusqlite::params;

pub(crate) mod db_string {
    pub fn stringer(dbstr: String) -> Result<(), rusqlite::Error> {
        let conn: Connection =
            Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

        match conn.execute(&dbstr, params![]) {
            Ok(updated) => println!("{} rows were updated by match", updated),
            Err(err) => println!("update failed: {}", err),
        };

        Ok(())
    }
}  // mod dbstring

