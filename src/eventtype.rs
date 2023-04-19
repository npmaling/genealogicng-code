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
