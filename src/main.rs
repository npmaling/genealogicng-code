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

use crate::activity_test::ActivityTest;
mod activity_test;

use crate::import::search_file_line_by_line;
mod import;

fn main() {
    let conn: &str = "C:/Users/npmal/projects/genealogicng-code/database.db";

    let td1 = ActivityTest {
        activityid: 16,
        projectid: 1,
        researcherid: 1,
        scheddate: "20230101".to_string(),
        completedate: "20230101".to_string(),
        typecode: "a".to_string(),
        status: "a".to_string(),
        description: "First Activity".to_string(),
        priority: "a".to_string(),
        comments: "a".to_string(),
    };

    let td2 = td1.clone();
    let td3: ActivityTest = td1.clone();
    let td4 = td1.clone();
    
    if let Err(err) = ActivityTest::make_activity_a(td1, conn.to_string()) {
        println!("Error: {}", err);
    }

    if let Err(err) = ActivityTest::read_activity_a(td2, conn.to_string()) {
        println!("Error: {}", err);
    }

    if let Err(err) = ActivityTest::update_activity_a(td3, conn.to_string()) {
        println!("Error: {}", err);
    }

    if let Err(err) = ActivityTest::delete_activity_a(td4, conn.to_string()) {
        println!("Error: {}", err);
    }

    search_file_line_by_line("/Users/npmal/projects/glngimport/ged");

} // main
