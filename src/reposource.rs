#[derive(Debug)]
pub struct RepoSource {
    pub reposourceid: i64,
    pub repositoryid: i64,
    pub sourceid: i64,
    pub rsactivityid: i64,
    pub callnumber: String,
    pub description: String,
}

impl RepoSource {
    pub fn create_reposource(
        RepoSource {
            reposourceid,
            repositoryid,
            sourceid,
            rsactivityid,
            callnumber,
            description,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
        "INSERT INTO reposource (reposourceid, repositoryid, sourceid, rsactivityid, callnumber, description) VALUES ({}, {}, {}, {}, \"{}\", \"{}\")",
        reposourceid.to_string(),
        repositoryid.to_string(),
        sourceid.to_string(),
        rsactivityid.to_string(),
        callnumber,
        description,
    );
        // println!("This is create_reposource: {}", parameters);
        parameters
    }

    pub fn read_reposource(
        RepoSource {
            reposourceid,
            repositoryid,
            sourceid,
            rsactivityid,
            callnumber,
            description,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM reposource WHERE reposourceid={}",
            reposourceid.to_string(),
        );
        // println!("This is read_reposource: {}", parameters);
        parameters
    }

    pub fn update_reposource(
        RepoSource {
            reposourceid,
            repositoryid,
            sourceid,
            rsactivityid,
            callnumber,
            description,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
        "UPDATE reposource SET reposourceid={}, repositoryid={}, sourceid={}, rsactivityid={}, callnumber=\"{}\", description=\"{}\" WHERE reposourceid={}",
        reposourceid.to_string(),
        repositoryid.to_string(),
        sourceid.to_string(),
        rsactivityid.to_string(),
        callnumber,
        description,
        reposourceid.to_string(),
    );
        // println!("This is update_reposource: {}", parameters);
        parameters
    }

    pub fn delete_reposource(
        RepoSource {
            reposourceid,
            repositoryid,
            sourceid,
            rsactivityid,
            callnumber,
            description,
        }: RepoSource,
    ) -> String {
        let parameters = format!(
            "DELETE FROM reposource WHERE reposourceid={}",
            reposourceid.to_string(),
        );
        // println!("This is delete_reposource: {}", parameters);
        parameters
    }
}
