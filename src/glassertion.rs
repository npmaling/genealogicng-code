#[derive(Debug)]
pub struct GlAssertion {
    pub glassertionid: i64,
    pub suretypartid: i64,
    pub researcherid: i64,
    pub sourceid: i64,
    pub subject1id: i64,
    pub subject1type: String,
    pub subject2id: i64,
    pub subject2type: String,
    pub value_role: i64,
    pub disproved: String,
    pub rationale: String,
}

impl GlAssertion {
    pub fn create_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid,
            researcherid,
            sourceid,
            subject1id,
            subject1type,
            subject2id,
            subject2type,
            value_role,
            disproved,
            rationale,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "INSERT INTO glassertion (glassertionid, suretypartid, researcherid, sourceid, subject1id, subject1type, subject2id, subject2type, value_role, disproved, rationale) VALUES ({}, {}, {}, {}, {}, \"{}\", {}, \"{}\", {}, \"{}\", \"{}\")",
            glassertionid.to_string(),
            suretypartid.to_string(),
            researcherid.to_string(),
            sourceid.to_string(),
            subject1id.to_string(),
            subject1type,
            subject2id.to_string(),
            subject2type,
            value_role.to_string(),
            disproved,
            rationale,
        );
        // println!("This is create_glassertion: {}", parameters);
        parameters
    }

    pub fn read_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid,
            researcherid,
            sourceid,
            subject1id,
            subject1type,
            subject2id,
            subject2type,
            value_role,
            disproved,
            rationale,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM glassertion WHERE glassertionid={}",
            glassertionid.to_string(),
        );
        // println!("This is read_glassertion: {}", parameters);
        parameters
    }

    pub fn update_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid,
            researcherid,
            sourceid,
            subject1id,
            subject1type,
            subject2id,
            subject2type,
            value_role,
            disproved,
            rationale,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "UPDATE glassertion SET glassertionid={}, suretypartid={}, researcherid={}, sourceid={}, subject1id={}, subject1type=\"{}\", subject2id={}, subject2type=\"{}\", value_role={}, disproved=\"{}\", rationale=\"{}\" WHERE glassertionid={}",
            glassertionid.to_string(),
            suretypartid.to_string(),
            researcherid.to_string(),
            sourceid.to_string(),
            subject1id.to_string(),
            subject1type,
            subject2id.to_string(),
            subject2type,
            value_role.to_string(),
            disproved,
            rationale,
            glassertionid.to_string(),
        );
        // println!("This is update_glassertion: {}", parameters);
        parameters
    }

    pub fn delete_glassertion(
        GlAssertion {
            glassertionid,
            suretypartid,
            researcherid,
            sourceid,
            subject1id,
            subject1type,
            subject2id,
            subject2type,
            value_role,
            disproved,
            rationale,
        }: GlAssertion,
    ) -> String {
        let parameters = format!(
            "DELETE FROM glassertion WHERE glassertionid={}",
            glassertionid.to_string(),
        );
        // println!("This is delete_glassertion: {}", parameters);
        parameters
    }
}
