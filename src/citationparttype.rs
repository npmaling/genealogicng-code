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
pub struct CitationPartType {
    pub citationparttypeid: i64,
    pub citationparttypename: String,
}

impl CitationPartType {
    pub fn create_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO citationparttype (citationparttypeid, citationparttypename) VALUES ({}, \"{}\")",
            citationparttypeid.to_string(),
            citationparttypename,
        );
        // println!("This is create_citationparttype: {}", parameters);
        parameters
    }

    pub fn read_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename: _,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM citationparttype WHERE citationparttypeid={}",
            citationparttypeid.to_string(),
        );
        // println!("This is read_citationparttype: {}", parameters);
        parameters
    }

    pub fn update_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "UPDATE citationparttype SET citationparttypeid={}, citationparttypename=\"{}\" WHERE citationparttypeid={}",
            citationparttypeid.to_string(),
            citationparttypename,
            citationparttypeid.to_string(),
        );
        // println!("This is update_citationparttype: {}", parameters);
        parameters
    }

    pub fn delete_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename: _,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM citationparttype WHERE citationparttypeid={}",
            citationparttypeid.to_string(),
        );
        // println!("This is delete_citationparttype: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_citationparttype() {
        let citation_part_type = CitationPartType {
            citationparttypeid: 1,
            citationparttypename: String::from("Test citation part type"),
        };
        let expected_query = "INSERT INTO citationparttype (citationparttypeid, citationparttypename) VALUES (1, \"Test citation part type\")";
        assert_eq!(CitationPartType::create_citationparttype(citation_part_type), expected_query);
    }

    #[test]
    fn test_read_citationparttype() {
        let citation_part_type = CitationPartType {
            citationparttypeid: 1,
            citationparttypename: String::from("Test type"),
        };
        let expected_query = "SELECT * FROM citationparttype WHERE citationparttypeid=1";
        assert_eq!(CitationPartType::read_citationparttype(citation_part_type), expected_query);
    }

    #[test]
    fn test_update_citationparttype() {
        let input = CitationPartType {
            citationparttypeid: 1,
            citationparttypename: String::from("Test type"),
        };
        let expected_query = "UPDATE citationparttype SET citationparttypeid=1, citationparttypename=\"Test type\" WHERE citationparttypeid=1";
        assert_eq!(CitationPartType::update_citationparttype(input), expected_query);
    }

    #[test]
    fn test_delete_citationparttype() {
        let citation_part_type = CitationPartType {
            citationparttypeid: 1,
            citationparttypename: String::from("Test type"),
        };
        let expected_query = "DELETE FROM citationparttype WHERE citationparttypeid=1";
        assert_eq!(CitationPartType::delete_citationparttype(citation_part_type), expected_query);
    }


}
