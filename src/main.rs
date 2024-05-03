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

use rusqlite::Connection;

use crate::activity_test::ActivityTest;
mod activity_test;

use crate::dbstring::dbstring;
mod dbstring;

fn main() {
    let conn = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db").unwrap();

    let td = ActivityTest {
        activityid: 16,
        projectid: 1,
        researcherid: 1,
        scheddate: "20230101".to_string(),
        completedate: "20230101".to_string(),
        typecode: "a".to_string(),
        status: "a".to_string(),
        description: "First Activity".to_string(),
        priority: "a".to_string(),
        comments: "a".to_string(),
    };

    ActivityTest::make_activity_a();
    let _ = ActivityTest::read_activity_a();
    ActivityTest::update_activity_a();
    ActivityTest::delete_activity_a();

    if let Err(err) = ActivityTest::test_read(td, conn) {
        println!("Error: {}", err);
    }

}

/* 
pub fn test_read(datatest: Activity, conn: Connection) -> Result<(), rusqlite::Error> {

    let bactivity = Activity::read_activity(datatest);

    let mut stmt = conn.prepare(&bactivity)?;
    let activity_iter = stmt.query_map([], |row| {
        Ok( Activity {
            activityid: row.get(0)?,
            projectid: row.get(1)?,
            researcherid: row.get(2)?,
            scheddate: row.get(3)?,
            completedate: row.get(4)?,
            typecode: row.get(5)?,
            status: row.get(6)?,
            description: row.get(7)?,
            priority: row.get(8)?,
            comments: row.get(9)?,
        })
    })?;

    for activityitem in activity_iter {
        println!("Found TEST activity data {:?}", activityitem.unwrap());
    }

    Ok(())
} // test_read

 */