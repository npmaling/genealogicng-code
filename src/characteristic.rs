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
pub struct Characteristic {
    pub characteristicid: i64,
    pub placeid: i64,
    pub characteristicdate: String,
    pub ascdescnone: String,
}

impl Characteristic {
    pub fn create_characteristic(
        Characteristic {
            characteristicid,
            placeid,
            characteristicdate,
            ascdescnone,
        }: Characteristic,
    ) -> String {
        let parameters = format!(
            "INSERT INTO characteristic (characteristicid, placeid, characteristicdate, ascdescnone) VALUES ({}, {}, \"{}\", \"{}\")",
            &characteristicid.to_string(),
            &placeid.to_string(),
            &characteristicdate,
            &ascdescnone,
        );
        // println!("This is create_characteristic: {}", parameters);
        parameters
    }

    pub fn read_characteristic(
        Characteristic {
            characteristicid,
            placeid: _,
            characteristicdate: _,
            ascdescnone: _,
        }: Characteristic,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM characteristic WHERE characteristicid={}",
            &characteristicid.to_string(),
        );
        // println!("This is read_characteristic: {}", parameters);
        parameters
    }

    pub fn update_characteristic(
        Characteristic {
            characteristicid,
            placeid,
            characteristicdate,
            ascdescnone,
        }: Characteristic,
    ) -> String {
        let parameters = format!(
            "UPDATE characteristic SET characteristicid={}, placeid={}, characteristicdate={}, ascdescnone=\"{}\" WHERE characteristicid={}",
            &characteristicid.to_string(),
            &placeid.to_string(),
            &characteristicdate,
            &ascdescnone,
            &characteristicid.to_string()
        );
        // println!("This is update_characteristic: {}", parameters);
        parameters
    }

    pub fn delete_characteristic(
        Characteristic {
            characteristicid,
            placeid: _,
            characteristicdate: _,
            ascdescnone: _,
        }: Characteristic,
    ) -> String {
        let parameters = format!(
            "DELETE FROM characteristic WHERE characteristicid={}",
            &characteristicid.to_string(),
        );
        // println!("This is delete_characteristic: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_characteristic() {
        let characteristic = Characteristic {
            characteristicid: 1,
            placeid: 2,
            characteristicdate: String::from("2021-01-01"),
            ascdescnone: String::from("asc"),
        };
        let expected_query = "INSERT INTO characteristic (characteristicid, placeid, characteristicdate, ascdescnone) VALUES (1, 2, \"2021-01-01\", \"asc\")";
        assert_eq!(Characteristic::create_characteristic(characteristic), expected_query);
    }

    #[test]
    fn test_read_characteristic() {
        let characteristic = Characteristic {
            characteristicid: 1,
            placeid: 1,
            characteristicdate: String::from("2021-01-01"),
            ascdescnone: String::from("none"),
        };
        let expected_query = "SELECT * FROM characteristic WHERE characteristicid=1";
        assert_eq!(Characteristic::read_characteristic(characteristic), expected_query);
    }

    #[test]
    fn test_update_characteristic() {
        let characteristic = Characteristic {
            characteristicid: 1,
            placeid: 1,
            characteristicdate: String::from("2021-01-01"),
            ascdescnone: String::from("none"),
        };
        let expected_query = "UPDATE characteristic SET characteristicid=1, placeid=1, characteristicdate=2021-01-01, ascdescnone=\"none\" WHERE characteristicid=1";
        assert_eq!(Characteristic::update_characteristic(characteristic), expected_query);
    }

    #[test]
    fn test_delete_characteristic() {
        let characteristic = Characteristic {
            characteristicid: 1,
            placeid: 1,
            characteristicdate: String::from("2021-01-01"),
            ascdescnone: String::from("none"),
        };
        let expected_query = "DELETE FROM characteristic WHERE characteristicid=1";
        assert_eq!(Characteristic::delete_characteristic(characteristic), expected_query);
    }

}

