#[derive(Debug)]
pub struct AssertAssert {
    pub assertassertid: i64,
    pub idlo: i64,
    pub idhi: i64,
    pub seq: i64,
}

impl AssertAssert {
    pub fn create_assertassert(
        AssertAssert {
            assertassertid,
            idlo,
            idhi,
            seq,
        }: AssertAssert,
    ) -> String {
        let parameters = format!(
            "INSERT INTO assertassert (assertassertid, idlo, idhi, seq) VALUES ({}, {}, {}, {})",
            assertassertid.to_string(),
            idlo.to_string(),
            idhi.to_string(),
            seq.to_string(),
        );
        // println!("This is create_assertassert: {}", parameters);
        parameters
    }

    pub fn read_assertassert(
        AssertAssert {
            assertassertid,
            idlo,
            idhi,
            seq,
        }: AssertAssert,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM assertassert WHERE assertassertid={}",
            assertassertid.to_string(),
            //                idlo.to_string(),
            //                idhi.to_string(),
            //                seq.to_string(),
        );
        // println!("This is read_assertassert: {}", parameters);
        parameters
    }

    pub fn update_assertassert(
        AssertAssert {
            assertassertid,
            idlo,
            idhi,
            seq,
        }: AssertAssert,
    ) -> String {
        let parameters = format!(
            "UPDATE assertassert SET assertassertid={}, idlo={}, idhi={}, seq={} WHERE assertassertid={}",
            assertassertid.to_string(),
            idlo.to_string(),
            idhi.to_string(),
            seq.to_string(),
            assertassertid.to_string(),
        );
        // println!("This is update_assertassert: {}", parameters);
        parameters
    }

    pub fn delete_assertassert(
        AssertAssert {
            assertassertid,
            idlo,
            idhi,
            seq,
        }: AssertAssert,
    ) -> String {
        let parameters = format!(
            "DELETE FROM assertassert WHERE assertassertid={}",
            assertassertid.to_string(),
        );
        // println!("This is delete_assertassert: {}", parameters);
        parameters
    }
}
