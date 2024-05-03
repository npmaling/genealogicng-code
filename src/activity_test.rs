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

use genealogicng::db_string::dbconn;
use rusqlite::Connection;

#[derive(Clone, Debug)]
pub struct ActivityTest {
    pub activityid: i64,
    pub projectid: i64,
    pub researcherid: i64,
    pub scheddate: String,
    pub completedate: String,
    pub typecode: String,
    pub status: String,
    pub description: String,
    pub priority: String,
    pub comments: String,
}

impl ActivityTest {
    pub fn make_activity_a(dt: ActivityTest, conn: String) -> Result<(), rusqlite::Error> {
        let aactivity = ActivityTest::create_activity(dt);
        let cnnctn = conn;

        if let Err(err) = dbconn(&aactivity, cnnctn) {
            println!("Error: {:?}", err);
        }

        Ok(())
    }

    pub fn read_activity_a(dt: ActivityTest, cnn: String) -> Result<(), rusqlite::Error> {
        let bactivity = Self::read_activity(dt);
        let conn: Connection = Connection::open(cnn)?;

        let mut stmt = conn.prepare(&bactivity)?;
        let activity_iter = stmt.query_map([], |row| {
            Ok(ActivityTest {
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
            println!("Found activity TEST data {:?}", activityitem.unwrap());
        }

        Ok(())
    } // read_activity_a

    pub fn update_activity_a(dt: ActivityTest, conn: String) -> Result<(), rusqlite::Error> {
        let cactivity = ActivityTest::update_activity(dt);
        let cnnctn = conn;

        if let Err(err) = dbconn(&cactivity, cnnctn) {
            println!("Error: {:?}", err);
        }

        Ok(())
    }

    pub fn delete_activity_a(dt: ActivityTest, conn: String) -> Result<(), rusqlite::Error> {
        let dactivity = ActivityTest::delete_activity(dt);
        let cnnctn = conn;

        if let Err(err) = dbconn(&dactivity, cnnctn) {
            println!("Error: {:?}", err);
        }

        Ok(())
    }

    pub fn create_activity(
        ActivityTest {
            activityid,
            projectid,
            researcherid,
            scheddate,
            completedate,
            typecode,
            status,
            description,
            priority,
            comments,
        }: ActivityTest,
    ) -> String {
        let parameters = format!(
            "INSERT INTO activity (activityid, projectid, researcherid, scheddate, completedate, typecode, status, description, priority, comments) VALUES ({}, {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")",
            &activityid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &scheddate,
            &completedate,
            &typecode,
            &status,
            &description,
            &priority,
            &comments
        );
        parameters
    }

    pub fn read_activity(
        ActivityTest {
            activityid,
            projectid: _,
            researcherid: _,
            scheddate: _,
            completedate: _,
            typecode: _,
            status: _,
            description: _,
            priority: _,
            comments: _,
        }: ActivityTest,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM activity WHERE activityid={}",
            &activityid.to_string(),
        );
        parameters
    }

    pub fn update_activity(
        ActivityTest {
            activityid,
            projectid,
            researcherid,
            scheddate,
            completedate,
            typecode,
            status,
            description,
            priority,
            comments,
        }: ActivityTest,
    ) -> String {
        let parameters = format!(
            "UPDATE activity SET projectid={}, researcherid={}, scheddate=\"{}\", completedate=\"{}\", typecode=\"{}\", status=\"{}\", description=\"{}\", priority=\"{}\", comments=\"{}\" WHERE activityid={}",
            // &activityid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &scheddate,
            &completedate,
            &typecode,
            &status,
            &description,
            &priority,
            &comments,
            &activityid.to_string(),
        );
        parameters
    }

    pub fn delete_activity(
        ActivityTest {
            activityid,
            projectid: _,
            researcherid: _,
            scheddate: _,
            completedate: _,
            typecode: _,
            status: _,
            description: _,
            priority: _,
            comments: _,
        }: ActivityTest,
    ) -> String {
        let parameters = format!(
            "DELETE FROM activity WHERE activityid={}",
            &activityid.to_string(),
        );
        parameters
    }
} // impl ActivityTest
