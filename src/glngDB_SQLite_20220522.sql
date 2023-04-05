-- Copyright 2013 N. P. Maling
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
    activityid INT NOT NULL,
    projectid INT NOT NULL,
    researcherid INT NOT NULL,
    scheddate DATE NOT NULL,
    completedate DATE NOT NULL,
    typecode CHAR(1) NOT NULL,
    status TEXT(16) NOT NULL,
    description TEXT(32672) NOT NULL,
    priority TEXT(16) NOT NULL,
    comments TEXT(32672) NOT NULL,
    -- PRIMARY KEY (activityid)
);
-- CREATE INDEX activity_project_idx ON activity (projectid ASC);
-- CREATE INDEX activity_researcher_idx ON activity (researcherid ASC);
-- 
-- DROP TABLE persona;
CREATE TABLE persona (
    personaid INT NOT NULL,
    persona_name TEXT(128) NOT NULL,
    description_comments TEXT(32672) NOT NULL
    -- PRIMARY KEY (personaid)
);
-- CREATE INDEX personanameidx ON persona (persona_name ASC);
-- DROP TABLE place;
CREATE TABLE place (
    placeid INT NOT NULL,
    startdate TEXT(96) NOT NULL,
    enddate TEXT(96) NOT NULL,
    ascdescnone CHAR(1) NOT NULL,
    placecomment TEXT(32672) NOT NULL,
    -- PRIMARY KEY (placeid)
);
-- CREATE INDEX plstartdate ON place (startdate ASC);
-- CREATE INDEX plenddate ON place (enddate ASC);
-- DROP TABLE placeparttype;
CREATE TABLE placeparttype (
    placeparttypeid INT NOT NULL,
    pptname TEXT(32) NOT NULL,
    -- PRIMARY KEY (placeparttypeid)
);
-- DROP TABLE placepart;
CREATE TABLE placepart (
    placepartid INT NOT NULL,
    placeid INT NOT NULL,
    placeparttypeid INT NOT NULL,
    name TEXT(128) NOT NULL,
    sequencenumber INT NOT NULL,
    -- PRIMARY KEY (placepartid),
    -- FOREIGN KEY (PLACEPARTTYPEID) REFERENCES PLACEPARTTYPE (PLACEPARTTYPEID),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE characteristic;
CREATE TABLE characteristic (
    characteristicid INT NOT NULL,
    placeid INT NOT NULL,
    characteristicdate TEXT(96) NOT NULL,
    ascdescnone CHAR(1) NOT NULL,
    -- PRIMARY KEY (characteristicid),
    -- FOREIGN KEY (placeid) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE charparttype;
-- GEDCOMTAG in this table is not in the GDM and is only here to accommodate
-- that standard and provide some future consistency with imports/exports
CREATE TABLE charparttype (
    charparttypeid INT NOT NULL,
    charparttypename TEXT(32) NOT NULL,
    gedcomtag TEXT(4) NOT NULL,
    -- PRIMARY KEY (charparttypeid)
);
-- DROP TABLE charpart;
CREATE TABLE charpart (
    characteristicpartid INT NOT NULL,
    characteristicid INT NOT NULL,
    charparttypeid INT NOT NULL,
    charpartname TEXT(32) NOT NULL,
    charpartseq INT NOT NULL,
    -- PRIMARY KEY (characteristicpartid),
    -- FOREIGN KEY (CHARACTERISTICID) REFERENCES CHARACTERISTIC (CHARACTERISTICID),
    -- FOREIGN KEY (CHARPARTTYPEID) REFERENCES CHARPARTTYPE (CHARPARTTYPEID)
);
-- DROP TABLE project;
CREATE TABLE project (
    projectid INT NOT NULL,
    name TEXT(128) NOT NULL,
    projectdesc TEXT(16384) NOT NULL,
    clientdata TEXT(16384) NOT NULL,
    -- PRIMARY KEY (projectid)
);
-- DROP TABLE repository;
CREATE TABLE repository (
    repositoryid INT NOT NULL,
    placeid INT NOT NULL,
    reponame TEXT(128) NOT NULL,
    comments TEXT(16384) NOT NULL,
    -- PRIMARY KEY (repositoryid),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE resobjective;
CREATE TABLE resobjective (
    resobjid INT NOT NULL,
    projectid INT NOT NULL,
    subjectid INT NOT NULL,
    subjecttype CHAR(1) NOT NULL,
    name TEXT(32) NOT NULL,
    description TEXT(16384) NOT NULL,
    sequencenumber INT NOT NULL,
    priority TEXT(16) NOT NULL,
    status TEXT(16) NOT NULL,
    -- PRIMARY KEY (resobjid),
    -- FOREIGN KEY (PROJECTID) REFERENCES PROJECT (PROJECTID)
);
-- DROP TABLE resobjactivity;
CREATE TABLE resobjactivity (
    resobjactivityid INT NOT NULL,
    resobjid INT NOT NULL,
    activityid INT NOT NULL,
    -- PRIMARY KEY (resobjactivityid),
    -- FOREIGN KEY (RESOBJID) REFERENCES RESOBJECTIVE (RESOBJID),
    -- FOREIGN KEY (ACTIVITYID) REFERENCES ACTIVITY (ACTIVITYID)
);
-- DROP TABLE researcher;
CREATE TABLE researcher (
    researcherid INT NOT NULL,
    name TEXT(128) NOT NULL,
    addressid INT NOT NULL,
    comments TEXT(16384) NOT NULL,
    -- PRIMARY KEY (researcherid)
);
-- DROP TABLE resproj;
CREATE TABLE resproj (
    resprojid INT NOT NULL,
    projectid INT NOT NULL,
    researcherid INT NOT NULL,
    researcherrole TEXT(32) NOT NULL,
    -- PRIMARY KEY (resprojid),
    -- FOREIGN KEY (RESEARCHERID) REFERENCES RESEARCHER (RESEARCHERID),
    -- FOREIGN KEY (PROJECTID) REFERENCES PROJECT (PROJECTID)
);
-- DROP TABLE source;
CREATE TABLE source (
    sourceid INT NOT NULL,
    highersourceid INT NOT NULL,
    subjectplaceid INT NOT NULL,
    jurisplaceid INT NOT NULL,
    researcherid INT NOT NULL,
    subjectdate TEXT(96),
    comments TEXT(16384),
    --  FOREIGN KEY (HIGHERSOURCEID) REFERENCES SOURCE (HIGHERSOURCEID),
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
    sourcegroupid INT NOT NULL,
    sourcegroupname TEXT(96),
    -- PRIMARY KEY (sourcegroupid)
);
-- DROP TABLE srcgrpsrc;
CREATE TABLE srcgrpsrc (
    srcgrpsrcid INT NOT NULL,
    sourceid INT NOT NULL,
    sourcegroupid INT NOT NULL,
    -- PRIMARY KEY (srsgrpsrcid),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (SOURCEGROUPID) REFERENCES SOURCEGROUP (SOURCEGROUPID)
);
-- DROP TABLE citationparttype;
CREATE TABLE citationparttype (
    citationparttypeid INT NOT NULL,
    citationparttypename TEXT(32) NOT NULL -- PRIMARY KEY (citationparttypeid)
);
-- DROP TABLE citationpart;
CREATE TABLE citationpart (
    citationpartid INT NOT NULL,
    sourceid INT NOT NULL,
    citeparttypeid INT NOT NULL,
    citepartvalue TEXT(512) NOT NULL,
    -- PRIMARY KEY (citationpartid) --  FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    --  FOREIGN KEY (CITEPARTTYPEID) REFERENCES CITATIONPARTTYPE (CITATIONPARTTYPEID)
);
-- DROP TABLE suretyscheme;
CREATE TABLE suretyscheme (
    suretyschemeid INT NOT NULL,
    name TEXT(32) NOT NULL,
    description TEXT(16384) NOT NULL,
    -- PRIMARY KEY (suretyschemeid)
);
-- DROP TABLE suretypart;
CREATE TABLE suretypart (
    suretypartid INT NOT NULL,
    schemeid INT NOT NULL,
    name TEXT(32) NOT NULL,
    description TEXT(32) NOT NULL,
    sequencenumber INT NOT NULL,
    -- PRIMARY KEY (suretypartid),
    -- FOREIGN KEY (SCHEMEID) REFERENCES SURETYSCHEME (SURETYSCHEMEID)
);
CREATE INDEX spschemeidx ON suretypart (schemeid ASC);
-- DROP TABLE glassertion;
CREATE TABLE glassertion (
    glassertionid INT NOT NULL,
    suretypartid INT NOT NULL,
    researcherid INT NOT NULL,
    sourceid INT NOT NULL,
    subject1id INT NOT NULL,
    subject1type CHAR(1) NOT NULL,
    subject2id INT NOT NULL,
    subject2type CHAR(1) NOT NULL,
    value_role INT NOT NULL,
    disproved BOOLEAN,
    rationale TEXT(32672),
    -- PRIMARY KEY (glassertionid) --  FOREIGN KEY (SURETYPARTID) REFERENCES SURETYPART (SURETYPARTID),
    -- FOREIGN KEY (RESEARCHERID) REFERENCES RESEARCHER (RESEARCHERID),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID)
);
-- DROP TABLE assertassert;
CREATE TABLE assertassert (
    assertassertid INT NOT NULL,
    idlo INT NOT NULL,
    idhi INT NOT NULL,
    seq INT NOT NULL,
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
    reprmediaid INT NOT NULL,
    reprmedianame TEXT(128) NOT NULL,
    -- PRIMARY KEY (reprmediaid)
);
-- END new table
-- DROP TABLE representtype;
CREATE TABLE representtype (
    reprtypeid INT NOT NULL,
    name TEXT(128) NOT NULL,
    -- PRIMARY KEY (reprtypeid)
);
-- DROP TABLE representation;
CREATE TABLE representation (
    representationid INT NOT NULL,
    sourceid INT NOT NULL,
    reprtypeid INT NOT NULL,
    reprmediaid INT NOT NULL,
    physfilecode TEXT(8192),
    comments TEXT(16384),
    externallink TEXT(255),
    -- PRIMARY KEY (representationid),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (REPRTYPEID) REFERENCES REPRESENTTYPE (REPRTYPEID),
    -- FOREIGN KEY (REPRMEDIAID) REFERENCES REPRMEDIATYPE (REPRMEDIAID)
);
-- DROP TABLE search;
CREATE TABLE search (
    searchid INT NOT NULL,
    activityid INT NOT NULL,
    -- relates to ACTIVITY
    sourceid INT NOT NULL,
    repositoryid INT NOT NULL,
    searchedfor TEXT(16384) NOT NULL,
    -- PRIMARY KEY (searchid),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (REPOSITORYID) REFERENCES REPOSITORY (REPOSITORYID)
);
-- DROP TABLE eventtype;
-- GEDCOMTAG in this table is not in the GDM and is only here to accommodate
-- that standard and provide some future consistency with imports/exports
CREATE TABLE eventtype (
    eventtypeid INT NOT NULL,
    eventtypename TEXT(32) NOT NULL,
    gedcomtag TEXT(4) NOT NULL,
    -- PRIMARY KEY (eventtypeid)
);
-- DROP TABLE event;
CREATE TABLE event (
    eventid INT NOT NULL,
    eventtypeid INT NOT NULL,
    placeid INT NOT NULL,
    eventdate TEXT(96) NOT NULL,
    eventname TEXT(128) NOT NULL,
    -- PRIMARY KEY (eventid),
    -- FOREIGN KEY (EVENTTYPEID) REFERENCES EVENTTYPE (EVENTTYPEID),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE eventtyperole;
CREATE TABLE eventtyperole (
    eventtyperoleid INT NOT NULL,
    eventtypeid INT NOT NULL,
    eventtyperolename TEXT(32) NOT NULL,
    -- PRIMARY KEY (eventtyperoleid),
    -- FOREIGN KEY (EVENTTYPEID) REFERENCES EVENTTYPE (EVENTTYPEID)
);
-- DROP TABLE glgrouptype;
CREATE TABLE glgrouptype (
    glgrouptypeid INT NOT NULL,
    glgroupname TEXT(32) NOT NULL,
    ascdescnone CHAR(1) NOT NULL,
    -- PRIMARY KEY (glgrouptypeid)
);
-- DROP TABLE glgroup;
CREATE TABLE glgroup (
    glgroupid INT NOT NULL,
    glgrouptypeid INT NOT NULL,
    placeid INT NOT NULL,
    glgroupdate TEXT(96) NOT NULL,
    glgroupname TEXT(32) NOT NULL,
    glgroupcriteria TEXT(128) NOT NULL,
    -- PRIMARY KEY (glgroupid),
    -- FOREIGN KEY (GLGROUPTYPEID) REFERENCES GLGROUPTYPE (GLGROUPTYPEID),
    -- FOREIGN KEY (PLACEID) REFERENCES PLACE (PLACEID)
);
-- DROP TABLE glgrouptyperole;
CREATE TABLE glgrouptyperole (
    glgrouptyperoleid INT NOT NULL,
    glgrouptypeid INT NOT NULL,
    glgrouptypename TEXT(32) NOT NULL,
    sequencenumber INT NOT NULL,
    -- PRIMARY KEY (glgrouptyperoleid),
    -- FOREIGN KEY (GLGROUPTYPEID) REFERENCES GLGROUPTYPE (GLGROUPTYPEID)
);
-- DROP TABLE reposource;
-- The RSACTIVITYID FK in this table *should* refer to the SEARCH table rather
-- than the ACTIVITY table, but for some reason DERBY is failing to create the
-- link ...
CREATE TABLE reposource (
    repo_sourceid INT NOT NULL,
    repositoryid INT NOT NULL,
    sourceid INT NOT NULL,
    rsactivityid INT NOT NULL,
    callnumber TEXT(32),
    description TEXT(128),
    -- PRIMARY KEY (repo_sourceid),
    -- FOREIGN KEY (REPOSITORYID) REFERENCES REPOSITORY (REPOSITORYID),
    -- FOREIGN KEY (SOURCEID) REFERENCES SOURCE (SOURCEID),
    -- FOREIGN KEY (RSACTIVITYID) REFERENCES ACTIVITY (ACTIVITYID)
);
