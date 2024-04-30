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

#[derive(Debug)]
pub struct Persona {
    pub personaid: i64,
    pub persona_name: String,
    pub description_comments: String,
}

impl Persona {
    pub fn create_persona(
        Persona {
            personaid,
            persona_name,
            description_comments,
        }: Persona,
    ) -> String {
        let parameters = format!(
            "INSERT INTO Persona (personaid, persona_name, description_comments) VALUES ({}, \"{}\", \"{}\")",
            &personaid.to_string(),
            &persona_name,
            &description_comments
        );
        parameters
    }

    pub fn read_persona(
        Persona {
            personaid,
            persona_name,
            description_comments,
        }: Persona,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM Persona WHERE personaid={}",
            &personaid.to_string(),
        );
        // println!("This is read_persona: {}", parameters);
        parameters
    }

    pub fn update_persona(
        Persona {
            personaid,
            persona_name,
            description_comments,
        }: Persona,
    ) -> String {
        let parameters = format!(
            "UPDATE persona SET personaid={}, persona_name=\"{}\", description_comments=\"{}\" WHERE personaid={}",
            &personaid.to_string(),
            &persona_name,
            &description_comments,
            &personaid.to_string(),
        );
        // println!("This is update_persona: {}", parameters);
        parameters
    }

    pub fn delete_persona(
        Persona {
            personaid,
            persona_name,
            description_comments,
        }: Persona,
    ) -> String {
        let parameters = format!(
            "DELETE FROM persona WHERE personaid={}",
            &personaid.to_string(),
        );
        // println!("This is delete_persona: {}", parameters);
        parameters
    }
}

