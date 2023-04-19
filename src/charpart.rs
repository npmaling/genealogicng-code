#[derive(Debug)]
pub struct CharPart {
    pub characteristicpartid: i64,
    pub characteristicid: i64,
    pub charparttypeid: i64,
    pub charpartname: String,
    pub charpartseq: i64,
}

impl CharPart {
    pub fn create_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "INSERT INTO charpart (characteristicpartid, characteristicid, charparttypeid, charpartname, charpartseq) VALUES ({}, {}, {}, \"{}\", \"{}\")",
            &characteristicpartid.to_string(),
            &characteristicid.to_string(),
            &charparttypeid.to_string(),
            &charpartname,
            &charpartseq.to_string(),
        );
        // println!("This is create_charpart: {}", parameters);
        parameters
    }

    pub fn read_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM charpart WHERE characteristicpartid={}",
            &characteristicpartid.to_string(),
        );
        // println!("This is read_charpart: {}", parameters);
        parameters
    }

    pub fn update_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "UPDATE charpart SET characteristicpartid={}, characteristicid={}, charparttypeid={}, charpartname=\"{}\", charpartseq={} WHERE characteristicpartid={}",
            &characteristicpartid.to_string(),
            &characteristicid.to_string(),
            &charparttypeid.to_string(),
            &charpartname,
            &charpartseq.to_string(),
            &characteristicpartid.to_string(),
        );
        // println!("This is update_charpart: {}", parameters);
        parameters
    }

    pub fn delete_charpart(
        CharPart {
            characteristicpartid,
            characteristicid,
            charparttypeid,
            charpartname,
            charpartseq,
        }: CharPart,
    ) -> String {
        let parameters = format!(
            "DELETE FROM charpart WHERE characteristicpartid={}",
            &characteristicpartid.to_string(),
        );
        // println!("This is delete_charpart: {}", parameters);
        parameters
    }
}
