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
pub struct EventType {
    pub eventtypeid: i64,
    pub eventtypename: String,
    pub gedcomtag: String,
}

impl EventType {
    pub fn create_eventtype(
        EventType {
            eventtypeid,
            eventtypename,
            gedcomtag,
        }: EventType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO eventtype (eventtypeid, eventtypename, gedcomtag) VALUES ({}, \"{}\", \"{}\")",
            eventtypeid.to_string(),
            eventtypename,
            gedcomtag,
        );
        // println!("This is create_eventtype: {}", parameters);
        parameters
    }

    pub fn read_eventtype(
        EventType {
            eventtypeid,
            eventtypename,
            gedcomtag,
        }: EventType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM eventtype WHERE eventtypeid={}",
            eventtypeid.to_string(),
        );
        // println!("This is read_eventtype: {}", parameters);
        parameters
    }

    pub fn update_eventtype(
        EventType {
            eventtypeid,
            eventtypename,
            gedcomtag,
        }: EventType,
    ) -> String {
        let parameters = format!(
            "UPDATE eventtype SET eventtypeid={}, eventtypename=\"{}\", gedcomtag=\"{}\" WHERE eventtypeid={}",
            eventtypeid.to_string(),
            eventtypename,
            gedcomtag,
            eventtypeid.to_string(),
        );
        // println!("This is update_eventtype: {}", parameters);
        parameters
    }

    pub fn delete_eventtype(
        EventType {
            eventtypeid,
            eventtypename,
            gedcomtag,
        }: EventType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM eventtype WHERE eventtypeid={}",
            eventtypeid.to_string(),
        );
        // println!("This is delete_eventtype: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_eventtype() {
        let eventtype = EventType {
            eventtypeid: 1,
            eventtypename: String::from("Test event type"),
            gedcomtag: String::from("TEST"),
        };
        let expected_query = "INSERT INTO eventtype (eventtypeid, eventtypename, gedcomtag) VALUES (1, \"Test event type\", \"TEST\")";
        assert_eq!(EventType::create_eventtype(eventtype), expected_query);
    }

    #[test]
    fn test_read_eventtype() {
        let event_type = EventType {
            eventtypeid: 1,
            eventtypename: String::from("Test event type"),
            gedcomtag: String::from("TEST"),
        };
        let expected_query = "SELECT * FROM eventtype WHERE eventtypeid=1";
        assert_eq!(EventType::read_eventtype(event_type), expected_query);
    }

    #[test]
    fn test_update_eventtype() {
        let event_type = EventType {
            eventtypeid: 1,
            eventtypename: String::from("Test event"),
            gedcomtag: String::from("TEST"),
        };
        let expected_query = "UPDATE eventtype SET eventtypeid=1, eventtypename=\"Test event\", gedcomtag=\"TEST\" WHERE eventtypeid=1";
        assert_eq!(EventType::update_eventtype(event_type), expected_query);
    }

    #[test]
    fn test_delete_eventtype() {
        let eventtype = EventType {
            eventtypeid: 1,
            eventtypename: String::from("Test event type"),
            gedcomtag: String::from("TEST"),
        };
        let expected_query = "DELETE FROM eventtype WHERE eventtypeid=1";
        assert_eq!(EventType::delete_eventtype(eventtype), expected_query);
    }


}

