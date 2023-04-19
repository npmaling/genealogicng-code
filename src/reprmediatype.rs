#[derive(Debug)]
pub struct ReprMediaType {
    pub reprmediaid: i64,
    pub reprmedianame: String,
}

impl ReprMediaType {
    pub fn create_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO reprmediatype (reprmediaid, reprmedianame) VALUES ({}, \"{}\")",
            reprmediaid.to_string(),
            reprmedianame,
        );
        // println!("This is create_reprmediatype: {}", parameters);
        parameters
    }

    pub fn read_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM reprmediatype WHERE reprmediaid={}",
            reprmediaid.to_string(),
        );
        // println!("This is read_reprmediatype: {}", parameters);
        parameters
    }

    pub fn update_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "UPDATE reprmediatype SET reprmediaid={}, reprmedianame=\"{}\" WHERE reprmediaid={}",
            reprmediaid.to_string(),
            reprmedianame,
            reprmediaid.to_string(),
        );
        // println!("This is update_reprmediatype: {}", parameters);
        parameters
    }

    pub fn delete_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM reprmediatype WHERE reprmediaid={}",
            reprmediaid.to_string(),
        );
        // println!("This is delete_reprmediatype: {}", parameters);
        parameters
    }
}
