#[derive(Debug)]
pub struct SrcGrpSrc {
    pub srcgrpsrcid: i64,
    pub sourceid: i64,
    pub sourcegroupid: i64,
}

impl SrcGrpSrc {
    pub fn create_srcgrpsrc(
        SrcGrpSrc {
            srcgrpsrcid,
            sourceid,
            sourcegroupid,
        }: SrcGrpSrc,
    ) -> String {
        let parameters = format!(
            "INSERT INTO srcgrpsrc (srcgrpsrcid, sourceid, sourcegroupid) VALUES ({}, {}, {})",
            srcgrpsrcid.to_string(),
            sourceid.to_string(),
            sourcegroupid.to_string(),
        );
        // println!("This is create_srcgrpsrc: {}", parameters);
        parameters
    }

    pub fn read_srcgrpsrc(
        SrcGrpSrc {
            srcgrpsrcid,
            sourceid,
            sourcegroupid,
        }: SrcGrpSrc,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM srcgrpsrc WHERE srcgrpsrcid={}",
            srcgrpsrcid.to_string(),
        );
        // println!("This is read_srcgrpsrc: {}", parameters);
        parameters
    }

    pub fn update_srcgrpsrc(
        SrcGrpSrc {
            srcgrpsrcid,
            sourceid,
            sourcegroupid,
        }: SrcGrpSrc,
    ) -> String {
        let parameters = format!(
            "UPDATE srcgrpsrc SET srcgrpsrcid={}, sourceid={}, sourcegroupid={} WHERE srcgrpsrcid={}",
            srcgrpsrcid.to_string(),
            sourceid.to_string(),
            sourcegroupid.to_string(),
            srcgrpsrcid.to_string(),
        );
        // println!("This is update_srcgrpsrc: {}", parameters);
        parameters
    }

    pub fn delete_srcgrpsrc(
        SrcGrpSrc {
            srcgrpsrcid,
            sourceid,
            sourcegroupid,
        }: SrcGrpSrc,
    ) -> String {
        let parameters = format!(
            "DELETE FROM srcgrpsrc WHERE srcgrpsrcid={}",
            &srcgrpsrcid.to_string(),
        );
        // println!("This is delete_srcgrpsrc: {}", parameters);
        parameters
    }
}
