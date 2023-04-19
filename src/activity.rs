#[derive(Debug)]
pub struct Activity {
    pub activityid: i64,
    pub projectid: i64,
    pub researcherid: i64,
    pub scheddate: String,
    pub completedate: String,
    pub typecode: String,
    pub status: String,
    pub description: String,
    pub priority: String,
    pub comments: String,
}

impl Activity {
    pub fn create_activity(
        Activity {
            activityid,
            projectid,
            researcherid,
            scheddate,
            completedate,
            typecode,
            status,
            description,
            priority,
            comments,
        }: Activity,
    ) -> String {
        let parameters = format!(
            "INSERT INTO Activity (activityid, projectid, researcherid, scheddate, completedate, typecode, status, description, priority, comments
            ) VALUES ({}, {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")",
            &activityid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &scheddate,
            &completedate,
            &typecode,
            &status,
            &description,
            &priority,
            &comments
        );
        // println!("This is create_activity: {}", parameters);
        parameters
    }

    pub fn read_activity(
        Activity {
            activityid,
            projectid,
            researcherid,
            scheddate,
            completedate,
            typecode,
            status,
            description,
            priority,
            comments,
        }: Activity,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM activity WHERE activityid={}",
            &activityid.to_string(),
        );
        parameters
    }

    pub fn update_activity(
        Activity {
            activityid,
            projectid,
            researcherid,
            scheddate,
            completedate,
            typecode,
            status,
            description,
            priority,
            comments,
        }: Activity,
    ) -> String {
        let parameters = format!(
            "UPDATE activity SET projectid={}, researcherid={}, scheddate=\"{}\", completedate=\"{}\", typecode=\"{}\", status=\"{}\", description=\"{}\", priority=\"{}\", comments=\"{}\" WHERE activityid={}",
            // &activityid.to_string(),
            &projectid.to_string(),
            &researcherid.to_string(),
            &scheddate,
            &completedate,
            &typecode,
            &status,
            &description,
            &priority,
            &comments,
            &activityid.to_string(),
        );
        // println!("This is update_activity: {}", parameters);
        parameters
    }

    pub fn delete_activity(
        Activity {
            activityid,
            projectid,
            researcherid,
            scheddate,
            completedate,
            typecode,
            status,
            description,
            priority,
            comments,
        }: Activity,
    ) -> String {
        let parameters = format!(
            "DELETE FROM activity WHERE activityid={}",
            &activityid.to_string(),
        );
        parameters
    }
}

