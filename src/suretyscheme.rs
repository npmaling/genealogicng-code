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
pub struct SuretyScheme {
    pub suretyschemeid: i64,
    pub name: String,
    pub description: String,
}

impl SuretyScheme {
    pub fn create_suretyscheme(
        SuretyScheme {
            suretyschemeid,
            name,
            description,
        }: SuretyScheme,
    ) -> String {
        let parameters = format!(
            "INSERT INTO suretyscheme (suretyschemeid, name, description) VALUES ({}, \"{}\", \"{}\")",
            suretyschemeid.to_string(),
            name,
            description,
        );
        // println!("This is create_suretyscheme: {}", parameters);
        parameters
    }

    pub fn read_suretyscheme(
        SuretyScheme {
            suretyschemeid,
            name,
            description,
        }: SuretyScheme,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM suretyscheme WHERE suretyschemeid={}",
            suretyschemeid.to_string(),
        );
        // println!("This is read_suretyscheme: {}", parameters);
        parameters
    }

    pub fn update_suretyscheme(
        SuretyScheme {
            suretyschemeid,
            name,
            description,
        }: SuretyScheme,
    ) -> String {
        let parameters = format!(
            "UPDATE suretyscheme SET suretyschemeid={}, name=\"{}\", description=\"{}\" WHERE suretyschemeid={}",
            suretyschemeid.to_string(),
            name,
            description,
            suretyschemeid.to_string(),
        );
        // println!("This is update_suretyscheme: {}", parameters);
        parameters
    }

    pub fn delete_suretyscheme(
        SuretyScheme {
            suretyschemeid,
            name,
            description,
        }: SuretyScheme,
    ) -> String {
        let parameters = format!(
            "DELETE FROM suretyscheme WHERE suretyschemeid={}",
            suretyschemeid.to_string(),
        );
        // println!("This is delete_suretyscheme: {}", parameters);
        parameters
    }
}
