#[derive(Debug)]
pub struct CharPartType {
    pub charparttypeid: i64,
    pub charparttypename: String,
    pub gedcomtag: String,
}

impl CharPartType {
    pub fn create_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO charparttype (charparttypeid, charparttypename, gedcomtag) VALUES ({}, \"{}\", \"{}\")",
            &charparttypeid.to_string(),
            &charparttypename,
            &gedcomtag,
        );
        // println!("This is create_charparttype: {}", parameters);
        parameters
    }

    pub fn read_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM charparttype WHERE charparttypeid={}",
            &charparttypeid.to_string(),
        );
        // println!("This is read_charparttype: {}", parameters);
        parameters
    }

    pub fn update_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "UPDATE charparttype SET charparttypeid={}, charparttypename=\"{}\", gedcomtag=\"{}\" WHERE charparttypeid={}",
            &charparttypeid.to_string(),
            &charparttypename,
            &gedcomtag,
            &charparttypeid.to_string(),
        );
        // println!("This is update_charparttype: {}", parameters);
        parameters
    }

    pub fn delete_charparttype(
        CharPartType {
            charparttypeid,
            charparttypename,
            gedcomtag,
        }: CharPartType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM charparttype WHERE charparttypeid={}",
            &charparttypeid.to_string(),
        );
        // println!("This is delete_charparttype: {}", parameters);
        parameters
    }
}
