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
pub struct GlGroupType {
    pub glgrouptypeid: i64,
    pub glgroupname: String,
    pub ascdescnone: String,
}

impl GlGroupType {
    pub fn create_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname,
            ascdescnone,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
        "INSERT INTO glgrouptype (glgrouptypeid, glgroupname, ascdescnone) VALUES ({}, \"{}\", \"{}\")",
        glgrouptypeid.to_string(),
        glgroupname,
        ascdescnone,
    );
        // println!("This is create_glgrouptype: {}", parameters);
        parameters
    }

    pub fn read_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname: _,
            ascdescnone: _,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glgrouptype WHERE glgrouptypeid={}",
            glgrouptypeid.to_string(),
        );
        // println!("This is read_glgrouptype: {}", parameters);
        parameters
    }

    pub fn update_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname,
            ascdescnone,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
        "UPDATE glgrouptype SET glgrouptypeid={}, glgroupname=\"{}\", ascdescnone=\"{}\" WHERE glgrouptypeid={}",
        glgrouptypeid.to_string(),
        glgroupname,
        ascdescnone,
        glgrouptypeid.to_string(),
    );
        // println!("This is update_glgrouptype: {}", parameters);
        parameters
    }

    pub fn delete_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname: _,
            ascdescnone: _,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glgrouptype WHERE glgrouptypeid={}",
            glgrouptypeid.to_string(),
        );
        // println!("This is delete_glgrouptype: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_glgrouptype() {
        let glgrouptype = GlGroupType {
            glgrouptypeid: 1,
            glgroupname: String::from("Group 1"),
            ascdescnone: String::from("None"),
        };
        let expected = "INSERT INTO glgrouptype (glgrouptypeid, glgroupname, ascdescnone) VALUES (1, \"Group 1\", \"None\")";
        let result = GlGroupType::create_glgrouptype(glgrouptype);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_glgrouptype() {
        let glgrouptype = GlGroupType {
            glgrouptypeid: 1,
            glgroupname: String::from("Group 1"),
            ascdescnone: String::from("None"),
        };
        let expected = "SELECT * FROM glgrouptype WHERE glgrouptypeid=1";
        let result = GlGroupType::read_glgrouptype(glgrouptype);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_glgrouptype() {
        let glgrouptype = GlGroupType {
            glgrouptypeid: 1,
            glgroupname: String::from("Group 1"),
            ascdescnone: String::from("None"),
        };
        let expected = "UPDATE glgrouptype SET glgrouptypeid=1, glgroupname=\"Group 1\", ascdescnone=\"None\" WHERE glgrouptypeid=1";
        let result = GlGroupType::update_glgrouptype(glgrouptype);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_glgrouptype() {
        let glgrouptype = GlGroupType {
            glgrouptypeid: 1,
            glgroupname: String::from("Group 1"),
            ascdescnone: String::from("None"),
        };
        let expected = "DELETE FROM glgrouptype WHERE glgrouptypeid=1";
        let result = GlGroupType::delete_glgrouptype(glgrouptype);
        assert_eq!(result, expected);
    }
}
