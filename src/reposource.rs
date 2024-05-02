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
pub struct RepoSource {
    pub reposourceid: i64,
    pub repositoryid: i64,
    pub sourceid: i64,
    pub rsactivityid: i64,
    pub callnumber: String,
    pub description: String,
}

impl RepoSource {
    pub fn create_reposource(
        RepoSource {
            reposourceid,
            repositoryid,
            sourceid,
            rsactivityid,
            callnumber,
            description,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
        "INSERT INTO reposource (reposourceid, repositoryid, sourceid, rsactivityid, callnumber, description) VALUES ({}, {}, {}, {}, \"{}\", \"{}\")",
        reposourceid.to_string(),
        repositoryid.to_string(),
        sourceid.to_string(),
        rsactivityid.to_string(),
        callnumber,
        description,
    );
        // println!("This is create_reposource: {}", parameters);
        parameters
    }

    pub fn read_reposource(
        RepoSource {
            reposourceid,
            repositoryid: _,
            sourceid: _,
            rsactivityid: _,
            callnumber: _,
            description: _,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM reposource WHERE reposourceid={}",
            reposourceid.to_string(),
        );
        // println!("This is read_reposource: {}", parameters);
        parameters
    }

    pub fn update_reposource(
        RepoSource {
            reposourceid,
            repositoryid,
            sourceid,
            rsactivityid,
            callnumber,
            description,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
        "UPDATE reposource SET reposourceid={}, repositoryid={}, sourceid={}, rsactivityid={}, callnumber=\"{}\", description=\"{}\" WHERE reposourceid={}",
        reposourceid.to_string(),
        repositoryid.to_string(),
        sourceid.to_string(),
        rsactivityid.to_string(),
        callnumber,
        description,
        reposourceid.to_string(),
    );
        // println!("This is update_reposource: {}", parameters);
        parameters
    }

    pub fn delete_reposource(
        RepoSource {
            reposourceid,
            repositoryid: _,
            sourceid: _,
            rsactivityid: _,
            callnumber: _,
            description: _,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
            "DELETE FROM reposource WHERE reposourceid={}",
            reposourceid.to_string(),
        );
        // println!("This is delete_reposource: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_reposource() {
        let reposource = RepoSource {
            reposourceid: 1,
            repositoryid: 2,
            sourceid: 3,
            rsactivityid: 4,
            callnumber: String::from("ABC123"),
            description: String::from("Test description"),
        };

        let expected = "INSERT INTO reposource (reposourceid, repositoryid, sourceid, rsactivityid, callnumber, description) VALUES (1, 2, 3, 4, \"ABC123\", \"Test description\")";
        let result = RepoSource::create_reposource(reposource);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_reposource() {
        let reposource = RepoSource {
            reposourceid: 1,
            repositoryid: 2,
            sourceid: 3,
            rsactivityid: 4,
            callnumber: String::from("ABC123"),
            description: String::from("Test description"),
        };

        let expected = "SELECT * FROM reposource WHERE reposourceid=1";
        let result = RepoSource::read_reposource(reposource);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_reposource() {
        let reposource = RepoSource {
            reposourceid: 1,
            repositoryid: 2,
            sourceid: 3,
            rsactivityid: 4,
            callnumber: String::from("ABC123"),
            description: String::from("Test description"),
        };

        let expected = "UPDATE reposource SET reposourceid=1, repositoryid=2, sourceid=3, rsactivityid=4, callnumber=\"ABC123\", description=\"Test description\" WHERE reposourceid=1";
        let result = RepoSource::update_reposource(reposource);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_reposource() {
        let reposource = RepoSource {
            reposourceid: 1,
            repositoryid: 2,
            sourceid: 3,
            rsactivityid: 4,
            callnumber: String::from("ABC123"),
            description: String::from("Test description"),
        };

        let expected = "DELETE FROM reposource WHERE reposourceid=1";
        let result = RepoSource::delete_reposource(reposource);

        assert_eq!(result, expected);
    }
}
