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
pub struct ResProj {
    pub resprojid: i64,
    pub projectid: i64,
    pub researcherid: i64,
    pub researcherrole: String,
}

impl ResProj {
    pub fn create_resproj(
        ResProj {
            resprojid,
            projectid,
            researcherid,
            researcherrole,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "INSERT INTO resproj (resprojid, projectid, researcherid, researcherrole) VALUES ({}, {}, {}, \"{}\")",
            &resprojid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &researcherrole,
        );
        // println!("This is create_resproj: {}", parameters);
        parameters
    }

    pub fn read_resproj(
        ResProj {
            resprojid,
            projectid: _,
            researcherid: _,
            researcherrole: _,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM resproj WHERE resprojid={}",
            &resprojid.to_string(),
        );
        // println!("This is read_resproj: {}", parameters);
        parameters
    }

    pub fn update_resproj(
        ResProj {
            resprojid,
            projectid,
            researcherid,
            researcherrole,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "UPDATE resproj SET resprojid={}, projectid={}, researcherid={}, researcherrole=\"{}\" WHERE resprojid={}",
            &resprojid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &researcherrole,
            &resprojid.to_string(),
        );
        // println!("This is update_resproj: {}", parameters);
        parameters
    }

    pub fn delete_resproj(
        ResProj {
            resprojid,
            projectid: _,
            researcherid: _,
            researcherrole: _,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "DELETE FROM resproj WHERE resprojid={}",
            &resprojid.to_string(),
        );
        // println!("This is delete_resproj: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_resproj() {
        let resproj = ResProj {
            resprojid: 1,
            projectid: 1,
            researcherid: 1,
            researcherrole: String::from("Role"),
        };
        let expected = "INSERT INTO resproj (resprojid, projectid, researcherid, researcherrole) VALUES (1, 1, 1, \"Role\")";
        let result = ResProj::create_resproj(resproj);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_resproj() {
        let resproj = ResProj {
            resprojid: 1,
            projectid: 1,
            researcherid: 1,
            researcherrole: String::from("Role"),
        };
        let expected = "SELECT * FROM resproj WHERE resprojid=1";
        let result = ResProj::read_resproj(resproj);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_resproj() {
        let resproj = ResProj {
            resprojid: 1,
            projectid: 1,
            researcherid: 1,
            researcherrole: String::from("Role"),
        };
        let expected = "UPDATE resproj SET resprojid=1, projectid=1, researcherid=1, researcherrole=\"Role\" WHERE resprojid=1";
        let result = ResProj::update_resproj(resproj);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_resproj() {
        let resproj = ResProj {
            resprojid: 1,
            projectid: 1,
            researcherid: 1,
            researcherrole: String::from("Role"),
        };
        let expected = "DELETE FROM resproj WHERE resprojid=1";
        let result = ResProj::delete_resproj(resproj);
        assert_eq!(result, expected);
    }
}


