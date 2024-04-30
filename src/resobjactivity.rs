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
pub struct ResObjActivity {
    pub resobjactivityid: i64,
    pub resobjid: i64,
    pub activityid: i64,
}

impl ResObjActivity {
    pub fn create_resobjactivity(
        ResObjActivity {
            resobjactivityid,
            resobjid,
            activityid,
        }: ResObjActivity,
    ) -> String {
        let parameters = format!(
            "INSERT INTO resobjactivity (resobjactivityid, resobjid, activityid) VALUES ({}, {}, {})",
            &resobjactivityid.to_string(),
            &resobjid.to_string(),
            &activityid.to_string(),
        );
        // println!("This is create_resobjactivity: {}", parameters);
        parameters
    }

    pub fn read_resobjactivity(
        ResObjActivity {
            resobjactivityid,
            resobjid,
            activityid,
        }: ResObjActivity,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM resobjactivity WHERE resobjactivityid={}",
            &resobjactivityid.to_string(),
        );
        // println!("This is read_resobjactivity: {}", parameters);
        parameters
    }

    pub fn update_resobjactivity(
        ResObjActivity {
            resobjactivityid,
            resobjid,
            activityid,
        }: ResObjActivity,
    ) -> String {
        let parameters = format!(
            "UPDATE resobjactivity SET resobjactivityid={}, resobjid={}, activityid={} WHERE resobjactivityid={}",
            &resobjactivityid.to_string(),
            &resobjid.to_string(),
            &activityid.to_string(),
            &resobjactivityid.to_string(),
        );
        // println!("This is update_resobjactivity: {}", parameters);
        parameters
    }

    pub fn delete_resobjactivity(
        ResObjActivity {
            resobjactivityid,
            resobjid,
            activityid,
        }: ResObjActivity,
    ) -> String {
        let parameters = format!(
            "DELETE FROM resobjactivity WHERE resobjactivityid={}",
            &resobjactivityid.to_string(),
        );
        // println!("This is delete_resobjactivity: {}", parameters);
        parameters
    }
}

