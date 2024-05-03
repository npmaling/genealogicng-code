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

/**
 * This file contains the implementation of various structs and functions 
 * related to genealogical activities, assertions, characteristics, and more.
 * It also includes database operations using the rusqlite library.
 *
 * The code is organized into modules, each representing a specific entity or concept in genealogy.
 * Each module is defined in a separate file and imported into this file using the `mod` keyword.
 *
 * The main functionality of this code is to perform CRUD (Create, Read, Update, Delete) operations on genealogical data stored in a SQLite database.
 * The code defines functions for creating, reading, updating, and deleting activities, assertions, characteristics, and other genealogical entities.
 * These functions interact with the database using the rusqlite library.
 *
 * The code also includes utility functions for executing SQL queries and printing the results.
 *
 * To use this code, you need to have the rusqlite library installed and a SQLite database file named "database.db" in the specified path.
 * The code assumes that the database schema is already set up with the required tables and columns.
 *
 * Example usage:
 * - `make_activity_a()`: Creates a new activity in the database.
 * - `read_activity_a()`: Reads an activity from the database.
 * - `update_activity_a()`: Updates an existing activity in the database.
 * - `delete_activity_a()`: Deletes an activity from the database.
 *
 * Note: This code is subject to the Apache License, Version 2.0. Please refer to the license file for more details.
 */

use crate::activity::Activity;
mod activity;

use crate::assertassert::AssertAssert;
mod assertassert;

use crate::characteristic::Characteristic;
mod characteristic;

use crate::charpart::CharPart;
mod charpart;

use crate::charparttype::CharPartType;
mod charparttype;

use crate::citationpart::CitationPart;
mod citationpart;

use crate::citationparttype::CitationPartType;
mod citationparttype;

use crate::event::Event;
mod event;

use crate::eventtype::EventType;
mod eventtype;

use crate::eventtyperole::EventTypeRole;
mod eventtyperole;

use crate::glassertion::GlAssertion;
mod glassertion;

use crate::glgroup::GlGroup;
mod glgroup;

use crate::glgrouptype::GlGroupType;
mod glgrouptype;

use crate::glgrouptyperole::GlGroupTypeRole;
mod glgrouptyperole;

use crate::persona::Persona;
mod persona;

use crate::place::Place;
mod place;

use crate::placepart::PlacePart;
mod placepart;

use crate::placeparttype::PlacePartType;
mod placeparttype;

use crate::project::Project;
mod project;

use crate::repository::Repository;
mod repository;

use crate::reposource::RepoSource;
mod reposource;

use crate::representation::Representation;
mod representation;

use crate::representtype::RepresentType;
mod representtype;

use crate::reprmediatype::ReprMediaType;
mod reprmediatype;

use crate::researcher::Researcher;
mod researcher;

use crate::resobjactivity::ResObjActivity;
mod resobjactivity;

use crate::resobjective::ResObjective;
mod resobjective;

use crate::resproj::ResProj;
mod resproj;

use crate::search::Search;
mod search;

use crate::source::Source;
mod source;

use crate::sourcegroup::SourceGroup;
mod sourcegroup;

use crate::srcgrpsrc::SrcGrpSrc;
mod srcgrpsrc;

use crate::suretypart::SuretyPart;
mod suretypart;

use crate::suretyscheme::SuretyScheme;
mod suretyscheme;

use rusqlite::{params, Connection};

/* ------------------------------------------------------------------------- */

/* 
fn dbstring(dbstr: String) -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    match conn.execute(&dbstr, params![]) {
        Ok(updated) => println!("{} rows were updated by match", updated),
        Err(err) => println!("update failed: {}", err),
    };

    Ok(())
}

 */
/* ------------------------------------------------------------------------- */

pub fn make_activity_a() {
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

    if let Err(err) = dbstring(aactivity) {
        println!("Error: {}", err);
    }
}


pub fn test_read(datatest: Activity, conn: Connection) -> Result<(), rusqlite::Error> {

    println!("Found TEST activity data {:?}", &datatest);

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
        println!("Found activity data {:?}", activityitem.unwrap());
    }

    Ok(())
} // test_read


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
} // read_activity_a

pub fn update_activity_a() {
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

    // println!("cactivity : {:?}", &cactivity);
    if let Err(err) = dbstring(cactivity) {
        println!("Error: {}", err);
    }

}

pub fn delete_activity_a() {
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

    // println!("dactivity : {:?}", &dactivity);
    if let Err(err) = dbstring(dactivity) {
        println!("Error: {}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_assertassert_a() {
    let assertassert_a = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let aassertassert = AssertAssert::create_assertassert(assertassert_a);

    // println!("aassertassert : {:?}", &aassertassert);
    if let Err(err) = dbstring(aassertassert) {
        println!("Error: {}", err);
    }
}

pub fn read_assertassert_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let assertassert_b = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let bassertassert = AssertAssert::read_assertassert(assertassert_b);

    // println!("bassertassert : {:?}", &bassertassert);

    let mut stmt = conn.prepare(&bassertassert)?;
    let assertassert_iter = stmt.query_map([], |row| {
        Ok(AssertAssert {
            assertassertid: row.get(0)?,
            idlo: row.get(1)?,
            idhi: row.get(2)?,
            seq: row.get(3)?,
        })
    })?;

    for assertassertitem in assertassert_iter {
        println!("Found assertassert data {:?}", assertassertitem.unwrap());
    }

    Ok(())
}

pub fn update_assertassert_a() {
    let assertassert_c = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let cassertassert = AssertAssert::update_assertassert(assertassert_c);

    // println!("cassertassert : {:?}", &cassertassert);
    if let Err(err) = dbstring(cassertassert) {
        println!("Error: {}", err);
    }
}

pub fn delete_assertassert_a() {
    let assertassert_d = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let dassertassert = AssertAssert::delete_assertassert(assertassert_d);

    // println!("dassertassert : {:?}", &dassertassert);
    if let Err(err) = dbstring(dassertassert) {
        println!("Error: {}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_characteristic_a() {
    let characteristic_a = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let acharacteristic = Characteristic::create_characteristic(characteristic_a);

    // println!("acharacteristic : {:?}", &acharacteristic);
    if let Err(err) = dbstring(acharacteristic) {
        println!("Error: {}", err);
    }
}

pub fn read_characteristic_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let characteristic_b = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let bcharacteristic = Characteristic::read_characteristic(characteristic_b);

    // println!("bcharacteristic : {:?}", &bcharacteristic);

    let mut stmt = conn.prepare(&bcharacteristic)?;
    let characteristic_iter = stmt.query_map([], |row| {
        Ok(Characteristic {
            characteristicid: row.get(0)?,
            placeid: row.get(1)?,
            characteristicdate: row.get(2)?,
            ascdescnone: row.get(3)?,
        })
    })?;

    for characteristicitem in characteristic_iter {
        println!("Found characteristic data {:?}", characteristicitem.unwrap());
    }

    Ok(())
}

pub fn update_characteristic_a() {
    let characteristic_c = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let ccharacteristic = Characteristic::update_characteristic(characteristic_c);

    // println!("ccharacteristic : {:?}", &ccharacteristic);
    if let Err(err) = dbstring(ccharacteristic) {
        println!("Error: {}", err);
    }
}

pub fn delete_characteristic_a() {
    let characteristic_d = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let dcharacteristic = Characteristic::delete_characteristic(characteristic_d);

    // println!("dcharacteristic : {:?}", &dcharacteristic);
    if let Err(err) = dbstring(dcharacteristic) {
        println!("Error: {}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_charpart_a() {
    let charpart_a = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let acharpart = CharPart::create_charpart(charpart_a);

    // println!("acharpart : {:?}", &acharpart);
    if let Err(err) = dbstring(acharpart) {
        println!("Error: {:?}", err);
    }
}

pub fn read_charpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charpart_b = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let bcharpart = CharPart::read_charpart(charpart_b);

    // println!("bcharpart : {:?}", &bcharpart);

    let mut stmt = conn.prepare(&bcharpart)?;
    let charpart_iter = stmt.query_map([], |row| {
        Ok(CharPart {
            characteristicpartid: row.get(0)?,
            characteristicid: row.get(1)?,
            charparttypeid: row.get(2)?,
            charpartname: row.get(3)?,
            charpartseq: row.get(4)?,
        })
    })?;

    for charpartitem in charpart_iter {
        println!("Found charpart data {:?}", charpartitem.unwrap());
    }

    Ok(())
}

pub fn update_charpart_a() {
    let charpart_c = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let ccharpart = CharPart::update_charpart(charpart_c);

    // println!("ccharpart : {:?}", &ccharpart);
    if let Err(err) = dbstring(ccharpart) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_charpart_a() {
    let charpart_d = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let dcharpart = CharPart::delete_charpart(charpart_d);

    // println!("dcharpart : {:?}", &dcharpart);
    if let Err(err) = dbstring(dcharpart) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_charparttype_a() {
    let charparttype_a = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let acharparttype = CharPartType::create_charparttype(charparttype_a);

    // println!("acharparttype : {:?}", &acharparttype);
    if let Err(err) = dbstring(acharparttype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_charparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charparttype_b = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let bcharparttype = CharPartType::read_charparttype(charparttype_b);

    // println!("bcharparttype : {:?}", &bcharparttype);

    let mut stmt = conn.prepare(&bcharparttype)?;
    let charparttype_iter = stmt.query_map([], |row| {
        Ok(CharPartType {
            charparttypeid: row.get(0)?,
            charparttypename: row.get(1)?,
            gedcomtag: row.get(2)?,
        })
    })?;

    for charparttypeitem in charparttype_iter {
        println!("Found charparttype data {:?}", charparttypeitem.unwrap());
    }

    Ok(())
}

pub fn update_charparttype_a() {
    let charparttype_c = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let ccharparttype = CharPartType::update_charparttype(charparttype_c);

    // println!("ccharparttype : {:?}", &ccharparttype);
    if let Err(err) = dbstring(ccharparttype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_charparttype_a() {
    let charparttype_d = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let dcharparttype = CharPartType::delete_charparttype(charparttype_d);

    // println!("dcharparttype : {:?}", &dcharparttype);
    if let Err(err) = dbstring(dcharparttype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_citationpart_a() {
    let citationpart_a = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let acitationpart = CitationPart::create_citationpart(citationpart_a);

    // println!("acitationpart : {:?}", &acitationpart);
    if let Err(err) = dbstring(acitationpart) {
        println!("Error: {:?}", err);
    }
}

pub fn read_citationpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationpart_b = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let bcitationpart = CitationPart::read_citationpart(citationpart_b);

    // println!("bcitationpart : {:?}", &bcitationpart);

    let mut stmt = conn.prepare(&bcitationpart)?;
    let citationpart_iter = stmt.query_map([], |row| {
        Ok(CitationPart {
            citationpartid: row.get(0)?,
            sourceid: row.get(1)?,
            citeparttypeid: row.get(2)?,
            citepartvalue: row.get(3)?,
        })
    })?;

    for citationpartitem in citationpart_iter {
        println!("Found citationpart data {:?}", citationpartitem.unwrap());
    }

    Ok(())
}

pub fn update_citationpart_a() {
    let citationpart_c = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let ccitationpart = CitationPart::update_citationpart(citationpart_c);

    // println!("ccitationpart : {:?}", &ccitationpart);
    if let Err(err) = dbstring(ccitationpart) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_citationpart_a() {
    let citationpart_d = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let dcitationpart = CitationPart::delete_citationpart(citationpart_d);

    // println!("dcitationpart : {:?}", &dcitationpart);
    if let Err(err) = dbstring(dcitationpart) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_citationparttype_a() {
    let citationparttype_a = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let acitationparttype = CitationPartType::create_citationparttype(citationparttype_a);

    // println!("acitationparttype : {:?}", &acitationparttype);
    if let Err(err) = dbstring(acitationparttype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_citationparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationparttype_b = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let bcitationparttype = CitationPartType::read_citationparttype(citationparttype_b);

    // println!("bcitationparttype : {:?}", &bcitationparttype);

    let mut stmt = conn.prepare(&bcitationparttype)?;
    let citationparttype_iter = stmt.query_map([], |row| {
        Ok(CitationPartType {
            citationparttypeid: row.get(0)?,
            citationparttypename: row.get(1)?,
        })
    })?;

    for citationparttypeitem in citationparttype_iter {
        println!("Found citationparttype data {:?}", citationparttypeitem.unwrap());
    }

    Ok(())
}

pub fn update_citationparttype_a() {
    let citationparttype_c = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let ccitationparttype = CitationPartType::update_citationparttype(citationparttype_c);

    // println!("ccitationparttype : {:?}", &ccitationparttype);
    if let Err(err) = dbstring(ccitationparttype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_citationparttype_a() {
    let citationparttype_d = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let dcitationparttype = CitationPartType::delete_citationparttype(citationparttype_d);

    // println!("dcitationparttype : {:?}", &dcitationparttype);
    if let Err(err) = dbstring(dcitationparttype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_event_a() {
    let event_a = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let aevent = Event::create_event(event_a);

    // println!("aevent : {:?}", &aevent);
    if let Err(err) = dbstring(aevent) {
        println!("Error: {:?}", err);
    }
}

pub fn read_event_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let event_b = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let bevent = Event::read_event(event_b);

    // println!("bevent : {:?}", &bevent);

    let mut stmt = conn.prepare(&bevent)?;
    let event_iter = stmt.query_map([], |row| {
        Ok(Event {
            eventid: row.get(0)?,
            eventtypeid: row.get(1)?,
            placeid: row.get(2)?,
            eventdate: row.get(3)?,
            eventname: row.get(4)?,
        })
    })?;

    for eventitem in event_iter {
        println!("Found event data {:?}", eventitem.unwrap());
    }

    Ok(())
}

pub fn update_event_a() {
    let event_c = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let cevent = Event::update_event(event_c);

    // println!("cevent : {:?}", &cevent);
    if let Err(err) = dbstring(cevent) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_event_a() {
    let event_d = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let devent = Event::delete_event(event_d);

    // println!("devent : {:?}", &devent);
    if let Err(err) = dbstring(devent) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_eventtype_a() {
    let eventtype_a = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let aeventtype = EventType::create_eventtype(eventtype_a);

    // println!("aeventtype : {:?}", &aeventtype);
    if let Err(err) = dbstring(aeventtype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_eventtype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtype_b = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let beventtype = EventType::read_eventtype(eventtype_b);

    // println!("beventtype : {:?}", &beventtype);

    let mut stmt = conn.prepare(&beventtype)?;
    let eventtype_iter = stmt.query_map([], |row| {
        Ok(EventType {
            eventtypeid: row.get(0)?,
            eventtypename: row.get(1)?,
            gedcomtag: row.get(2)?,
        })
    })?;

    for eventtypeitem in eventtype_iter {
        println!("Found eventtype data {:?}", eventtypeitem.unwrap());
    }

    Ok(())
}

pub fn update_eventtype_a() {
    let eventtype_c = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let ceventtype = EventType::update_eventtype(eventtype_c);

    // println!("ceventtype : {:?}", &ceventtype);
    if let Err(err) = dbstring(ceventtype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_eventtype_a() {
    let eventtype_d = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let deventtype = EventType::delete_eventtype(eventtype_d);

    // println!("deventtype : {:?}", &deventtype);
    if let Err(err) = dbstring(deventtype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_eventtyperole_a() {
    let eventtyperole_a = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let aeventtyperole = EventTypeRole::create_eventtyperole(eventtyperole_a);

    // println!("aeventtyperole : {:?}", &aeventtyperole);
    if let Err(err) = dbstring(aeventtyperole) {
        println!("Error: {:?}", err);
    }
}

pub fn read_eventtyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtyperole_b = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let beventtyperole = EventTypeRole::read_eventtyperole(eventtyperole_b);

    // println!("beventtyperole : {:?}", &beventtyperole);

    let mut stmt = conn.prepare(&beventtyperole)?;
    let eventtyperole_iter = stmt.query_map([], |row| {
        Ok(EventTypeRole {
            eventtyperoleid: row.get(0)?,
            eventtypeid: row.get(1)?,
            eventtyperolename: row.get(2)?,
        })
    })?;

    for eventtyperoleitem in eventtyperole_iter {
        println!("Found eventtyperole data {:?}", eventtyperoleitem.unwrap());
    }

    Ok(())
}

pub fn update_eventtyperole_a() {
    let eventtyperole_c = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let ceventtyperole = EventTypeRole::update_eventtyperole(eventtyperole_c);

    // println!("ceventtyperole : {:?}", &ceventtyperole);
    if let Err(err) = dbstring(ceventtyperole) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_eventtyperole_a() {
    let eventtyperole_d = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let deventtyperole = EventTypeRole::delete_eventtyperole(eventtyperole_d);

    // println!("deventtyperole : {:?}", &deventtyperole);
    if let Err(err) = dbstring(deventtyperole) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_glassertion_a() {
    let glassertion_a = GlAssertion {
        glassertionid: 16,
        suretypartid: 1,
        researcherid: 1,
        sourceid: 1,
        subject1id: 1,
        subject1type: "a".to_string(),
        subject2id: 1,
        subject2type: "b".to_string(),
        value_role: 1,
        disproved: "true".to_string(),
        rationale: "d".to_string(),
    };

    let aglassertion = GlAssertion::create_glassertion(glassertion_a);

    // println!("aglassertion : {:?}", &aglassertion);
    if let Err(err) = dbstring(aglassertion) {
        println!("Error: {:?}", err);
    }
}

pub fn read_glassertion_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glassertion_b = GlAssertion {
        glassertionid: 16,
        suretypartid: 1,
        researcherid: 1,
        sourceid: 1,
        subject1id: 1,
        subject1type: "a".to_string(),
        subject2id: 1,
        subject2type: "b".to_string(),
        value_role: 1,
        disproved: "true".to_string(),
        rationale: "d".to_string(),
    };

    let bglassertion = GlAssertion::read_glassertion(glassertion_b);

    // println!("bglassertion : {:?}", &bglassertion);

    let mut stmt = conn.prepare(&bglassertion)?;
    let glassertion_iter = stmt.query_map([], |row| {
        Ok(GlAssertion {
            glassertionid: row.get(0)?,
            suretypartid: row.get(1)?,
            researcherid: row.get(2)?,
            sourceid: row.get(3)?,
            subject1id: row.get(4)?,
            subject1type: row.get(5)?,
            subject2id: row.get(6)?,
            subject2type: row.get(7)?,
            value_role: row.get(8)?,
            disproved: row.get(9)?,
            rationale: row.get(10)?,
        })
    })?;

    for glassertionitem in glassertion_iter {
        println!("Found glassertion data {:?}", glassertionitem.unwrap());
    }

    Ok(())
}

pub fn update_glassertion_a() {
    let glassertion_c = GlAssertion {
        glassertionid: 16,
        suretypartid: 1,
        researcherid: 1,
        sourceid: 1,
        subject1id: 1,
        subject1type: "a".to_string(),
        subject2id: 1,
        subject2type: "b".to_string(),
        value_role: 1,
        disproved: "true".to_string(),
        rationale: "d".to_string(),
    };

    let cglassertion = GlAssertion::update_glassertion(glassertion_c);

    // println!("cglassertion : {:?}", &cglassertion);
    if let Err(err) = dbstring(cglassertion) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_glassertion_a() {
    let glassertion_d = GlAssertion {
        glassertionid: 16,
        suretypartid: 1,
        researcherid: 1,
        sourceid: 1,
        subject1id: 1,
        subject1type: "a".to_string(),
        subject2id: 1,
        subject2type: "b".to_string(),
        value_role: 1,
        disproved: "true".to_string(),
        rationale: "d".to_string(),
    };

    let dglassertion = GlAssertion::delete_glassertion(glassertion_d);

    // println!("dglassertion : {:?}", &dglassertion);
    if let Err(err) = dbstring(dglassertion) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_glgroup_a() {
    let glgroup_a = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let aglgroup = GlGroup::create_glgroup(glgroup_a);

    // println!("aglgroup : {:?}", &aglgroup);
    if let Err(err) = dbstring(aglgroup) {
        println!("Error: {:?}", err);
    }
}

pub fn read_glgroup_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgroup_b = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let bglgroup = GlGroup::read_glgroup(glgroup_b);

    // println!("bglgroup : {:?}", &bglgroup);

    let mut stmt = conn.prepare(&bglgroup)?;
    let glgroup_iter = stmt.query_map([], |row| {
        Ok(GlGroup {
            glgroupid: row.get(0)?,
            glgrouptypeid: row.get(1)?,
            placeid: row.get(2)?,
            glgroupdate: row.get(3)?,
            glgroupname: row.get(4)?,
            glgroupcriteria: row.get(5)?,
        })
    })?;

    for glgroupitem in glgroup_iter {
        println!("Found glgroup data {:?}", glgroupitem.unwrap());
    }

    Ok(())
}

pub fn update_glgroup_a() {
    let glgroup_c = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let cglgroup = GlGroup::update_glgroup(glgroup_c);

    // println!("cglgroup : {:?}", &cglgroup);
    if let Err(err) = dbstring(cglgroup) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_glgroup_a() {
    let glgroup_d = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let dglgroup = GlGroup::delete_glgroup(glgroup_d);

    // println!("dglgroup : {:?}", &dglgroup);
    if let Err(err) = dbstring(dglgroup) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_glgrouptype_a() {
    let glgrouptype_a = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let aglgrouptype = GlGroupType::create_glgrouptype(glgrouptype_a);

    // println!("aglgrouptype : {:?}", &aglgrouptype);
    if let Err(err) = dbstring(aglgrouptype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_glgrouptype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptype_b = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let bglgrouptype = GlGroupType::read_glgrouptype(glgrouptype_b);

    // println!("bglgrouptype : {:?}", &bglgrouptype);

    let mut stmt = conn.prepare(&bglgrouptype)?;
    let glgrouptype_iter = stmt.query_map([], |row| {
        Ok(GlGroupType {
            glgrouptypeid: row.get(0)?,
            glgroupname: row.get(1)?,
            ascdescnone: row.get(2)?,
        })
    })?;

    for glgrouptypeitem in glgrouptype_iter {
        println!("Found glgrouptype data {:?}", glgrouptypeitem.unwrap());
    }

    Ok(())
}

pub fn update_glgrouptype_a() {
    let glgrouptype_c = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let cglgrouptype = GlGroupType::update_glgrouptype(glgrouptype_c);

    // println!("cglgrouptype : {:?}", &cglgrouptype);
    if let Err(err) = dbstring(cglgrouptype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_glgrouptype_a() {
    let glgrouptype_d = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let dglgrouptype = GlGroupType::delete_glgrouptype(glgrouptype_d);

    // println!("dglgrouptype : {:?}", &dglgrouptype);
    if let Err(err) = dbstring(dglgrouptype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_glgrouptyperole_a() {
    let glgrouptyperole_a = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let aglgrouptyperole = GlGroupTypeRole::create_glgrouptyperole(glgrouptyperole_a);

    // println!("aglgrouptyperole : {:?}", &aglgrouptyperole);
    if let Err(err) = dbstring(aglgrouptyperole) {
        println!("Error: {:?}", err);
    }
}

pub fn read_glgrouptyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptyperole_b = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let bglgrouptyperole = GlGroupTypeRole::read_glgrouptyperole(glgrouptyperole_b);

    // println!("bglgrouptyperole : {:?}", &bglgrouptyperole);

    let mut stmt = conn.prepare(&bglgrouptyperole)?;
    let glgrouptyperole_iter = stmt.query_map([], |row| {
        Ok(GlGroupTypeRole {
            glgrouptyperoleid: row.get(0)?,
            glgrouptypeid: row.get(1)?,
            glgrouptypename: row.get(2)?,
            sequencenumber: row.get(3)?,
        })
    })?;

    for glgrouptyperoleitem in glgrouptyperole_iter {
        println!("Found glgrouptyperole data {:?}", glgrouptyperoleitem.unwrap());
    }

    Ok(())
}

pub fn update_glgrouptyperole_a() {
    let glgrouptyperole_c = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let cglgrouptyperole = GlGroupTypeRole::update_glgrouptyperole(glgrouptyperole_c);

    // println!("cglgrouptyperole : {:?}", &cglgrouptyperole);
    if let Err(err) = dbstring(cglgrouptyperole) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_glgrouptyperole_a() {
    let glgrouptyperole_d = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let dglgrouptyperole = GlGroupTypeRole::delete_glgrouptyperole(glgrouptyperole_d);

    // println!("dglgrouptyperole : {:?}", &dglgrouptyperole);
    if let Err(err) = dbstring(dglgrouptyperole) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_persona_a() {
    let persona_a = Persona {
        personaid: 16,
        persona_name: "First Persona".to_string(),
        description_comments: "a".to_string(),
    };

    let apersona = Persona::create_persona(persona_a);

    // println!("apersona : {:?}", &apersona);
    if let Err(err) = dbstring(apersona) {
        println!("Error: {:?}", err);
    }
}

pub fn read_persona_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let persona_b = Persona {
        personaid: 16,
        persona_name: "First Persona".to_string(),
        description_comments: "a".to_string(),
    };

    let bpersona = Persona::read_persona(persona_b);

    // println!("bpersona : {:?}", &bpersona);

    let mut stmt = conn.prepare(&bpersona)?;
    let persona_iter = stmt.query_map([], |row| {
        Ok(Persona {
            personaid: row.get(0)?,
            persona_name: row.get(1)?,
            description_comments: row.get(2)?,
        })
    })?;

    for personitem in persona_iter {
        println!("Found persona data {:?}", personitem.unwrap());
    }

    Ok(())
}

pub fn update_persona_a() {
    let persona_c = Persona {
        personaid: 16,
        persona_name: "First Persona".to_string(),
        description_comments: "a".to_string(),
    };

    let cpersona = Persona::update_persona(persona_c);

    // println!("cpersona : {:?}", &cpersona);
    if let Err(err) = dbstring(cpersona) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_persona_a() {
    let persona_d = Persona {
        personaid: 16,
        persona_name: "First Persona".to_string(),
        description_comments: "a".to_string(),
    };

    let dpersona = Persona::delete_persona(persona_d);

    // println!("dpersona : {:?}", &dpersona);
    if let Err(err) = dbstring(dpersona) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_place_a() {
    let place_a = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "a".to_string(),
        placecomment: "First Place".to_string(),
    };

    let aplace = Place::create_place(place_a);

    // println!("aplace : {:?}", &aplace);
    if let Err(err) = dbstring(aplace) {
        println!("Error: {:?}", err);
    }
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

    // println!("bplace : {:?}", &bplace);

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

pub fn update_place_a() {
    let place_c = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "d".to_string(),
        placecomment: "Second Place".to_string(),
    };
    
    let cplace = Place::update_place(place_c);
    
    // println!("cplace : {:?}", &cplace);
    if let Err(err) = dbstring(cplace) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_place_a() {
    let place_d = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "d".to_string(),
        placecomment: "Second Place".to_string(),
    };
    
    let dplace = Place::delete_place(place_d);
    
    // println!("dplace : {:?}", &dplace);
    if let Err(err) = dbstring(dplace) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_placepart_a() {
    let placepart_a = PlacePart {
        placepartid: 16,
        placeid: 1,
        placeparttypeid: 1,
        name: "First PlacePart".to_string(),
        sequencenumber: 1,
    };

    let aplacepart = PlacePart::create_placepart(placepart_a);

    // println!("aplacepart : {:?}", &aplacepart);
    if let Err(err) = dbstring(aplacepart) {
        println!("Error: {:?}", err);
    }
}

pub fn read_placepart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let placepart_b = PlacePart {
        placepartid: 16,
        placeid: 1,
        placeparttypeid: 1,
        name: "First PlacePart".to_string(),
        sequencenumber: 1,
    };

    let bplacepart = PlacePart::read_placepart(placepart_b);

    // println!("bplacepart : {:?}", &bplacepart);

    let mut stmt = conn.prepare(&bplacepart)?;
    let placepart_iter = stmt.query_map([], |row| {
        Ok(PlacePart {
            placepartid: row.get(0)?,
            placeid: row.get(1)?,
            placeparttypeid: row.get(2)?,
            name: row.get(3)?,
            sequencenumber: row.get(4)?,
        })
    })?;

    for placepartitem in placepart_iter {
        println!("Found placepart data {:?}", placepartitem.unwrap());
    }

    Ok(())
}

pub fn update_placepart_a() {
    let placepart_c = PlacePart {
        placepartid: 16,
        placeid: 1,
        placeparttypeid: 1,
        name: "First PlacePart".to_string(),
        sequencenumber: 1,
    };

    let cplacepart = PlacePart::update_placepart(placepart_c);

    // println!("cplacepart : {:?}", &cplacepart);
    if let Err(err) = dbstring(cplacepart) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_placepart_a() {
    let placepart_d = PlacePart {
        placepartid: 16,
        placeid: 1,
        placeparttypeid: 1,
        name: "First PlacePart".to_string(),
        sequencenumber: 1,
    };

    let dplacepart = PlacePart::delete_placepart(placepart_d);

    // println!("dplacepart : {:?}", &dplacepart);
    if let Err(err) = dbstring(dplacepart) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_placeparttype_a() {
    let placeparttype_a = PlacePartType {
        placeparttypeid: 16,
        pptname: "First PlacePartType".to_string(),
    };

    let aplaceparttype = PlacePartType::create_placeparttype(placeparttype_a);

    // println!("aplaceparttype : {:?}", &aplaceparttype);
    if let Err(err) = dbstring(aplaceparttype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_placeparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let placeparttype_b = PlacePartType {
        placeparttypeid: 16,
        pptname: "First PlacePartType".to_string(),
    };

    let bplaceparttype = PlacePartType::read_placeparttype(placeparttype_b);

    // println!("bplaceparttype : {:?}", &bplaceparttype);

    let mut stmt = conn.prepare(&bplaceparttype)?;
    let placeparttype_iter = stmt.query_map([], |row| {
        Ok(PlacePartType {
            placeparttypeid: row.get(0)?,
            pptname: row.get(1)?,
        })
    })?;

    for placeparttypeitem in placeparttype_iter {
        println!("Found placeparttype data {:?}", placeparttypeitem.unwrap());
    }

    Ok(())
}

pub fn update_placeparttype_a() {
    let placeparttype_c = PlacePartType {
        placeparttypeid: 16,
        pptname: "First PlacePartType".to_string(),
    };

    let cplaceparttype = PlacePartType::update_placeparttype(placeparttype_c);

    // println!("cplaceparttype : {:?}", &cplaceparttype);
    if let Err(err) = dbstring(cplaceparttype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_placeparttype_a() {
    let placeparttype_d = PlacePartType {
        placeparttypeid: 16,
        pptname: "First PlacePartType".to_string(),
    };

    let dplaceparttype = PlacePartType::delete_placeparttype(placeparttype_d);

    // println!("dplaceparttype : {:?}", &dplaceparttype);
    if let Err(err) = dbstring(dplaceparttype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_project_a() {
    let project_a = Project {
        projectid: 16,
        name: "First Project".to_string(),
        projectdesc: "a".to_string(),
        clientdata: "a".to_string(),
    };

    let aproject = Project::create_project(project_a);

    // println!("aproject : {:?}", &aproject);
    if let Err(err) = dbstring(aproject) {
        println!("Error: {:?}", err);
    }
}

pub fn read_project_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let project_b = Project {
        projectid: 16,
        name: "First Project".to_string(),
        projectdesc: "a".to_string(),
        clientdata: "a".to_string(),
    };

    let bproject = Project::read_project(project_b);

    // println!("bproject : {:?}", &bproject);

    let mut stmt = conn.prepare(&bproject)?;
    let project_iter = stmt.query_map([], |row| {
        Ok(Project {
            projectid: row.get(0)?,
            name: row.get(1)?,
            projectdesc: row.get(2)?,
            clientdata: row.get(3)?,
        })
    })?;

    for projectitem in project_iter {
        println!("Found project data {:?}", projectitem.unwrap());
    }

    Ok(())
}

pub fn update_project_a() {
    let project_c = Project {
        projectid: 16,
        name: "First Project".to_string(),
        projectdesc: "a".to_string(),
        clientdata: "a".to_string(),
    };

    let cproject = Project::update_project(project_c);

    // println!("cproject : {:?}", &cproject);
    if let Err(err) = dbstring(cproject) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_project_a() {
    let project_d = Project {
        projectid: 16,
        name: "First Project".to_string(),
        projectdesc: "a".to_string(),
        clientdata: "a".to_string(),
    };

    let dproject = Project::delete_project(project_d);

    // println!("dproject : {:?}", &dproject);
    if let Err(err) = dbstring(dproject) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_repository_a() {
    let repository_a = Repository {
        repositoryid: 16,
        placeid: 1,
        reponame: "First Repository".to_string(),
        comments: "a".to_string(),
    };

    let arepository = Repository::create_repository(repository_a);

    // println!("arepository : {:?}", &arepository);
    if let Err(err) = dbstring(arepository) {
        println!("Error: {:?}", err);
    }
}

pub fn read_repository_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let repository_b = Repository {
        repositoryid: 16,
        placeid: 1,
        reponame: "First Repository".to_string(),
        comments: "a".to_string(),
    };

    let brepository = Repository::read_repository(repository_b);

    // println!("brepository : {:?}", &brepository);

    let mut stmt = conn.prepare(&brepository)?;
    let repository_iter = stmt.query_map([], |row| {
        Ok(Repository {
            repositoryid: row.get(0)?,
            placeid: row.get(1)?,
            reponame: row.get(2)?,
            comments: row.get(3)?,
        })
    })?;

    for repositoryitem in repository_iter {
        println!("Found repository data {:?}", repositoryitem.unwrap());
    }

    Ok(())
}

pub fn update_repository_a() {
    let repository_c = Repository {
        repositoryid: 16,
        placeid: 1,
        reponame: "First Repository".to_string(),
        comments: "a".to_string(),
    };

    let crepository = Repository::update_repository(repository_c);

    // println!("crepository : {:?}", &crepository);
    if let Err(err) = dbstring(crepository) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_repository_a() {
    let repository_d = Repository {
        repositoryid: 16,
        placeid: 1,
        reponame: "First Repository".to_string(),
        comments: "a".to_string(),
    };

    let drepository = Repository::delete_repository(repository_d);

    // println!("drepository : {:?}", &drepository);
    if let Err(err) = dbstring(drepository) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_reposource_a() {
    let reposource_a = RepoSource {
        reposourceid: 16,
        repositoryid: 1,
        sourceid: 1,
        rsactivityid: 1,
        callnumber: "a".to_string(),
        description: "a".to_string(),
    };

    let areposource = RepoSource::create_reposource(reposource_a);

    // println!("areposource : {:?}", &areposource);
    if let Err(err) = dbstring(areposource) {
        println!("Error: {:?}", err);
    }
}

pub fn read_reposource_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let reposource_b = RepoSource {
        reposourceid: 16,
        repositoryid: 1,
        sourceid: 1,
        rsactivityid: 1,
        callnumber: "a".to_string(),
        description: "a".to_string(),
    };

    let breposource = RepoSource::read_reposource(reposource_b);

    // println!("breposource : {:?}", &breposource);

    let mut stmt = conn.prepare(&breposource)?;
    let reposource_iter = stmt.query_map([], |row| {
        Ok(RepoSource {
            reposourceid: row.get(0)?,
            repositoryid: row.get(1)?,
            sourceid: row.get(2)?,
            rsactivityid: row.get(3)?,
            callnumber: row.get(4)?,
            description: row.get(5)?,
        })
    })?;

    for reposourceitem in reposource_iter {
        println!("Found reposource data {:?}", reposourceitem.unwrap());
    }

    Ok(())
}

pub fn update_reposource_a() {
    let reposource_c = RepoSource {
        reposourceid: 16,
        repositoryid: 1,
        sourceid: 1,
        rsactivityid: 1,
        callnumber: "a".to_string(),
        description: "a".to_string(),
    };

    let creposource = RepoSource::update_reposource(reposource_c);

    // println!("creposource : {:?}", &creposource);
    if let Err(err) = dbstring(creposource) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_reposource_a() {
    let reposource_d = RepoSource {
        reposourceid: 16,
        repositoryid: 1,
        sourceid: 1,
        rsactivityid: 1,
        callnumber: "a".to_string(),
        description: "a".to_string(),
    };

    let dreposource = RepoSource::delete_reposource(reposource_d);

    // println!("dreposource : {:?}", &dreposource);
    if let Err(err) = dbstring(dreposource) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_representation_a() {
    let representation_a = Representation {
        representationid: 16,
        sourceid: 1,
        reprtypeid: 1,
        reprmediaid: 1,
        physfilecode: "a".to_string(),
        comments: "a".to_string(),
        externallink: "a".to_string(),
    };

    let arepresentation = Representation::create_representation(representation_a);

    println!("arepresentation : {:?}", &arepresentation);
    if let Err(err) = dbstring(arepresentation) {
        println!("Error: {:?}", err);
    }
}

pub fn read_representation_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let representation_b = Representation {
        representationid: 16,
        sourceid: 1,
        reprtypeid: 1,
        reprmediaid: 1,
        physfilecode: "a".to_string(),
        comments: "a".to_string(),
        externallink: "a".to_string(),
    };

    let brepresentation = Representation::read_representation(representation_b);

    println!("brepresentation : {:?}", &brepresentation);

    let mut stmt = conn.prepare(&brepresentation)?;
    let representation_iter = stmt.query_map([], |row| {
        Ok(Representation {
            representationid: row.get(0)?,
            sourceid: row.get(1)?,
            reprtypeid: row.get(2)?,
            reprmediaid: row.get(3)?,
            physfilecode: row.get(4)?,
            comments: row.get(5)?,
            externallink: row.get(6)?,
        })
    })?;

    for representationitem in representation_iter {
        println!("Found representation data {:?}", representationitem.unwrap());
    }

    Ok(())
}

pub fn update_representation_a() {
    let representation_c = Representation {
        representationid: 16,
        sourceid: 1,
        reprtypeid: 1,
        reprmediaid: 1,
        physfilecode: "a".to_string(),
        comments: "a".to_string(),
        externallink: "a".to_string(),
    };

    let crepresentation = Representation::update_representation(representation_c);

    println!("crepresentation : {:?}", &crepresentation);
    if let Err(err) = dbstring(crepresentation) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_representation_a() {
    let representation_d = Representation {
        representationid: 16,
        sourceid: 1,
        reprtypeid: 1,
        reprmediaid: 1,
        physfilecode: "a".to_string(),
        comments: "a".to_string(),
        externallink: "a".to_string(),
    };

    let drepresentation = Representation::delete_representation(representation_d);

    println!("drepresentation : {:?}", &drepresentation);
    if let Err(err) = dbstring(drepresentation) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_representtype_a() {
    let representtype_a = RepresentType {
        reprtypeid: 16,
        name: "First RepresentType".to_string(),
    };

    let arepresenttype = RepresentType::create_representtype(representtype_a);

    // println!("arepresenttype : {:?}", &arepresenttype);
    if let Err(err) = dbstring(arepresenttype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_representtype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let representtype_b = RepresentType {
        reprtypeid: 16,
        name: "First RepresentType".to_string(),
    };

    let brepresenttype = RepresentType::read_representtype(representtype_b);

    // println!("brepresenttype : {:?}", &brepresenttype);

    let mut stmt = conn.prepare(&brepresenttype)?;
    let representtype_iter = stmt.query_map([], |row| {
        Ok(RepresentType {
            reprtypeid: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    for representtypeitem in representtype_iter {
        println!("Found representtype data {:?}", representtypeitem.unwrap());
    }

    Ok(())
}

pub fn update_representtype_a() {
    let representtype_c = RepresentType {
        reprtypeid: 16,
        name: "First RepresentType".to_string(),
    };

    let crepresenttype = RepresentType::update_representtype(representtype_c);

    // println!("crepresenttype : {:?}", &crepresenttype);
    if let Err(err) = dbstring(crepresenttype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_representtype_a() {
    let representtype_d = RepresentType {
        reprtypeid: 16,
        name: "First RepresentType".to_string(),
    };

    let drepresenttype = RepresentType::delete_representtype(representtype_d);

    // println!("drepresenttype : {:?}", &drepresenttype);
    if let Err(err) = dbstring(drepresenttype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_reprmediatype_a() {
    let reprmediatype_a = ReprMediaType {
        reprmediaid: 16,
        reprmedianame: "First ReprMediaNameType".to_string(),
    };

    let areprmediatype = ReprMediaType::create_reprmediatype(reprmediatype_a);

    // println!("areprmediatype : {:?}", &areprmediatype);
    if let Err(err) = dbstring(areprmediatype) {
        println!("Error: {:?}", err);
    }
}

pub fn read_reprmediatype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let reprmediatype_b = ReprMediaType {
        reprmediaid: 16,
        reprmedianame: "First ReprMediaNameType".to_string(),
    };

    let breprmediatype = ReprMediaType::read_reprmediatype(reprmediatype_b);

    // println!("breprmediatype : {:?}", &breprmediatype);

    let mut stmt = conn.prepare(&breprmediatype)?;
    let reprmediatype_iter = stmt.query_map([], |row| {
        Ok(ReprMediaType {
            reprmediaid: row.get(0)?,
            reprmedianame: row.get(1)?,
        })
    })?;

    for reprmediatypeitem in reprmediatype_iter {
        println!("Found reprmediatype data {:?}", reprmediatypeitem.unwrap());
    }

    Ok(())
}

pub fn update_reprmediatype_a() {
    let reprmediatype_c = ReprMediaType {
        reprmediaid: 16,
        reprmedianame: "First ReprMediaNameType".to_string(),
    };

    let creprmediatype = ReprMediaType::update_reprmediatype(reprmediatype_c);

    // println!("creprmediatype : {:?}", &creprmediatype);
    if let Err(err) = dbstring(creprmediatype) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_reprmediatype_a() {
    let reprmediatype_d = ReprMediaType {
        reprmediaid: 16,
        reprmedianame: "First ReprMediaNameType".to_string(),
    };

    let dreprmediatype = ReprMediaType::delete_reprmediatype(reprmediatype_d);

    // println!("dreprmediatype : {:?}", &dreprmediatype);
    if let Err(err) = dbstring(dreprmediatype) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_researcher_a() {
    let researcher_a = Researcher {
        researcherid: 16,
        name: "First Researcher".to_string(),
        addressid: 1,
        comments: "a".to_string(),
    };

    let aresearcher = Researcher::create_researcher(researcher_a);

    // println!("aresearcher : {:?}", &aresearcher);
    if let Err(err) = dbstring(aresearcher) {
        println!("Error: {:?}", err);
    }
}

pub fn read_researcher_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let researcher_b = Researcher {
        researcherid: 16,
        name: "First Researcher".to_string(),
        addressid: 1,
        comments: "a".to_string(),
    };

    let bresearcher = Researcher::read_researcher(researcher_b);

    // println!("bresearcher : {:?}", &bresearcher);

    let mut stmt = conn.prepare(&bresearcher)?;
    let researcher_iter = stmt.query_map([], |row| {
        Ok(Researcher {
            researcherid: row.get(0)?,
            name: row.get(1)?,
            addressid: row.get(2)?,
            comments: row.get(3)?,
        })
    })?;

    for researcheritem in researcher_iter {
        println!("Found researcher data {:?}", researcheritem.unwrap());
    }

    Ok(())
}

pub fn update_researcher_a() {
    let researcher_c = Researcher {
        researcherid: 16,
        name: "First Researcher".to_string(),
        addressid: 1,
        comments: "a".to_string(),
    };

    let cresearcher = Researcher::update_researcher(researcher_c);

    // println!("cresearcher : {:?}", &cresearcher);
    if let Err(err) = dbstring(cresearcher) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_researcher_a() {
    let researcher_d = Researcher {
        researcherid: 16,
        name: "First Researcher".to_string(),
        addressid: 1,
        comments: "a".to_string(),
    };

    let dresearcher = Researcher::delete_researcher(researcher_d);

    // println!("dresearcher : {:?}", &dresearcher);
    if let Err(err) = dbstring(dresearcher) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_resobjactivity_a() {
    let resobjactivity_a = ResObjActivity {
        resobjactivityid: 16,
        resobjid: 1,
        activityid: 1,
    };

    let aresobjactivity = ResObjActivity::create_resobjactivity(resobjactivity_a);

    // println!("aresobjactivity : {:?}", &aresobjactivity);
    if let Err(err) = dbstring(aresobjactivity) {
        println!("Error: {:?}", err);
    }
}

pub fn read_resobjactivity_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let resobjactivity_b = ResObjActivity {
        resobjactivityid: 16,
        resobjid: 1,
        activityid: 1,
    };

    let bresobjactivity = ResObjActivity::read_resobjactivity(resobjactivity_b);

    // println!("bresobjactivity : {:?}", &bresobjactivity);

    let mut stmt = conn.prepare(&bresobjactivity)?;
    let resobjactivity_iter = stmt.query_map([], |row| {
        Ok(ResObjActivity {
            resobjactivityid: row.get(0)?,
            resobjid: row.get(1)?,
            activityid: row.get(2)?,
        })
    })?;

    for resobjactivityitem in resobjactivity_iter {
        println!("Found resobjactivity data {:?}", resobjactivityitem.unwrap());
    }

    Ok(())
}

pub fn update_resobjactivity_a() {
    let resobjactivity_c = ResObjActivity {
        resobjactivityid: 16,
        resobjid: 1,
        activityid: 1,
    };

    let cresobjactivity = ResObjActivity::update_resobjactivity(resobjactivity_c);

    // println!("cresobjactivity : {:?}", &cresobjactivity);
    if let Err(err) = dbstring(cresobjactivity) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_resobjactivity_a() {
    let resobjactivity_d = ResObjActivity {
        resobjactivityid: 16,
        resobjid: 1,
        activityid: 1,
    };

    let dresobjactivity = ResObjActivity::delete_resobjactivity(resobjactivity_d);

    // println!("dresobjactivity : {:?}", &dresobjactivity);
    if let Err(err) = dbstring(dresobjactivity) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_resobjective_a() {
    let resobjective_a = ResObjective {
        resobjid: 16,
        projectid: 1,
        subjectid: 1,
        subjecttype: "a".to_string(),
        name: "First ResObjective".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
        priority: "high".to_string(),
        status: "active".to_string(),
    };

    let aresobjective = ResObjective::create_resobjective(resobjective_a);

    // println!("aresobjective : {:?}", &aresobjective);
    if let Err(err) = dbstring(aresobjective) {
        println!("Error: {:?}", err);
    }
}

pub fn read_resobjective_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let resobjective_b = ResObjective {
        resobjid: 16,
        projectid: 1,
        subjectid: 1,
        subjecttype: "a".to_string(),
        name: "First ResObjective".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
        priority: "high".to_string(),
        status: "active".to_string(),
    };

    let bresobjective = ResObjective::read_resobjective(resobjective_b);

    // println!("bresobjective : {:?}", &bresobjective);

    let mut stmt = conn.prepare(&bresobjective)?;
    let resobjective_iter = stmt.query_map([], |row| {
        Ok(ResObjective {
            resobjid: row.get(0)?,
            projectid: row.get(1)?,
            subjectid: row.get(2)?,
            subjecttype: row.get(3)?,
            name: row.get(4)?,
            description: row.get(5)?,
            sequencenumber: row.get(6)?,
            priority: row.get(7)?,
            status: row.get(8)?,
        })
    })?;

    for resobjectiveitem in resobjective_iter {
        println!("Found resobjective data {:?}", resobjectiveitem.unwrap());
    }

    Ok(())
}

pub fn update_resobjective_a() {
    let resobjective_c = ResObjective {
        resobjid: 16,
        projectid: 1,
        subjectid: 1,
        subjecttype: "a".to_string(),
        name: "First ResObjective".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
        priority: "high".to_string(),
        status: "active".to_string(),
    };

    let cresobjective = ResObjective::update_resobjective(resobjective_c);

    // println!("cresobjective : {:?}", &cresobjective);
    if let Err(err) = dbstring(cresobjective) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_resobjective_a() {
    let resobjective_d = ResObjective {
        resobjid: 16,
        projectid: 1,
        subjectid: 1,
        subjecttype: "a".to_string(),
        name: "First ResObjective".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
        priority: "high".to_string(),
        status: "active".to_string(),
    };

    let dresobjective = ResObjective::delete_resobjective(resobjective_d);

    // println!("dresobjective : {:?}", &dresobjective);
    if let Err(err) = dbstring(dresobjective) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_resproj_a() {
    let resproj_a = ResProj {
        resprojid: 16,
        projectid: 1,
        researcherid: 1,
        researcherrole: "a".to_string(),
    };

    let aresproj = ResProj::create_resproj(resproj_a);

    // println!("aresproj : {:?}", &aresproj);
    if let Err(err) = dbstring(aresproj) {
        println!("Error: {:?}", err);
    }
}

pub fn read_resproj_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let resproj_b = ResProj {
        resprojid: 16,
        projectid: 1,
        researcherid: 1,
        researcherrole: "a".to_string(),
    };

    let bresproj = ResProj::read_resproj(resproj_b);

    // println!("bresproj : {:?}", &bresproj);

    let mut stmt = conn.prepare(&bresproj)?;
    let resproj_iter = stmt.query_map([], |row| {
        Ok(ResProj {
            resprojid: row.get(0)?,
            projectid: row.get(1)?,
            researcherid: row.get(2)?,
            researcherrole: row.get(3)?,
        })
    })?;

    for resprojitem in resproj_iter {
        println!("Found resproj data {:?}", resprojitem.unwrap());
    }

    Ok(())
}

pub fn update_resproj_a() {
    let resproj_c = ResProj {
        resprojid: 16,
        projectid: 1,
        researcherid: 1,
        researcherrole: "a".to_string(),
    };

    let cresproj = ResProj::update_resproj(resproj_c);

    // println!("cresproj : {:?}", &cresproj);
    if let Err(err) = dbstring(cresproj) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_resproj_a() {
    let resproj_d = ResProj {
        resprojid: 16,
        projectid: 1,
        researcherid: 1,
        researcherrole: "a".to_string(),
    };

    let dresproj = ResProj::delete_resproj(resproj_d);

    // println!("dresproj : {:?}", &dresproj);
    if let Err(err) = dbstring(dresproj) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_search_a() {
    let search_a = Search {
        searchid: 16,
        activityid: 1,
        sourceid: 1,
        repositoryid: 1,
        searchedfor: "a".to_string(),
    };

    let asearch = Search::create_search(search_a);

    // println!("asearch : {:?}", &asearch);
    if let Err(err) = dbstring(asearch) {
        println!("Error: {:?}", err);
    }
}

pub fn read_search_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let search_b = Search {
        searchid: 16,
        activityid: 1,
        sourceid: 1,
        repositoryid: 1,
        searchedfor: "a".to_string(),
    };

    let bsearch = Search::read_search(search_b);

    // println!("bsearch : {:?}", &bsearch);

    let mut stmt = conn.prepare(&bsearch)?;
    let search_iter = stmt.query_map([], |row| {
        Ok(Search {
            searchid: row.get(0)?,
            activityid: row.get(1)?,
            sourceid: row.get(2)?,
            repositoryid: row.get(3)?,
            searchedfor: row.get(4)?,
        })
    })?;

    for searchitem in search_iter {
        println!("Found search data {:?}", searchitem.unwrap());
    }

    Ok(())
}

pub fn update_search_a() {
    let search_c = Search {
        searchid: 16,
        activityid: 1,
        sourceid: 1,
        repositoryid: 1,
        searchedfor: "a".to_string(),
    };

    let csearch = Search::update_search(search_c);

    // println!("csearch : {:?}", &csearch);
    if let Err(err) = dbstring(csearch) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_search_a() {
    let search_d = Search {
        searchid: 16,
        activityid: 1,
        sourceid: 1,
        repositoryid: 1,
        searchedfor: "a".to_string(),
    };

    let dsearch = Search::delete_search(search_d);

    // println!("dsearch : {:?}", &dsearch);
    if let Err(err) = dbstring(dsearch) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_source_a() {
    let source_a = Source {
        sourceid: 16,
        highersourceid: 1,
        subjectplaceid: 1,
        jurisplaceid: 1, 
        researcherid: 1,
        subjectdate: "a".to_string(),
        comments: "a".to_string(),
    };

    let asource = Source::create_source(source_a);

    // println!("asource : {:?}", &asource);
    if let Err(err) = dbstring(asource) {
        println!("Error: {:?}", err);
    }
}

pub fn read_source_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let source_b = Source {
        sourceid: 16,
        highersourceid: 1,
        subjectplaceid: 1,
        jurisplaceid: 1, 
        researcherid: 1,
        subjectdate: "a".to_string(),
        comments: "a".to_string(),
    };

    let bsource = Source::read_source(source_b);

    // println!("bsource : {:?}", &bsource);

    let mut stmt = conn.prepare(&bsource)?;
    let source_iter = stmt.query_map([], |row| {
        Ok(Source {
            sourceid: row.get(0)?,
            highersourceid: row.get(1)?,
            subjectplaceid: row.get(2)?,
            jurisplaceid: row.get(3)?, 
            researcherid: row.get(4)?,
            subjectdate: row.get(5)?,
            comments: row.get(6)?,
        })
    })?;

    for sourceitem in source_iter {
        println!("Found source data {:?}", sourceitem.unwrap());
    }

    Ok(())
}

pub fn update_source_a() {
    let source_c = Source {
        sourceid: 16,
        highersourceid: 1,
        subjectplaceid: 1,
        jurisplaceid: 1, 
        researcherid: 1,
        subjectdate: "a".to_string(),
        comments: "a".to_string(),
    };

    let csource = Source::update_source(source_c);

    // println!("csource : {:?}", &csource);
    if let Err(err) = dbstring(csource) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_source_a() {
    let source_d = Source {
        sourceid: 16,
        highersourceid: 1,
        subjectplaceid: 1,
        jurisplaceid: 1, 
        researcherid: 1,
        subjectdate: "a".to_string(),
        comments: "a".to_string(),
    };

    let dsource = Source::delete_source(source_d);

    // println!("dsource : {:?}", &dsource);
    if let Err(err) = dbstring(dsource) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_sourcegroup_a() {
    let sourcegroup_a = SourceGroup {
        sourcegroupid: 16,
        sourcegroupname: "First SourceGroup".to_string(),
    };

    let asourcegroup = SourceGroup::create_sourcegroup(sourcegroup_a);

    // println!("asourcegroup : {:?}", &asourcegroup);
    if let Err(err) = dbstring(asourcegroup) {
        println!("Error: {:?}", err);
    }
}

pub fn read_sourcegroup_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let sourcegroup_b = SourceGroup {
        sourcegroupid: 16,
        sourcegroupname: "First SourceGroup".to_string(),
    };

    let bsourcegroup = SourceGroup::read_sourcegroup(sourcegroup_b);

    // println!("bsourcegroup : {:?}", &bsourcegroup);

    let mut stmt = conn.prepare(&bsourcegroup)?;
    let sourcegroup_iter = stmt.query_map([], |row| {
        Ok(SourceGroup {
            sourcegroupid: row.get(0)?,
            sourcegroupname: row.get(1)?,
        })
    })?;

    for sourcegroupitem in sourcegroup_iter {
        println!("Found sourcegroup data {:?}", sourcegroupitem.unwrap());
    }

    Ok(())
}

pub fn update_sourcegroup_a() {
    let sourcegroup_c = SourceGroup {
        sourcegroupid: 16,
        sourcegroupname: "First SourceGroup".to_string(),
    };

    let csourcegroup = SourceGroup::update_sourcegroup(sourcegroup_c);

    // println!("csourcegroup : {:?}", &csourcegroup);
    if let Err(err) = dbstring(csourcegroup) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_sourcegroup_a() {
    let sourcegroup_d = SourceGroup {
        sourcegroupid: 16,
        sourcegroupname: "First SourceGroup".to_string(),
    };

    let dsourcegroup = SourceGroup::delete_sourcegroup(sourcegroup_d);

    // println!("dsourcegroup : {:?}", &dsourcegroup);
    if let Err(err) = dbstring(dsourcegroup) {
        println!("Error: {:?}", err);
    }
}

/* -------------------------------------------------------------------------- */

pub fn make_srcgrpsrc_a() {
    let srcgrpsrc_a = SrcGrpSrc {
        srcgrpsrcid: 16,
        sourceid: 1,
        sourcegroupid: 1,
    };

    let asrcgrpsrc = SrcGrpSrc::create_srcgrpsrc(srcgrpsrc_a);

    // println!("asrcgrpsrc : {:?}", &asrcgrpsrc);
    if let Err(err) = dbstring(asrcgrpsrc) {
        println!("Error: {:?}", err);
    }
}

pub fn read_srcgrpsrc_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let srcgrpsrc_b = SrcGrpSrc {
        srcgrpsrcid: 16,
        sourceid: 1,
        sourcegroupid: 1,
    };

    let bsrcgrpsrc = SrcGrpSrc::read_srcgrpsrc(srcgrpsrc_b);

    // println!("bsrcgrpsrc : {:?}", &bsrcgrpsrc);

    let mut stmt = conn.prepare(&bsrcgrpsrc)?;
    let srcgrpsrc_iter = stmt.query_map([], |row| {
        Ok(SrcGrpSrc {
            srcgrpsrcid: row.get(0)?,
            sourcegroupid: row.get(1)?,
            sourceid: row.get(2)?,
        })
    })?;

    for srcgrpsrcitem in srcgrpsrc_iter {
        println!("Found srcgrpsrc data {:?}", srcgrpsrcitem.unwrap());
    }

    Ok(())
}

pub fn update_srcgrpsrc_a() {
    let srcgrpsrc_c = SrcGrpSrc {
        srcgrpsrcid: 16,
        sourceid: 1,
        sourcegroupid: 1,
    };

    let csrcgrpsrc = SrcGrpSrc::update_srcgrpsrc(srcgrpsrc_c);

    // println!("csrcgrpsrc : {:?}", &csrcgrpsrc);
    if let Err(err) = dbstring(csrcgrpsrc) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_srcgrpsrc_a() {
    let srcgrpsrc_d = SrcGrpSrc {
        srcgrpsrcid: 16,
        sourceid: 1,
        sourcegroupid: 1,
    };

    let dsrcgrpsrc = SrcGrpSrc::delete_srcgrpsrc(srcgrpsrc_d);

    // println!("dsrcgrpsrc : {:?}", &dsrcgrpsrc);
    if let Err(err) = dbstring(dsrcgrpsrc) {
        println!("Error: {:?}", err);
    }
}   

/* ------------------------------------------------------------------------- */

pub fn make_suretypart_a() {
    let suretypart_a = SuretyPart {
        suretypartid: 16,
        schemeid: 1,
        name: "First SuretyPart".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
    };

    let asuretypart = SuretyPart::create_suretypart(suretypart_a);

    // println!("asuretypart : {:?}", &asuretypart);
    if let Err(err) = dbstring(asuretypart) {
        println!("Error: {:?}", err);
    }
}

pub fn read_suretypart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let suretypart_b = SuretyPart {
        suretypartid: 16,
        schemeid: 1,
        name: "First SuretyPart".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
    };

    let bsuretypart = SuretyPart::read_suretypart(suretypart_b);

    // println!("bsuretypart : {:?}", &bsuretypart);

    let mut stmt = conn.prepare(&bsuretypart)?;
    let suretypart_iter = stmt.query_map([], |row| {
        Ok(SuretyPart {
            suretypartid: row.get(0)?,
            schemeid: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            sequencenumber: row.get(4)?,
        })
    })?;

    for suretypartitem in suretypart_iter {
        println!("Found suretypart data {:?}", suretypartitem.unwrap());
    }

    Ok(())
}

pub fn update_suretypart_a() {
    let suretypart_c = SuretyPart {
        suretypartid: 16,
        schemeid: 1,
        name: "First SuretyPart".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
    };

    let csuretypart = SuretyPart::update_suretypart(suretypart_c);

    // println!("csuretypart : {:?}", &csuretypart);
    if let Err(err) = dbstring(csuretypart) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_suretypart_a() {
    let suretypart_d = SuretyPart {
        suretypartid: 16,
        schemeid: 1,
        name: "First SuretyPart".to_string(),
        description: "a".to_string(),
        sequencenumber: 1,
    };

    let dsuretypart = SuretyPart::delete_suretypart(suretypart_d);

    // println!("dsuretypart : {:?}", &dsuretypart);
    if let Err(err) = dbstring(dsuretypart) {
        println!("Error: {:?}", err);
    }
}

/* ------------------------------------------------------------------------- */

pub fn make_suretyscheme_a() {
    let suretyscheme_a = SuretyScheme {
        suretyschemeid: 16,
        name: "First SuretyScheme".to_string(),
        description: "a".to_string(),
    };

    let asuretyscheme = SuretyScheme::create_suretyscheme(suretyscheme_a);

    // println!("asuretypescheme : {:?}", &asuretyscheme);
    if let Err(err) = dbstring(asuretyscheme) {
        println!("Error: {:?}", err);
    }
}

pub fn read_suretyscheme_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let suretyscheme_b = SuretyScheme {
        suretyschemeid: 16,
        name: "First SuretyScheme".to_string(),
        description: "a".to_string(),
    };

    let bsuretyscheme = SuretyScheme::read_suretyscheme(suretyscheme_b);

    // println!("bsuretypescheme : {:?}", &bsuretyscheme);

    let mut stmt = conn.prepare(&bsuretyscheme)?;
    let suretyscheme_iter = stmt.query_map([], |row| {
        Ok(SuretyScheme {
            suretyschemeid: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
        })
    })?;

    for suretyschemeitem in suretyscheme_iter {
        println!("Found suretyscheme data {:?}", suretyschemeitem.unwrap());
    }

    Ok(())
}

pub fn update_suretyscheme_a() {
    let suretyscheme_c = SuretyScheme {
        suretyschemeid: 16,
        name: "First SuretyScheme".to_string(),
        description: "a".to_string(),
    };

    let csuretyscheme = SuretyScheme::update_suretyscheme(suretyscheme_c);

    // println!("csuretypescheme : {:?}", &csuretyscheme);
    if let Err(err) = dbstring(csuretyscheme) {
        println!("Error: {:?}", err);
    }
}

pub fn delete_suretyscheme_a() {
    let suretyscheme_d = SuretyScheme {
        suretyschemeid: 16,
        name: "First SuretyScheme".to_string(),
        description: "a".to_string(),
    };

    let dsuretyscheme = SuretyScheme::delete_suretyscheme(suretyscheme_d);

    // println!("dsuretypescheme : {:?}", &dsuretyscheme);
    if let Err(err) = dbstring(dsuretyscheme) {
        // Handle the error here
        println!("Error: {:?}", err);
    }
}
