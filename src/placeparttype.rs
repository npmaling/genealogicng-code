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
pub struct PlacePartType {
    pub placeparttypeid: i64,
    pub pptname: String,
}

impl PlacePartType {
    pub fn create_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO PlacePartType (placeparttypeid, pptname) VALUES ({}, \"{}\")",
            &placeparttypeid.to_string(),
            &pptname
        );
        // println!("This is create_placePartType: {}", parameters);
        parameters
    }

    pub fn read_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname: _,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM PlacePartType WHERE placeparttypeid={}",
            &placeparttypeid.to_string(),
        );
        // println!("This is read_placePartType: {}", parameters);
        parameters
    }

    pub fn update_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "UPDATE placePartType SET placeparttypeid={}, pptname=\"{}\" WHERE placeparttypeid={}",
            &placeparttypeid.to_string(),
            &pptname,
            &placeparttypeid.to_string()
        );
        // println!("This is update_placePartType: {}", parameters);
        parameters
    }

    pub fn delete_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname: _,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM PlacePartType WHERE placeparttypeid={}",
            &placeparttypeid.to_string(),
        );
        // println!("This is delete_placePartType: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_placeparttype() {
        let placeparttype = PlacePartType {
            placeparttypeid: 1,
            pptname: String::from("Test"),
        };
        let expected = "INSERT INTO PlacePartType (placeparttypeid, pptname) VALUES (1, \"Test\")";
        let result = PlacePartType::create_placeparttype(placeparttype);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_placeparttype() {
        let placeparttype = PlacePartType {
            placeparttypeid: 1,
            pptname: String::from("Test"),
        };
        let expected = "SELECT * FROM PlacePartType WHERE placeparttypeid=1";
        let result = PlacePartType::read_placeparttype(placeparttype);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_placeparttype() {
        let placeparttype = PlacePartType {
            placeparttypeid: 1,
            pptname: String::from("Test"),
        };
        let expected = "UPDATE placePartType SET placeparttypeid=1, pptname=\"Test\" WHERE placeparttypeid=1";
        let result = PlacePartType::update_placeparttype(placeparttype);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_placeparttype() {
        let placeparttype = PlacePartType {
            placeparttypeid: 1,
            pptname: String::from("Test"),
        };
        let expected = "DELETE FROM PlacePartType WHERE placeparttypeid=1";
        let result = PlacePartType::delete_placeparttype(placeparttype);
        assert_eq!(result, expected);
    }
}
