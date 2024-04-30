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
pub struct ReprMediaType {
    pub reprmediaid: i64,
    pub reprmedianame: String,
}

impl ReprMediaType {
    pub fn create_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "INSERT INTO reprmediatype (reprmediaid, reprmedianame) VALUES ({}, \"{}\")",
            reprmediaid.to_string(),
            reprmedianame,
        );
        // println!("This is create_reprmediatype: {}", parameters);
        parameters
    }

    pub fn read_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM reprmediatype WHERE reprmediaid={}",
            reprmediaid.to_string(),
        );
        // println!("This is read_reprmediatype: {}", parameters);
        parameters
    }

    pub fn update_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "UPDATE reprmediatype SET reprmediaid={}, reprmedianame=\"{}\" WHERE reprmediaid={}",
            reprmediaid.to_string(),
            reprmedianame,
            reprmediaid.to_string(),
        );
        // println!("This is update_reprmediatype: {}", parameters);
        parameters
    }

    pub fn delete_reprmediatype(
        ReprMediaType {
            reprmediaid,
            reprmedianame,
        }: ReprMediaType,
    ) -> String {
        let parameters = format!(
            "DELETE FROM reprmediatype WHERE reprmediaid={}",
            reprmediaid.to_string(),
        );
        // println!("This is delete_reprmediatype: {}", parameters);
        parameters
    }
}
