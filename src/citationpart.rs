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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_citationpart() {
        let citation_part = CitationPart {
            citationpartid: 1,
            sourceid: 2,
            citeparttypeid: 3,
            citepartvalue: "Test citation part".to_string(),
        };
        let expected_query = "INSERT INTO citationpart (citationpartid, sourceid, citeparttypeid, citepartvalue) VALUES (1, 2, 3, \"Test citation part\")";
        assert_eq!(CitationPart::create_citationpart(citation_part), expected_query);
    }

    #[test]
    fn test_read_citationpart() {
        let citation_part = CitationPart {
            citationpartid: 1,
            sourceid: 2,
            citeparttypeid: 3,
            citepartvalue: String::from("test"),
        };
        let expected_query = "SELECT * FROM citationpart WHERE citationpartid=1";
        assert_eq!(CitationPart::read_citationpart(citation_part), expected_query);
    }

    #[test]
    fn test_update_citationpart() {
        let citation_part = CitationPart {
            citationpartid: 1,
            sourceid: 1,
            citeparttypeid: 1,
            citepartvalue: String::from("test"),
        };
        let expected_query = "UPDATE citationpart SET citationpartid=1, sourceid=1, citeparttypeid=1, citepartvalue=\"test\" WHERE citationpartid=1";
        assert_eq!(CitationPart::update_citationpart(citation_part), expected_query);
    }

    #[test]
    fn test_delete_citationpart() {
        let citation_part = CitationPart {
            citationpartid: 1,
            sourceid: 2,
            citeparttypeid: 3,
            citepartvalue: String::from("Test value"),
        };
        let expected_query = "DELETE FROM citationpart WHERE citationpartid=1";
        assert_eq!(CitationPart::delete_citationpart(citation_part), expected_query);
    }
}

