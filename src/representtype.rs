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
pub struct RepresentType {
    pub reprtypeid: i64,
    pub name: String,
}

impl RepresentType {
    pub fn create_representtype(RepresentType { reprtypeid, name }: RepresentType) -> String {
        let parameters = format!(
            "INSERT INTO representtype (reprtypeid, name) VALUES ({}, \"{}\")",
            reprtypeid.to_string(),
            name,
        );
        // println!("This is create_representtype: {}", parameters);
        parameters
    }

    pub fn read_representtype(RepresentType { reprtypeid, name: _ }: RepresentType) -> String {
        let parameters = format!(
            "SELECT * FROM representtype WHERE reprtypeid={}",
            reprtypeid.to_string(),
            // name: _,
        );
        // println!("This is read_representtype: {}", parameters);
        parameters
    }

    pub fn update_representtype(RepresentType { reprtypeid, name }: RepresentType) -> String {
        let parameters = format!(
            "UPDATE representtype SET reprtypeid={}, name=\"{}\" WHERE reprtypeid={}",
            reprtypeid.to_string(),
            name,
            reprtypeid.to_string(),
        );
        // println!("This is update_representtype: {}", parameters);
        parameters
    }

    pub fn delete_representtype(RepresentType { reprtypeid, name: _ }: RepresentType) -> String {
        let parameters = format!(
            "DELETE FROM representtype WHERE reprtypeid={}",
            reprtypeid.to_string(),
            // name: _,
        );
        // println!("This is delete_representtype: {}", parameters);
        parameters
    }
}
