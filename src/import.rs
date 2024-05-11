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

use persona::Persona;
mod persona;

pub fn fileopen() {
    // let file_path = "/Users/npmal/My Drive/GDrive/genea/gedcom/Maling_20230807.ged";
    // let file_path = "/Users/npmal/projects/glngimport/allged.ged";
    let file_path = "/Users/npmal/projects/glngimport/ged";

    search_file_line_by_line(file_path);
}

fn search_file_line_by_line(file_path: &str) {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut output: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
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
                            persona_name = c.to_string();
                            output.push(c.to_string());
                        }
                    }
                    "SEX" => {
                        let c = line.get(6..).unwrap();
                        output.push(c.to_string());
                    }
                    "BIRT" => {
                        if let Some(next_line) = lines.next() {
                            let next_line = next_line.unwrap();
                            let d = next_line.get(7..).unwrap();
                            let e: String = "Birth date: ".to_string() + d;
                            output.push(e.to_string());
                            if let Some(next_line) = lines.next() {
                                let next_line = next_line.unwrap();
                                if next_line.contains("PLAC") {
                                    let d = next_line.get(7..).unwrap();
                                    let e: String = "Birth place: ".to_string() + d;
                                    output.push(e.to_string());
                                }
                            }
                        }
                    }
                    "CHR" | "BAPM" | "BARM" | "BASM" | "BLES" => {
                        if let Some(next_line) = lines.next() {
                            let next_line = next_line.unwrap();
                            let d = next_line.get(7..).unwrap();
                            let e: String = "Christening/Baptism date: ".to_string() + d;
                            output.push(e.to_string());
                            if let Some(next_line) = lines.next() {
                                let next_line = next_line.unwrap();
                                if next_line.contains("PLAC") {
                                    let d = next_line.get(7..).unwrap();
                                    let e: String = "Christening/Baptism place: ".to_string() + d;
                                    output.push(e.to_string());
                                }
                            }
                        }
                    }
                    "DEAT" => match *token_three {
                        "Y" => {
                            output.push("Death date not known".to_string());
                        }
                        _ => {
                            if let Some(next_line) = lines.next() {
                                let next_line = next_line.unwrap();
                                let d = next_line.get(7..).unwrap();
                                let e: String = "Death date: ".to_string() + d;
                                output.push(e.to_string());
                                if let Some(next_line) = lines.next() {
                                    let next_line = next_line.unwrap();
                                    if next_line.contains("PLAC") {
                                        let d = next_line.get(7..).unwrap();
                                        let e: String = "Death place: ".to_string() + d;
                                        output.push(e.to_string());
                                    }
                                }
                            }
                        }
                    }
                    "BURI" => match *token_three {
                        "Y" => {
                            output.push("Burial/Cremation date not known".to_string());
                        }
                        _ => {
                            if let Some(next_line) = lines.next() {
                                let next_line = next_line.unwrap();
                                if next_line.contains("DATE") {
                                    let d = next_line.get(7..).unwrap();
                                    let e: String = "Burial date: ".to_string() + d;
                                    output.push(e.to_string());
                                }
                                if let Some(next_line) = lines.next() {
                                    let next_line = next_line.unwrap();
                                    if next_line.contains("PLAC") {
                                        let d = next_line.get(7..).unwrap();
                                        let e: String = "Burial place: ".to_string() + d;
                                        output.push(e.to_string());
                                    }
                                }
                            }
                        }
                    }
                    "CREM" => match *token_three {
                        "Y" => {
                            output.push("Cremation date not known".to_string());
                        }
                        _ => {
                            if let Some(next_line) = lines.next() {
                                let next_line = next_line.unwrap();
                                let d = next_line.get(7..).unwrap();
                                let e: String = "Cremation date: ".to_string() + d;
                                output.push(e.to_string());
                                if let Some(next_line) = lines.next() {
                                    let next_line = next_line.unwrap();
                                    if next_line.contains("PLAC") {
                                        let d = next_line.get(7..).unwrap();
                                        let e: String = "Cremation place: ".to_string() + d;
                                        output.push(e.to_string());
                                    }
                                }
                            }
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
