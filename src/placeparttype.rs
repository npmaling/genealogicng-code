#[derive(Debug)]
pub struct PlacePartType {
    pub placeparttypeid: i64,
    pub pptname: String,
}

impl PlacePartType {
    pub fn create_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO PlacePartType (placeparttypeid, pptname) VALUES ({}, \"{}\")",
            &placeparttypeid.to_string(),
            &pptname
        );
        // println!("This is create_placePartType: {}", parameters);
        parameters
    }

    pub fn read_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM PlacePartType WHERE placeparttypeid={}",
            &placeparttypeid.to_string(),
        );
        // println!("This is read_placePartType: {}", parameters);
        parameters
    }

    pub fn update_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "UPDATE placePartType SET placeparttypeid={}, pptname=\"{}\" WHERE placeparttypeid={}",
            &placeparttypeid.to_string(),
            &pptname,
            &placeparttypeid.to_string()
        );
        // println!("This is update_placePartType: {}", parameters);
        parameters
    }

    pub fn delete_placeparttype(
        PlacePartType {
            placeparttypeid,
            pptname,
        }: PlacePartType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM PlacePartType WHERE placeparttypeid={}",
            &placeparttypeid.to_string(),
        );
        // println!("This is delete_placePartType: {}", parameters);
        parameters
    }
}
