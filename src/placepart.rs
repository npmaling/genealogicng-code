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
pub struct PlacePart {
    pub placepartid: i64,
    pub placeid: i64,
    pub placeparttypeid: i64,
    pub name: String,
    pub sequencenumber: i64,
}

impl PlacePart {
    pub fn create_placepart(
        PlacePart {
            placepartid,
            placeid,
            placeparttypeid,
            name,
            sequencenumber,
        }: PlacePart,
    ) -> String {
        let parameters = format!(
            "INSERT INTO PlacePart (placepartid, placeid, placeparttypeid, name, sequencenumber) VALUES ({}, {}, {}, \"{}\", {})",
            &placepartid.to_string(),
            &placeid.to_string(),
            &placeparttypeid.to_string(),
            &name,
            &sequencenumber.to_string(),
        );
        // println!("This is create_placepart: {}", parameters);
        parameters
    }

    pub fn read_placepart(
        PlacePart {
            placepartid,
            placeid: _,
            placeparttypeid: _,
            name: _,
            sequencenumber: _,
        }: PlacePart,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM PlacePart WHERE placepartid={}",
            &placepartid.to_string(),
        );
        // println!("This is read_placepart: {}", parameters);
        parameters
    }

    pub fn update_placepart(
        PlacePart {
            placepartid,
            placeid,
            placeparttypeid,
            name,
            sequencenumber,
        }: PlacePart,
    ) -> String {
        let parameters = format!(
            "UPDATE placePart SET placepartid={}, placeid={}, placeparttypeid={}, name=\"{}\", sequencenumber={} WHERE placepartid={}",
            &placepartid.to_string(),
            &placeid.to_string(),
            &placeparttypeid.to_string(),
            &name,
            &sequencenumber.to_string(),
            &placepartid.to_string()
        );
        // println!("This is update_placepart: {}", parameters);
        parameters
    }

    pub fn delete_placepart(
        PlacePart {
            placepartid,
            placeid: _,
            placeparttypeid: _,
            name: _,
            sequencenumber: _,
        }: PlacePart,
    ) -> String {
        let parameters = format!(
            "DELETE FROM PlacePart WHERE placepartid={}",
            &placepartid.to_string(),
        );
        // println!("This is delete_placepart: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_placepart() {
        let placepart = PlacePart {
            placepartid: 1,
            placeid: 2,
            placeparttypeid: 3,
            name: String::from("Test Place Part"),
            sequencenumber: 4,
        };

        let expected = "INSERT INTO PlacePart (placepartid, placeid, placeparttypeid, name, sequencenumber) VALUES (1, 2, 3, \"Test Place Part\", 4)";
        let result = PlacePart::create_placepart(placepart);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_placepart() {
        let placepart = PlacePart {
            placepartid: 1,
            placeid: 2,
            placeparttypeid: 3,
            name: String::from("Test Place Part"),
            sequencenumber: 4,
        };

        let expected = "SELECT * FROM PlacePart WHERE placepartid=1";
        let result = PlacePart::read_placepart(placepart);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_placepart() {
        let placepart = PlacePart {
            placepartid: 1,
            placeid: 2,
            placeparttypeid: 3,
            name: String::from("Test Place Part"),
            sequencenumber: 4,
        };

        let expected = "UPDATE placePart SET placepartid=1, placeid=2, placeparttypeid=3, name=\"Test Place Part\", sequencenumber=4 WHERE placepartid=1";
        let result = PlacePart::update_placepart(placepart);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_placepart() {
        let placepart = PlacePart {
            placepartid: 1,
            placeid: 2,
            placeparttypeid: 3,
            name: String::from("Test Place Part"),
            sequencenumber: 4,
        };

        let expected = "DELETE FROM PlacePart WHERE placepartid=1";
        let result = PlacePart::delete_placepart(placepart);

        assert_eq!(result, expected);
    }
}
