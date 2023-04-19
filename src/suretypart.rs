#[derive(Debug)]
pub struct SuretyPart {
    pub suretypartid: i64,
    pub schemeid: i64,
    pub name: String,
    pub description: String,
    pub sequencenumber: i64,
}

impl SuretyPart {
    pub fn create_suretypart(
        SuretyPart {
            suretypartid,
            schemeid,
            name,
            description,
            sequencenumber,
        }: SuretyPart,
    ) -> String {
        let parameters = format!(
            "INSERT INTO suretypart (suretypartid, schemeid, name, description, sequencenumber) VALUES ({}, {}, \"{}\", \"{}\", {})",
            suretypartid.to_string(),
            schemeid.to_string(),
            name,
            description,
            sequencenumber.to_string(),
        );
        // println!("This is create_suretypart: {}", parameters);
        parameters
    }

    pub fn read_suretypart(
        SuretyPart {
            suretypartid,
            schemeid,
            name,
            description,
            sequencenumber,
        }: SuretyPart,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM suretypart WHERE suretypartid={}",
            suretypartid.to_string(),
        );
        // println!("This is read_suretypart: {}", parameters);
        parameters
    }

    pub fn update_suretypart(
        SuretyPart {
            suretypartid,
            schemeid,
            name,
            description,
            sequencenumber,
        }: SuretyPart,
    ) -> String {
        let parameters = format!(
            "UPDATE suretypart SET suretypartid={}, schemeid={}, name=\"{}\", description=\"{}\", sequencenumber={} WHERE suretypartid={}",
            suretypartid.to_string(),
            schemeid.to_string(),
            name,
            description,
            sequencenumber.to_string(),
            suretypartid.to_string(),
        );
        // println!("This is update_suretypart: {}", parameters);
        parameters
    }

    pub fn delete_suretypart(
        SuretyPart {
            suretypartid,
            schemeid,
            name,
            description,
            sequencenumber,
        }: SuretyPart,
    ) -> String {
        let parameters = format!(
            "DELETE FROM suretypart WHERE suretypartid={}",
            suretypartid.to_string(),
        );
        // println!("This is delete_suretypart: {}", parameters);
        parameters
    }
}
