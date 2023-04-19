#[derive(Debug)]
pub struct GlGroupTypeRole {
    pub glgrouptyperoleid: i64,
    pub glgrouptypeid: i64,
    pub glgrouptypename: String,
    pub sequencenumber: i64,
}


impl GlGroupTypeRole {
    pub fn create_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid,
            glgrouptypename,
            sequencenumber,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
        "INSERT INTO glgrouptyperole (glgrouptyperoleid, glgrouptypeid, glgrouptypename, sequencenumber) VALUES ({}, {}, \"{}\", {})",
        glgrouptyperoleid.to_string(),
        glgrouptypeid.to_string(),
        glgrouptypename,
        sequencenumber.to_string(),
    );
        // println!("This is create_glgrouptyperole: {}", parameters);
        parameters
    }

    pub fn read_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid,
            glgrouptypename,
            sequencenumber,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glgrouptyperole WHERE glgrouptyperoleid={}",
            glgrouptyperoleid.to_string(),
        );
        // println!("This is read_glgrouptyperole: {}", parameters);
        parameters
    }

    pub fn update_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid,
            glgrouptypename,
            sequencenumber,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
        "UPDATE glgrouptyperole SET glgrouptyperoleid={}, glgrouptypeid={}, glgrouptypename=\"{}\", sequencenumber={} WHERE glgrouptyperoleid={}",
        glgrouptyperoleid.to_string(),
        glgrouptypeid.to_string(),
        glgrouptypename,
        sequencenumber.to_string(),
        glgrouptyperoleid.to_string(),
    );
        // println!("This is update_glgrouptyperole: {}", parameters);
        parameters
    }

    pub fn delete_glgrouptyperole(
        GlGroupTypeRole {
            glgrouptyperoleid,
            glgrouptypeid,
            glgrouptypename,
            sequencenumber,
        }: GlGroupTypeRole,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glgrouptyperole WHERE glgrouptyperoleid={}",
            glgrouptyperoleid.to_string(),
        );
        // println!("This is delete_glgrouptyperole: {}", parameters);
        parameters
    }
}
