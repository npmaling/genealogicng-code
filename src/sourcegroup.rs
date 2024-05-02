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
pub struct SourceGroup {
    pub sourcegroupid: i64,
    pub sourcegroupname: String,
}

impl SourceGroup {
    pub fn create_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "INSERT INTO sourcegroup (sourcegroupid, sourcegroupname) VALUES ({}, \"{}\")",
            &sourcegroupid.to_string(),
            &sourcegroupname,
        );
        // println!("This is create_sourcegroup: {}", parameters);
        parameters
    }

    pub fn read_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname: _,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM sourcegroup WHERE sourcegroupid={}",
            &sourcegroupid.to_string(),
        );
        // println!("This is read_sourcegroup: {}", parameters);
        parameters
    }

    pub fn update_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "UPDATE sourcegroup SET sourcegroupid={}, sourcegroupname=\"{}\" WHERE sourcegroupid={}",
            &sourcegroupid.to_string(),
            &sourcegroupname,
            &sourcegroupid.to_string(),
        );
        // println!("This is update_sourcegroup: {}", parameters);
        parameters
    }

    pub fn delete_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname: _,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "DELETE FROM sourcegroup WHERE sourcegroupid={}",
            &sourcegroupid.to_string(),
        );
        // println!("This is delete_sourcegroup: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sourcegroup() {
        let sourcegroup = SourceGroup {
            sourcegroupid: 1,
            sourcegroupname: String::from("Group 1"),
        };
        let expected = "INSERT INTO sourcegroup (sourcegroupid, sourcegroupname) VALUES (1, \"Group 1\")";
        let result = SourceGroup::create_sourcegroup(sourcegroup);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_sourcegroup() {
        let sourcegroup = SourceGroup {
            sourcegroupid: 1,
            sourcegroupname: String::from("Group 1"),
        };
        let expected = "SELECT * FROM sourcegroup WHERE sourcegroupid=1";
        let result = SourceGroup::read_sourcegroup(sourcegroup);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_sourcegroup() {
        let sourcegroup = SourceGroup {
            sourcegroupid: 1,
            sourcegroupname: String::from("Group 1"),
        };
        let expected = "UPDATE sourcegroup SET sourcegroupid=1, sourcegroupname=\"Group 1\" WHERE sourcegroupid=1";
        let result = SourceGroup::update_sourcegroup(sourcegroup);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_sourcegroup() {
        let sourcegroup = SourceGroup {
            sourcegroupid: 1,
            sourcegroupname: String::from("Group 1"),
        };
        let expected = "DELETE FROM sourcegroup WHERE sourcegroupid=1";
        let result = SourceGroup::delete_sourcegroup(sourcegroup);
        assert_eq!(result, expected);
    }
}




