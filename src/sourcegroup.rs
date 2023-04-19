#[derive(Debug)]
pub struct SourceGroup {
    pub sourcegroupid: i64,
    pub sourcegroupname: String,
}

impl SourceGroup {
    pub fn create_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "INSERT INTO sourcegroup (sourcegroupid, sourcegroupname) VALUES ({}, \"{}\")",
            &sourcegroupid.to_string(),
            &sourcegroupname,
        );
        // println!("This is create_sourcegroup: {}", parameters);
        parameters
    }

    pub fn read_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM sourcegroup WHERE sourcegroupid={}",
            &sourcegroupid.to_string(),
        );
        // println!("This is read_sourcegroup: {}", parameters);
        parameters
    }

    pub fn update_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "UPDATE sourcegroup SET sourcegroupid={}, sourcegroupname=\"{}\" WHERE sourcegroupid={}",
            &sourcegroupid.to_string(),
            &sourcegroupname,
            &sourcegroupid.to_string(),
        );
        // println!("This is update_sourcegroup: {}", parameters);
        parameters
    }

    pub fn delete_sourcegroup(
        SourceGroup {
            sourcegroupid,
            sourcegroupname,
        }: SourceGroup,
    ) -> String {
        let parameters = format!(
            "DELETE FROM sourcegroup WHERE sourcegroupid={}",
            &sourcegroupid.to_string(),
        );
        // println!("This is delete_sourcegroup: {}", parameters);
        parameters
    }
}
