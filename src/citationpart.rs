#[derive(Debug)]
pub struct CitationPart {
    pub citationpartid: i64,
    pub sourceid: i64,
    pub citeparttypeid: i64,
    pub citepartvalue: String,
}

impl CitationPart {
    pub fn create_citationpart(
        CitationPart {
            citationpartid,
            sourceid,
            citeparttypeid,
            citepartvalue,
        }: CitationPart,
    ) -> String {
        let parameters = format!(
            "INSERT INTO citationpart (citationpartid, sourceid, citeparttypeid, citepartvalue) VALUES ({}, {}, {}, \"{}\")",
            citationpartid.to_string(),
            sourceid.to_string(),
            citeparttypeid.to_string(),
            citepartvalue,
        );
        // println!("This is create_citationpart: {}", parameters);
        parameters
    }

    pub fn read_citationpart(
        CitationPart {
            citationpartid,
            sourceid,
            citeparttypeid,
            citepartvalue,
        }: CitationPart,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM citationpart WHERE citationpartid={}",
            citationpartid.to_string(),
        );
        // println!("This is read_citationpart: {}", parameters);
        parameters
    }

    pub fn update_citationpart(
        CitationPart {
            citationpartid,
            sourceid,
            citeparttypeid,
            citepartvalue,
        }: CitationPart,
    ) -> String {
        let parameters = format!(
            "UPDATE citationpart SET citationpartid={}, sourceid={}, citeparttypeid={}, citepartvalue=\"{}\" WHERE citationpartid={}",
            citationpartid.to_string(),
            sourceid.to_string(),
            citeparttypeid.to_string(),
            citepartvalue,
            citationpartid.to_string(),
        );
        // println!("This is update_citationpart: {}", parameters);
        parameters
    }

    pub fn delete_citationpart(
        CitationPart {
            citationpartid,
            sourceid,
            citeparttypeid,
            citepartvalue,
        }: CitationPart,
    ) -> String {
        let parameters = format!(
            "DELETE FROM citationpart WHERE citationpartid={}",
            citationpartid.to_string(),
        );
        // println!("This is delete_citationpart: {}", parameters);
        parameters
    }
}
