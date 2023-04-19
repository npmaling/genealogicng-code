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
pub struct Place {
    pub placeid: i64,
    pub startdate: String,
    pub enddate: String,
    pub ascdescnone: String,
    pub placecomment: String,
}

impl Place {
    pub fn create_place(
        Place {
            placeid,
            startdate,
            enddate,
            ascdescnone,
            placecomment,
        }: Place,
    ) -> String {
        let parameters = format!(
            "INSERT INTO place (placeid, startdate, enddate, ascdescnone, placecomment) VALUES ({}, \"{}\", \"{}\", \"{}\", \"{}\")",
            &placeid.to_string(),
            &startdate,
            &enddate,
            &ascdescnone,
            &placecomment
        );
        // println!("This is create_place: {}", parameters);
        parameters
    }

    pub fn read_place(
        Place {
            placeid,
            startdate,
            enddate,
            ascdescnone,
            placecomment,
        }: Place,
    ) -> String {
        let parameters = format!("SELECT * FROM place WHERE placeid={}", &placeid.to_string());
        // println!("This is read_place: {}", parameters);
        parameters
    }

    pub fn update_place(
        Place {
            placeid,
            startdate,
            enddate,
            ascdescnone,
            placecomment,
        }: Place,
    ) -> String {
        let parameters = format!(
            "UPDATE place SET placeid={}, startdate=\"{}\", enddate=\"{}\", ascdescnone=\"{}\", placecomment=\"{}\" WHERE placeid={}",
            &placeid.to_string(),
            &startdate,
            &enddate,
            &ascdescnone,
            &placecomment,
            &placeid.to_string()
        );
        // println!("This is update_place: {}", parameters);
        parameters
    }

    pub fn delete_place(
        Place {
            placeid,
            startdate,
            enddate,
            ascdescnone,
            placecomment,
        }: Place,
    ) -> String {
        let parameters = format!("DELETE FROM place WHERE placeid={}", &placeid.to_string());
        // println!("This is delete_place: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {

    use super::Place;

    #[test]
    fn test_create_place() {
        let place = Place {
            placeid: 1,
            startdate: String::from("2021-01-01"),
            enddate: String::from("2021-01-31"),
            ascdescnone: String::from("none"),
            placecomment: String::from("Test place"),
        };
        let expected_query = "INSERT INTO place (placeid, startdate, enddate, ascdescnone, placecomment) VALUES (1, \"2021-01-01\", \"2021-01-31\", \"none\", \"Test place\")";
        assert_eq!(Place::create_place(place), expected_query);
    }

    #[test]
    fn test_read_place() {
        let place = Place {
            placeid: 1,
            startdate: String::from("2021-01-01"),
            enddate: String::from("2021-01-31"),
            ascdescnone: String::from("none"),
            placecomment: String::from("Test place"),
        };
        let expected_query = "SELECT * FROM place WHERE placeid=1";
        assert_eq!(Place::read_place(place), expected_query);
    }

    #[test]
    fn test_update_place() {
        let place = Place {
            placeid: 1,
            startdate: String::from("2021-01-01"),
            enddate: String::from("2021-01-31"),
            ascdescnone: String::from("none"),
            placecomment: String::from("Test place"),
        };
        let expected_query = "UPDATE place SET placeid=1, startdate=\"2021-01-01\", enddate=\"2021-01-31\", ascdescnone=\"none\", placecomment=\"Test place\" WHERE placeid=1";
        assert_eq!(Place::update_place(place), expected_query);
    }

    #[test]
    fn test_delete_place() {
        let place = Place {
            placeid: 1,
            startdate: String::from("2021-01-01"),
            enddate: String::from("2021-01-31"),
            ascdescnone: String::from("none"),
            placecomment: String::from("Test place"),
        };
        let expected_query = "DELETE FROM place WHERE placeid=1";
        assert_eq!(Place::delete_place(place), expected_query);
    }
}
