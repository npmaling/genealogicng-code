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

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use genealogicng::db_string::dbconn;

use persona::Persona;
mod persona;

use event::Event;
mod event;

pub fn search_file_line_by_line(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut line_count = 0;

    let mut output: Vec<String> = Vec::new();

    let mut try_persona: Persona = Persona {
        personaid: 0,
        persona_name: "".to_string(),
        description_comments: "".to_string(),
    };

    let mut try_event: Event = Event {
        eventid: 0,
        eventtypeid: 0,
        placeid: 0,
        eventdate: "".to_string(),
        eventname: "".to_string(),
    };

    // this is the main/major part of the function.
    while let Some(line) = lines.next() {
        let line = line.unwrap();

        line_count += 1;

        let tokens: Vec<&str> = line.split_whitespace().collect();
        let token_one = tokens.get(0).unwrap_or(&""); // 0..n usually 0 or 1
        let token_two = tokens.get(1).unwrap_or(&""); // TOKEN usually NAME, BIRT, DEAT, etc.
        let token_three = tokens.get(2).unwrap_or(&""); // Y or empty; or the rest of the string

        match *token_one {
            "0" => {
                if (*token_one == "0" && *token_three == "INDI") || *token_two == "TRLR" {
                    if output.len() > 0 {
                        println!("{:?}", output);
                        output.clear();
                    }
                    output.clear();
                }
            }
            "1" => {
                match *token_two {
                    "ADOP"=> {
                        process_event(&mut lines, &mut try_event, "Chr/Bapt");
                    }
                    "BAPM" | "BARM" | "BASM" | "BLES" => {
                        process_event(&mut lines, &mut try_event, "Baptism/Bar Mitzvah/Bat Mitzvah/Blessing");
                    }
                    // Can have a "Y" tag, like "DEAT", so we need to check for that.
                    "BIRT" => match *token_three {
                        // This may cause a problem.... If there *is* a BIRT date, the birth date/place will be ignored.
                        // This is because the next line is not checked for the "DATE" or "PLAC" keyword.
                        // Some GEDCOM files contain out-of-order data, so this is a problem.
                        "Y" => {
                            try_event.eventdate = "Birth date not known".to_string();
                            try_event.eventname = "Birth place not known".to_string();
                            let dbstr = Event::create_event(try_event.clone());
                            touch_database(dbstr);
                            try_event.eventdate = "".to_string();
                            try_event.eventname = "".to_string();
                        }
                        _ => {
                            process_event(&mut lines, &mut try_event, "Birth");
                        }
                    }
                    "BURI" => {
                        process_event(&mut lines, &mut try_event, "Burial");
                    }
                    "CAST" => {
                        process_event(&mut lines, &mut try_event, "Caste Information");
                    }
                    "CENS" => {
                        process_event(&mut lines, &mut try_event, "Census");
                    }
                    // Can have a "Y" tag, like "DEAT", so we need to check for that.
                    "CHR" => match *token_three  {
                        // This may cause a problem.... If there *is* a CHR date, the Christening date/place 
                        // will be ignored. This is because the next line is not checked for the "DATE" or 
                        // "PLAC" keyword. Some GEDCOM files contain out-of-order data, so this is a problem.
                        "Y" => {
                            try_event.eventdate = "Christening date not known".to_string();
                            try_event.eventname = "Christening place not known".to_string();
                            let dbstr = Event::create_event(try_event.clone());
                            touch_database(dbstr);
                            try_event.eventdate = "".to_string();
                            try_event.eventname = "".to_string();
                        }
                        _ => {
                            process_event(&mut lines, &mut try_event, "Death");
                        }
                    }
                    "CHRA" | "CONF" | "FCOM" | "ORDN" => {
                        process_event(&mut lines, &mut try_event, "Confirmation/First Communion/Ordination");
                    }
                    "CREM" => {
                        process_event(&mut lines, &mut try_event, "Cremation");
                    }
                    "DEAT" => match *token_three {
                        // This may cause a problem.... If there *is* a death date, the death date/place will be ignored.
                        // This is because the next line is not checked for the "DATE" or "PLAC" keyword.
                        // Some GEDCOM files contain out-of-order data, so this is a problem.
                        "Y" => {
                            try_event.eventdate = "Death date not known".to_string();
                            try_event.eventname = "Death place not known".to_string();
                            let dbstr = Event::create_event(try_event.clone());
                            touch_database(dbstr);
                            try_event.eventdate = "".to_string();
                            try_event.eventname = "".to_string();
                        }
                        _ => {
                            process_event(&mut lines, &mut try_event, "Death");
                        }
                    }
                    "DSCR" => {
                        process_event(&mut lines, &mut try_event, "Description");
                    }

                    "EDUC" => {
                        process_event(&mut lines, &mut try_event, "Education");
                    }
                    "EMIG" => {
                        process_event(&mut lines, &mut try_event, "Emigration");
                    }
                    // synonym for "EVEN", so do we process it the same way? The 5.5.1.5 spec says 
                    // it is a synonym, so needs a "TYPE" tag lke "EVEN" does.
                    "FACT" => {
                        process_event(&mut lines, &mut try_event, "Fact");
                    }
                    "GRAD" => {
                        process_event(&mut lines, &mut try_event, "Graduation");
                    }
                    // The 5.5.1.5 spec says it needs a "TYPE" tag lke "EVEN" does.
                    "IDNO" => {
                        process_event(&mut lines, &mut try_event, "Identification Number");
                    }
                    "IMMI" => {
                        process_event(&mut lines, &mut try_event, "Immigration");
                    }
                    "NAME" => {
                        if line.contains("/") {
                            let c = line.get(7..).unwrap();
                            try_persona.personaid = line_count;
                            try_persona.persona_name = c.to_string();
                            try_event.eventid = line_count;
                        }
                    }
                    "NATI" => {
                        process_event(&mut lines, &mut try_event, "Nationality");
                    }
                    "NATU" => {
                        process_event(&mut lines, &mut try_event, "Naturalization");
                    }
                    "NCHI" => {
                        process_event(&mut lines, &mut try_event, "Number of Children");
                    }
                    "NMR" => {
                        process_event(&mut lines, &mut try_event, "Number of Marriages");
                    }
                    "OCCU" => {
                        process_event(&mut lines, &mut try_event, "Occupation");
                    }
                    "PROB" => {
                        process_event(&mut lines, &mut try_event, "Probate");
                    }
                    "PROP" => {
                        process_event(&mut lines, &mut try_event, "Property");
                    }
                    "RELI" => {
                        process_event(&mut lines, &mut try_event, "Religion");
                    }
                    "RESI" => {
                        process_event(&mut lines, &mut try_event, "Residence");
                    }
                    "RETI" => {
                        process_event(&mut lines, &mut try_event, "Retirement");
                    }
                    "SEX" => {
                        let c = line.get(6..).unwrap();
                        try_persona.description_comments = c.to_string();
                        let dbstr = Persona::create_persona(try_persona.clone());
                        touch_database(dbstr);
                        try_persona.personaid = 0;
                        try_persona.persona_name = "".to_string();
                        try_persona.description_comments = "".to_string();
                    }
                    "SSN" => {
                        process_event(&mut lines, &mut try_event, "Social Security Number");
                    }
                    "TITL" => {
                        process_event(&mut lines, &mut try_event, "Title");
                    }
                    "WILL" => {
                        process_event(&mut lines, &mut try_event, "Will");
                    }

                    "EVEN" => {
                        let mut e_type = String::new();
                        let mut e_place = String::new();

                        let v = vec!["type", "date", "place"];

                        for _ in &v {
                            if let Some(next_line) = lines.next() {
                                let next_line = next_line.unwrap_or_default();
                                if next_line.contains("TYPE") {
                                    e_type = next_line.get(7..).unwrap_or_default().to_string();
                                }
                                if next_line.contains("DATE") {
                                    let e_date = next_line.get(7..).unwrap_or_default();
                                    let e: String = format!("Event Date: {}", &e_date);
                                    try_event.eventdate = e.to_string();
                                }
                                if next_line.contains("PLAC") {
                                    e_place = next_line.get(7..).unwrap_or_default().to_string();
                                }
                            }
                        }
                        let e: String = format!("Type: {}; Place: {}", &e_type, &e_place);
                        try_event.eventname = e.to_string();
                        let dbstr = Event::create_event(try_event.clone());
                        touch_database(dbstr);
                        try_event.eventdate = "".to_string();
                        try_event.eventname = "".to_string();
                    }

                    _ => {
                        // ignore the rest
                    }
                }
            }
            "2" => {
                match *token_two {
                    "GIVN" | "SURN" => {
                        let c = line.get(7..).unwrap_or_default();
                        output.push(c.to_string());
                    }
                    _ => {
                        // ignore the rest
                    }
                }
            }
            _ => {
                // ignore the rest
            }
        }
        continue;
    }
}

fn process_event(
    lines: &mut dyn Iterator<Item = Result<String, std::io::Error>>,
    try_event: &mut Event,
    event_type: &str,
) {
    let d = String::new();
//    let e = String::new();
    
    if let Some(next_line) = lines.next() {
        let next_line = next_line.unwrap_or_default();
        if next_line.contains("DATE") && next_line.len() > 7 {
            let d = next_line.get(7..).unwrap_or_default();
            let e: String = format!("{} date: {}", &event_type, d);
            try_event.eventdate = e.to_string();
        }
        else {
            let e: String = format!("{} date not known {}", &event_type, d);
            try_event.eventdate = e.to_string();
        }
        if let Some(next_line) = lines.next() {
            let next_line = next_line.unwrap_or_default();
            if next_line.contains("PLAC") && next_line.len() > 7 {
                let d = next_line.get(7..).unwrap_or_default();
                let e: String = format!("{} place: {}", &event_type, d);
                try_event.eventname = e.to_string();
            }
            else {
                let e: String = format!("{} place not known {}", &event_type, d);
                try_event.eventname = e.to_string();
            }
            }
        let dbstr = Event::create_event(try_event.clone());
        touch_database(dbstr);
        try_event.eventdate = "".to_string();
        try_event.eventname = "".to_string();
    }
}

fn touch_database(dbstr: String) {
    let _ = dbconn(
        &dbstr,
        "C:/Users/npmal/projects/genealogicng-code/glNG.db".to_string(),
    );
}
