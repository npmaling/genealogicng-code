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

use crate::import::search_file_line_by_line;
mod import;

fn main() {
//    search_file_line_by_line("/Users/npmal/projects/glngimport/Ancestry-data.ged");
    search_file_line_by_line("/Users/npmal/projects/glngimport/complete.ged");
//    search_file_line_by_line("/Users/npmal/projects/glngimport/FS-20240604_Legacy.ged");
} // main
