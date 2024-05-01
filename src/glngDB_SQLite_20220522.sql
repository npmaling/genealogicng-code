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

CREATE TABLE activity (
    activityid INT,
    projectid INT,
    researcherid INT,
    scheddate TEXT(128),
    completedate TEXT(128),
    typecode CHAR(1),
    status TEXT(16),
    description TEXT(32672),
    priority TEXT(16),
    comments TEXT(32672)
    -- PRIMARY KEY (activityid)
);
-- CREATE INDEX activity_project_idx ON activity (projectid ASC);
-- CREATE INDEX activity_researcher_idx ON activity (researcherid ASC);
-- 
-- DROP TABLE persona;
CREATE TABLE persona (
    personaid INT,
    persona_name TEXT(128),
    description_comments TEXT(32672)
    -- PRIMARY KEY (personaid)
);
-- CREATE INDEX personanameidx ON persona (persona_name ASC);
-- DROP TABLE place;
CREATE TABLE place (
    placeid INT,
    startdate TEXT(96),
    enddate TEXT(96),
    ascdescnone CHAR(1),
    placecomment TEXT(32672)
    -- PRIMARY KEY (placeid)
);
-- CREATE INDEX plstartdate ON place (startdate ASC);
-- CREATE INDEX plenddate ON place (enddate ASC);
-- DROP TABLE placeparttype;
CREATE TABLE placeparttype (
    placeparttypeid INT,
    pptname TEXT(32)
    -- PRIMARY KEY (placeparttypeid)
);
-- DROP TABLE placepart;
CREATE TABLE placepart (
    placepartid INT,
    placeid INT,
    placeparttypeid INT,
    name TEXT(128),
    sequencenumber INT
    -- PRIMARY KEY (placepartid),
    -- FOREIGN KEY (PLACEPARTTYPEID) REFERENCES PLACEPARTTYPE (PLACEPARTTYPEID),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE characteristic;
CREATE TABLE characteristic (
    characteristicid INT,
    placeid INT,
    characteristicdate TEXT(96),
    ascdescnone CHAR(1)
    -- PRIMARY KEY (characteristicid),
    -- FOREIGN KEY (placeid) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE charparttype;
-- GEDCOMTAG in this table is not in the GDM and is only here to accommodate
-- that standard and provide some future consistency with imports/exports
CREATE TABLE charparttype (
    charparttypeid INT,
    charparttypename TEXT(32),
    gedcomtag TEXT(4)
    -- PRIMARY KEY (charparttypeid)
);
-- DROP TABLE charpart;
CREATE TABLE charpart (
    characteristicpartid INT,
    characteristicid INT,
    charparttypeid INT,
    charpartname TEXT(32),
    charpartseq INT
    -- PRIMARY KEY (characteristicpartid),
    -- FOREIGN KEY (CHARACTERISTICID) REFERENCES CHARACTERISTIC (CHARACTERISTICID),
    -- FOREIGN KEY (CHARPARTTYPEID) REFERENCES CHARPARTTYPE (CHARPARTTYPEID)
);
-- DROP TABLE project;
CREATE TABLE project (
    projectid INT,
    name TEXT(128),
    projectdesc TEXT(16384),
    clientdata TEXT(16384)
    -- PRIMARY KEY (projectid)
);
-- DROP TABLE repository;
CREATE TABLE repository (
    repositoryid INT,
    placeid INT,
    reponame TEXT(128),
    comments TEXT(16384)
    -- PRIMARY KEY (repositoryid),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE resobjective;
CREATE TABLE resobjective (
    resobjid INT,
    projectid INT,
    subjectid INT,
    subjecttype CHAR(1),
    name TEXT(32),
    description TEXT(16384),
    sequencenumber INT,
    priority TEXT(16),
    status TEXT(16)
    -- PRIMARY KEY (resobjid),
    -- FOREIGN KEY (PROJECTID) REFERENCES PROJECT (PROJECTID)
);
-- DROP TABLE resobjactivity;
CREATE TABLE resobjactivity (
    resobjactivityid INT,
    resobjid INT,
    activityid INT
    -- PRIMARY KEY (resobjactivityid),
    -- FOREIGN KEY (RESOBJID) REFERENCES RESOBJECTIVE (RESOBJID),
    -- FOREIGN KEY (ACTIVITYID) REFERENCES ACTIVITY (ACTIVITYID)
);
-- DROP TABLE researcher;
CREATE TABLE researcher (
    researcherid INT,
    name TEXT(128),
    addressid INT,
    comments TEXT(16384)
    -- PRIMARY KEY (researcherid)
);
-- DROP TABLE resproj;
CREATE TABLE resproj (
    resprojid INT,
    projectid INT,
    researcherid INT,
    researcherrole TEXT(32)
    -- PRIMARY KEY (resprojid),
    -- FOREIGN KEY (RESEARCHERID) REFERENCES RESEARCHER (RESEARCHERID),
    -- FOREIGN KEY (PROJECTID) REFERENCES PROJECT (PROJECTID)
);
-- DROP TABLE source;
CREATE TABLE source (
    sourceid INT,
    highersourceid INT,
    subjectplaceid INT,
    jurisplaceid INT,
    researcherid INT,
    subjectdate TEXT(96),
    comments TEXT(16384)
    -- FOREIGN KEY (HIGHERSOURCEID) REFERENCES SOURCE (HIGHERSOURCEID),
    -- FOREIGN KEY (SUBJECTPLACEID) REFERENCES PLACE (PLACEID),
    -- FOREIGN KEY (JURISPLACEID) REFERENCES PLACE (PLACEID),
    -- FOREIGN KEY (RESEARCHERID) REFERENCES RESEARCHER (RESEARCHERID)
);
-- ALTER TABLE SOURCE ADD CONSTRAINT HIGHSOURCE_FK FOREIGN KEY (HIGHERSOURCEID) REFERENCES SOURCE (HIGHERSOURCEID);
-- ALTER TABLE SOURCE ADD CONSTRAINT SUBJPLACE_FK FOREIGN KEY (SUBJECTPLACEID) REFERENCES PLACE (PLACEID);
-- ALTER TABLE SOURCE ADD CONSTRAINT JURISPLACE_FK FOREIGN KEY (JURISPLACEID) REFERENCES PLACE (PLACEID);
-- ALTER TABLE SOURCE ADD CONSTRAINT RESEARCHER_FK FOREIGN KEY (RESEARCHERID) REFERENCES RESEARCHER (RESEARCHERID);
-- DROP TABLE sourcegroup;
CREATE TABLE sourcegroup (
    sourcegroupid INT,
    sourcegroupname TEXT(96)
    -- PRIMARY KEY (sourcegroupid)
);
-- DROP TABLE srcgrpsrc;
CREATE TABLE srcgrpsrc (
    srcgrpsrcid INT,
    sourceid INT,
    sourcegroupid INT
    -- PRIMARY KEY (srsgrpsrcid),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (SOURCEGROUPID) REFERENCES SOURCEGROUP (SOURCEGROUPID)
);
-- DROP TABLE citationparttype;
CREATE TABLE citationparttype (
    citationparttypeid INT,
    citationparttypename TEXT(32) 
    -- PRIMARY KEY (citationparttypeid)
);
-- DROP TABLE citationpart;
CREATE TABLE citationpart (
    citationpartid INT,
    sourceid INT,
    citeparttypeid INT,
    citepartvalue TEXT(512)
    -- PRIMARY KEY (citationpartid) --  FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    --  FOREIGN KEY (CITEPARTTYPEID) REFERENCES CITATIONPARTTYPE (CITATIONPARTTYPEID)
);
-- DROP TABLE suretyscheme;
CREATE TABLE suretyscheme (
    suretyschemeid INT,
    name TEXT(32),
    description TEXT(16384)
    -- PRIMARY KEY (suretyschemeid)
);
-- DROP TABLE suretypart;
CREATE TABLE suretypart (
    suretypartid INT,
    schemeid INT,
    name TEXT(32),
    description TEXT(32),
    sequencenumber INT
    -- PRIMARY KEY (suretypartid),
    -- FOREIGN KEY (SCHEMEID) REFERENCES SURETYSCHEME (SURETYSCHEMEID)
);
CREATE INDEX spschemeidx ON suretypart (schemeid ASC);
-- DROP TABLE glassertion;
CREATE TABLE glassertion (
    glassertionid INT,
    suretypartid INT,
    researcherid INT,
    sourceid INT,
    subject1id INT,
    subject1type CHAR(1),
    subject2id INT,
    subject2type CHAR(1),
    value_role INT,
    disproved BOOLEAN,
    rationale TEXT(32672)
    -- PRIMARY KEY (glassertionid) --  FOREIGN KEY (SURETYPARTID) REFERENCES SURETYPART (SURETYPARTID),
    -- FOREIGN KEY (RESEARCHERID) REFERENCES RESEARCHER (RESEARCHERID),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID)
);
-- DROP TABLE assertassert;
CREATE TABLE assertassert (
    assertassertid INT,
    idlo INT,
    idhi INT,
    seq INT
    -- PRIMARY KEY (assertassertid),
    -- FOREIGN KEY (idlo) REFERENCES GLASSERTION (GLASSERTIONID),
    -- FOREIGN KEY (idhi) REFERENCES GLASSERTION (GLASSERTIONID)
);
-- BEGIN new table REPRESENTATION-MEDIA-TYPE (not in GDM)
-- - depended on by REPRESENTATION's reprmediaid FK
-- Holds the data GDM REPRESENTATION-TYPE would, but in a more accessible way
-- as user alterable/addable/updatable. Also contrary to TMG's internal enums
-- which were not modifiable
-- DROP TABLE reprmediatype;
CREATE TABLE reprmediatype (
    reprmediaid INT,
    reprmedianame TEXT(128)
    -- PRIMARY KEY (reprmediaid)
);
-- END new table
-- DROP TABLE representtype;
CREATE TABLE representtype (
    reprtypeid INT,
    name TEXT(128)
    -- PRIMARY KEY (reprtypeid)
);
-- DROP TABLE representation;
CREATE TABLE representation (
    representationid INT,
    sourceid INT,
    reprtypeid INT,
    reprmediaid INT,
    physfilecode TEXT(8192),
    comments TEXT(16384),
    externallink TEXT(255)
    -- PRIMARY KEY (representationid),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (REPRTYPEID) REFERENCES REPRESENTTYPE (REPRTYPEID),
    -- FOREIGN KEY (REPRMEDIAID) REFERENCES REPRMEDIATYPE (REPRMEDIAID)
);
-- DROP TABLE search;
CREATE TABLE search (
    searchid INT,
    activityid INT,
    -- relates to ACTIVITY
    sourceid INT,
    repositoryid INT,
    searchedfor TEXT(16384)
    -- PRIMARY KEY (searchid),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (REPOSITORYID) REFERENCES REPOSITORY (REPOSITORYID)
);
-- DROP TABLE eventtype;
-- GEDCOMTAG in this table is not in the GDM and is only here to accommodate
-- that standard and provide some future consistency with imports/exports
CREATE TABLE eventtype (
    eventtypeid INT,
    eventtypename TEXT(32),
    gedcomtag TEXT(4)
    -- PRIMARY KEY (eventtypeid)
);
-- DROP TABLE event;
CREATE TABLE event (
    eventid INT,
    eventtypeid INT,
    placeid INT,
    eventdate TEXT(96),
    eventname TEXT(128)
    -- PRIMARY KEY (eventid),
    -- FOREIGN KEY (EVENTTYPEID) REFERENCES EVENTTYPE (EVENTTYPEID),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE eventtyperole;
CREATE TABLE eventtyperole (
    eventtyperoleid INT,
    eventtypeid INT,
    eventtyperolename TEXT(32)
    -- PRIMARY KEY (eventtyperoleid),
    -- FOREIGN KEY (EVENTTYPEID) REFERENCES EVENTTYPE (EVENTTYPEID)
);
-- DROP TABLE glgrouptype;
CREATE TABLE glgrouptype (
    glgrouptypeid INT,
    glgroupname TEXT(32),
    ascdescnone CHAR(1)
    -- PRIMARY KEY (glgrouptypeid)
);
-- DROP TABLE glgroup;
CREATE TABLE glgroup (
    glgroupid INT,
    glgrouptypeid INT,
    placeid INT,
    glgroupdate TEXT(96),
    glgroupname TEXT(32),
    glgroupcriteria TEXT(128)
    -- PRIMARY KEY (glgroupid),
    -- FOREIGN KEY (GLGROUPTYPEID) REFERENCES GLGROUPTYPE (GLGROUPTYPEID),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE glgrouptyperole;
CREATE TABLE glgrouptyperole (
    glgrouptyperoleid INT,
    glgrouptypeid INT,
    glgrouptypename TEXT(32),
    sequencenumber INT
    -- PRIMARY KEY (glgrouptyperoleid),
    -- FOREIGN KEY (GLGROUPTYPEID) REFERENCES GLGROUPTYPE (GLGROUPTYPEID)
);
-- DROP TABLE reposource;
-- The RSACTIVITYID FK in this table *should* refer to the SEARCH table rather
-- than the ACTIVITY table, but for some reason DERBY is failing to create the
-- link ...
CREATE TABLE reposource (
    repo_sourceid INT,
    repositoryid INT,
    sourceid INT,
    rsactivityid INT,
    callnumber TEXT(32),
    description TEXT(128)
    -- PRIMARY KEY (repo_sourceid),
    -- FOREIGN KEY (REPOSITORYID) REFERENCES REPOSITORY (REPOSITORYID),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (RSACTIVITYID) REFERENCES ACTIVITY (ACTIVITYID)
);
