#[derive(Debug)]
pub struct Event {
    pub eventid: i64,
    pub eventtypeid: i64,
    pub placeid: i64,
    pub eventdate: String,
    pub eventname: String,
}

impl Event {
    pub fn create_event(
        Event {
            eventid,
            eventtypeid,
            placeid,
            eventdate,
            eventname,
        }: Event,
    ) -> String {
        let parameters = format!(
            "INSERT INTO event (eventid, eventtypeid, placeid, eventdate, eventname) VALUES ({}, {}, {}, \"{}\", \"{}\")",
            eventid.to_string(),
            eventtypeid.to_string(),
            placeid.to_string(),
            eventdate,
            eventname,
        );
        // println!("This is create_event: {}", parameters);
        parameters
    }

    pub fn read_event(
        Event {
            eventid,
            eventtypeid,
            placeid,
            eventdate,
            eventname,
        }: Event,
    ) -> String {
        let parameters = format!("SELECT * FROM event WHERE eventid={}", eventid.to_string(),);
        // println!("This is read_event: {}", parameters);
        parameters
    }

    pub fn update_event(
        Event {
            eventid,
            eventtypeid,
            placeid,
            eventdate,
            eventname,
        }: Event,
    ) -> String {
        let parameters = format!(
            "UPDATE event SET eventid={}, eventtypeid={}, placeid={}, eventdate=\"{}\", eventname=\"{}\" WHERE eventid={}",
            eventid.to_string(),
            eventtypeid.to_string(),
            placeid.to_string(),
            eventdate,
            eventname,
            eventid.to_string(),
        );
        // println!("This is update_event: {}", parameters);
        parameters
    }

    pub fn delete_event(
        Event {
            eventid,
            eventtypeid,
            placeid,
            eventdate,
            eventname,
        }: Event,
    ) -> String {
        let parameters = format!("DELETE FROM event WHERE eventid={}", eventid.to_string(),);
        // println!("This is delete_event: {}", parameters);
        parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event() {
        let event = Event {
            eventid: 1,
            eventtypeid: 2,
            placeid: 3,
            eventdate: String::from("2021-01-01"),
            eventname: String::from("Test event"),
        };
        let expected_query = "INSERT INTO event (eventid, eventtypeid, placeid, eventdate, eventname) VALUES (1, 2, 3, \"2021-01-01\", \"Test event\")";
        assert_eq!(Event::create_event(event), expected_query);
    }

    #[test]
    fn test_read_event() {
        let event = Event {
            eventid: 1,
            eventtypeid: 1,
            placeid: 1,
            eventdate: String::from("2021-01-01"),
            eventname: String::from("Test event"),
        };
        let expected_query = "SELECT * FROM event WHERE eventid=1";
        assert_eq!(Event::read_event(event), expected_query);
    }

    #[test]
    fn test_update_event() {
        let event = Event {
            eventid: 1,
            eventtypeid: 2,
            placeid: 3,
            eventdate: String::from("2021-01-01"),
            eventname: String::from("Test event"),
        };
        let expected_query = "UPDATE event SET eventid=1, eventtypeid=2, placeid=3, eventdate=\"2021-01-01\", eventname=\"Test event\" WHERE eventid=1";
        assert_eq!(Event::update_event(event), expected_query);
    }

    #[test]
    fn test_delete_event() {
        let event = Event {
            eventid: 1,
            eventtypeid: 2,
            placeid: 3,
            eventdate: String::from("2021-01-01"),
            eventname: String::from("Test event"),
        };
        let expected_query = "DELETE FROM event WHERE eventid=1";
        assert_eq!(Event::delete_event(event), expected_query);
    }

}
