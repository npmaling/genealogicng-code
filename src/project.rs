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
pub struct Project {
    pub projectid: i64,
    pub name: String,
    pub projectdesc: String,
    pub clientdata: String,
}

impl Project {
    pub fn create_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "INSERT INTO project (projectid, name, projectdesc, clientdata) VALUES ({}, \"{}\", \"{}\", \"{}\")",
            &projectid.to_string(),
            &name,
            &projectdesc,
            &clientdata,
        );
        // println!("This is create_project: {}", parameters);
        parameters
    }

    pub fn read_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "SELECT * FROM project WHERE projectid={}",
            &projectid.to_string(),
        );
        // println!("This is read_project: {}", parameters);
        parameters
    }

    pub fn update_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "UPDATE project SET projectid={}, name=\"{}\", projectdesc=\"{}\", clientdata=\"{}\" WHERE projectid={}",
            &projectid.to_string(),
            &name,
            &projectdesc,
            &clientdata,
            &projectid.to_string(),
        );
        // println!("This is update_project: {}", parameters);
        parameters
    }

    pub fn delete_project(
        Project {
            projectid,
            name,
            projectdesc,
            clientdata,
        }: Project,
    ) -> String {
        let parameters = format!(
            "DELETE FROM project WHERE projectid={}",
            &projectid.to_string(),
        );
        // println!("This is delete_project: {}", parameters);
        parameters
    }
}
