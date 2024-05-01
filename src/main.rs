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

use crate::researcher::Researcher;
mod researcher;

use crate::representation::Representation;
mod representation;

use crate::representtype::RepresentType;
mod representtype;

use crate::reprmediatype::ReprMediaType;
mod reprmediatype;

use crate::resobjactivity::ResObjActivity;
mod resobjactivity;

use crate::resobjective::ResObjective;
mod resobjective;

use crate::resproj::ResProj;
mod resproj;

use crate::search::Search;
mod search;

use crate::source::Source;
mod source;

use crate::sourcegroup::SourceGroup;
mod sourcegroup;

use crate::srcgrpsrc::SrcGrpSrc;
mod srcgrpsrc;

use crate::suretyscheme::SuretyScheme;
mod suretyscheme;

use crate::suretypart::SuretyPart;
mod suretypart;

use rusqlite::{params, Connection, Result};

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

fn main() -> Result<()> {
    let path: &str = "C:/Users/npmal/projects/genealogicng-code/database.db";
    let conn: Connection = Connection::open(&path)?;

    /* ------------------------------------------------------------------------- */

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

    /* ------------------------------------------------------------------------- */

    let pp_a = ResObjective {
        resobjid: 1,
        projectid: 1,
        subjectid: 1,
        subjecttype: "Subject Type One".to_string(),
        name: "Name One".to_string(),
        description: "Description One".to_string(),
        sequencenumber: 1,
        priority: "Priority One".to_string(),
        status: "Status One".to_string(),
    };

    let pp_b = ResObjective {
        resobjid: 1,
        projectid: 1,
        subjectid: 1,
        subjecttype: "Subject Type Two".to_string(),
        name: "Name Two".to_string(),
        description: "Description Two".to_string(),
        sequencenumber: 1,
        priority: "Priority Two".to_string(),
        status: "Status Two".to_string(),
    };

    let pp_c = ResObjective {
        resobjid: 1,
        projectid: 1,
        subjectid: 1,
        subjecttype: "Subject Type 3".to_string(),
        name: "Name 3".to_string(),
        description: "Description 3".to_string(),
        sequencenumber: 1,
        priority: "Priority 3".to_string(),
        status: "Status 3".to_string(),
    };

    let pp_d = ResObjective {
        resobjid: 1,
        projectid: 1,
        subjectid: 1,
        subjecttype: "Subject Type 4".to_string(),
        name: "Name 4".to_string(),
        description: "Description 4".to_string(),
        sequencenumber: 1,
        priority: "Priority 4".to_string(),
        status: "Status 4".to_string(),
    };

    let app = ResObjective::create_resobjective(pp_a);

    dbstring(&conn, app);

    let bpp = ResObjective::read_resobjective(pp_b);

    dbstring(&conn, bpp);

    let cpp = ResObjective::update_resobjective(pp_c);

    dbstring(&conn, cpp);

    let dpp = ResObjective::delete_resobjective(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = ResObjActivity {
        resobjactivityid: 1,
        resobjid: 2,
        activityid: 3,
    };

    let pp_b = ResObjActivity {
        resobjactivityid: 4,
        resobjid: 5,
        activityid: 6,
    };

    let pp_c = ResObjActivity {
        resobjactivityid: 7,
        resobjid: 8,
        activityid: 9,
    };

    let pp_d = ResObjActivity {
        resobjactivityid: 10,
        resobjid: 11,
        activityid: 12,
    };

    let app = ResObjActivity::create_resobjactivity(pp_a);

    dbstring(&conn, app);

    let bpp = ResObjActivity::read_resobjactivity(pp_b);

    dbstring(&conn, bpp);

    let cpp = ResObjActivity::update_resobjactivity(pp_c);

    dbstring(&conn, cpp);

    let dpp = ResObjActivity::delete_resobjactivity(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Researcher {
        researcherid: 3,
        name: "First Middle Last".to_string(),
        addressid: 4,
        comments: "Researcher Comment One".to_string(),
    };

    let pp_b = Researcher {
        researcherid: 5,
        name: "First Middle Last".to_string(),
        addressid: 6,
        comments: "Researcher Comment Two".to_string(),
    };

    let pp_c = Researcher {
        researcherid: 7,
        name: "First Middle Last".to_string(),
        addressid: 8,
        comments: "Researcher Comment Three".to_string(),
    };

    let pp_d = Researcher {
        researcherid: 9,
        name: "First Middle Last".to_string(),
        addressid: 10,
        comments: "Researcher Comment Four".to_string(),
    };

    let app = Researcher::create_researcher(pp_a);

    dbstring(&conn, app);

    let bpp = Researcher::read_researcher(pp_b);

    dbstring(&conn, bpp);

    let cpp = Researcher::update_researcher(pp_c);

    dbstring(&conn, cpp);

    let dpp = Researcher::delete_researcher(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = ResProj {
        resprojid: 1,
        projectid: 2,
        researcherid: 3,
        researcherrole: "Researcher Role Note One".to_string(),
    };

    let pp_b = ResProj {
        resprojid: 4,
        projectid: 5,
        researcherid: 6,
        researcherrole: "Researcher Role Note Two".to_string(),
    };

    let pp_c = ResProj {
        resprojid: 7,
        projectid: 8,
        researcherid: 9,
        researcherrole: "Researcher Role Note Three".to_string(),
    };

    let pp_d = ResProj {
        resprojid: 10,
        projectid: 11,
        researcherid: 12,
        researcherrole: "Researcher Role Note Four".to_string(),
    };

    let app = ResProj::create_resproj(pp_a);

    dbstring(&conn, app);

    let bpp = ResProj::read_resproj(pp_b);

    dbstring(&conn, bpp);

    let cpp = ResProj::update_resproj(pp_c);

    dbstring(&conn, cpp);

    let dpp = ResProj::delete_resproj(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Source {
        sourceid: 1,
        highersourceid: 1,
        subjectplaceid: 1,
        jurisplaceid: 1,
        researcherid: 1,
        subjectdate: "20230403".to_string(),
        comments: "Source Comments One".to_string(),
    };

    let pp_b = Source {
        sourceid: 2,
        highersourceid: 2,
        subjectplaceid: 2,
        jurisplaceid: 2,
        researcherid: 2,
        subjectdate: "20230403".to_string(),
        comments: "Source Comments Two".to_string(),
    };

    let pp_c = Source {
        sourceid: 3,
        highersourceid: 3,
        subjectplaceid: 3,
        jurisplaceid: 3,
        researcherid: 3,
        subjectdate: "20230403".to_string(),
        comments: "Source Comments Three".to_string(),
    };

    let pp_d = Source {
        sourceid: 4,
        highersourceid: 4,
        subjectplaceid: 4,
        jurisplaceid: 4,
        researcherid: 4,
        subjectdate: "20230403".to_string(),
        comments: "Source Comments Four".to_string(),
    };

    let app = Source::create_source(pp_a);

    dbstring(&conn, app);

    let bpp = Source::read_source(pp_b);

    dbstring(&conn, bpp);

    let cpp = Source::update_source(pp_c);

    dbstring(&conn, cpp);

    let dpp = Source::delete_source(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = SourceGroup {
        sourcegroupid: 1,
        sourcegroupname: "Source Group Name One".to_string(),
    };

    let pp_b = SourceGroup {
        sourcegroupid: 1,
        sourcegroupname: "Source Group Name One".to_string(),
    };

    let pp_c = SourceGroup {
        sourcegroupid: 1,
        sourcegroupname: "Source Group Name One".to_string(),
    };

    let pp_d = SourceGroup {
        sourcegroupid: 1,
        sourcegroupname: "Source Group Name One".to_string(),
    };

    let app = SourceGroup::create_sourcegroup(pp_a);

    dbstring(&conn, app);

    let bpp = SourceGroup::read_sourcegroup(pp_b);

    dbstring(&conn, bpp);

    let cpp = SourceGroup::update_sourcegroup(pp_c);

    dbstring(&conn, cpp);

    let dpp = SourceGroup::delete_sourcegroup(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = SrcGrpSrc {
        srcgrpsrcid: 1,
        sourceid: 1,
        sourcegroupid: 1,
    };

    let pp_b = SrcGrpSrc {
        srcgrpsrcid: 2,
        sourceid: 2,
        sourcegroupid: 2,
    };

    let pp_c = SrcGrpSrc {
        srcgrpsrcid: 3,
        sourceid: 3,
        sourcegroupid: 3,
    };

    let pp_d = SrcGrpSrc {
        srcgrpsrcid: 4,
        sourceid: 4,
        sourcegroupid: 4,
    };

    let app = SrcGrpSrc::create_srcgrpsrc(pp_a);

    dbstring(&conn, app);

    let bpp = SrcGrpSrc::read_srcgrpsrc(pp_b);

    dbstring(&conn, bpp);

    let cpp = SrcGrpSrc::update_srcgrpsrc(pp_c);

    dbstring(&conn, cpp);

    let dpp = SrcGrpSrc::delete_srcgrpsrc(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = SuretyScheme {
        suretyschemeid: 1,
        name: "Surety Scheme One".to_string(),
        description: "Description of Surety Scheme One".to_string(),
    };

    let pp_b = SuretyScheme {
        suretyschemeid: 1,
        name: "Surety Scheme Two".to_string(),
        description: "Description of Surety Scheme Two".to_string(),
    };

    let pp_c = SuretyScheme {
        suretyschemeid: 1,
        name: "Surety Scheme Three".to_string(),
        description: "Description of Surety Scheme Three".to_string(),
    };

    let pp_d = SuretyScheme {
        suretyschemeid: 1,
        name: "Surety Scheme Four".to_string(),
        description: "Description of Surety Scheme Four".to_string(),
    };

    let app = SuretyScheme::create_suretyscheme(pp_a);

    dbstring(&conn, app);

    let bpp = SuretyScheme::read_suretyscheme(pp_b);

    dbstring(&conn, bpp);

    let cpp = SuretyScheme::update_suretyscheme(pp_c);

    dbstring(&conn, cpp);

    let dpp = SuretyScheme::delete_suretyscheme(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = SuretyPart {
        suretypartid: 1,
        schemeid: 1,
        name: "Surety Part One".to_string(),
        description: "Surety Part One".to_string(),
        sequencenumber: 1,
    };

    let pp_b = SuretyPart {
        suretypartid: 2,
        schemeid: 2,
        name: "Surety Part One".to_string(),
        description: "Surety Part One".to_string(),
        sequencenumber: 2,
    };

    let pp_c = SuretyPart {
        suretypartid: 3,
        schemeid: 3,
        name: "Surety Part One".to_string(),
        description: "Surety Part One".to_string(),
        sequencenumber: 3,
    };

    let pp_d = SuretyPart {
        suretypartid: 4,
        schemeid: 4,
        name: "Surety Part One".to_string(),
        description: "Surety Part One".to_string(),
        sequencenumber: 4,
    };

    let app = SuretyPart::create_suretypart(pp_a);

    dbstring(&conn, app);

    let bpp = SuretyPart::read_suretypart(pp_b);

    dbstring(&conn, bpp);

    let cpp = SuretyPart::update_suretypart(pp_c);

    dbstring(&conn, cpp);

    let dpp = SuretyPart::delete_suretypart(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = ReprMediaType {
        reprmediaid: 1,
        reprmedianame: "Representative Media Type Name One".to_string(),
    };

    let pp_b = ReprMediaType {
        reprmediaid: 2,
        reprmedianame: "Representative Media Type Name Two".to_string(),
    };

    let pp_c = ReprMediaType {
        reprmediaid: 3,
        reprmedianame: "Representative Media Type Name Three".to_string(),
    };

    let pp_d = ReprMediaType {
        reprmediaid: 4,
        reprmedianame: "Representative Media Type Name Four".to_string(),
    };

    let app = ReprMediaType::create_reprmediatype(pp_a);

    dbstring(&conn, app);

    let bpp = ReprMediaType::read_reprmediatype(pp_b);

    dbstring(&conn, bpp);

    let cpp = ReprMediaType::update_reprmediatype(pp_c);

    dbstring(&conn, cpp);

    let dpp = ReprMediaType::delete_reprmediatype(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = RepresentType {
        reprtypeid: 1,
        name: "Represent Name One".to_string(),
    };

    let pp_b = RepresentType {
        reprtypeid: 2,
        name: "Represent Name Two".to_string(),
    };

    let pp_c = RepresentType {
        reprtypeid: 3,
        name: "Represent Name Three".to_string(),
    };

    let pp_d = RepresentType {
        reprtypeid: 4,
        name: "Represent Name Four".to_string(),
    };

    let app = RepresentType::create_representtype(pp_a);

    dbstring(&conn, app);

    let bpp = RepresentType::read_representtype(pp_b);

    dbstring(&conn, bpp);

    let cpp = RepresentType::update_representtype(pp_c);

    dbstring(&conn, cpp);

    let dpp = RepresentType::delete_representtype(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Representation {
        representationid: 1,
        sourceid: 1,
        reprtypeid: 1,
        reprmediaid: 1,
        physfilecode: "Physical File Code One".to_string(),
        comments: "Comments One".to_string(),
        externallink: "External Link One".to_string(),
    };

    let pp_b = Representation {
        representationid: 2,
        sourceid: 2,
        reprtypeid: 2,
        reprmediaid: 2,
        physfilecode: "Physical File Code Two".to_string(),
        comments: "Comments Two".to_string(),
        externallink: "External Link Two".to_string(),
    };

    let pp_c = Representation {
        representationid: 3,
        sourceid: 3,
        reprtypeid: 3,
        reprmediaid: 3,
        physfilecode: "Physical File Code Three".to_string(),
        comments: "Comments Three".to_string(),
        externallink: "External Link Three".to_string(),
    };

    let pp_d = Representation {
        representationid: 4,
        sourceid: 4,
        reprtypeid: 4,
        reprmediaid: 4,
        physfilecode: "Physical File Code Four".to_string(),
        comments: "Comments Four".to_string(),
        externallink: "External Link Four".to_string(),
    };

    let app = Representation::create_representation(pp_a);

    dbstring(&conn, app);

    let bpp = Representation::read_representation(pp_b);

    dbstring(&conn, bpp);

    let cpp = Representation::update_representation(pp_c);

    dbstring(&conn, cpp);

    let dpp = Representation::delete_representation(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Search {
        searchid: 1,
        activityid: 1,
        sourceid: 1,
        repositoryid: 1,
        searchedfor: "Searched For One".to_string(),
    };

    let pp_b = Search {
        searchid: 2,
        activityid: 2,
        sourceid: 2,
        repositoryid: 2,
        searchedfor: "Searched For Two".to_string(),
    };

    let pp_c = Search {
        searchid: 3,
        activityid: 3,
        sourceid: 3,
        repositoryid: 3,
        searchedfor: "Searched For Three".to_string(),
    };

    let pp_d = Search {
        searchid: 4,
        activityid: 4,
        sourceid: 4,
        repositoryid: 4,
        searchedfor: "Searched For Four".to_string(),
    };

    let app = Search::create_search(pp_a);

    dbstring(&conn, app);

    let bpp = Search::read_search(pp_b);

    dbstring(&conn, bpp);

    let cpp = Search::update_search(pp_c);

    dbstring(&conn, cpp);

    let dpp = Search::delete_search(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    fn dbstring(conn: &Connection, dbstr: String) {
        match conn.execute(&dbstr, params![]) {
            Ok(updated) => println!("{} rows were updated by match", updated),
            Err(err) => println!("update failed: {}", err),
        };
    }

    Ok(())

}
