use crate::structs::dbtables::{
    Activity, AssertAssert, CharPart, CharPartType, Characteristic, CitationPart, CitationPartType,
    Event, EventType, EventTypeRole, GlAssertion, GlGroup, GlGroupType, GlGroupTypeRole, Persona,
    Place, PlacePart, PlacePartType, Project, Repository, ReprMediaType, RepresentType,
    Representation, ResObjActivity, ResObjective, ResProj, Researcher, Search, Source, SourceGroup,
    SrcGrpSrc, SuretyPart, SuretyScheme, RepoSource
};
use rusqlite::{params, Connection, Result};

mod structs;

fn main() -> Result<()> {
    let path = "C:/Users/npmal/projects/genealogicng/glNG.db";
    let conn = Connection::open(&path)?;

    /* ------------------------------------------------------------------------- */

    let place_a = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "a".to_string(),
        placecomment: "First Place".to_string(),
    };

    let aplace = structs::dbtables::Place::create_place(place_a);

    dbstring(&conn, aplace);

    let place_b = Place {
        placeid: 16,
        startdate: "20010101".to_string(),
        enddate: "20231231".to_string(),
        ascdescnone: "a".to_string(),
        placecomment: "First Place".to_string(),
    };

    let bplace = structs::dbtables::Place::read_place(place_b);

    dbstring(&conn, bplace);

    let place_c = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "b".to_string(),
        placecomment: "Second Place".to_string(),
    };

    let cplace = structs::dbtables::Place::update_place(place_c);

    dbstring(&conn, cplace);

    let place_d = Place {
        placeid: 16,
        startdate: "20230101".to_string(),
        enddate: "20011231".to_string(),
        ascdescnone: "b".to_string(),
        placecomment: "Second Place".to_string(),
    };

    let dplace = structs::dbtables::Place::delete_place(place_d);

    dbstring(&conn, dplace);

    /* ------------------------------------------------------------------------- */

    let ppt_a = PlacePartType {
        placeparttypeid: 16,
        pptname: "Second Part".to_string(),
    };

    let appt = structs::dbtables::PlacePartType::create_placeparttype(ppt_a);

    dbstring(&conn, appt);

    let ppt_b = PlacePartType {
        placeparttypeid: 16,
        pptname: "Second Part".to_string(),
    };

    let bppt = structs::dbtables::PlacePartType::read_placeparttype(ppt_b);

    dbstring(&conn, bppt);

    let ppt_c = PlacePartType {
        placeparttypeid: 16,
        pptname: "Third Part".to_string(),
    };

    let cppt = structs::dbtables::PlacePartType::update_placeparttype(ppt_c);

    dbstring(&conn, cppt);

    let ppt_d = PlacePartType {
        placeparttypeid: 16,
        pptname: "Third Part".to_string(),
    };

    let dppt = structs::dbtables::PlacePartType::delete_placeparttype(ppt_d);

    dbstring(&conn, dppt);

    /* ------------------------------------------------------------------------- */

    let pp_a = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let appt = structs::dbtables::PlacePart::create_placepart(pp_a);

    dbstring(&conn, appt);

    let pp_b = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let bpp = structs::dbtables::PlacePart::read_placepart(pp_b);

    dbstring(&conn, bpp);

    let pp_c = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let cpp = structs::dbtables::PlacePart::update_placepart(pp_c);

    dbstring(&conn, cpp);

    let pp_d = PlacePart {
        placepartid: 1,
        placeid: 2,
        placeparttypeid: 16,
        name: "Place Part".to_string(),
        sequencenumber: 16,
    };

    let dpp = structs::dbtables::PlacePart::delete_placepart(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Characteristic {
        characteristicid: 1,
        placeid: 2,
        characteristicdate: "20230402".to_string(),
        ascdescnone: "Place Part One".to_string(),
    };

    let pp_b = Characteristic {
        characteristicid: 3,
        placeid: 4,
        characteristicdate: "20230403".to_string(),
        ascdescnone: "Place Part Two".to_string(),
    };

    let pp_c = Characteristic {
        characteristicid: 5,
        placeid: 6,
        characteristicdate: "20230404".to_string(),
        ascdescnone: "Place Part Three".to_string(),
    };

    let pp_d = Characteristic {
        characteristicid: 7,
        placeid: 8,
        characteristicdate: "20230405".to_string(),
        ascdescnone: "Place Part Four".to_string(),
    };

    let app = structs::dbtables::Characteristic::create_characteristic(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Characteristic::read_characteristic(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Characteristic::update_characteristic(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Characteristic::delete_characteristic(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = CharPartType {
        charparttypeid: 1,
        charparttypename: "20230403".to_string(),
        gedcomtag: "Place Part One".to_string(),
    };

    let pp_b = CharPartType {
        charparttypeid: 3,
        charparttypename: "20230405".to_string(),
        gedcomtag: "Place Part Two".to_string(),
    };

    let pp_c = CharPartType {
        charparttypeid: 3,
        charparttypename: "20230407".to_string(),
        gedcomtag: "Place Part Three".to_string(),
    };

    let pp_d = CharPartType {
        charparttypeid: 7,
        charparttypename: "20230409".to_string(),
        gedcomtag: "Place Part Four".to_string(),
    };

    let app = structs::dbtables::CharPartType::create_charparttype(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::CharPartType::read_charparttype(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::CharPartType::update_charparttype(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::CharPartType::delete_charparttype(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = CharPart {
        characteristicpartid: 1,
        characteristicid: 2,
        charparttypeid: 3,
        charpartname: "Characteristic Part Name".to_string(),
        charpartseq: 4,
    };

    let pp_b = CharPart {
        characteristicpartid: 5,
        characteristicid: 6,
        charparttypeid: 7,
        charpartname: "Characteristic Part Name".to_string(),
        charpartseq: 8,
    };

    let pp_c = CharPart {
        characteristicpartid: 9,
        characteristicid: 10,
        charparttypeid: 11,
        charpartname: "Characteristic Part Name".to_string(),
        charpartseq: 12,
    };

    let pp_d = CharPart {
        characteristicpartid: 13,
        characteristicid: 14,
        charparttypeid: 15,
        charpartname: "Characteristic Part Name".to_string(),
        charpartseq: 16,
    };

    let app = structs::dbtables::CharPart::create_charpart(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::CharPart::read_charpart(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::CharPart::update_charpart(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::CharPart::delete_charpart(pp_d);

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

    let app = structs::dbtables::Project::create_project(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Project::read_project(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Project::update_project(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Project::delete_project(pp_d);

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

    let app = structs::dbtables::Repository::create_repository(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Repository::read_repository(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Repository::update_repository(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Repository::delete_repository(pp_d);

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

    let app = structs::dbtables::ResObjective::create_resobjective(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::ResObjective::read_resobjective(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::ResObjective::update_resobjective(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::ResObjective::delete_resobjective(pp_d);

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

    let app = structs::dbtables::ResObjActivity::create_resobjactivity(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::ResObjActivity::read_resobjactivity(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::ResObjActivity::update_resobjactivity(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::ResObjActivity::delete_resobjactivity(pp_d);

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

    let app = structs::dbtables::Researcher::create_researcher(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Researcher::read_researcher(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Researcher::update_researcher(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Researcher::delete_researcher(pp_d);

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

    let app = structs::dbtables::ResProj::create_resproj(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::ResProj::read_resproj(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::ResProj::update_resproj(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::ResProj::delete_resproj(pp_d);

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

    let app = structs::dbtables::Source::create_source(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Source::read_source(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Source::update_source(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Source::delete_source(pp_d);

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

    let app = structs::dbtables::SourceGroup::create_sourcegroup(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::SourceGroup::read_sourcegroup(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::SourceGroup::update_sourcegroup(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::SourceGroup::delete_sourcegroup(pp_d);

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

    let app = structs::dbtables::SrcGrpSrc::create_srcgrpsrc(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::SrcGrpSrc::read_srcgrpsrc(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::SrcGrpSrc::update_srcgrpsrc(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::SrcGrpSrc::delete_srcgrpsrc(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = CitationPartType {
        citationparttypeid: 1,
        citationparttypename: "Source Group Name One".to_string(),
    };

    let pp_b = CitationPartType {
        citationparttypeid: 2,
        citationparttypename: "Source Group Name Two".to_string(),
    };

    let pp_c = CitationPartType {
        citationparttypeid: 3,
        citationparttypename: "Source Group Name Three".to_string(),
    };

    let pp_d = CitationPartType {
        citationparttypeid: 4,
        citationparttypename: "Source Group Name Four".to_string(),
    };

    let app = structs::dbtables::CitationPartType::create_citationparttype(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::CitationPartType::read_citationparttype(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::CitationPartType::update_citationparttype(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::CitationPartType::delete_citationparttype(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = CitationPart {
        citationpartid: 1,
        sourceid: 1,
        citeparttypeid: 1,
        citepartvalue: "Citation Part One".to_string(),
    };

    let pp_b = CitationPart {
        citationpartid: 2,
        sourceid: 2,
        citeparttypeid: 2,
        citepartvalue: "Citation Part Two".to_string(),
    };

    let pp_c = CitationPart {
        citationpartid: 3,
        sourceid: 3,
        citeparttypeid: 3,
        citepartvalue: "Citation Part Three".to_string(),
    };

    let pp_d = CitationPart {
        citationpartid: 4,
        sourceid: 4,
        citeparttypeid: 4,
        citepartvalue: "Citation Part Four".to_string(),
    };

    let app = structs::dbtables::CitationPart::create_citationpart(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::CitationPart::read_citationpart(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::CitationPart::update_citationpart(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::CitationPart::delete_citationpart(pp_d);

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

    let app = structs::dbtables::SuretyScheme::create_suretyscheme(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::SuretyScheme::read_suretyscheme(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::SuretyScheme::update_suretyscheme(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::SuretyScheme::delete_suretyscheme(pp_d);

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

    let app = structs::dbtables::SuretyPart::create_suretypart(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::SuretyPart::read_suretypart(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::SuretyPart::update_suretypart(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::SuretyPart::delete_suretypart(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = GlAssertion {
        glassertionid: 1,
        suretypartid: 1,
        researcherid: 1,
        sourceid: 1,
        subject1id: 1,
        subject1type: "Subject 1 Type".to_string(),
        subject2id: 1,
        subject2type: "Subject 2 Type".to_string(),
        value_role: 1,
        disproved: "true".to_string(),
        rationale: "GlAssertion One".to_string(),
    };

    let pp_b = GlAssertion {
        glassertionid: 2,
        suretypartid: 2,
        researcherid: 2,
        sourceid: 2,
        subject1id: 2,
        subject1type: "Subject 1 Type".to_string(),
        subject2id: 2,
        subject2type: "Subject 2 Type".to_string(),
        value_role: 2,
        disproved: "true".to_string(),
        rationale: "GlAssertion Two".to_string(),
    };

    let pp_c = GlAssertion {
        glassertionid: 3,
        suretypartid: 3,
        researcherid: 3,
        sourceid: 3,
        subject1id: 3,
        subject1type: "Subject 1 Type".to_string(),
        subject2id: 3,
        subject2type: "Subject 2 Type".to_string(),
        value_role: 3,
        disproved: "true".to_string(),
        rationale: "GlAssertion Three".to_string(),
    };

    let pp_d = GlAssertion {
        glassertionid: 4,
        suretypartid: 4,
        researcherid: 4,
        sourceid: 4,
        subject1id: 4,
        subject1type: "Subject 1 Type".to_string(),
        subject2id: 4,
        subject2type: "Subject 2 Type".to_string(),
        value_role: 4,
        disproved: "true".to_string(),
        rationale: "GlAssertion Four".to_string(),
    };

    let app = structs::dbtables::GlAssertion::create_glassertion(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::GlAssertion::read_glassertion(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::GlAssertion::update_glassertion(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::GlAssertion::delete_glassertion(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = AssertAssert {
        assertassertid: 1,
        idlo: 1,
        idhi: 1,
        seq: 1,
    };

    let pp_b = AssertAssert {
        assertassertid: 2,
        idlo: 2,
        idhi: 2,
        seq: 2,
    };

    let pp_c = AssertAssert {
        assertassertid: 3,
        idlo: 3,
        idhi: 3,
        seq: 3,
    };

    let pp_d = AssertAssert {
        assertassertid: 4,
        idlo: 4,
        idhi: 4,
        seq: 4,
    };

    let app = structs::dbtables::AssertAssert::create_assertassert(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::AssertAssert::read_assertassert(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::AssertAssert::update_assertassert(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::AssertAssert::delete_assertassert(pp_d);

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

    let app = structs::dbtables::ReprMediaType::create_reprmediatype(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::ReprMediaType::read_reprmediatype(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::ReprMediaType::update_reprmediatype(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::ReprMediaType::delete_reprmediatype(pp_d);

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

    let app = structs::dbtables::RepresentType::create_representtype(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::RepresentType::read_representtype(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::RepresentType::update_representtype(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::RepresentType::delete_representtype(pp_d);

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

    let app = structs::dbtables::Representation::create_representation(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Representation::read_representation(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Representation::update_representation(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Representation::delete_representation(pp_d);

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

    let app = structs::dbtables::Search::create_search(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Search::read_search(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Search::update_search(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Search::delete_search(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = EventType {
        eventtypeid: 1,
        eventtypename: "Event Type Name One".to_string(),
        gedcomtag: "GEDCOM Tag One".to_string(),
    };

    let pp_b = EventType {
        eventtypeid: 2,
        eventtypename: "Event Type Name Two".to_string(),
        gedcomtag: "GEDCOM Tag Two".to_string(),
    };

    let pp_c = EventType {
        eventtypeid: 3,
        eventtypename: "Event Type Name Three".to_string(),
        gedcomtag: "GEDCOM Tag Three".to_string(),
    };

    let pp_d = EventType {
        eventtypeid: 4,
        eventtypename: "Event Type Name Four".to_string(),
        gedcomtag: "GEDCOM Tag Four".to_string(),
    };

    let app = structs::dbtables::EventType::create_eventtype(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::EventType::read_eventtype(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::EventType::update_eventtype(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::EventType::delete_eventtype(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = Event {
        eventid: 1,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "Event Date One".to_string(),
        eventname: "Event Name One".to_string(),
    };

    let pp_b = Event {
        eventid: 1,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "Event Date Two".to_string(),
        eventname: "Event Name Two".to_string(),
    };

    let pp_c = Event {
        eventid: 1,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "Event Date Three".to_string(),
        eventname: "Event Name Three".to_string(),
    };

    let pp_d = Event {
        eventid: 1,
        eventtypeid: 1,
        placeid: 1,
        eventdate: "Event Date Four".to_string(),
        eventname: "Event Name Four".to_string(),
    };

    let app = structs::dbtables::Event::create_event(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::Event::read_event(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::Event::update_event(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::Event::delete_event(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let pp_a = EventTypeRole {
        eventtyperoleid: 1,
        eventtypeid: 1,
        eventtyperolename: "Event Type Role Name One".to_string(),
    };

    let pp_b = EventTypeRole {
        eventtyperoleid: 2,
        eventtypeid: 2,
        eventtyperolename: "Event Type Role Name Two".to_string(),
    };

    let pp_c = EventTypeRole {
        eventtyperoleid: 3,
        eventtypeid: 3,
        eventtyperolename: "Event Type Role Name Three".to_string(),
    };

    let pp_d = EventTypeRole {
        eventtyperoleid: 4,
        eventtypeid: 4,
        eventtyperolename: "Event Type Role Name Four".to_string(),
    };

    let app = structs::dbtables::EventTypeRole::create_eventtyperole(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::EventTypeRole::read_eventtyperole(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::EventTypeRole::update_eventtyperole(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::EventTypeRole::delete_eventtyperole(pp_d);

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

    let app = structs::dbtables::GlGroupType::create_glgrouptype(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::GlGroupType::read_glgrouptype(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::GlGroupType::update_glgrouptype(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::GlGroupType::delete_glgrouptype(pp_d);

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

    let app = structs::dbtables::GlGroup::create_glgroup(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::GlGroup::read_glgroup(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::GlGroup::update_glgroup(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::GlGroup::delete_glgroup(pp_d);

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

    let app = structs::dbtables::GlGroupTypeRole::create_glgrouptyperole(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::GlGroupTypeRole::read_glgrouptyperole(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::GlGroupTypeRole::update_glgrouptyperole(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::GlGroupTypeRole::delete_glgrouptyperole(pp_d);

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

    let app = structs::dbtables::RepoSource::create_reposource(pp_a);

    dbstring(&conn, app);

    let bpp = structs::dbtables::RepoSource::read_reposource(pp_b);

    dbstring(&conn, bpp);

    let cpp = structs::dbtables::RepoSource::update_reposource(pp_c);

    dbstring(&conn, cpp);

    let dpp = structs::dbtables::RepoSource::delete_reposource(pp_d);

    dbstring(&conn, dpp);

    /* ------------------------------------------------------------------------- */

    let person_a = Persona {
        personaid: 16,
        persona_name: "Sixteenth Persona".to_string(),
        description_comments: "A Sixteenth persona has been created.".to_string(),
    };

    let aname = structs::dbtables::Persona::create_persona(person_a);

    let person_b = Persona {
        personaid: 17,
        persona_name: "Seventeenth Persona".to_string(),
        description_comments: "A Seventeenth persona has been created.".to_string(),
    };

    let bname: String = structs::dbtables::Persona::update_persona(person_b);

    let person_c = Persona {
        personaid: 18,
        persona_name: "Eighteenth Persona".to_string(),
        description_comments: "A Eighteenth persona has been created.".to_string(),
    };

    let cname: String = structs::dbtables::Persona::delete_persona(person_c);

    let person_d = Persona {
        personaid: 19,
        persona_name: "Nineteenth Persona".to_string(),
        description_comments: "A Nineteenth persona has been created.".to_string(),
    };

    let dname: String = structs::dbtables::Persona::read_persona(person_d);

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

    let aactivity: String = structs::dbtables::Activity::create_activity(activity_a);

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

    let bactivity: String = structs::dbtables::Activity::read_activity(activity_b);

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

    let cactivity: String = structs::dbtables::Activity::update_activity(activity_c);

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
