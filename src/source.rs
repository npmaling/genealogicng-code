#[derive(Debug)]
pub struct Source {
    pub sourceid: i64,
    pub highersourceid: i64,
    pub subjectplaceid: i64,
    pub jurisplaceid: i64,
    pub researcherid: i64,
    pub subjectdate: String,
    pub comments: String,
}

impl Source {
    pub fn create_source(
        Source {
            sourceid,
            highersourceid,
            subjectplaceid,
            jurisplaceid,
            researcherid,
            subjectdate,
            comments,
        }: Source,
    ) -> String {
        let parameters = format!(
            "INSERT INTO source (sourceid, highersourceid, subjectplaceid, jurisplaceid, researcherid, subjectdate, comments) VALUES ({}, {}, {}, {}, {}, \"{}\", \"{}\")",
            &sourceid.to_string(),
            &highersourceid.to_string(),
            &subjectplaceid.to_string(),
            &jurisplaceid.to_string(),
            &researcherid.to_string(),
            &subjectdate,
            &comments,
        );
        // println!("This is create_source: {}", parameters);
        parameters
    }

    pub fn read_source(
        Source {
            sourceid,
            highersourceid,
            subjectplaceid,
            jurisplaceid,
            researcherid,
            subjectdate,
            comments,
        }: Source,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM source WHERE sourceid={}",
            &sourceid.to_string(),
        );
        // println!("This is read_source: {}", parameters);
        parameters
    }

    pub fn update_source(
        Source {
            sourceid,
            highersourceid,
            subjectplaceid,
            jurisplaceid,
            researcherid,
            subjectdate,
            comments,
        }: Source,
    ) -> String {
        let parameters = format!(
            "UPDATE source SET sourceid={}, highersourceid={}, subjectplaceid={}, jurisplaceid={}, researcherid={}, subjectdate=\"{}\", comments=\"{}\" WHERE sourceid={}",
            &sourceid.to_string(),
            &highersourceid.to_string(),
            &subjectplaceid.to_string(),
            &jurisplaceid.to_string(),
            &researcherid.to_string(),
            &subjectdate,
            &comments,
            &sourceid.to_string(),
        );
        // println!("This is update_source: {}", parameters);
        parameters
    }

    pub fn delete_source(
        Source {
            sourceid,
            highersourceid,
            subjectplaceid,
            jurisplaceid,
            researcherid,
            subjectdate,
            comments,
        }: Source,
    ) -> String {
        let parameters = format!(
            "DELETE FROM source WHERE sourceid={}",
            &sourceid.to_string(),
        );
        // println!("This is delete_source: {}", parameters);
        parameters
    }
}
