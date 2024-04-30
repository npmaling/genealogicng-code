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
pub struct CharPartType {
    pub charparttypeid: i64,
    pub charparttypename: String,
    pub gedcomtag: String,
}

impl CharPartType {
    pub fn create_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO charparttype (charparttypeid, charparttypename, gedcomtag) VALUES ({}, \"{}\", \"{}\")",
            &charparttypeid.to_string(),
            &charparttypename,
            &gedcomtag,
        );
        // println!("This is create_charparttype: {}", parameters);
        parameters
    }

    pub fn read_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM charparttype WHERE charparttypeid={}",
            &charparttypeid.to_string(),
        );
        // println!("This is read_charparttype: {}", parameters);
        parameters
    }

    pub fn update_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "UPDATE charparttype SET charparttypeid={}, charparttypename=\"{}\", gedcomtag=\"{}\" WHERE charparttypeid={}",
            &charparttypeid.to_string(),
            &charparttypename,
            &gedcomtag,
            &charparttypeid.to_string(),
        );
        // println!("This is update_charparttype: {}", parameters);
        parameters
    }

    pub fn delete_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM charparttype WHERE charparttypeid={}",
            &charparttypeid.to_string(),
        );
        // println!("This is delete_charparttype: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_charparttype() {
        let charparttype = CharPartType {
            charparttypeid: 1,
            charparttypename: String::from("Test name"),
            gedcomtag: String::from("Test tag"),
        };
        let expected_query = "INSERT INTO charparttype (charparttypeid, charparttypename, gedcomtag) VALUES (1, \"Test name\", \"Test tag\")";
        assert_eq!(CharPartType::create_charparttype(charparttype), expected_query);
    }

    #[test]
    fn test_read_charparttype() {
        let charparttype = CharPartType {
            charparttypeid: 1,
            charparttypename: String::from("Test charparttype"),
            gedcomtag: String::from("TAG"),
        };
        let expected_query = "SELECT * FROM charparttype WHERE charparttypeid=1";
        assert_eq!(CharPartType::read_charparttype(charparttype), expected_query);
    }

    #[test]
    fn test_update_charparttype() {
        let charparttype = CharPartType {
            charparttypeid: 1,
            charparttypename: String::from("Test charparttypename"),
            gedcomtag: String::from("Test gedcomtag"),
        };
        let expected_query = "UPDATE charparttype SET charparttypeid=1, charparttypename=\"Test charparttypename\", gedcomtag=\"Test gedcomtag\" WHERE charparttypeid=1";
        assert_eq!(CharPartType::update_charparttype(charparttype), expected_query);
    }

    #[test]
    fn test_delete_charparttype() {
        let charparttype = CharPartType {
            charparttypeid: 1,
            charparttypename: String::from("Test"),
            gedcomtag: String::from("TST"),
        };
        let expected_query = "DELETE FROM charparttype WHERE charparttypeid=1";
        assert_eq!(CharPartType::delete_charparttype(charparttype), expected_query);
    }

}
