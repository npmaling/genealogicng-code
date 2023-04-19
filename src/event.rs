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
