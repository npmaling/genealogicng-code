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

pub fn make_assertassert_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let assertassert_a = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let aassertassert = AssertAssert::create_assertassert(assertassert_a);

    println!("aassertassert : {:?}", &aassertassert);
    dbstring(&conn, aassertassert);

    Ok(())
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

    println!("bassertassert : {:?}", &bassertassert);

    // dbstring(&conn, &bassertassert);

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

pub fn update_assertassert_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let assertassert_c = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let cassertassert = AssertAssert::update_assertassert(assertassert_c);

    println!("cassertassert : {:?}", &cassertassert);
    dbstring(&conn, cassertassert);

    Ok(())
}

pub fn delete_assertassert_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let assertassert_d = AssertAssert {
        assertassertid: 16,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let dassertassert = AssertAssert::delete_assertassert(assertassert_d);

    println!("dassertassert : {:?}", &dassertassert);
    dbstring(&conn, dassertassert);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_characteristic_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let characteristic_a = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let acharacteristic = Characteristic::create_characteristic(characteristic_a);

    println!("acharacteristic : {:?}", &acharacteristic);
    dbstring(&conn, acharacteristic);

    Ok(())
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

    println!("bcharacteristic : {:?}", &bcharacteristic);

    // dbstring(&conn, &bcharacteristic);

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

pub fn update_characteristic_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let characteristic_c = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let ccharacteristic = Characteristic::update_characteristic(characteristic_c);

    println!("ccharacteristic : {:?}", &ccharacteristic);
    dbstring(&conn, ccharacteristic);

    Ok(())
}

pub fn delete_characteristic_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let characteristic_d = Characteristic {
        characteristicid: 16,
        placeid: 1,
        characteristicdate: "20230101".to_string(),
        ascdescnone: "a".to_string(),
    };

    let dcharacteristic = Characteristic::delete_characteristic(characteristic_d);

    println!("dcharacteristic : {:?}", &dcharacteristic);
    dbstring(&conn, dcharacteristic);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_charpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charpart_a = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let acharpart = CharPart::create_charpart(charpart_a);

    println!("acharpart : {:?}", &acharpart);
    dbstring(&conn, acharpart);

    Ok(())
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

    println!("bcharpart : {:?}", &bcharpart);

    // dbstring(&conn, &bcharpart);

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

pub fn update_charpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charpart_c = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let ccharpart = CharPart::update_charpart(charpart_c);

    println!("ccharpart : {:?}", &ccharpart);
    dbstring(&conn, ccharpart);

    Ok(())
}

pub fn delete_charpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charpart_d = CharPart {
        characteristicpartid: 16,
        characteristicid: 1,
        charparttypeid: 1,
        charpartname: "First CharPart".to_string(),
        charpartseq: 1,
    };

    let dcharpart = CharPart::delete_charpart(charpart_d);

    println!("dcharpart : {:?}", &dcharpart);
    dbstring(&conn, dcharpart);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_charparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charparttype_a = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let acharparttype = CharPartType::create_charparttype(charparttype_a);

    println!("acharparttype : {:?}", &acharparttype);
    dbstring(&conn, acharparttype);

    Ok(())
}

pub fn read_charparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charparttype_b = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let bcharparttype = CharPartType::read_charparttype(charparttype_b);

    println!("bcharparttype : {:?}", &bcharparttype);

    // dbstring(&conn, &bcharparttype);

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

pub fn update_charparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charparttype_c = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let ccharparttype = CharPartType::update_charparttype(charparttype_c);

    println!("ccharparttype : {:?}", &ccharparttype);
    dbstring(&conn, ccharparttype);

    Ok(())
}

pub fn delete_charparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let charparttype_d = CharPartType {
        charparttypeid: 16,
        charparttypename: "First CharPartType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let dcharparttype = CharPartType::delete_charparttype(charparttype_d);

    println!("dcharparttype : {:?}", &dcharparttype);
    dbstring(&conn, dcharparttype);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_citationpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationpart_a = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let acitationpart = CitationPart::create_citationpart(citationpart_a);

    println!("acitationpart : {:?}", &acitationpart);
    dbstring(&conn, acitationpart);

    Ok(())
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

    println!("bcitationpart : {:?}", &bcitationpart);

    // dbstring(&conn, &bcitationpart);

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

pub fn update_citationpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationpart_c = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let ccitationpart = CitationPart::update_citationpart(citationpart_c);

    println!("ccitationpart : {:?}", &ccitationpart);
    dbstring(&conn, ccitationpart);

    Ok(())
}

pub fn delete_citationpart_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationpart_d = CitationPart {
        citationpartid: 16,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "First CitationPart".to_string(),
    };

    let dcitationpart = CitationPart::delete_citationpart(citationpart_d);

    println!("dcitationpart : {:?}", &dcitationpart);
    dbstring(&conn, dcitationpart);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_citationparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationparttype_a = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let acitationparttype = CitationPartType::create_citationparttype(citationparttype_a);

    println!("acitationparttype : {:?}", &acitationparttype);
    dbstring(&conn, acitationparttype);

    Ok(())
}

pub fn read_citationparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationparttype_b = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let bcitationparttype = CitationPartType::read_citationparttype(citationparttype_b);

    println!("bcitationparttype : {:?}", &bcitationparttype);

    // dbstring(&conn, &bcitationparttype);

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

pub fn update_citationparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationparttype_c = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let ccitationparttype = CitationPartType::update_citationparttype(citationparttype_c);

    println!("ccitationparttype : {:?}", &ccitationparttype);
    dbstring(&conn, ccitationparttype);

    Ok(())
}

pub fn delete_citationparttype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let citationparttype_d = CitationPartType {
        citationparttypeid: 16,
        citationparttypename: "First CitationPartType".to_string(),
    };

    let dcitationparttype = CitationPartType::delete_citationparttype(citationparttype_d);

    println!("dcitationparttype : {:?}", &dcitationparttype);
    dbstring(&conn, dcitationparttype);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_event_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let event_a = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let aevent = Event::create_event(event_a);

    println!("aevent : {:?}", &aevent);
    dbstring(&conn, aevent);

    Ok(())
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

    println!("bevent : {:?}", &bevent);

    // dbstring(&conn, &bevent);

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

pub fn update_event_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let event_c = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let cevent = Event::update_event(event_c);

    println!("cevent : {:?}", &cevent);
    dbstring(&conn, cevent);

    Ok(())
}

pub fn delete_event_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let event_d = Event {
        eventid: 16,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "20230101".to_string(),
        eventname: "First Event".to_string(),
    };

    let devent = Event::delete_event(event_d);

    println!("devent : {:?}", &devent);
    dbstring(&conn, devent);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_eventtype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtype_a = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let aeventtype = EventType::create_eventtype(eventtype_a);

    println!("aeventtype : {:?}", &aeventtype);
    dbstring(&conn, aeventtype);

    Ok(())
}

pub fn read_eventtype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtype_b = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let beventtype = EventType::read_eventtype(eventtype_b);

    println!("beventtype : {:?}", &beventtype);

    // dbstring(&conn, &beventtype);

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

pub fn update_eventtype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtype_c = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let ceventtype = EventType::update_eventtype(eventtype_c);

    println!("ceventtype : {:?}", &ceventtype);
    dbstring(&conn, ceventtype);

    Ok(())
}

pub fn delete_eventtype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtype_d = EventType {
        eventtypeid: 16,
        eventtypename: "First EventType".to_string(),
        gedcomtag: "a".to_string(),
    };

    let deventtype = EventType::delete_eventtype(eventtype_d);

    println!("deventtype : {:?}", &deventtype);
    dbstring(&conn, deventtype);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_eventtyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtyperole_a = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let aeventtyperole = EventTypeRole::create_eventtyperole(eventtyperole_a);

    println!("aeventtyperole : {:?}", &aeventtyperole);
    dbstring(&conn, aeventtyperole);

    Ok(())
}

pub fn read_eventtyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtyperole_b = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let beventtyperole = EventTypeRole::read_eventtyperole(eventtyperole_b);

    println!("beventtyperole : {:?}", &beventtyperole);

    // dbstring(&conn, &beventtyperole);

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

pub fn update_eventtyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtyperole_c = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let ceventtyperole = EventTypeRole::update_eventtyperole(eventtyperole_c);

    println!("ceventtyperole : {:?}", &ceventtyperole);
    dbstring(&conn, ceventtyperole);

    Ok(())
}

pub fn delete_eventtyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let eventtyperole_d = EventTypeRole {
        eventtyperoleid: 16,
        eventtypeid: 1,
        eventtyperolename: "EventTypeRole name test string".to_string(),
    };

    let deventtyperole = EventTypeRole::delete_eventtyperole(eventtyperole_d);

    println!("deventtyperole : {:?}", &deventtyperole);
    dbstring(&conn, deventtyperole);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_glassertion_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

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

    println!("aglassertion : {:?}", &aglassertion);
    dbstring(&conn, aglassertion);

    Ok(())
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

    println!("bglassertion : {:?}", &bglassertion);

    // dbstring(&conn, &bglassertion);

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

pub fn update_glassertion_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

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

    println!("cglassertion : {:?}", &cglassertion);
    dbstring(&conn, cglassertion);

    Ok(())
}

pub fn delete_glassertion_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

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

    println!("dglassertion : {:?}", &dglassertion);
    dbstring(&conn, dglassertion);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_glgroup_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgroup_a = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let aglgroup = GlGroup::create_glgroup(glgroup_a);

    println!("aglgroup : {:?}", &aglgroup);
    dbstring(&conn, aglgroup);

    Ok(())
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

    println!("bglgroup : {:?}", &bglgroup);

    // dbstring(&conn, &bglgroup);

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

pub fn update_glgroup_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgroup_c = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let cglgroup = GlGroup::update_glgroup(glgroup_c);

    println!("cglgroup : {:?}", &cglgroup);
    dbstring(&conn, cglgroup);

    Ok(())
}

pub fn delete_glgroup_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgroup_d = GlGroup {
        glgroupid: 16,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230101".to_string(),
        glgroupname: "First Group".to_string(),
        glgroupcriteria: "a".to_string(),
    };

    let dglgroup = GlGroup::delete_glgroup(glgroup_d);

    println!("dglgroup : {:?}", &dglgroup);
    dbstring(&conn, dglgroup);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_glgrouptype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptype_a = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let aglgrouptype = GlGroupType::create_glgrouptype(glgrouptype_a);

    println!("aglgrouptype : {:?}", &aglgrouptype);
    dbstring(&conn, aglgrouptype);

    Ok(())
}

pub fn read_glgrouptype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptype_b = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let bglgrouptype = GlGroupType::read_glgrouptype(glgrouptype_b);

    println!("bglgrouptype : {:?}", &bglgrouptype);

    // dbstring(&conn, &bglgrouptype);

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

pub fn update_glgrouptype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptype_c = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let cglgrouptype = GlGroupType::update_glgrouptype(glgrouptype_c);

    println!("cglgrouptype : {:?}", &cglgrouptype);
    dbstring(&conn, cglgrouptype);

    Ok(())
}

pub fn delete_glgrouptype_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptype_d = GlGroupType {
        glgrouptypeid: 16,
        glgroupname: "First GroupType".to_string(),
        ascdescnone: "a".to_string(),
    };

    let dglgrouptype = GlGroupType::delete_glgrouptype(glgrouptype_d);

    println!("dglgrouptype : {:?}", &dglgrouptype);
    dbstring(&conn, dglgrouptype);

    Ok(())
}

/* ------------------------------------------------------------------------- */

pub fn make_glgrouptyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptyperole_a = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let aglgrouptyperole = GlGroupTypeRole::create_glgrouptyperole(glgrouptyperole_a);

    println!("aglgrouptyperole : {:?}", &aglgrouptyperole);
    dbstring(&conn, aglgrouptyperole);

    Ok(())
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

    println!("bglgrouptyperole : {:?}", &bglgrouptyperole);

    // dbstring(&conn, &bglgrouptyperole);

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

pub fn update_glgrouptyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptyperole_c = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let cglgrouptyperole = GlGroupTypeRole::update_glgrouptyperole(glgrouptyperole_c);

    println!("cglgrouptyperole : {:?}", &cglgrouptyperole);
    dbstring(&conn, cglgrouptyperole);

    Ok(())
}

pub fn delete_glgrouptyperole_a() -> Result<(), rusqlite::Error> {
    let conn: Connection = Connection::open("C:/Users/npmal/projects/genealogicng-code/database.db")?;

    let glgrouptyperole_d = GlGroupTypeRole {
        glgrouptyperoleid: 16,
        glgrouptypeid: 1,
        glgrouptypename: "First GroupTypeRole".to_string(),
        sequencenumber: 1,
    };

    let dglgrouptyperole = GlGroupTypeRole::delete_glgrouptyperole(glgrouptyperole_d);

    println!("dglgrouptyperole : {:?}", &dglgrouptyperole);
    dbstring(&conn, dglgrouptyperole);

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
