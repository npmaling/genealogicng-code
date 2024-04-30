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
pub struct CharPart {
    pub characteristicpartid: i64,
    pub characteristicid: i64,
    pub charparttypeid: i64,
    pub charpartname: String,
    pub charpartseq: i64,
}

impl CharPart {
    pub fn create_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "INSERT INTO charpart (characteristicpartid, characteristicid, charparttypeid, charpartname, charpartseq) VALUES ({}, {}, {}, \"{}\", {})",
            &characteristicpartid.to_string(),
            &characteristicid.to_string(),
            &charparttypeid.to_string(),
            &charpartname,
            &charpartseq.to_string(),
        );
        // println!("This is create_charpart: {}", parameters);
        parameters
    }

    pub fn read_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM charpart WHERE characteristicpartid={}",
            &characteristicpartid.to_string(),
        );
        // println!("This is read_charpart: {}", parameters);
        parameters
    }

    pub fn update_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "UPDATE charpart SET characteristicpartid={}, characteristicid={}, charparttypeid={}, charpartname=\"{}\", charpartseq={} WHERE characteristicpartid={}",
            &characteristicpartid.to_string(),
            &characteristicid.to_string(),
            &charparttypeid.to_string(),
            &charpartname,
            &charpartseq.to_string(),
            &characteristicpartid.to_string(),
        );
        // println!("This is update_charpart: {}", parameters);
        parameters
    }

    pub fn delete_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "DELETE FROM charpart WHERE characteristicpartid={}",
            &characteristicpartid.to_string(),
        );
        // println!("This is delete_charpart: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_charpart() {
        let charpart = CharPart {
            characteristicpartid: 1,
            characteristicid: 2,
            charparttypeid: 3,
            charpartname: String::from("Test charpart"),
            charpartseq: 4,
        };
        let expected_query = "INSERT INTO charpart (characteristicpartid, characteristicid, charparttypeid, charpartname, charpartseq) VALUES (1, 2, 3, \"Test charpart\", 4)";
        assert_eq!(CharPart::create_charpart(charpart), expected_query);
    }

    #[test]
    fn test_read_charpart() {
        let charpart = CharPart {
            characteristicpartid: 1,
            characteristicid: 1,
            charparttypeid: 1,
            charpartname: String::from("Test charpart"),
            charpartseq: 1,
        };
        let expected_query = "SELECT * FROM charpart WHERE characteristicpartid=1";
        assert_eq!(CharPart::read_charpart(charpart), expected_query);
    }

    #[test]
    fn test_update_charpart() {
        let charpart = CharPart {
            characteristicpartid: 1,
            characteristicid: 2,
            charparttypeid: 3,
            charpartname: String::from("Test charpart"),
            charpartseq: 4,
        };
        let expected_query = "UPDATE charpart SET characteristicpartid=1, characteristicid=2, charparttypeid=3, charpartname=\"Test charpart\", charpartseq=4 WHERE characteristicpartid=1";
        assert_eq!(CharPart::update_charpart(charpart), expected_query);
    }

    #[test]
    fn test_delete_charpart() {
        let charpart = CharPart {
            characteristicpartid: 1,
            characteristicid: 2,
            charparttypeid: 3,
            charpartname: String::from("Test charpart"),
            charpartseq: 4,
        };
        let expected_query = "DELETE FROM charpart WHERE characteristicpartid=1";
        assert_eq!(CharPart::delete_charpart(charpart), expected_query);
    }

}

