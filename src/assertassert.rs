/*
-- Copyright 2023 N. P. Maling
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
-- http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.
*/

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_assertassert() {
        let assertassert = AssertAssert {
            assertassertid: 1,
            idlo: 100,
            idhi: 200,
            seq: 1,
        };
        let expected_query = "INSERT INTO assertassert (assertassertid, idlo, idhi, seq) VALUES (1, 100, 200, 1)";
        assert_eq!(AssertAssert::create_assertassert(assertassert), expected_query);
    }

    #[test]
    fn test_read_assertassert() {
        let assertassert = AssertAssert {
            assertassertid: 1,
            idlo: 0,
            idhi: 0,
            seq: 0,
        };
        let expected_query = "SELECT * FROM assertassert WHERE assertassertid=1";
        assert_eq!(AssertAssert::read_assertassert(assertassert), expected_query);
    }

    #[test]
    fn test_update_assertassert() {
        let assert_assert = AssertAssert {
            assertassertid: 1,
            idlo: 1,
            idhi: 10,
            seq: 1,
        };
        let expected_query =
            "UPDATE assertassert SET assertassertid=1, idlo=1, idhi=10, seq=1 WHERE assertassertid=1";
        assert_eq!(AssertAssert::update_assertassert(assert_assert), expected_query);
    }

    #[test]
    fn test_delete_assertassert() {
        let assertassert = AssertAssert {
            assertassertid: 1,
            idlo: 2,
            idhi: 3,
            seq: 4,
        };
        let expected_query = "DELETE FROM assertassert WHERE assertassertid=1";
        assert_eq!(AssertAssert::delete_assertassert(assertassert), expected_query);
    }


}
