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
pub struct Source {
    pub sourceid: i64,
    pub highersourceid: i64,
    pub subjectplaceid: i64,
    pub jurisplaceid: i64,
    pub researcherid: i64,
    pub subjectdate: String,
    pub comments: String,
}

impl Source {
    pub fn create_source(
        Source {
            sourceid,
            highersourceid,
            subjectplaceid,
            jurisplaceid,
            researcherid,
            subjectdate,
            comments,
        }: Source,
    ) -> String {
        let parameters = format!(
            "INSERT INTO source (sourceid, highersourceid, subjectplaceid, jurisplaceid, researcherid, subjectdate, comments) VALUES ({}, {}, {}, {}, {}, \"{}\", \"{}\")",
            &sourceid.to_string(),
            &highersourceid.to_string(),
            &subjectplaceid.to_string(),
            &jurisplaceid.to_string(),
            &researcherid.to_string(),
            &subjectdate,
            &comments,
        );
        // println!("This is create_source: {}", parameters);
        parameters
    }

    pub fn read_source(
        Source {
            sourceid,
            highersourceid: _,
            subjectplaceid: _,
            jurisplaceid: _,
            researcherid: _,
            subjectdate: _,
            comments: _,
        }: Source,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM source WHERE sourceid={}",
            &sourceid.to_string(),
        );
        // println!("This is read_source: {}", parameters);
        parameters
    }

    pub fn update_source(
        Source {
            sourceid,
            highersourceid,
            subjectplaceid,
            jurisplaceid,
            researcherid,
            subjectdate,
            comments,
        }: Source,
    ) -> String {
        let parameters = format!(
            "UPDATE source SET sourceid={}, highersourceid={}, subjectplaceid={}, jurisplaceid={}, researcherid={}, subjectdate=\"{}\", comments=\"{}\" WHERE sourceid={}",
            &sourceid.to_string(),
            &highersourceid.to_string(),
            &subjectplaceid.to_string(),
            &jurisplaceid.to_string(),
            &researcherid.to_string(),
            &subjectdate,
            &comments,
            &sourceid.to_string(),
        );
        // println!("This is update_source: {}", parameters);
        parameters
    }

    pub fn delete_source(
        Source {
            sourceid,
            highersourceid: _,
            subjectplaceid: _,
            jurisplaceid: _,
            researcherid: _,
            subjectdate: _,
            comments: _,
        }: Source,
    ) -> String {
        let parameters = format!(
            "DELETE FROM source WHERE sourceid={}",
            &sourceid.to_string(),
        );
        // println!("This is delete_source: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_source() {
        let source = Source {
            sourceid: 1,
            highersourceid: 2,
            subjectplaceid: 3,
            jurisplaceid: 4,
            researcherid: 5,
            subjectdate: "2022-01-01".to_string(),
            comments: "Test comments".to_string(),
        };

        let expected = "INSERT INTO source (sourceid, highersourceid, subjectplaceid, jurisplaceid, researcherid, subjectdate, comments) VALUES (1, 2, 3, 4, 5, \"2022-01-01\", \"Test comments\")";
        let result = Source::create_source(source);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_read_source() {
        let source = Source {
            sourceid: 1,
            highersourceid: 2,
            subjectplaceid: 3,
            jurisplaceid: 4,
            researcherid: 5,
            subjectdate: "2022-01-01".to_string(),
            comments: "Test comments".to_string(),
        };

        let expected = "SELECT * FROM source WHERE sourceid=1";
        let result = Source::read_source(source);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_update_source() {
        let source = Source {
            sourceid: 1,
            highersourceid: 2,
            subjectplaceid: 3,
            jurisplaceid: 4,
            researcherid: 5,
            subjectdate: "2022-01-01".to_string(),
            comments: "Test comments".to_string(),
        };

        let expected = "UPDATE source SET sourceid=1, highersourceid=2, subjectplaceid=3, jurisplaceid=4, researcherid=5, subjectdate=\"2022-01-01\", comments=\"Test comments\" WHERE sourceid=1";
        let result = Source::update_source(source);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_delete_source() {
        let source = Source {
            sourceid: 1,
            highersourceid: 2,
            subjectplaceid: 3,
            jurisplaceid: 4,
            researcherid: 5,
            subjectdate: "2022-01-01".to_string(),
            comments: "Test comments".to_string(),
        };

        let expected = "DELETE FROM source WHERE sourceid=1";
        let result = Source::delete_source(source);

        assert_eq!(result, expected);
    }
}



