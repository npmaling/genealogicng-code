#[derive(Debug)]
pub struct Representation {
    pub representationid: i64,
    pub sourceid: i64,
    pub reprtypeid: i64,
    pub reprmediaid: i64,
    pub physfilecode: String,
    pub comments: String,
    pub externallink: String,
}

impl Representation {
    pub fn create_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "INSERT INTO representation (representationid, sourceid, reprtypeid, reprmediaid, physfilecode, comments, externallink) VALUES ({}, {}, {}, {}, \"{}\", \"{}\", \"{}\")",
            representationid.to_string(),
            sourceid.to_string(),
            reprtypeid.to_string(),
            reprmediaid.to_string(),
            physfilecode,
            comments,
            externallink,
        );
        // println!("This is create_representation: {}", parameters);
        parameters
    }

    pub fn read_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM representation WHERE representationid={}",
            representationid.to_string(),
        );
        // println!("This is read_representation: {}", parameters);
        parameters
    }

    pub fn update_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "UPDATE representation SET representationid={}, sourceid={}, reprtypeid={}, reprmediaid={}, physfilecode=\"{}\", comments=\"{}\", externallink=\"{}\" WHERE reprtypeid={}",
            representationid.to_string(),
            sourceid.to_string(),
            reprtypeid.to_string(),
            reprmediaid.to_string(),
            physfilecode,
            comments,
            externallink,
            representationid.to_string(),
        );
        // println!("This is update_representation: {}", parameters);
        parameters
    }

    pub fn delete_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "DELETE FROM representation WHERE representationid={}",
            representationid.to_string(),
        );
        // println!("This is delete_representation: {}", parameters);
        parameters
    }
}
