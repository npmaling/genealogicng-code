#[derive(Debug)]
pub struct GlGroup {
    pub glgroupid: i64,
    pub glgrouptypeid: i64,
    pub placeid: i64,
    pub glgroupdate: String,
    pub glgroupname: String,
    pub glgroupcriteria: String,
}

impl GlGroup {
    pub fn create_glgroup(
        GlGroup {
            glgroupid,
            glgrouptypeid,
            placeid,
            glgroupdate,
            glgroupname,
            glgroupcriteria,
        }: GlGroup,
    ) -> String {
        let parameters = format!(
        "INSERT INTO glgroup (glgroupid, glgrouptypeid, placeid, glgroupdate, glgroupname, glgroupcriteria) VALUES ({}, {}, {}, \"{}\",\"{}\", \"{}\")",
        glgroupid.to_string(),
        glgrouptypeid.to_string(),
        placeid.to_string(),
        glgroupdate,
        glgroupname,
        glgroupcriteria,
    );
        // println!("This is create_glgroup: {}", parameters);
        parameters
    }

    pub fn read_glgroup(
        GlGroup {
            glgroupid,
            glgrouptypeid,
            placeid,
            glgroupdate,
            glgroupname,
            glgroupcriteria,
        }: GlGroup,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glgroup WHERE glgroupid={}",
            glgroupid.to_string(),
        );
        // println!("This is read_glgroup: {}", parameters);
        parameters
    }

    pub fn update_glgroup(
        GlGroup {
            glgroupid,
            glgrouptypeid,
            placeid,
            glgroupdate,
            glgroupname,
            glgroupcriteria,
        }: GlGroup,
    ) -> String {
        let parameters = format!(
        "UPDATE glgroup SET glgroupid={}, glgrouptypeid={}, placeid={}, glgroupdate=\"{}\", glgroupname=\"{}\", glgroupcriteria=\"{}\" WHERE glgroupid={}",
        glgroupid.to_string(),
        glgrouptypeid,
        placeid,
        glgroupdate,
        glgroupname,
        glgroupcriteria,
        glgroupid.to_string(),
    );
        // println!("This is update_glgroup: {}", parameters);
        parameters
    }

    pub fn delete_glgroup(
        GlGroup {
            glgroupid,
            glgrouptypeid,
            placeid,
            glgroupdate,
            glgroupname,
            glgroupcriteria,
        }: GlGroup,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glgroup WHERE glgroupid={}",
            glgroupid.to_string(),
        );
        // println!("This is delete_glgroup: {}", parameters);
        parameters
    }
}
