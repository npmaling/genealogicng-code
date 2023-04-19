#[derive(Debug)]
pub struct CitationPartType {
    pub citationparttypeid: i64,
    pub citationparttypename: String,
}

impl CitationPartType {
    pub fn create_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO citationparttype (citationparttypeid, citationparttypename) VALUES ({}, \"{}\")",
            citationparttypeid.to_string(),
            citationparttypename,
        );
        // println!("This is create_citationparttype: {}", parameters);
        parameters
    }

    pub fn read_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM citationparttype WHERE citationparttypeid={}",
            citationparttypeid.to_string(),
        );
        // println!("This is read_citationparttype: {}", parameters);
        parameters
    }

    pub fn update_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "UPDATE citationparttype SET citationparttypeid={}, citationparttypename=\"{}\" WHERE citationparttypeid={}",
            citationparttypeid.to_string(),
            citationparttypename,
            citationparttypeid.to_string(),
        );
        // println!("This is update_citationparttype: {}", parameters);
        parameters
    }

    pub fn delete_citationparttype(
        CitationPartType {
            citationparttypeid,
            citationparttypename,
        }: CitationPartType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM citationparttype WHERE citationparttypeid={}",
            citationparttypeid.to_string(),
        );
        // println!("This is delete_citationparttype: {}", parameters);
        parameters
    }
}

