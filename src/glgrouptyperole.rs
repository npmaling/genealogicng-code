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
pub struct GlGroupTypeRole {
    pub glgrouptyperoleid: i64,
    pub glgrouptypeid: i64,
    pub glgrouptypename: String,
    pub sequencenumber: i64,
}

impl GlGroupTypeRole {
    pub fn create_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid,
            glgrouptypename,
            sequencenumber,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
        "INSERT INTO glgrouptyperole (glgrouptyperoleid, glgrouptypeid, glgrouptypename, sequencenumber) VALUES ({}, {}, \"{}\", {})",
        glgrouptyperoleid.to_string(),
        glgrouptypeid.to_string(),
        glgrouptypename,
        sequencenumber.to_string(),
    );
        // println!("This is create_glgrouptyperole: {}", parameters);
        parameters
    }

    pub fn read_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid: _,
            glgrouptypename: _,
            sequencenumber: _,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glgrouptyperole WHERE glgrouptyperoleid={}",
            glgrouptyperoleid.to_string(),
        );
        // println!("This is read_glgrouptyperole: {}", parameters);
        parameters
    }

    pub fn update_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid,
            glgrouptypename,
            sequencenumber,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
        "UPDATE glgrouptyperole SET glgrouptyperoleid={}, glgrouptypeid={}, glgrouptypename=\"{}\", sequencenumber={} WHERE glgrouptyperoleid={}",
        glgrouptyperoleid.to_string(),
        glgrouptypeid.to_string(),
        glgrouptypename,
        sequencenumber.to_string(),
        glgrouptyperoleid.to_string(),
    );
        // println!("This is update_glgrouptyperole: {}", parameters);
        parameters
    }

    pub fn delete_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid: _,
            glgrouptypename: _,
            sequencenumber: _,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glgrouptyperole WHERE glgrouptyperoleid={}",
            glgrouptyperoleid.to_string(),
        );
        // println!("This is delete_glgrouptyperole: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_glgrouptyperole() {
        let glgrouptyperole = GlGroupTypeRole {
            glgrouptyperoleid: 1,
            glgrouptypeid: 2,
            glgrouptypename: String::from("Test Group Type"),
            sequencenumber: 3,
        };

        let expected = "INSERT INTO glgrouptyperole (glgrouptyperoleid, glgrouptypeid, glgrouptypename, sequencenumber) VALUES (1, 2, \"Test Group Type\", 3)";
        let result = GlGroupTypeRole::create_glgrouptyperole(glgrouptyperole);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_glgrouptyperole() {
        let glgrouptyperole = GlGroupTypeRole {
            glgrouptyperoleid: 1,
            glgrouptypeid: 2,
            glgrouptypename: String::from("Test Group Type"),
            sequencenumber: 3,
        };

        let expected = "SELECT * FROM glgrouptyperole WHERE glgrouptyperoleid=1";
        let result = GlGroupTypeRole::read_glgrouptyperole(glgrouptyperole);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_glgrouptyperole() {
        let glgrouptyperole = GlGroupTypeRole {
            glgrouptyperoleid: 1,
            glgrouptypeid: 2,
            glgrouptypename: String::from("Test Group Type"),
            sequencenumber: 3,
        };

        let expected = "UPDATE glgrouptyperole SET glgrouptyperoleid=1, glgrouptypeid=2, glgrouptypename=\"Test Group Type\", sequencenumber=3 WHERE glgrouptyperoleid=1";
        let result = GlGroupTypeRole::update_glgrouptyperole(glgrouptyperole);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_glgrouptyperole() {
        let glgrouptyperole = GlGroupTypeRole {
            glgrouptyperoleid: 1,
            glgrouptypeid: 2,
            glgrouptypename: String::from("Test Group Type"),
            sequencenumber: 3,
        };

        let expected = "DELETE FROM glgrouptyperole WHERE glgrouptyperoleid=1";
        let result = GlGroupTypeRole::delete_glgrouptyperole(glgrouptyperole);

        assert_eq!(result, expected);
    }
}

