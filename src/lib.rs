/*
-- Copyright 2023 N. P. Maling
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
-- http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.
*/

pub mod db_string {
    use rusqlite::{params, Connection};

    pub fn stringer(dbstr: String, cn: String) -> Result<(), rusqlite::Error> {
        let conn: Connection = Connection::open(cn)?;

        match conn.execute(&dbstr, params![]) {
            Ok(updated) => println!("{} rows were updated by match", updated),
            Err(err) => println!("update failed: {}", err),
        };

        Ok(())
    }

    pub fn dbconn(parameters: &str, cnnctn: String) -> Result<(), rusqlite::Error> {
        if let Err(err) = stringer(parameters.to_string(), cnnctn) {
            return Err(err);
        }

        Ok(())
    }
    
}  // mod db_string

