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

use genealogicng::make_activity_a;
use genealogicng::read_activity_a;
use genealogicng::update_activity_a;
use genealogicng::delete_activity_a;

use genealogicng::make_assertassert_a;
use genealogicng::read_assertassert_a;
use genealogicng::update_assertassert_a;
use genealogicng::delete_assertassert_a;

use genealogicng::make_characteristic_a;
use genealogicng::read_characteristic_a;
use genealogicng::update_characteristic_a;
use genealogicng::delete_characteristic_a;

use genealogicng::make_charpart_a;
use genealogicng::read_charpart_a;
use genealogicng::update_charpart_a;
use genealogicng::delete_charpart_a;

use genealogicng::make_charparttype_a;
use genealogicng::read_charparttype_a;
use genealogicng::update_charparttype_a;
use genealogicng::delete_charparttype_a;

use genealogicng::make_citationpart_a;
use genealogicng::read_citationpart_a;
use genealogicng::update_citationpart_a;
use genealogicng::delete_citationpart_a;

use genealogicng::make_citationparttype_a;
use genealogicng::read_citationparttype_a;
use genealogicng::update_citationparttype_a;
use genealogicng::delete_citationparttype_a;

use genealogicng::make_event_a;
use genealogicng::read_event_a;
use genealogicng::update_event_a;
use genealogicng::delete_event_a;

use genealogicng::make_eventtype_a;
use genealogicng::read_eventtype_a;
use genealogicng::update_eventtype_a;
use genealogicng::delete_eventtype_a;

use genealogicng::make_eventtyperole_a;
use genealogicng::read_eventtyperole_a;
use genealogicng::update_eventtyperole_a;
use genealogicng::delete_eventtyperole_a;

use genealogicng::make_glassertion_a;
use genealogicng::read_glassertion_a;
use genealogicng::update_glassertion_a;
use genealogicng::delete_glassertion_a;

use genealogicng::make_glgroup_a;
use genealogicng::read_glgroup_a;
use genealogicng::update_glgroup_a;
use genealogicng::delete_glgroup_a;

use genealogicng::make_glgrouptype_a;
use genealogicng::read_glgrouptype_a;
use genealogicng::update_glgrouptype_a;
use genealogicng::delete_glgrouptype_a;

use genealogicng::make_glgrouptyperole_a;
use genealogicng::read_glgrouptyperole_a;
use genealogicng::update_glgrouptyperole_a;
use genealogicng::delete_glgrouptyperole_a;

use genealogicng::make_persona_a;
use genealogicng::read_persona_a;
use genealogicng::update_persona_a;
use genealogicng::delete_persona_a;

use genealogicng::make_place_a;
use genealogicng::read_place_a;
use genealogicng::update_place_a;
use genealogicng::delete_place_a;

use genealogicng::make_placepart_a;
use genealogicng::read_placepart_a;
use genealogicng::update_placepart_a;
use genealogicng::delete_placepart_a;

use genealogicng::make_placeparttype_a;
use genealogicng::read_placeparttype_a;
use genealogicng::update_placeparttype_a;
use genealogicng::delete_placeparttype_a;

use genealogicng::make_project_a;
use genealogicng::read_project_a;
use genealogicng::update_project_a;
use genealogicng::delete_project_a;

use genealogicng::make_repository_a;
use genealogicng::read_repository_a;
use genealogicng::update_repository_a;
use genealogicng::delete_repository_a;

use genealogicng::make_reposource_a;
use genealogicng::read_reposource_a;
use genealogicng::update_reposource_a;
use genealogicng::delete_reposource_a;

use genealogicng::make_representation_a;
use genealogicng::read_representation_a;
use genealogicng::update_representation_a;
use genealogicng::delete_representation_a;

use genealogicng::make_representtype_a;
use genealogicng::read_representtype_a;
use genealogicng::update_representtype_a;
use genealogicng::delete_representtype_a;

use genealogicng::make_reprmediatype_a;
use genealogicng::read_reprmediatype_a;
use genealogicng::update_reprmediatype_a;
use genealogicng::delete_reprmediatype_a;

use genealogicng::make_researcher_a;
use genealogicng::read_researcher_a;
use genealogicng::update_researcher_a;
use genealogicng::delete_researcher_a;

use genealogicng::make_resobjactivity_a;
use genealogicng::read_resobjactivity_a;
use genealogicng::update_resobjactivity_a;
use genealogicng::delete_resobjactivity_a;

use genealogicng::make_resobjective_a;
use genealogicng::read_resobjective_a;
use genealogicng::update_resobjective_a;
use genealogicng::delete_resobjective_a;

use genealogicng::make_resproj_a;
use genealogicng::read_resproj_a;
use genealogicng::update_resproj_a;
use genealogicng::delete_resproj_a;

use genealogicng::make_search_a;
use genealogicng::read_search_a;
use genealogicng::update_search_a;
use genealogicng::delete_search_a;

use genealogicng::make_source_a;
use genealogicng::read_source_a;
use genealogicng::update_source_a;
use genealogicng::delete_source_a;

use genealogicng::make_sourcegroup_a;
use genealogicng::read_sourcegroup_a;
use genealogicng::update_sourcegroup_a;
use genealogicng::delete_sourcegroup_a;

use genealogicng::make_srcgrpsrc_a;
use genealogicng::read_srcgrpsrc_a;
use genealogicng::update_srcgrpsrc_a;
use genealogicng::delete_srcgrpsrc_a;

use genealogicng::make_suretypart_a;
use genealogicng::read_suretypart_a;
use genealogicng::update_suretypart_a;
use genealogicng::delete_suretypart_a;

use genealogicng::make_suretyscheme_a;
use genealogicng::read_suretyscheme_a;
use genealogicng::update_suretyscheme_a;
use genealogicng::delete_suretyscheme_a;

fn main() {
    let _ = make_activity_a();
    let _ = read_activity_a();
    let _ = update_activity_a();
    let _ = delete_activity_a();

    let _ = make_assertassert_a();
    let _ = read_assertassert_a();
    let _ = update_assertassert_a();
    let _ = delete_assertassert_a();

    let _ = make_characteristic_a();
    let _ = read_characteristic_a();
    let _ = update_characteristic_a();
    let _ = delete_characteristic_a();

    let _ = make_charpart_a();
    let _ = read_charpart_a();
    let _ = update_charpart_a();
    let _ = delete_charpart_a();

    let _ = make_charparttype_a();
    let _ = read_charparttype_a();
    let _ = update_charparttype_a();
    let _ = delete_charparttype_a();

    let _ = make_citationpart_a();
    let _ = read_citationpart_a();
    let _ = update_citationpart_a();
    let _ = delete_citationpart_a();

    let _ = make_citationparttype_a();
    let _ = read_citationparttype_a();
    let _ = update_citationparttype_a();
    let _ = delete_citationparttype_a();

    let _ = make_event_a();
    let _ = read_event_a();
    let _ = update_event_a();
    let _ = delete_event_a();

    let _ = make_eventtype_a();
    let _ = read_eventtype_a();
    let _ = update_eventtype_a();
    let _ = delete_eventtype_a();

    let _ = make_eventtyperole_a();
    let _ = read_eventtyperole_a();
    let _ = update_eventtyperole_a();
    let _ = delete_eventtyperole_a();

    let _ = make_glassertion_a();
    let _ = read_glassertion_a();
    let _ = update_glassertion_a();
    let _ = delete_glassertion_a();

    let _ = make_glgroup_a();
    let _ = read_glgroup_a();
    let _ = update_glgroup_a();
    let _ = delete_glgroup_a();

    let _ = make_glgrouptype_a();
    let _ = read_glgrouptype_a();
    let _ = update_glgrouptype_a();
    let _ = delete_glgrouptype_a();

    let _ = make_glgrouptyperole_a();
    let _ = read_glgrouptyperole_a();
    let _ = update_glgrouptyperole_a();
    let _ = delete_glgrouptyperole_a();

    let _ = make_persona_a();
    let _ = read_persona_a();
    let _ = update_persona_a();
    let _ = delete_persona_a();

    let _ = make_place_a();
    let _ = read_place_a();
    let _ = update_place_a();
    let _ = delete_place_a();

    let _ = make_placepart_a();
    let _ = read_placepart_a();
    let _ = update_placepart_a();
    let _ = delete_placepart_a();

    let _ = make_placeparttype_a();
    let _ = read_placeparttype_a();
    let _ = update_placeparttype_a();
    let _ = delete_placeparttype_a();

    let _ = make_project_a();
    let _ = read_project_a();
    let _ = update_project_a();
    let _ = delete_project_a();

    let _ = make_repository_a();
    let _ = read_repository_a();
    let _ = update_repository_a();
    let _ = delete_repository_a();

    let _ = make_reposource_a();
    let _ = read_reposource_a();
    let _ = update_reposource_a();
    let _ = delete_reposource_a();

    let _ = make_representation_a();
    let _ = read_representation_a();
    let _ = update_representation_a();
    let _ = delete_representation_a();

    let _ = make_representtype_a();
    let _ = read_representtype_a();
    let _ = update_representtype_a();
    let _ = delete_representtype_a();

    let _ = make_reprmediatype_a();
    let _ = read_reprmediatype_a();
    let _ = update_reprmediatype_a();
    let _ = delete_reprmediatype_a();

    let _ = make_researcher_a();
    let _ = read_researcher_a();
    let _ = update_researcher_a();
    let _ = delete_researcher_a();

    let _ = make_resobjactivity_a();
    let _ = read_resobjactivity_a();
    let _ = update_resobjactivity_a();
    let _ = delete_resobjactivity_a();

    let _ = make_resobjective_a();
    let _ = read_resobjective_a();
    let _ = update_resobjective_a();
    let _ = delete_resobjective_a();

    let _ = make_resproj_a();
    let _ = read_resproj_a();
    let _ = update_resproj_a();
    let _ = delete_resproj_a();

    let _ = make_search_a();
    let _ = read_search_a();
    let _ = update_search_a();
    let _ = delete_search_a();

    let _ = make_source_a();
    let _ = read_source_a();
    let _ = update_source_a();
    let _ = delete_source_a();

    let _ = make_sourcegroup_a();
    let _ = read_sourcegroup_a();
    let _ = update_sourcegroup_a();
    let _ = delete_sourcegroup_a();

    let _ = make_srcgrpsrc_a();
    let _ = read_srcgrpsrc_a();
    let _ = update_srcgrpsrc_a();
    let _ = delete_srcgrpsrc_a();

    let _ = make_suretypart_a();
    let _ = read_suretypart_a();
    let _ = update_suretypart_a();
    let _ = delete_suretypart_a();

    let _ = make_suretyscheme_a();
    let _ = read_suretyscheme_a();
    let _ = update_suretyscheme_a();
    let _ = delete_suretyscheme_a();    

}
