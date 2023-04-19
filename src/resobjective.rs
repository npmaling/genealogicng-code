#[derive(Debug)]
pub struct ResObjective {
    pub resobjid: i64,
    pub projectid: i64,
    pub subjectid: i64,
    pub subjecttype: String,
    pub name: String,
    pub description: String,
    pub sequencenumber: i64,
    pub priority: String,
    pub status: String,
}

impl ResObjective {
    pub fn create_resobjective(
        ResObjective {
            resobjid,
            projectid,
            subjectid,
            subjecttype,
            name,
            description,
            sequencenumber,
            priority,
            status,
        }: ResObjective,
    ) -> String {
        let parameters = format!(
            "INSERT INTO resobjective (resobjid, projectid, subjectid, subjecttype, name, description, sequencenumber, priority, status) VALUES ({}, {}, {}, \"{}\", \"{}\", \"{}\", {}, \"{}\", \"{}\")",
            &resobjid.to_string(),
            &projectid.to_string(),
            &subjectid.to_string(),
            &subjecttype,
            &name,
            &description,
            &sequencenumber.to_string(),
            &priority,
            &status,
        );
        // println!("This is create_resobjective: {}", parameters);
        parameters
    }

    pub fn read_resobjective(
        ResObjective {
            resobjid,
            projectid,
            subjectid,
            subjecttype,
            name,
            description,
            sequencenumber,
            priority,
            status,
        }: ResObjective,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM resobjective WHERE resobjid={}",
            &resobjid.to_string(),
        );
        // println!("This is read_resobjective: {}", parameters);
        parameters
    }

    pub fn update_resobjective(
        ResObjective {
            resobjid,
            projectid,
            subjectid,
            subjecttype,
            name,
            description,
            sequencenumber,
            priority,
            status,
        }: ResObjective,
    ) -> String {
        let parameters = format!(
            "UPDATE resobjective SET resobjid={}, projectid={}, subjectid={}, subjecttype=\"{}\", name=\"{}\", description=\"{}\", sequencenumber={}, priority=\"{}\", status=\"{}\" WHERE resobjid={}",
            &resobjid.to_string(),
            &projectid.to_string(),
            &subjectid.to_string(),
            &subjecttype,
            &name,
            &description,
            &sequencenumber.to_string(),
            &priority,
            &status,
            &resobjid.to_string(),
        );
        // println!("This is update_resobjective: {}", parameters);
        parameters
    }

    pub fn delete_resobjective(
        ResObjective {
            resobjid,
            projectid,
            subjectid,
            subjecttype,
            name,
            description,
            sequencenumber,
            priority,
            status,
        }: ResObjective,
    ) -> String {
        let parameters = format!(
            "DELETE FROM ResObjective WHERE resobjid={}",
            &resobjid.to_string(),
        );
        // println!("This is delete_resobjective: {}", parameters);
        parameters
    }
}
