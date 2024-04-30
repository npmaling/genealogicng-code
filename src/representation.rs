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
pub struct Representation {
    pub representationid: i64,
    pub sourceid: i64,
    pub reprtypeid: i64,
    pub reprmediaid: i64,
    pub physfilecode: String,
    pub comments: String,
    pub externallink: String,
}

impl Representation {
    pub fn create_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "INSERT INTO representation (representationid, sourceid, reprtypeid, reprmediaid, physfilecode, comments, externallink) VALUES ({}, {}, {}, {}, \"{}\", \"{}\", \"{}\")",
            representationid.to_string(),
            sourceid.to_string(),
            reprtypeid.to_string(),
            reprmediaid.to_string(),
            physfilecode,
            comments,
            externallink,
        );
        // println!("This is create_representation: {}", parameters);
        parameters
    }

    pub fn read_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM representation WHERE representationid={}",
            representationid.to_string(),
        );
        // println!("This is read_representation: {}", parameters);
        parameters
    }

    pub fn update_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "UPDATE representation SET representationid={}, sourceid={}, reprtypeid={}, reprmediaid={}, physfilecode=\"{}\", comments=\"{}\", externallink=\"{}\" WHERE reprtypeid={}",
            representationid.to_string(),
            sourceid.to_string(),
            reprtypeid.to_string(),
            reprmediaid.to_string(),
            physfilecode,
            comments,
            externallink,
            representationid.to_string(),
        );
        // println!("This is update_representation: {}", parameters);
        parameters
    }

    pub fn delete_representation(
        Representation {
            representationid,
            sourceid,
            reprtypeid,
            reprmediaid,
            physfilecode,
            comments,
            externallink,
        }: Representation,
    ) -> String {
        let parameters = format!(
            "DELETE FROM representation WHERE representationid={}",
            representationid.to_string(),
        );
        // println!("This is delete_representation: {}", parameters);
        parameters
    }
}
