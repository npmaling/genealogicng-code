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

        if let Err(err) = dbconn(&aactivity, conn) {
            println!("Error: {:?}", err);
        }

        Ok(())
    }

    pub fn read_activity_a(dt: ActivityTest, conn: String) -> Result<(), rusqlite::Error> {
        let bactivity = Self::read_activity(dt);
        let conn: Connection = Connection::open(conn)?;

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
    }

    pub fn update_activity_a(dt: ActivityTest, conn: String) -> Result<(), rusqlite::Error> {
        let cactivity = ActivityTest::update_activity(dt);

        if let Err(err) = dbconn(&cactivity, conn) {
            println!("Error: {:?}", err);
        }

        Ok(())
    }

    pub fn delete_activity_a(dt: ActivityTest, conn: String) -> Result<(), rusqlite::Error> {
        let dactivity = ActivityTest::delete_activity(dt);

        if let Err(err) = dbconn(&dactivity, conn) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_activity() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 1,
            researcherid: 1,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-31"),
            typecode: String::from("test"),
            status: String::from("completed"),
            description: String::from("Test activity"),
            priority: String::from("high"),
            comments: String::from("Test comments"),
        };
        let expected_query = "INSERT INTO activity (activityid, projectid, researcherid, scheddate, completedate, typecode, status, description, priority, comments) VALUES (1, 1, 1, \"2021-01-01\", \"2021-01-31\", \"test\", \"completed\", \"Test activity\", \"high\", \"Test comments\")";
        assert_eq!(ActivityTest::create_activity(activity), expected_query);
    }

    #[test]
    fn test_read_activity() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 1,
            researcherid: 1,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-01"),
            typecode: String::from("Test type"),
            status: String::from("Test status"),
            description: String::from("Test description"),
            priority: String::from("Test priority"),
            comments: String::from("Test comments"),
        };
        let expected_query = "SELECT * FROM activity WHERE activityid=1";
        assert_eq!(ActivityTest::read_activity(activity), expected_query);
    }

    #[test]
    fn test_update_activity() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 2,
            researcherid: 3,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-31"),
            typecode: String::from("test"),
            status: String::from("pending"),
            description: String::from("Test activity"),
            priority: String::from("high"),
            comments: String::from("Test comments"),
        };
        let expected_query = "UPDATE activity SET projectid=2, researcherid=3, scheddate=\"2021-01-01\", completedate=\"2021-01-31\", typecode=\"test\", status=\"pending\", description=\"Test activity\", priority=\"high\", comments=\"Test comments\" WHERE activityid=1";
        assert_eq!(ActivityTest::update_activity(activity), expected_query);
    }

    #[test]
    fn test_delete_activity() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 2,
            researcherid: 3,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-02"),
            typecode: String::from("test"),
            status: String::from("completed"),
            description: String::from("test description"),
            priority: String::from("high"),
            comments: String::from("test comments"),
        };
        let expected_query = "DELETE FROM activity WHERE activityid=1";
        assert_eq!(ActivityTest::delete_activity(activity), expected_query);
    }

    /*

    /* The following tests require a database connection string to work */
    #[test]
    fn test_make_activity_a() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 1,
            researcherid: 1,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-31"),
            typecode: String::from("test"),
            status: String::from("completed"),
            description: String::from("Test activity"),
            priority: String::from("high"),
            comments: String::from("Test comments"),
        };
        let conn = String::from("your_database_connection_string");
        assert_eq!(ActivityTest::make_activity_a(activity, conn), Ok(()));
    }

    #[test]
    fn test_read_activity_a() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 1,
            researcherid: 1,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-01"),
            typecode: String::from("Test type"),
            status: String::from("Test status"),
            description: String::from("Test description"),
            priority: String::from("Test priority"),
            comments: String::from("Test comments"),
        };
        let conn = String::from("your_database_connection_string");
        assert_eq!(ActivityTest::read_activity_a(activity, conn), Ok(()));
    }

    #[test]
    fn test_update_activity_a() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 2,
            researcherid: 3,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-31"),
            typecode: String::from("test"),
            status: String::from("pending"),
            description: String::from("Test activity"),
            priority: String::from("high"),
            comments: String::from("Test comments"),
        };
        let conn = String::from("your_database_connection_string");
        assert_eq!(ActivityTest::update_activity_a(activity, conn), Ok(()));
    }

    #[test]
    fn test_delete_activity_a() {
        let activity = ActivityTest {
            activityid: 1,
            projectid: 2,
            researcherid: 3,
            scheddate: String::from("2021-01-01"),
            completedate: String::from("2021-01-02"),
            typecode: String::from("test"),
            status: String::from("completed"),
            description: String::from("test description"),
            priority: String::from("high"),
            comments: String::from("test comments"),
        };
        let conn = String::from("your_database_connection_string");
        assert_eq!(ActivityTest::delete_activity_a(activity, conn), Ok(()));
    }
    */

} // mod tests
