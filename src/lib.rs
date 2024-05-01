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

use crate::activity::Activity;
mod activity;

use crate::place::Place;
mod place;

use rusqlite::{params, Connection};

/* ------------------------------------------------------------------------- */

pub fn make_activity_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let activity_a = Activity {
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

    let aactivity = Activity::create_activity(activity_a);

    println!("aactivity : {:?}", &aactivity);
    dbstring(&conn, aactivity);

    Ok(())
}

pub fn read_activity_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let activity_b = Activity {
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

    let bactivity = Activity::read_activity(activity_b);

    println!("bactivity : {:?}", &bactivity);

    // dbstring(&conn, &bactivity);

    let mut stmt = conn.prepare(&bactivity)?;
    let activity_iter = stmt.query_map([], |row| {
        Ok(Activity {
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
        println!("Found activity data {:?}", activityitem.unwrap());
    }

    Ok(())
}

pub fn update_activity_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let activity_c = Activity {
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

    let cactivity = Activity::update_activity(activity_c);

    println!("cactivity : {:?}", &cactivity);
    dbstring(&conn, cactivity);

    Ok(())
}

pub fn delete_activity_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let activity_d = Activity {
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

    let dactivity = Activity::delete_activity(activity_d);

    println!("dactivity : {:?}", &dactivity);
    dbstring(&conn, dactivity);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_place_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let place_a = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "a".to_string(),
        placecomment: "First Place".to_string(),
    };

    let aplace = Place::create_place(place_a);

    println!("aplace : {:?}", &aplace);
    dbstring(&conn, aplace);

    Ok(())
}

pub fn read_place_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let place_b = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "d".to_string(),
        placecomment: "Second Place".to_string(),
    };

    let bplace = Place::read_place(place_b);

    println!("bplace : {:?}", &bplace);

    // dbstring(&conn, &bplace);

    let mut stmt = conn.prepare(&bplace)?;
    let place_iter = stmt.query_map([], |row| {
        Ok(Place {
            placeid: row.get(0)?,
            startdate: row.get(1)?,
            enddate: row.get(2)?,
            ascdescnone: row.get(3)?,
            placecomment: row.get(4)?,
        })
    })?;

    for placeitem in place_iter {
        println!("Found place data {:?}", placeitem.unwrap());
    }

    Ok(())
}

pub fn update_place_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let place_c = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "d".to_string(),
        placecomment: "Second Place".to_string(),
    };
    
    let cplace = Place::update_place(place_c);
    
    println!("cplace : {:?}", &cplace);
    dbstring(&conn, cplace);

    Ok(())
}

pub fn delete_place_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let place_d = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "d".to_string(),
        placecomment: "Second Place".to_string(),
    };
    
    let dplace = Place::delete_place(place_d);
    
    println!("dplace : {:?}", &dplace);
    dbstring(&conn, dplace);

    Ok(())
}

/* ------------------------------------------------------------------------- */

fn dbstring(conn: &Connection, dbstr: String) {
    match conn.execute(&dbstr, params![]) {
        Ok(updated) => println!("{} rows were updated by match", updated),
        Err(err) => println!("update failed: {}", err),
    };
}
