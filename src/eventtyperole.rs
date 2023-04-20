#[derive(Debug)]
pub struct EventTypeRole {
    pub eventtyperoleid: i64,
    pub eventtypeid: i64,
    pub eventtyperolename: String,
}

impl EventTypeRole {
    pub fn create_eventtyperole(
        EventTypeRole {
            eventtyperoleid,
            eventtypeid,
            eventtyperolename,
        }: EventTypeRole,
    ) -> String {
        let parameters = format!(
            "INSERT INTO eventtyperole (eventtyperoleid, eventtypeid, eventtyperolename) VALUES ({}, {}, \"{}\")",
            eventtyperoleid.to_string(),
            eventtypeid.to_string(),
            eventtyperolename,
        );
        // println!("This is create_eventtyperole: {}", parameters);
        parameters
    }

    pub fn read_eventtyperole(
        EventTypeRole {
            eventtyperoleid,
            eventtypeid,
            eventtyperolename,
        }: EventTypeRole,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM eventtyperole WHERE eventtyperoleid={}",
            eventtyperoleid.to_string(),
        );
        // println!("This is read_eventtyperole: {}", parameters);
        parameters
    }

    pub fn update_eventtyperole(
        EventTypeRole {
            eventtyperoleid,
            eventtypeid,
            eventtyperolename,
        }: EventTypeRole,
    ) -> String {
        let parameters = format!(
            "UPDATE eventtyperole SET eventtyperoleid={}, eventtypeid={}, eventtyperolename=\"{}\" WHERE eventtyperoleid={}",
            eventtyperoleid.to_string(),
            eventtypeid.to_string(),
            eventtyperolename,
            eventtyperoleid.to_string(),
        );
        // println!("This is update_eventtyperole: {}", parameters);
        parameters
    }

    pub fn delete_eventtyperole(
        EventTypeRole {
            eventtyperoleid,
            eventtypeid,
            eventtyperolename,
        }: EventTypeRole,
    ) -> String {
        let parameters = format!(
            "DELETE FROM eventtyperole WHERE eventtyperoleid={}",
            eventtyperoleid.to_string(),
        );
        // println!("This is delete_eventtyperole: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_eventtyperole() {
        let event_type_role = EventTypeRole {
            eventtyperoleid: 1,
            eventtypeid: 2,
            eventtyperolename: String::from("Test event type role"),
        };
        let expected_query = "INSERT INTO eventtyperole (eventtyperoleid, eventtypeid, eventtyperolename) VALUES (1, 2, \"Test event type role\")";
        assert_eq!(EventTypeRole::create_eventtyperole(event_type_role), expected_query);
    }

    #[test]
    fn test_read_eventtyperole() {
        let event_type_role = EventTypeRole {
            eventtyperoleid: 1,
            eventtypeid: 2,
            eventtyperolename: String::from("Test event type role"),
        };
        let expected_query = "SELECT * FROM eventtyperole WHERE eventtyperoleid=1";
        assert_eq!(EventTypeRole::read_eventtyperole(event_type_role), expected_query);
    }

    #[test]
    fn test_update_eventtyperole() {
        let event_type_role = EventTypeRole {
            eventtyperoleid: 1,
            eventtypeid: 2,
            eventtyperolename: String::from("Test event type role"),
        };
        let expected_query = "UPDATE eventtyperole SET eventtyperoleid=1, eventtypeid=2, eventtyperolename=\"Test event type role\" WHERE eventtyperoleid=1";
        assert_eq!(EventTypeRole::update_eventtyperole(event_type_role), expected_query);
    }

    #[test]
    fn test_delete_eventtyperole() {
        let event_type_role = EventTypeRole {
            eventtyperoleid: 1,
            eventtypeid: 2,
            eventtyperolename: String::from("Test role"),
        };
        let expected_query = "DELETE FROM eventtyperole WHERE eventtyperoleid=1";
        assert_eq!(EventTypeRole::delete_eventtyperole(event_type_role), expected_query);
    }

}

