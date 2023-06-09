#[derive(Debug)]
pub struct Repository {
    pub repositoryid: i64,
    pub placeid: i64,
    pub reponame: String,
    pub comments: String,
}

impl Repository {
    pub fn create_repository(
        Repository {
            repositoryid,
            placeid,
            reponame,
            comments,
        }: Repository,
    ) -> String {
        let parameters = format!(
            "INSERT INTO repository (repositoryid, placeid, reponame, comments) VALUES ({}, {}, \"{}\", \"{}\")",
            &repositoryid.to_string(),
            &placeid.to_string(),
            &reponame,
            &comments,
        );
        // println!("This is create_repository: {}", parameters);
        parameters
    }

    pub fn read_repository(
        Repository {
            repositoryid,
            placeid,
            reponame,
            comments,
        }: Repository,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM repository WHERE repositoryid={}",
            &repositoryid.to_string(),
        );
        // println!("This is read_repository: {}", parameters);
        parameters
    }

    pub fn update_repository(
        Repository {
            repositoryid,
            placeid,
            reponame,
            comments,
        }: Repository,
    ) -> String {
        let parameters = format!(
            "UPDATE repository SET repositoryid={}, placeid={}, reponame=\"{}\", comments=\"{}\" WHERE repositoryid={}",
            &repositoryid.to_string(),
            &placeid.to_string(),
            &reponame,
            &comments,
            &repositoryid.to_string(),
        );
        // println!("This is update_repository: {}", parameters);
        parameters
    }

    pub fn delete_repository(
        Repository {
            repositoryid,
            placeid,
            reponame,
            comments,
        }: Repository,
    ) -> String {
        let parameters = format!(
            "DELETE FROM repository WHERE repositoryid={}",
            &repositoryid.to_string(),
        );
        // println!("This is delete_repository: {}", parameters);
        parameters
    }
}
