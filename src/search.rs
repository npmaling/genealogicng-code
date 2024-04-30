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
pub struct Search {
    pub searchid: i64,
    pub activityid: i64,
    pub sourceid: i64,
    pub repositoryid: i64,
    pub searchedfor: String,
}

impl Search {
    pub fn create_search(
        Search {
            searchid,
            activityid,
            sourceid,
            repositoryid,
            searchedfor,
        }: Search,
    ) -> String {
        let parameters = format!(
            "INSERT INTO search (searchid, activityid, sourceid, repositoryid, searchedfor) VALUES ({}, {}, {}, {}, \"{}\")",
            searchid.to_string(),
            activityid.to_string(),
            sourceid.to_string(),
            repositoryid.to_string(),
            searchedfor,
        );
        // println!("This is create_search: {}", parameters);
        parameters
    }

    pub fn read_search(
        Search {
            searchid,
            activityid,
            sourceid,
            repositoryid,
            searchedfor,
        }: Search,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM search WHERE searchid={}",
            searchid.to_string(),
        );
        // println!("This is read_search: {}", parameters);
        parameters
    }

    pub fn update_search(
        Search {
            searchid,
            activityid,
            sourceid,
            repositoryid,
            searchedfor,
        }: Search,
    ) -> String {
        let parameters = format!(
            "UPDATE search SET searchid={}, activityid={}, sourceid={}, repositoryid={}, searchedfor=\"{}\" WHERE searchid={}",
            searchid.to_string(),
            activityid.to_string(),
            sourceid.to_string(),
            repositoryid.to_string(),
            searchedfor,
            searchid.to_string(),
        );
        // println!("This is update_search: {}", parameters);
        parameters
    }

    pub fn delete_search(
        Search {
            searchid,
            activityid,
            sourceid,
            repositoryid,
            searchedfor,
        }: Search,
    ) -> String {
        let parameters = format!("DELETE FROM search WHERE searchid={}", searchid.to_string(),);
        // println!("This is delete_search: {}", parameters);
        parameters
    }
}
