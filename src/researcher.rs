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
pub struct Researcher {
    pub researcherid: i64,
    pub name: String,
    pub addressid: i64,
    pub comments: String,
}

impl Researcher {
    pub fn create_researcher(
        Researcher {
            researcherid,
            name,
            addressid,
            comments,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "INSERT INTO researcher (researcherid, name, addressid, comments) VALUES ({}, \"{}\", {}, \"{}\")",
            &researcherid.to_string(),
            &name,
            &addressid.to_string(),
            &comments,
        );
        // println!("This is create_researcher: {}", parameters);
        parameters
    }

    pub fn read_researcher(
        Researcher {
            researcherid,
            name: _,
            addressid: _,
            comments: _,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM researcher WHERE researcherid={}",
            &researcherid.to_string(),
        );
        // println!("This is read_researcher: {}", parameters);
        parameters
    }

    pub fn update_researcher(
        Researcher {
            researcherid,
            name,
            addressid,
            comments,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "UPDATE researcher SET researcherid={}, name=\"{}\", addressid={}, comments=\"{}\" WHERE researcherid={}",
            &researcherid.to_string(),
            &name,
            &addressid.to_string(),
            &comments,
            &researcherid.to_string(),
        );
        // println!("This is update_researcher: {}", parameters);
        parameters
    }

    pub fn delete_researcher(
        Researcher {
            researcherid,
            name: _,
            addressid: _,
            comments: _,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "DELETE FROM researcher WHERE researcherid={}",
            &researcherid.to_string(),
        );
        // println!("This is delete_researcher: {}", parameters);
        parameters
    }
}
