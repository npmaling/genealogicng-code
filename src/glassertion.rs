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
pub struct GlAssertion {
    pub glassertionid: i64,
    pub suretypartid: i64,
    pub researcherid: i64,
    pub sourceid: i64,
    pub subject1id: i64,
    pub subject1type: String,
    pub subject2id: i64,
    pub subject2type: String,
    pub value_role: i64,
    pub disproved: String,
    pub rationale: String,
}

impl GlAssertion {
    pub fn create_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid,
            researcherid,
            sourceid,
            subject1id,
            subject1type,
            subject2id,
            subject2type,
            value_role,
            disproved,
            rationale,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "INSERT INTO glassertion (glassertionid, suretypartid, researcherid, sourceid, subject1id, subject1type, subject2id, subject2type, value_role, disproved, rationale) VALUES ({}, {}, {}, {}, {}, \"{}\", {}, \"{}\", {}, \"{}\", \"{}\")",
            glassertionid.to_string(),
            suretypartid.to_string(),
            researcherid.to_string(),
            sourceid.to_string(),
            subject1id.to_string(),
            subject1type,
            subject2id.to_string(),
            subject2type,
            value_role.to_string(),
            disproved,
            rationale,
        );
        // println!("This is create_glassertion: {}", parameters);
        parameters
    }

    pub fn read_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid: _,
            researcherid: _,
            sourceid: _,
            subject1id: _,
            subject1type: _,
            subject2id: _,
            subject2type: _,
            value_role: _,
            disproved: _,
            rationale: _,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glassertion WHERE glassertionid={}",
            glassertionid.to_string(),
        );
        // println!("This is read_glassertion: {}", parameters);
        parameters
    }

    pub fn update_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid,
            researcherid,
            sourceid,
            subject1id,
            subject1type,
            subject2id,
            subject2type,
            value_role,
            disproved,
            rationale,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "UPDATE glassertion SET glassertionid={}, suretypartid={}, researcherid={}, sourceid={}, subject1id={}, subject1type=\"{}\", subject2id={}, subject2type=\"{}\", value_role={}, disproved=\"{}\", rationale=\"{}\" WHERE glassertionid={}",
            glassertionid.to_string(),
            suretypartid.to_string(),
            researcherid.to_string(),
            sourceid.to_string(),
            subject1id.to_string(),
            subject1type,
            subject2id.to_string(),
            subject2type,
            value_role.to_string(),
            disproved,
            rationale,
            glassertionid.to_string(),
        );
        // println!("This is update_glassertion: {}", parameters);
        parameters
    }

    pub fn delete_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid: _,
            researcherid: _,
            sourceid: _,
            subject1id: _,
            subject1type: _,
            subject2id: _,
            subject2type: _,
            value_role: _,
            disproved: _,
            rationale: _,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glassertion WHERE glassertionid={}",
            glassertionid.to_string(),
        );
        // println!("This is delete_glassertion: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_glassertion() {
        let assertion = GlAssertion {
            glassertionid: 1,
            suretypartid: 2,
            researcherid: 3,
            sourceid: 4,
            subject1id: 5,
            subject1type: String::from("type1"),
            subject2id: 6,
            subject2type: String::from("type2"),
            value_role: 7,
            disproved: String::from("no"),
            rationale: String::from("Test rationale"),
        };
        let expected_query = "INSERT INTO glassertion (glassertionid, suretypartid, researcherid, sourceid, subject1id, subject1type, subject2id, subject2type, value_role, disproved, rationale) VALUES (1, 2, 3, 4, 5, \"type1\", 6, \"type2\", 7, \"no\", \"Test rationale\")";
        assert_eq!(GlAssertion::create_glassertion(assertion), expected_query);
    }

    #[test]
    fn test_read_glassertion() {
        let gl_assertion = GlAssertion {
            glassertionid: 1,
            suretypartid: 2,
            researcherid: 3,
            sourceid: 4,
            subject1id: 5,
            subject1type: String::from("type1"),
            subject2id: 6,
            subject2type: String::from("type2"),
            value_role: 7,
            disproved: String::from("no"),
            rationale: String::from("rationale"),
        };
        let expected_query = "SELECT * FROM glassertion WHERE glassertionid=1";
        assert_eq!(GlAssertion::read_glassertion(gl_assertion), expected_query);
    }

    #[test]
    fn test_update_glassertion() {
        let gl_assertion = GlAssertion {
            glassertionid: 1,
            suretypartid: 2,
            researcherid: 3,
            sourceid: 4,
            subject1id: 5,
            subject1type: String::from("type1"),
            subject2id: 6,
            subject2type: String::from("type2"),
            value_role: 7,
            disproved: String::from("false"),
            rationale: String::from("Test rationale"),
        };
        let expected_query = "UPDATE glassertion SET glassertionid=1, suretypartid=2, researcherid=3, sourceid=4, subject1id=5, subject1type=\"type1\", subject2id=6, subject2type=\"type2\", value_role=7, disproved=\"false\", rationale=\"Test rationale\" WHERE glassertionid=1";
        assert_eq!(GlAssertion::update_glassertion(gl_assertion), expected_query);
    }

    #[test]
    fn test_delete_glassertion() {
        let assertion = GlAssertion {
            glassertionid: 1,
            suretypartid: 2,
            researcherid: 3,
            sourceid: 4,
            subject1id: 5,
            subject1type: String::from("test"),
            subject2id: 6,
            subject2type: String::from("test"),
            value_role: 7,
            disproved: String::from("no"),
            rationale: String::from("test"),
        };
        let expected_query = "DELETE FROM glassertion WHERE glassertionid=1";
        assert_eq!(GlAssertion::delete_glassertion(assertion), expected_query);
    }


}
