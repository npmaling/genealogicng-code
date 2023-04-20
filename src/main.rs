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

use crate::activity::Activity;
mod activity;

use crate::glgroup::GlGroup;
mod glgroup;

use crate::glgrouptype::GlGroupType;
mod glgrouptype;

use crate::glgrouptyperole::GlGroupTypeRole;
mod glgrouptyperole;

use crate::persona::Persona;
mod persona;

use crate::place::Place;
mod place;

use crate::placepart::PlacePart;
mod placepart;

use crate::placeparttype::PlacePartType;
mod placeparttype;

use crate::project::Project;
mod project;

use crate::researcher::Researcher;
mod researcher;

use crate::repository::Repository;
mod repository;

use crate::reposource::RepoSource;
mod reposource;

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

fn main() -> Result<()> {
    let path = "C:/Users/npmal/projects/genealogicng-code/glNG.db";
    let conn = Connection::open(&path)?;

    /* ------------------------------------------------------------------------- */

    let place_a = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "a".to_string(),
        placecomment: "First Place".to_string(),
    };

    let aplace = Place::create_place(place_a);

    dbstring(&conn, aplace);

    let place_b = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "a".to_string(),
        placecomment: "First Place".to_string(),
    };

    let bplace = Place::read_place(place_b);

    dbstring(&conn, bplace);

    let place_c = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "b".to_string(),
        placecomment: "Second Place".to_string(),
    };

    let cplace = Place::update_place(place_c);

    dbstring(&conn, cplace);

    let place_d = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "b".to_string(),
        placecomment: "Second Place".to_string(),
    };

    let dplace = Place::delete_place(place_d);

    dbstring(&conn, dplace);

    /* ------------------------------------------------------------------------- */

    let ppt_a = PlacePartType {
        placeparttypeid: 16,
        pptname: "Second Part".to_string(),
    };

    let appt = PlacePartType::create_placeparttype(ppt_a);

    dbstring(&conn, appt);

    let ppt_b = PlacePartType {
        placeparttypeid: 16,
        pptname: "Second Part".to_string(),
    };

    let bppt = PlacePartType::read_placeparttype(ppt_b);

    dbstring(&conn, bppt);

    let ppt_c = PlacePartType {
        placeparttypeid: 16,
        pptname: "Third Part".to_string(),
    };

    let cppt = PlacePartType::update_placeparttype(ppt_c);

    dbstring(&conn, cppt);

    let ppt_d = PlacePartType {
        placeparttypeid: 16,
        pptname: "Third Part".to_string(),
    };

    let dppt = PlacePartType::delete_placeparttype(ppt_d);

    dbstring(&conn, dppt);

    /* ------------------------------------------------------------------------- */

    let pp_a = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let appt = PlacePart::create_placepart(pp_a);

    dbstring(&conn, appt);

    let pp_b = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let bpp = PlacePart::read_placepart(pp_b);

    dbstring(&conn, bpp);

    let pp_c = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let cpp = PlacePart::update_placepart(pp_c);

    dbstring(&conn, cpp);

    let pp_d = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let dpp = PlacePart::delete_placepart(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Project {
        projectid: 1,
        name: "Project Name One".to_string(),
        projectdesc: "Project Description".to_string(),
        clientdata: "Client Data".to_string(),
    };

    let pp_b = Project {
        projectid: 2,
        name: "Project Name Two".to_string(),
        projectdesc: "Project Description".to_string(),
        clientdata: "Client Data".to_string(),
    };

    let pp_c = Project {
        projectid: 3,
        name: "Project Name Three".to_string(),
        projectdesc: "Project Description".to_string(),
        clientdata: "Client Data".to_string(),
    };

    let pp_d = Project {
        projectid: 4,
        name: "Project Name Four".to_string(),
        projectdesc: "Project Description".to_string(),
        clientdata: "Client Data".to_string(),
    };

    let app = Project::create_project(pp_a);

    dbstring(&conn, app);

    let bpp = Project::read_project(pp_b);

    dbstring(&conn, bpp);

    let cpp = Project::update_project(pp_c);

    dbstring(&conn, cpp);

    let dpp = Project::delete_project(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Repository {
        repositoryid: 1,
        placeid: 1,
        reponame: "Repository One Name".to_string(),
        comments: "Repository Comment Data".to_string(),
    };

    let pp_b = Repository {
        repositoryid: 2,
        placeid: 2,
        reponame: "Repository Two Name".to_string(),
        comments: "Repository Comment Data".to_string(),
    };

    let pp_c = Repository {
        repositoryid: 3,
        placeid: 3,
        reponame: "Repository Three Name".to_string(),
        comments: "Repository Comment Data".to_string(),
    };

    let pp_d = Repository {
        repositoryid: 4,
        placeid: 4,
        reponame: "Repository Four Name".to_string(),
        comments: "Repository Comment Data".to_string(),
    };

    let app = Repository::create_repository(pp_a);

    dbstring(&conn, app);

    let bpp = Repository::read_repository(pp_b);

    dbstring(&conn, bpp);

    let cpp = Repository::update_repository(pp_c);

    dbstring(&conn, cpp);

    let dpp = Repository::delete_repository(pp_d);

    dbstring(&conn, dpp);

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

    /* ------------------------------------------------------------------------- */

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

    let pp_a = GlGroupType {
        glgrouptypeid: 1,
        glgroupname: "GlGroupNameOne".to_string(),
        ascdescnone: "Asc".to_string(),
    };

    let pp_b = GlGroupType {
        glgrouptypeid: 2,
        glgroupname: "GlGroupNameTwo".to_string(),
        ascdescnone: "Asc".to_string(),
    };

    let pp_c = GlGroupType {
        glgrouptypeid: 3,
        glgroupname: "GlGroupNameThree".to_string(),
        ascdescnone: "Asc".to_string(),
    };

    let pp_d = GlGroupType {
        glgrouptypeid: 4,
        glgroupname: "GlGroupNameFour".to_string(),
        ascdescnone: "Asc".to_string(),
    };

    let app = GlGroupType::create_glgrouptype(pp_a);

    dbstring(&conn, app);

    let bpp = GlGroupType::read_glgrouptype(pp_b);

    dbstring(&conn, bpp);

    let cpp = GlGroupType::update_glgrouptype(pp_c);

    dbstring(&conn, cpp);

    let dpp = GlGroupType::delete_glgrouptype(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = GlGroup {
        glgroupid: 1,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230405".to_string(),
        glgroupname: "Gl Group Name One".to_string(),
        glgroupcriteria: "Gl Group Criteria One".to_string(),
    };

    let pp_b = GlGroup {
        glgroupid: 1,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230405".to_string(),
        glgroupname: "Gl Group Name One".to_string(),
        glgroupcriteria: "Gl Group Criteria One".to_string(),
    };

    let pp_c = GlGroup {
        glgroupid: 1,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230405".to_string(),
        glgroupname: "Gl Group Name One".to_string(),
        glgroupcriteria: "Gl Group Criteria One".to_string(),
    };

    let pp_d = GlGroup {
        glgroupid: 1,
        glgrouptypeid: 1,
        placeid: 1,
        glgroupdate: "20230405".to_string(),
        glgroupname: "Gl Group Name One".to_string(),
        glgroupcriteria: "Gl Group Criteria One".to_string(),
    };

    let app = GlGroup::create_glgroup(pp_a);

    dbstring(&conn, app);

    let bpp = GlGroup::read_glgroup(pp_b);

    dbstring(&conn, bpp);

    let cpp = GlGroup::update_glgroup(pp_c);

    dbstring(&conn, cpp);

    let dpp = GlGroup::delete_glgroup(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = GlGroupTypeRole {
        glgrouptyperoleid: 1,
        glgrouptypeid: 1,
        glgrouptypename: "Gl Group Type Name One".to_string(),
        sequencenumber: 1,
    };

    let pp_b = GlGroupTypeRole {
        glgrouptyperoleid: 2,
        glgrouptypeid: 2,
        glgrouptypename: "Gl Group Type Name One".to_string(),
        sequencenumber: 2,
    };

    let pp_c = GlGroupTypeRole {
        glgrouptyperoleid: 3,
        glgrouptypeid: 3,
        glgrouptypename: "Gl Group Type Name One".to_string(),
        sequencenumber: 3,
    };

    let pp_d = GlGroupTypeRole {
        glgrouptyperoleid: 4,
        glgrouptypeid: 4,
        glgrouptypename: "Gl Group Type Name One".to_string(),
        sequencenumber: 4,
    };

    let app = GlGroupTypeRole::create_glgrouptyperole(pp_a);

    dbstring(&conn, app);

    let bpp = GlGroupTypeRole::read_glgrouptyperole(pp_b);

    dbstring(&conn, bpp);

    let cpp = GlGroupTypeRole::update_glgrouptyperole(pp_c);

    dbstring(&conn, cpp);

    let dpp = GlGroupTypeRole::delete_glgrouptyperole(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = RepoSource {
        reposourceid: 1,
        repositoryid: 1,
        sourceid: 1,
        rsactivityid: 1,
        callnumber: "Repository Source Call Number".to_string(),
        description: "Repository Source Description One".to_string(),
    };

    let pp_b = RepoSource {
        reposourceid: 2,
        repositoryid: 2,
        sourceid: 2,
        rsactivityid: 2,
        callnumber: "Repository Source Call Number".to_string(),
        description: "Repository Source Description Two".to_string(),
    };

    let pp_c = RepoSource {
        reposourceid: 3,
        repositoryid: 3,
        sourceid: 3,
        rsactivityid: 3,
        callnumber: "Repository Source Call Number".to_string(),
        description: "Repository Source Description Three".to_string(),
    };

    let pp_d = RepoSource {
        reposourceid: 4,
        repositoryid: 4,
        sourceid: 4,
        rsactivityid: 4,
        callnumber: "Repository Source Call Number".to_string(),
        description: "Repository Source Description Four".to_string(),
    };

    let app = RepoSource::create_reposource(pp_a);

    dbstring(&conn, app);

    let bpp = RepoSource::read_reposource(pp_b);

    dbstring(&conn, bpp);

    let cpp = RepoSource::update_reposource(pp_c);

    dbstring(&conn, cpp);

    let dpp = RepoSource::delete_reposource(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let person_a = Persona {
        personaid: 16,
        persona_name: "Sixteenth Persona".to_string(),
        description_comments: "A Sixteenth persona has been created.".to_string(),
    };

    let aname = Persona::create_persona(person_a);

    let person_b = Persona {
        personaid: 17,
        persona_name: "Seventeenth Persona".to_string(),
        description_comments: "A Seventeenth persona has been created.".to_string(),
    };

    let bname: String = Persona::update_persona(person_b);

    let person_c = Persona {
        personaid: 18,
        persona_name: "Eighteenth Persona".to_string(),
        description_comments: "A Eighteenth persona has been created.".to_string(),
    };

    let cname: String = Persona::delete_persona(person_c);

    let person_d = Persona {
        personaid: 19,
        persona_name: "Nineteenth Persona".to_string(),
        description_comments: "A Nineteenth persona has been created.".to_string(),
    };

    let dname: String = Persona::read_persona(person_d);

    dbstring(&conn, aname);

    dbstring(&conn, bname);

    dbstring(&conn, cname);

    // dbstring(&conn, dname);

    fn dbstring(conn: &Connection, dbstr: String) {
        match conn.execute(&dbstr, params![]) {
            Ok(updated) => println!("{} rows were updated by match", updated),
            Err(err) => println!("update failed: {}", err),
        };
    }

    let mut stmt = conn.prepare(&dname)?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Persona {
            personaid: row.get(0)?,
            persona_name: row.get(1)?,
            description_comments: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    let activity_a = Activity {
        activityid: 1,
        projectid: 1,
        researcherid: 1,
        scheddate: "1 June 2023".to_string(),
        completedate: "30 June 2023".to_string(),
        typecode: "A".to_string(),
        status: "In Progress".to_string(),
        description: "A Test Activity".to_string(),
        priority: "High".to_string(),
        comments: "These are comments".to_string(),
    };

    let aactivity: String = Activity::create_activity(activity_a);

    let activity_b = Activity {
        activityid: 2,
        projectid: 2,
        researcherid: 2,
        scheddate: "2 June 2023".to_string(),
        completedate: "29 June 2023".to_string(),
        typecode: "B".to_string(),
        status: "In Progress".to_string(),
        description: "A Second Test Activity".to_string(),
        priority: "Medium".to_string(),
        comments: "These are more comments".to_string(),
    };

    let bactivity: String = Activity::read_activity(activity_b);

    let activity_c = Activity {
        activityid: 1,
        projectid: 1,
        researcherid: 1,
        scheddate: "3 June 2023".to_string(),
        completedate: "28 June 2023".to_string(),
        typecode: "c".to_string(),
        status: "In Progress".to_string(),
        description: "Another Test Activity".to_string(),
        priority: "Low".to_string(),
        comments: "These are another set of comments".to_string(),
    };

    let cactivity: String = Activity::update_activity(activity_c);

    dbstring(&conn, aactivity);

    // dbstring(&conn, bactivity);

    dbstring(&conn, cactivity);

    let mut stmt = conn.prepare(&bactivity)?;
    let activity_iter = stmt.query_map([], |row| {
        Ok(Activity {
            activityid: row.get(0)?,
            projectid: row.get(1)?,
            researcherid: row.get(2)?,
            scheddate: row.get(3)?,
            completedate: row.get(4)?,
            typecode: row.get(5)?,
            status: row.get(6)?,
            description: row.get(7)?,
            priority: row.get(8)?,
            comments: row.get(9)?,
        })
    })?;

    for activity in activity_iter {
        println!("Found activity {:?}", activity.unwrap());
    }

    Ok(())
}
