#[derive(Debug)]
pub struct GlGroupType {
    pub glgrouptypeid: i64,
    pub glgroupname: String,
    pub ascdescnone: String,
}

impl GlGroupType {
    pub fn create_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname,
            ascdescnone,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
        "INSERT INTO glgrouptype (glgrouptypeid, glgroupname, ascdescnone) VALUES ({}, \"{}\", \"{}\")",
        glgrouptypeid.to_string(),
        glgroupname,
        ascdescnone,
    );
        // println!("This is create_glgrouptype: {}", parameters);
        parameters
    }

    pub fn read_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname,
            ascdescnone,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glgrouptype WHERE glgrouptypeid={}",
            glgrouptypeid.to_string(),
        );
        // println!("This is read_glgrouptype: {}", parameters);
        parameters
    }

    pub fn update_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname,
            ascdescnone,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
        "UPDATE glgrouptype SET glgrouptypeid={}, glgroupname=\"{}\", ascdescnone=\"{}\" WHERE glgrouptypeid={}",
        glgrouptypeid.to_string(),
        glgroupname,
        ascdescnone,
        glgrouptypeid.to_string(),
    );
        // println!("This is update_glgrouptype: {}", parameters);
        parameters
    }

    pub fn delete_glgrouptype(
        GlGroupType {
            glgrouptypeid,
            glgroupname,
            ascdescnone,
        }: GlGroupType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glgrouptype WHERE glgrouptypeid={}",
            glgrouptypeid.to_string(),
        );
        // println!("This is delete_glgrouptype: {}", parameters);
        parameters
    }
}
