#[derive(Debug)]
pub struct Researcher {
    pub researcherid: i64,
    pub name: String,
    pub addressid: i64,
    pub comments: String,
}

impl Researcher {
    pub fn create_researcher(
        Researcher {
            researcherid,
            name,
            addressid,
            comments,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "INSERT INTO researcher (researcherid, name, addressid, comments) VALUES ({}, \"{}\", {}, \"{}\")",
            &researcherid.to_string(),
            &name,
            &addressid.to_string(),
            &comments,
        );
        // println!("This is create_researcher: {}", parameters);
        parameters
    }

    pub fn read_researcher(
        Researcher {
            researcherid,
            name,
            addressid,
            comments,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM researcher WHERE researcherid={}",
            &researcherid.to_string(),
        );
        // println!("This is read_researcher: {}", parameters);
        parameters
    }

    pub fn update_researcher(
        Researcher {
            researcherid,
            name,
            addressid,
            comments,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "UPDATE researcher SET researcherid={}, name=\"{}\", addressid={}, comments=\"{}\" WHERE researcherid={}",
            &researcherid.to_string(),
            &name,
            &addressid.to_string(),
            &comments,
            &researcherid.to_string(),
        );
        // println!("This is update_researcher: {}", parameters);
        parameters
    }

    pub fn delete_researcher(
        Researcher {
            researcherid,
            name,
            addressid,
            comments,
        }: Researcher,
    ) -> String {
        let parameters = format!(
            "DELETE FROM researcher WHERE researcherid={}",
            &researcherid.to_string(),
        );
        // println!("This is delete_researcher: {}", parameters);
        parameters
    }
}
