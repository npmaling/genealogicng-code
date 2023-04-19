#[derive(Debug)]
pub struct Project {
    pub projectid: i64,
    pub name: String,
    pub projectdesc: String,
    pub clientdata: String,
}

impl Project {
    pub fn create_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "INSERT INTO project (projectid, name, projectdesc, clientdata) VALUES ({}, \"{}\", \"{}\", \"{}\")",
            &projectid.to_string(),
            &name,
            &projectdesc,
            &clientdata,
        );
        // println!("This is create_project: {}", parameters);
        parameters
    }

    pub fn read_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM project WHERE projectid={}",
            &projectid.to_string(),
        );
        // println!("This is read_project: {}", parameters);
        parameters
    }

    pub fn update_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "UPDATE project SET projectid={}, name=\"{}\", projectdesc=\"{}\", clientdata=\"{}\" WHERE projectid={}",
            &projectid.to_string(),
            &name,
            &projectdesc,
            &clientdata,
            &projectid.to_string(),
        );
        // println!("This is update_project: {}", parameters);
        parameters
    }

    pub fn delete_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "DELETE FROM project WHERE projectid={}",
            &projectid.to_string(),
        );
        // println!("This is delete_project: {}", parameters);
        parameters
    }
}
