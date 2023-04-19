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
