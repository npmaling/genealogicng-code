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

    // this is the main/major part of the funtion.
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
                    "NAME" => {
                        if line.contains("/") {
                            let c = line.get(7..).unwrap();
                            try_persona.personaid = line_count;
                            try_persona.persona_name = c.to_string();
                            try_event.eventid = line_count;
                        }
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
                    "BIRT" => {
                        let event_type: &str = "Birth";
                        process_event(&mut lines, &mut try_event, &event_type);
                    }
                    "CHR" | "BAPM" | "BARM" | "BASM" | "BLES" => {
                        let event_type: &str = "Chr/Bapt";
                        process_event(&mut lines, &mut try_event, &event_type);
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
                            let event_type: &str = "Death";
                            process_event(&mut lines, &mut try_event, &event_type);
                        }
                    }
                    "BURI" => match *token_three {
                        "Y" => {
                            try_event.eventdate = "Burial date not known".to_string();
                            try_event.eventname = "Burial place not known".to_string();
                            let dbstr = Event::create_event(try_event.clone());
                            touch_database(dbstr);
                            try_event.eventdate = "".to_string();
                            try_event.eventname = "".to_string();
                    }
                        _ => {
                            let event_type: &str = "Burial";
                            process_event(&mut lines, &mut try_event, &event_type);
                        }
                    }
                    "CREM" => match *token_three {
                        "Y" => {
                            try_event.eventdate = "Cremation date not known".to_string();
                            try_event.eventname = "Cremation place not known".to_string();
                            let dbstr = Event::create_event(try_event.clone());
                            touch_database(dbstr);
                            try_event.eventdate = "".to_string();
                            try_event.eventname = "".to_string();
                    }
                        _ => {
                            let event_type: &str = "Cremation";
                            process_event(&mut lines, &mut try_event, &event_type);
                        }
                    },
                    _ => {
                        // ignore the rest
                    }
                }
            }
            "2" => {
                match *token_two {
                    "GIVN" | "SURN" => {
                        let c = line.get(7..).unwrap();
                        output.push(c.to_string());
                    },
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

fn process_event(lines: &mut dyn Iterator<Item = Result<String, std::io::Error>>, try_event: &mut Event, event_type: &str) {
    if let Some(next_line) = lines.next() {
        let next_line = next_line.unwrap();
        if next_line.contains("DATE") && next_line.len() > 7 {
            let d = next_line.get(7..).unwrap();
            let e: String = format!("{} date: {}", &event_type, d);
            try_event.eventdate = e.to_string();
        }
        if let Some(next_line) = lines.next() {
            let next_line = next_line.unwrap();
            if next_line.contains("PLAC") && next_line.len() > 7 {
                let d = next_line.get(7..).unwrap();
                let e: String = format!("{} place: {}", &event_type, d);
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
    let _ = dbconn(&dbstr, "C:/Users/npmal/projects/genealogicng-code/database.db".to_string());
}
