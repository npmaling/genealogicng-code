#[derive(Debug)]
pub struct ResProj {
    pub resprojid: i64,
    pub projectid: i64,
    pub researcherid: i64,
    pub researcherrole: String,
}

impl ResProj {
    pub fn create_resproj(
        ResProj {
            resprojid,
            projectid,
            researcherid,
            researcherrole,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "INSERT INTO resproj (resprojid, projectid, researcherid, researcherrole) VALUES ({}, {}, {}, \"{}\")",
            &resprojid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &researcherrole,
        );
        // println!("This is create_resproj: {}", parameters);
        parameters
    }

    pub fn read_resproj(
        ResProj {
            resprojid,
            projectid,
            researcherid,
            researcherrole,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM resproj WHERE resprojid={}",
            &resprojid.to_string(),
        );
        // println!("This is read_resproj: {}", parameters);
        parameters
    }

    pub fn update_resproj(
        ResProj {
            resprojid,
            projectid,
            researcherid,
            researcherrole,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "UPDATE resproj SET resprojid={}, projectid={}, researcherid={}, researcherrole=\"{}\" WHERE resprojid={}",
            &resprojid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &researcherrole,
            &resprojid.to_string(),
        );
        // println!("This is update_resproj: {}", parameters);
        parameters
    }

    pub fn delete_resproj(
        ResProj {
            resprojid,
            projectid,
            researcherid,
            researcherrole,
        }: ResProj,
    ) -> String {
        let parameters = format!(
            "DELETE FROM resproj WHERE resprojid={}",
            &resprojid.to_string(),
        );
        // println!("This is delete_resproj: {}", parameters);
        parameters
    }
}
