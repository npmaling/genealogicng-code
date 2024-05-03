use rusqlite::params;
use rusqlite::Connection;

#[derive(Debug)]
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
    pub fn make_activity_a() {
        let activity_a = ActivityTest {
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

        let aactivity = ActivityTest::create_activity(activity_a);

        if let Err(err) = Self::db_string(aactivity) {
            println!("Error: {}", err);
        }
    }

    pub fn read_activity_a(datatest: ActivityTest, conn: Connection) -> Result<(), rusqlite::Error> {
        println!("Found TEST activity data {:?}", &datatest);

        let bactivity = Self::read_activity(datatest);

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
    } // test_read

    pub fn update_activity_a() {
        let activity_c = ActivityTest {
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

        let cactivity = ActivityTest::update_activity(activity_c);

        if let Err(err) = Self::db_string(cactivity) {
            println!("Error: {}", err);
        }
    }

    pub fn delete_activity_a() {
        let activity_d = ActivityTest {
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

        let dactivity = ActivityTest::delete_activity(activity_d);

        // println!("dactivity : {:?}", &dactivity);
        if let Err(err) = Self::db_string(dactivity) {
            println!("Error: {}", err);
        }
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
        // println!("This is create_activity: {}", parameters);
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
        // println!("This is update_activity: {}", parameters);
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

    /* ------------------------------------------------------------------------- */

    pub fn db_string(dbstr: String) -> Result<(), rusqlite::Error> {
        let conn: Connection =
            Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

        match conn.execute(&dbstr, params![]) {
            Ok(updated) => println!("{} rows were updated by match", updated),
            Err(err) => println!("update failed: {}", err),
        };

        Ok(())
    }

    /* ------------------------------------------------------------------------- */
} // impl ActivityTest

