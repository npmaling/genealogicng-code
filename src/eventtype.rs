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

