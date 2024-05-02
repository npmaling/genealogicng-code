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
            sourceid: _,
            sourcegroupid: _,
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
            sourceid: _,
            sourcegroupid: _,
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
