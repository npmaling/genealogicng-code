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

pub mod dbtables {

    #[derive(Debug)]
    pub(crate) struct Activity {
        pub activityid: i64,
        pub projectid: i64,
        pub researcherid: i64,
        pub scheddate: String,
        pub completedate: String,
        pub typecode: String,
        pub status: String,
        pub description: String,
        pub priority: String,
        pub comments: String,
    }

    impl Activity {
        pub fn create_activity(
            Activity {
                activityid,
                projectid,
                researcherid,
                scheddate,
                completedate,
                typecode,
                status,
                description,
                priority,
                comments,
            }: Activity,
        ) -> String {
            let parameters = format!(
                "INSERT INTO Activity (activityid, projectid, researcherid, scheddate, completedate, typecode, status, description, priority, comments
                ) VALUES ({}, {}, {}, \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\")",
                &activityid.to_string(),
                &projectid.to_string(),
                &researcherid.to_string(),
                &scheddate,
                &completedate,
                &typecode,
                &status,
                &description,
                &priority,
                &comments
            );
            // println!("This is create_activity: {}", parameters);
            parameters
        }

        pub fn read_activity(
            Activity {
                activityid,
                projectid,
                researcherid,
                scheddate,
                completedate,
                typecode,
                status,
                description,
                priority,
                comments,
            }: Activity,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM activity WHERE activityid={}",
                &activityid.to_string(),
            );
            parameters
        }

        pub fn update_activity(
            Activity {
                activityid,
                projectid,
                researcherid,
                scheddate,
                completedate,
                typecode,
                status,
                description,
                priority,
                comments,
            }: Activity,
        ) -> String {
            let parameters = format!(
                "UPDATE activity SET projectid={}, researcherid={}, scheddate=\"{}\", completedate=\"{}\", typecode=\"{}\", status=\"{}\", description=\"{}\", priority=\"{}\", comments=\"{}\" WHERE activityid={}",
                // &activityid.to_string(),
                &projectid.to_string(),
                &researcherid.to_string(),
                &scheddate,
                &completedate,
                &typecode,
                &status,
                &description,
                &priority,
                &comments,
                &activityid.to_string(),
            );
            // println!("This is update_activity: {}", parameters);
            parameters
        }

        pub fn delete_activity(
            Activity {
                activityid,
                projectid,
                researcherid,
                scheddate,
                completedate,
                typecode,
                status,
                description,
                priority,
                comments,
            }: Activity,
        ) -> String {
            let parameters = format!(
                "DELETE FROM activity WHERE personaid={}",
                &activityid.to_string(),
            );
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Persona {
        pub personaid: i64,
        pub persona_name: String,
        pub description_comments: String,
    }

    impl Persona {
        pub fn create_persona(
            Persona {
                personaid,
                persona_name,
                description_comments,
            }: Persona,
        ) -> String {
            let parameters = format!(
                "INSERT INTO Persona (personaid, persona_name, description_comments) VALUES ({}, \"{}\", \"{}\")",
                &personaid.to_string(),
                &persona_name,
                &description_comments
            );
            parameters
        }

        pub fn read_persona(
            Persona {
                personaid,
                persona_name,
                description_comments,
            }: Persona,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM Persona WHERE personaid={}",
                &personaid.to_string(),
            );
            // println!("This is read_persona: {}", parameters);
            parameters
        }

        pub fn update_persona(
            Persona {
                personaid,
                persona_name,
                description_comments,
            }: Persona,
        ) -> String {
            let parameters = format!(
                "UPDATE persona SET personaid={}, persona_name=\"{}\", description_comments=\"{}\" WHERE personaid={}",
                &personaid.to_string(),
                &persona_name,
                &description_comments,
                &personaid.to_string(),
            );
            // println!("This is update_persona: {}", parameters);
            parameters
        }

        pub fn delete_persona(
            Persona {
                personaid,
                persona_name,
                description_comments,
            }: Persona,
        ) -> String {
            let parameters = format!(
                "DELETE FROM persona WHERE personaid={}",
                &personaid.to_string(),
            );
            // println!("This is delete_persona: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Place {
        pub placeid: i64,
        pub startdate: String,
        pub enddate: String,
        pub ascdescnone: String,
        pub placecomment: String,
    }

    impl Place {
        pub fn create_place(
            Place {
                placeid,
                startdate,
                enddate,
                ascdescnone,
                placecomment,
            }: Place,
        ) -> String {
            let parameters = format!(
                "INSERT INTO Place (placeid, startdate, enddate, ascdescnone, placecomment) VALUES ({}, \"{}\", \"{}\", \"{}\", \"{}\")",
                &placeid.to_string(),
                &startdate,
                &enddate,
                &ascdescnone,
                &placecomment
            );
            // println!("This is create_place: {}", parameters);
            parameters
        }

        pub fn read_place(
            Place {
                placeid,
                startdate,
                enddate,
                ascdescnone,
                placecomment,
            }: Place,
        ) -> String {
            let parameters = format!("SELECT * FROM Place WHERE placeid={}", &placeid.to_string());
            // println!("This is read_place: {}", parameters);
            parameters
        }

        pub fn update_place(
            Place {
                placeid,
                startdate,
                enddate,
                ascdescnone,
                placecomment,
            }: Place,
        ) -> String {
            let parameters = format!(
                "UPDATE place SET placeid={}, startdate=\"{}\", enddate=\"{}\", ascdescnone=\"{}\", placecomment=\"{}\" WHERE placeid={}",
                &placeid.to_string(),
                &startdate,
                &enddate,
                &ascdescnone,
                &placecomment,
                &placeid.to_string()
            );
            // println!("This is update_place: {}", parameters);
            parameters
        }

        pub fn delete_place(
            Place {
                placeid,
                startdate,
                enddate,
                ascdescnone,
                placecomment,
            }: Place,
        ) -> String {
            let parameters = format!("DELETE FROM place WHERE placeid={}", &placeid.to_string());
            // println!("This is delete_place: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct PlacePartType {
        pub placeparttypeid: i64,
        pub pptname: String,
    }

    impl PlacePartType {
        pub fn create_placeparttype(
            PlacePartType {
                placeparttypeid,
                pptname,
            }: PlacePartType,
        ) -> String {
            let parameters = format!(
                "INSERT INTO PlacePartType (placeparttypeid, pptname) VALUES ({}, \"{}\")",
                &placeparttypeid.to_string(),
                &pptname
            );
            // println!("This is create_placePartType: {}", parameters);
            parameters
        }

        pub fn read_placeparttype(
            PlacePartType {
                placeparttypeid,
                pptname,
            }: PlacePartType,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM PlacePartType WHERE placeparttypeid={}",
                &placeparttypeid.to_string(),
            );
            // println!("This is read_placePartType: {}", parameters);
            parameters
        }

        pub fn update_placeparttype(
            PlacePartType {
                placeparttypeid,
                pptname,
            }: PlacePartType,
        ) -> String {
            let parameters = format!(
                "UPDATE placePartType SET placeparttypeid={}, pptname=\"{}\" WHERE placeparttypeid={}",
                &placeparttypeid.to_string(),
                &pptname,
                &placeparttypeid.to_string()
            );
            // println!("This is update_placePartType: {}", parameters);
            parameters
        }

        pub fn delete_placeparttype(
            PlacePartType {
                placeparttypeid,
                pptname,
            }: PlacePartType,
        ) -> String {
            let parameters = format!(
                "DELETE FROM PlacePartType WHERE placeparttypeid={}",
                &placeparttypeid.to_string(),
            );
            // println!("This is delete_placePartType: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct PlacePart {
        pub placepartid: i64,
        pub placeid: i64,
        pub placeparttypeid: i64,
        pub name: String,
        pub sequencenumber: i64,
    }

    impl PlacePart {
        pub fn create_placepart(
            PlacePart {
                placepartid,
                placeid,
                placeparttypeid,
                name,
                sequencenumber,
            }: PlacePart,
        ) -> String {
            let parameters = format!(
                "INSERT INTO PlacePart (placepartid, placeid, placeparttypeid, name, sequencenumber) VALUES ({}, {}, {}, \"{}\", {})",
                &placepartid.to_string(),
                &placeid.to_string(),
                &placeparttypeid.to_string(),
                &name,
                &sequencenumber.to_string(),
            );
            // println!("This is create_placepart: {}", parameters);
            parameters
        }

        pub fn read_placepart(
            PlacePart {
                placepartid,
                placeid,
                placeparttypeid,
                name,
                sequencenumber,
            }: PlacePart,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM PlacePart WHERE placepartid={}",
                &placepartid.to_string(),
            );
            // println!("This is read_placepart: {}", parameters);
            parameters
        }

        pub fn update_placepart(
            PlacePart {
                placepartid,
                placeid,
                placeparttypeid,
                name,
                sequencenumber,
            }: PlacePart,
        ) -> String {
            let parameters = format!(
                "UPDATE placePart SET placepartid={}, placeid={}, placeparttypeid={}, name=\"{}\", sequencenumber={} WHERE placepartid={}",
                &placepartid.to_string(),
                &placeid.to_string(),
                &placeparttypeid.to_string(),
                &name,
                &sequencenumber.to_string(),
                &placepartid.to_string()
            );
            // println!("This is update_placepart: {}", parameters);
            parameters
        }

        pub fn delete_placepart(
            PlacePart {
                placepartid,
                placeid,
                placeparttypeid,
                name,
                sequencenumber,
            }: PlacePart,
        ) -> String {
            let parameters = format!(
                "DELETE FROM PlacePart WHERE placepartid={}",
                &placepartid.to_string(),
            );
            // println!("This is delete_placepart: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Characteristic {
        pub characteristicid: i64,
        pub placeid: i64,
        pub characteristicdate: String,
        pub ascdescnone: String,
    }

    impl Characteristic {
        pub fn create_characteristic(
            Characteristic {
                characteristicid,
                placeid,
                characteristicdate,
                ascdescnone,
            }: Characteristic,
        ) -> String {
            let parameters = format!(
                "INSERT INTO characteristic (characteristicid, placeid, characteristicdate, ascdescnone) VALUES ({}, {}, \"{}\", \"{}\")",
                &characteristicid.to_string(),
                &placeid.to_string(),
                &characteristicdate,
                &ascdescnone,
            );
            // println!("This is create_characteristic: {}", parameters);
            parameters
        }

        pub fn read_characteristic(
            Characteristic {
                characteristicid,
                placeid,
                characteristicdate,
                ascdescnone,
            }: Characteristic,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM characteristic WHERE characteristicid={}",
                &characteristicid.to_string(),
            );
            // println!("This is read_characteristic: {}", parameters);
            parameters
        }

        pub fn update_characteristic(
            Characteristic {
                characteristicid,
                placeid,
                characteristicdate,
                ascdescnone,
            }: Characteristic,
        ) -> String {
            let parameters = format!(
                "UPDATE characteristic SET characteristicid={}, placeid={}, characteristicdate={}, ascdescnone=\"{}\" WHERE characteristicid={}",
                &characteristicid.to_string(),
                &placeid.to_string(),
                &characteristicdate,
                &ascdescnone,
                &characteristicid.to_string()
            );
            // println!("This is update_characteristic: {}", parameters);
            parameters
        }

        pub fn delete_characteristic(
            Characteristic {
                characteristicid,
                placeid,
                characteristicdate,
                ascdescnone,
            }: Characteristic,
        ) -> String {
            let parameters = format!(
                "DELETE FROM characteristic WHERE characteristicid={}",
                &characteristicid.to_string(),
            );
            // println!("This is delete_characteristic: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct CharPartType {
        pub charparttypeid: i64,
        pub charparttypename: String,
        pub gedcomtag: String,
    }

    impl CharPartType {
        pub fn create_charparttype(
            CharPartType {
                charparttypeid,
                charparttypename,
                gedcomtag,
            }: CharPartType,
        ) -> String {
            let parameters = format!(
                "INSERT INTO charparttype (charparttypeid, charparttypename, gedcomtag) VALUES ({}, \"{}\", \"{}\")",
                &charparttypeid.to_string(),
                &charparttypename,
                &gedcomtag,
            );
            // println!("This is create_charparttype: {}", parameters);
            parameters
        }

        pub fn read_charparttype(
            CharPartType {
                charparttypeid,
                charparttypename,
                gedcomtag,
            }: CharPartType,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM charparttype WHERE charparttypeid={}",
                &charparttypeid.to_string(),
            );
            // println!("This is read_charparttype: {}", parameters);
            parameters
        }

        pub fn update_charparttype(
            CharPartType {
                charparttypeid,
                charparttypename,
                gedcomtag,
            }: CharPartType,
        ) -> String {
            let parameters = format!(
                "UPDATE charparttype SET charparttypeid={}, charparttypename=\"{}\", gedcomtag=\"{}\" WHERE charparttypeid={}",
                &charparttypeid.to_string(),
                &charparttypename,
                &gedcomtag,
                &charparttypeid.to_string(),
            );
            // println!("This is update_charparttype: {}", parameters);
            parameters
        }

        pub fn delete_charparttype(
            CharPartType {
                charparttypeid,
                charparttypename,
                gedcomtag,
            }: CharPartType,
        ) -> String {
            let parameters = format!(
                "DELETE FROM charparttype WHERE charparttypeid={}",
                &charparttypeid.to_string(),
            );
            // println!("This is delete_charparttype: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct CharPart {
        pub characteristicpartid: i64,
        pub characteristicid: i64,
        pub charparttypeid: i64,
        pub charpartname: String,
        pub charpartseq: i64,
    }

    impl CharPart {
        pub fn create_charpart(
            CharPart {
                characteristicpartid,
                characteristicid,
                charparttypeid,
                charpartname,
                charpartseq,
            }: CharPart,
        ) -> String {
            let parameters = format!(
                "INSERT INTO charpart (characteristicpartid, characteristicid, charparttypeid, charpartname, charpartseq) VALUES ({}, {}, {}, \"{}\", \"{}\")",
                &characteristicpartid.to_string(),
                &characteristicid.to_string(),
                &charparttypeid.to_string(),
                &charpartname,
                &charpartseq.to_string(),
            );
            // println!("This is create_charpart: {}", parameters);
            parameters
        }

        pub fn read_charpart(
            CharPart {
                characteristicpartid,
                characteristicid,
                charparttypeid,
                charpartname,
                charpartseq,
            }: CharPart,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM charpart WHERE characteristicpartid={}",
                &characteristicpartid.to_string(),
            );
            // println!("This is read_charpart: {}", parameters);
            parameters
        }

        pub fn update_charpart(
            CharPart {
                characteristicpartid,
                characteristicid,
                charparttypeid,
                charpartname,
                charpartseq,
            }: CharPart,
        ) -> String {
            let parameters = format!(
                "UPDATE charpart SET characteristicpartid={}, characteristicid={}, charparttypeid={}, charpartname=\"{}\", charpartseq={} WHERE characteristicpartid={}",
                &characteristicpartid.to_string(),
                &characteristicid.to_string(),
                &charparttypeid.to_string(),
                &charpartname,
                &charpartseq.to_string(),
                &characteristicpartid.to_string(),
            );
            // println!("This is update_charpart: {}", parameters);
            parameters
        }

        pub fn delete_charpart(
            CharPart {
                characteristicpartid,
                characteristicid,
                charparttypeid,
                charpartname,
                charpartseq,
            }: CharPart,
        ) -> String {
            let parameters = format!(
                "DELETE FROM charpart WHERE characteristicpartid={}",
                &characteristicpartid.to_string(),
            );
            // println!("This is delete_charpart: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Project {
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

    #[derive(Debug)]
    pub(crate) struct Repository {
        pub repositoryid: i64,
        pub placeid: i64,
        pub reponame: String,
        pub comments: String,
    }

    impl Repository {
        pub fn create_repository(
            Repository {
                repositoryid,
                placeid,
                reponame,
                comments,
            }: Repository,
        ) -> String {
            let parameters = format!(
                "INSERT INTO repository (repositoryid, placeid, reponame, comments) VALUES ({}, {}, \"{}\", \"{}\")",
                &repositoryid.to_string(),
                &placeid.to_string(),
                &reponame,
                &comments,
            );
            // println!("This is create_repository: {}", parameters);
            parameters
        }

        pub fn read_repository(
            Repository {
                repositoryid,
                placeid,
                reponame,
                comments,
            }: Repository,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM repository WHERE repositoryid={}",
                &repositoryid.to_string(),
            );
            // println!("This is read_repository: {}", parameters);
            parameters
        }

        pub fn update_repository(
            Repository {
                repositoryid,
                placeid,
                reponame,
                comments,
            }: Repository,
        ) -> String {
            let parameters = format!(
                "UPDATE repository SET repositoryid={}, placeid={}, reponame=\"{}\", comments=\"{}\" WHERE repositoryid={}",
                &repositoryid.to_string(),
                &placeid.to_string(),
                &reponame,
                &comments,
                &repositoryid.to_string(),
            );
            // println!("This is update_repository: {}", parameters);
            parameters
        }

        pub fn delete_repository(
            Repository {
                repositoryid,
                placeid,
                reponame,
                comments,
            }: Repository,
        ) -> String {
            let parameters = format!(
                "DELETE FROM repository WHERE repositoryid={}",
                &repositoryid.to_string(),
            );
            // println!("This is delete_repository: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct ResObjective {
        pub resobjid: i64,
        pub projectid: i64,
        pub subjectid: i64,
        pub subjecttype: String,
        pub name: String,
        pub description: String,
        pub sequencenumber: i64,
        pub priority: String,
        pub status: String,
    }

    impl ResObjective {
        pub fn create_resobjective(
            ResObjective {
                resobjid,
                projectid,
                subjectid,
                subjecttype,
                name,
                description,
                sequencenumber,
                priority,
                status,
            }: ResObjective,
        ) -> String {
            let parameters = format!(
                "INSERT INTO resobjective (resobjid, projectid, subjectid, subjecttype, name, description, sequencenumber, priority, status) VALUES ({}, {}, {}, \"{}\", \"{}\", \"{}\", {}, \"{}\", \"{}\")",
                &resobjid.to_string(),
                &projectid.to_string(),
                &subjectid.to_string(),
                &subjecttype,
                &name,
                &description,
                &sequencenumber.to_string(),
                &priority,
                &status,
            );
            // println!("This is create_resobjective: {}", parameters);
            parameters
        }

        pub fn read_resobjective(
            ResObjective {
                resobjid,
                projectid,
                subjectid,
                subjecttype,
                name,
                description,
                sequencenumber,
                priority,
                status,
            }: ResObjective,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM resobjective WHERE resobjid={}",
                &resobjid.to_string(),
            );
            // println!("This is read_resobjective: {}", parameters);
            parameters
        }

        pub fn update_resobjective(
            ResObjective {
                resobjid,
                projectid,
                subjectid,
                subjecttype,
                name,
                description,
                sequencenumber,
                priority,
                status,
            }: ResObjective,
        ) -> String {
            let parameters = format!(
                "UPDATE resobjective SET resobjid={}, projectid={}, subjectid={}, subjecttype=\"{}\", name=\"{}\", description=\"{}\", sequencenumber={}, priority=\"{}\", status=\"{}\" WHERE resobjid={}",
                &resobjid.to_string(),
                &projectid.to_string(),
                &subjectid.to_string(),
                &subjecttype,
                &name,
                &description,
                &sequencenumber.to_string(),
                &priority,
                &status,
                &resobjid.to_string(),
            );
            // println!("This is update_resobjective: {}", parameters);
            parameters
        }

        pub fn delete_resobjective(
            ResObjective {
                resobjid,
                projectid,
                subjectid,
                subjecttype,
                name,
                description,
                sequencenumber,
                priority,
                status,
            }: ResObjective,
        ) -> String {
            let parameters = format!(
                "DELETE FROM ResObjective WHERE resobjid={}",
                &resobjid.to_string(),
            );
            // println!("This is delete_resobjective: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct ResObjActivity {
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

    #[derive(Debug)]
    pub(crate) struct Researcher {
        pub researcherid: i64,
        pub name: String,
        pub addressid: i64,
        pub comments: String,
    }

    impl Researcher {
        pub fn create_researcher(
            Researcher {
                researcherid,
                name,
                addressid,
                comments,
            }: Researcher,
        ) -> String {
            let parameters = format!(
                "INSERT INTO researcher (researcherid, name, addressid, comments) VALUES ({}, \"{}\", {}, \"{}\")",
                &researcherid.to_string(),
                &name,
                &addressid.to_string(),
                &comments,
            );
            // println!("This is create_researcher: {}", parameters);
            parameters
        }

        pub fn read_researcher(
            Researcher {
                researcherid,
                name,
                addressid,
                comments,
            }: Researcher,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM researcher WHERE researcherid={}",
                &researcherid.to_string(),
            );
            // println!("This is read_researcher: {}", parameters);
            parameters
        }

        pub fn update_researcher(
            Researcher {
                researcherid,
                name,
                addressid,
                comments,
            }: Researcher,
        ) -> String {
            let parameters = format!(
                "UPDATE researcher SET researcherid={}, name=\"{}\", addressid={}, comments=\"{}\" WHERE researcherid={}",
                &researcherid.to_string(),
                &name,
                &addressid.to_string(),
                &comments,
                &researcherid.to_string(),
            );
            // println!("This is update_researcher: {}", parameters);
            parameters
        }

        pub fn delete_researcher(
            Researcher {
                researcherid,
                name,
                addressid,
                comments,
            }: Researcher,
        ) -> String {
            let parameters = format!(
                "DELETE FROM researcher WHERE researcherid={}",
                &researcherid.to_string(),
            );
            // println!("This is delete_researcher: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct ResProj {
        pub resprojid: i64,
        pub projectid: i64,
        pub researcherid: i64,
        pub researcherrole: String,
    }

    impl ResProj {
        pub fn create_resproj(
            ResProj {
                resprojid,
                projectid,
                researcherid,
                researcherrole,
            }: ResProj,
        ) -> String {
            let parameters = format!(
                "INSERT INTO resproj (resprojid, projectid, researcherid, researcherrole) VALUES ({}, {}, {}, \"{}\")",
                &resprojid.to_string(),
                &projectid.to_string(),
                &researcherid.to_string(),
                &researcherrole,
            );
            // println!("This is create_resproj: {}", parameters);
            parameters
        }

        pub fn read_resproj(
            ResProj {
                resprojid,
                projectid,
                researcherid,
                researcherrole,
            }: ResProj,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM resproj WHERE resprojid={}",
                &resprojid.to_string(),
            );
            // println!("This is read_resproj: {}", parameters);
            parameters
        }

        pub fn update_resproj(
            ResProj {
                resprojid,
                projectid,
                researcherid,
                researcherrole,
            }: ResProj,
        ) -> String {
            let parameters = format!(
                "UPDATE resproj SET resprojid={}, projectid={}, researcherid={}, researcherrole=\"{}\" WHERE resprojid={}",
                &resprojid.to_string(),
                &projectid.to_string(),
                &researcherid.to_string(),
                &researcherrole,
                &resprojid.to_string(),
            );
            // println!("This is update_resproj: {}", parameters);
            parameters
        }

        pub fn delete_resproj(
            ResProj {
                resprojid,
                projectid,
                researcherid,
                researcherrole,
            }: ResProj,
        ) -> String {
            let parameters = format!(
                "DELETE FROM resproj WHERE resprojid={}",
                &resprojid.to_string(),
            );
            // println!("This is delete_resproj: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Source {
        pub sourceid: i64,
        pub highersourceid: i64,
        pub subjectplaceid: i64,
        pub jurisplaceid: i64,
        pub researcherid: i64,
        pub subjectdate: String,
        pub comments: String,
    }

    impl Source {
        pub fn create_source(
            Source {
                sourceid,
                highersourceid,
                subjectplaceid,
                jurisplaceid,
                researcherid,
                subjectdate,
                comments,
            }: Source,
        ) -> String {
            let parameters = format!(
                "INSERT INTO source (sourceid, highersourceid, subjectplaceid, jurisplaceid, researcherid, subjectdate, comments) VALUES ({}, {}, {}, {}, {}, \"{}\", \"{}\")",
                &sourceid.to_string(),
                &highersourceid.to_string(),
                &subjectplaceid.to_string(),
                &jurisplaceid.to_string(),
                &researcherid.to_string(),
                &subjectdate,
                &comments,
            );
            // println!("This is create_source: {}", parameters);
            parameters
        }

        pub fn read_source(
            Source {
                sourceid,
                highersourceid,
                subjectplaceid,
                jurisplaceid,
                researcherid,
                subjectdate,
                comments,
            }: Source,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM source WHERE sourceid={}",
                &sourceid.to_string(),
            );
            // println!("This is read_source: {}", parameters);
            parameters
        }

        pub fn update_source(
            Source {
                sourceid,
                highersourceid,
                subjectplaceid,
                jurisplaceid,
                researcherid,
                subjectdate,
                comments,
            }: Source,
        ) -> String {
            let parameters = format!(
                "UPDATE source SET sourceid={}, highersourceid={}, subjectplaceid={}, jurisplaceid={}, researcherid={}, subjectdate=\"{}\", comments=\"{}\" WHERE sourceid={}",
                &sourceid.to_string(),
                &highersourceid.to_string(),
                &subjectplaceid.to_string(),
                &jurisplaceid.to_string(),
                &researcherid.to_string(),
                &subjectdate,
                &comments,
                &sourceid.to_string(),
            );
            // println!("This is update_source: {}", parameters);
            parameters
        }

        pub fn delete_source(
            Source {
                sourceid,
                highersourceid,
                subjectplaceid,
                jurisplaceid,
                researcherid,
                subjectdate,
                comments,
            }: Source,
        ) -> String {
            let parameters = format!(
                "DELETE FROM source WHERE sourceid={}",
                &sourceid.to_string(),
            );
            // println!("This is delete_source: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct SourceGroup {
        pub sourcegroupid: i64,
        pub sourcegroupname: String,
    }

    impl SourceGroup {
        pub fn create_sourcegroup(
            SourceGroup {
                sourcegroupid,
                sourcegroupname,
            }: SourceGroup,
        ) -> String {
            let parameters = format!(
                "INSERT INTO sourcegroup (sourcegroupid, sourcegroupname) VALUES ({}, \"{}\")",
                &sourcegroupid.to_string(),
                &sourcegroupname,
            );
            // println!("This is create_sourcegroup: {}", parameters);
            parameters
        }

        pub fn read_sourcegroup(
            SourceGroup {
                sourcegroupid,
                sourcegroupname,
            }: SourceGroup,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM sourcegroup WHERE sourcegroupid={}",
                &sourcegroupid.to_string(),
            );
            // println!("This is read_sourcegroup: {}", parameters);
            parameters
        }

        pub fn update_sourcegroup(
            SourceGroup {
                sourcegroupid,
                sourcegroupname,
            }: SourceGroup,
        ) -> String {
            let parameters = format!(
                "UPDATE sourcegroup SET sourcegroupid={}, sourcegroupname=\"{}\" WHERE sourcegroupid={}",
                &sourcegroupid.to_string(),
                &sourcegroupname,
                &sourcegroupid.to_string(),
            );
            // println!("This is update_sourcegroup: {}", parameters);
            parameters
        }

        pub fn delete_sourcegroup(
            SourceGroup {
                sourcegroupid,
                sourcegroupname,
            }: SourceGroup,
        ) -> String {
            let parameters = format!(
                "DELETE FROM sourcegroup WHERE sourcegroupid={}",
                &sourcegroupid.to_string(),
            );
            // println!("This is delete_sourcegroup: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct SrcGrpSrc {
        pub srcgrpsrcid: i64,
        pub sourceid: i64,
        pub sourcegroupid: i64,
    }

    impl SrcGrpSrc {
        pub fn create_srcgrpsrc(
            SrcGrpSrc {
                srcgrpsrcid,
                sourceid,
                sourcegroupid,
            }: SrcGrpSrc,
        ) -> String {
            let parameters = format!(
                "INSERT INTO srcgrpsrc (srcgrpsrcid, sourceid, sourcegroupid) VALUES ({}, {}, {})",
                srcgrpsrcid.to_string(),
                sourceid.to_string(),
                sourcegroupid.to_string(),
            );
            // println!("This is create_srcgrpsrc: {}", parameters);
            parameters
        }

        pub fn read_srcgrpsrc(
            SrcGrpSrc {
                srcgrpsrcid,
                sourceid,
                sourcegroupid,
            }: SrcGrpSrc,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM srcgrpsrc WHERE srcgrpsrcid={}",
                srcgrpsrcid.to_string(),
            );
            // println!("This is read_srcgrpsrc: {}", parameters);
            parameters
        }

        pub fn update_srcgrpsrc(
            SrcGrpSrc {
                srcgrpsrcid,
                sourceid,
                sourcegroupid,
            }: SrcGrpSrc,
        ) -> String {
            let parameters = format!(
                "UPDATE srcgrpsrc SET srcgrpsrcid={}, sourceid={}, sourcegroupid={} WHERE srcgrpsrcid={}",
                srcgrpsrcid.to_string(),
                sourceid.to_string(),
                sourcegroupid.to_string(),
                srcgrpsrcid.to_string(),
            );
            // println!("This is update_srcgrpsrc: {}", parameters);
            parameters
        }

        pub fn delete_srcgrpsrc(
            SrcGrpSrc {
                srcgrpsrcid,
                sourceid,
                sourcegroupid,
            }: SrcGrpSrc,
        ) -> String {
            let parameters = format!(
                "DELETE FROM srcgrpsrc WHERE srcgrpsrcid={}",
                &srcgrpsrcid.to_string(),
            );
            // println!("This is delete_srcgrpsrc: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct CitationPartType {
        pub citationparttypeid: i64,
        pub citationparttypename: String,
    }

    impl CitationPartType {
        pub fn create_citationparttype(
            CitationPartType {
                citationparttypeid,
                citationparttypename,
            }: CitationPartType,
        ) -> String {
            let parameters = format!(
                "INSERT INTO citationparttype (citationparttypeid, citationparttypename) VALUES ({}, \"{}\")",
                citationparttypeid.to_string(),
                citationparttypename,
            );
            // println!("This is create_citationparttype: {}", parameters);
            parameters
        }

        pub fn read_citationparttype(
            CitationPartType {
                citationparttypeid,
                citationparttypename,
            }: CitationPartType,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM citationparttype WHERE citationparttypeid={}",
                citationparttypeid.to_string(),
            );
            // println!("This is read_citationparttype: {}", parameters);
            parameters
        }

        pub fn update_citationparttype(
            CitationPartType {
                citationparttypeid,
                citationparttypename,
            }: CitationPartType,
        ) -> String {
            let parameters = format!(
                "UPDATE citationparttype SET citationparttypeid={}, citationparttypename=\"{}\" WHERE citationparttypeid={}",
                citationparttypeid.to_string(),
                citationparttypename,
                citationparttypeid.to_string(),
            );
            // println!("This is update_citationparttype: {}", parameters);
            parameters
        }

        pub fn delete_citationparttype(
            CitationPartType {
                citationparttypeid,
                citationparttypename,
            }: CitationPartType,
        ) -> String {
            let parameters = format!(
                "DELETE FROM citationparttype WHERE citationparttypeid={}",
                citationparttypeid.to_string(),
            );
            // println!("This is delete_citationparttype: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct CitationPart {
        pub citationpartid: i64,
        pub sourceid: i64,
        pub citeparttypeid: i64,
        pub citepartvalue: String,
    }

    impl CitationPart {
        pub fn create_citationpart(
            CitationPart {
                citationpartid,
                sourceid,
                citeparttypeid,
                citepartvalue,
            }: CitationPart,
        ) -> String {
            let parameters = format!(
                "INSERT INTO citationpart (citationpartid, sourceid, citeparttypeid, citepartvalue) VALUES ({}, {}, {}, \"{}\")",
                citationpartid.to_string(),
                sourceid.to_string(),
                citeparttypeid.to_string(),
                citepartvalue,
            );
            // println!("This is create_citationpart: {}", parameters);
            parameters
        }

        pub fn read_citationpart(
            CitationPart {
                citationpartid,
                sourceid,
                citeparttypeid,
                citepartvalue,
            }: CitationPart,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM citationpart WHERE citationpartid={}",
                citationpartid.to_string(),
            );
            // println!("This is read_citationpart: {}", parameters);
            parameters
        }

        pub fn update_citationpart(
            CitationPart {
                citationpartid,
                sourceid,
                citeparttypeid,
                citepartvalue,
            }: CitationPart,
        ) -> String {
            let parameters = format!(
                "UPDATE citationpart SET citationpartid={}, sourceid={}, citeparttypeid={}, citepartvalue=\"{}\" WHERE citationpartid={}",
                citationpartid.to_string(),
                sourceid.to_string(),
                citeparttypeid.to_string(),
                citepartvalue,
                citationpartid.to_string(),
            );
            // println!("This is update_citationpart: {}", parameters);
            parameters
        }

        pub fn delete_citationpart(
            CitationPart {
                citationpartid,
                sourceid,
                citeparttypeid,
                citepartvalue,
            }: CitationPart,
        ) -> String {
            let parameters = format!(
                "DELETE FROM citationpart WHERE citationpartid={}",
                citationpartid.to_string(),
            );
            // println!("This is delete_citationpart: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct SuretyScheme {
        pub suretyschemeid: i64,
        pub name: String,
        pub description: String,
    }

    impl SuretyScheme {
        pub fn create_suretyscheme(
            SuretyScheme {
                suretyschemeid,
                name,
                description,
            }: SuretyScheme,
        ) -> String {
            let parameters = format!(
                "INSERT INTO suretyscheme (suretyschemeid, name, description) VALUES ({}, \"{}\", \"{}\")",
                suretyschemeid.to_string(),
                name,
                description,
            );
            // println!("This is create_suretyscheme: {}", parameters);
            parameters
        }

        pub fn read_suretyscheme(
            SuretyScheme {
                suretyschemeid,
                name,
                description,
            }: SuretyScheme,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM suretyscheme WHERE suretyschemeid={}",
                suretyschemeid.to_string(),
            );
            // println!("This is read_suretyscheme: {}", parameters);
            parameters
        }

        pub fn update_suretyscheme(
            SuretyScheme {
                suretyschemeid,
                name,
                description,
            }: SuretyScheme,
        ) -> String {
            let parameters = format!(
                "UPDATE suretyscheme SET suretyschemeid={}, name=\"{}\", description=\"{}\" WHERE suretyschemeid={}",
                suretyschemeid.to_string(),
                name,
                description,
                suretyschemeid.to_string(),
            );
            // println!("This is update_suretyscheme: {}", parameters);
            parameters
        }

        pub fn delete_suretyscheme(
            SuretyScheme {
                suretyschemeid,
                name,
                description,
            }: SuretyScheme,
        ) -> String {
            let parameters = format!(
                "DELETE FROM suretyscheme WHERE suretyschemeid={}",
                suretyschemeid.to_string(),
            );
            // println!("This is delete_suretyscheme: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct SuretyPart {
        pub suretypartid: i64,
        pub schemeid: i64,
        pub name: String,
        pub description: String,
        pub sequencenumber: i64,
    }

    impl SuretyPart {
        pub fn create_suretypart(
            SuretyPart {
                suretypartid,
                schemeid,
                name,
                description,
                sequencenumber,
            }: SuretyPart,
        ) -> String {
            let parameters = format!(
                "INSERT INTO suretypart (suretypartid, schemeid, name, description, sequencenumber) VALUES ({}, {}, \"{}\", \"{}\", {})",
                suretypartid.to_string(),
                schemeid.to_string(),
                name,
                description,
                sequencenumber.to_string(),
            );
            // println!("This is create_suretypart: {}", parameters);
            parameters
        }

        pub fn read_suretypart(
            SuretyPart {
                suretypartid,
                schemeid,
                name,
                description,
                sequencenumber,
            }: SuretyPart,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM suretypart WHERE suretypartid={}",
                suretypartid.to_string(),
            );
            // println!("This is read_suretypart: {}", parameters);
            parameters
        }

        pub fn update_suretypart(
            SuretyPart {
                suretypartid,
                schemeid,
                name,
                description,
                sequencenumber,
            }: SuretyPart,
        ) -> String {
            let parameters = format!(
                "UPDATE suretypart SET suretypartid={}, schemeid={}, name=\"{}\", description=\"{}\", sequencenumber={} WHERE suretypartid={}",
                suretypartid.to_string(),
                schemeid.to_string(),
                name,
                description,
                sequencenumber.to_string(),
                suretypartid.to_string(),
            );
            // println!("This is update_suretypart: {}", parameters);
            parameters
        }

        pub fn delete_suretypart(
            SuretyPart {
                suretypartid,
                schemeid,
                name,
                description,
                sequencenumber,
            }: SuretyPart,
        ) -> String {
            let parameters = format!(
                "DELETE FROM suretypart WHERE suretypartid={}",
                suretypartid.to_string(),
            );
            // println!("This is delete_suretypart: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct GlAssertion {
        pub glassertionid: i64,
        pub suretypartid: i64,
        pub researcherid: i64,
        pub sourceid: i64,
        pub subject1id: i64,
        pub subject1type: String,
        pub subject2id: i64,
        pub subject2type: String,
        pub value_role: i64,
        pub disproved: String,
        pub rationale: String,
    }

    impl GlAssertion {
        pub fn create_glassertion(
            GlAssertion {
                glassertionid,
                suretypartid,
                researcherid,
                sourceid,
                subject1id,
                subject1type,
                subject2id,
                subject2type,
                value_role,
                disproved,
                rationale,
            }: GlAssertion,
        ) -> String {
            let parameters = format!(
                "INSERT INTO glassertion (glassertionid, suretypartid, researcherid, sourceid, subject1id, subject1type, subject2id, subject2type, value_role, disproved, rationale) VALUES ({}, {}, {}, {}, {}, \"{}\", {}, \"{}\", {}, \"{}\", \"{}\")",
                glassertionid.to_string(),
                suretypartid.to_string(),
                researcherid.to_string(),
                sourceid.to_string(),
                subject1id.to_string(),
                subject1type,
                subject2id.to_string(),
                subject2type,
                value_role.to_string(),
                disproved,
                rationale,
            );
            // println!("This is create_glassertion: {}", parameters);
            parameters
        }

        pub fn read_glassertion(
            GlAssertion {
                glassertionid,
                suretypartid,
                researcherid,
                sourceid,
                subject1id,
                subject1type,
                subject2id,
                subject2type,
                value_role,
                disproved,
                rationale,
            }: GlAssertion,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM glassertion WHERE glassertionid={}",
                glassertionid.to_string(),
            );
            // println!("This is read_glassertion: {}", parameters);
            parameters
        }

        pub fn update_glassertion(
            GlAssertion {
                glassertionid,
                suretypartid,
                researcherid,
                sourceid,
                subject1id,
                subject1type,
                subject2id,
                subject2type,
                value_role,
                disproved,
                rationale,
            }: GlAssertion,
        ) -> String {
            let parameters = format!(
                "UPDATE glassertion SET glassertionid={}, suretypartid={}, researcherid={}, sourceid={}, subject1id={}, subject1type=\"{}\", subject2id={}, subject2type=\"{}\", value_role={}, disproved=\"{}\", rationale=\"{}\" WHERE glassertionid={}",
                glassertionid.to_string(),
                suretypartid.to_string(),
                researcherid.to_string(),
                sourceid.to_string(),
                subject1id.to_string(),
                subject1type,
                subject2id.to_string(),
                subject2type,
                value_role.to_string(),
                disproved,
                rationale,
                glassertionid.to_string(),
            );
            // println!("This is update_glassertion: {}", parameters);
            parameters
        }

        pub fn delete_glassertion(
            GlAssertion {
                glassertionid,
                suretypartid,
                researcherid,
                sourceid,
                subject1id,
                subject1type,
                subject2id,
                subject2type,
                value_role,
                disproved,
                rationale,
            }: GlAssertion,
        ) -> String {
            let parameters = format!(
                "DELETE FROM glassertion WHERE glassertionid={}",
                glassertionid.to_string(),
            );
            // println!("This is delete_glassertion: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct AssertAssert {
        pub assertassertid: i64,
        pub idlo: i64,
        pub idhi: i64,
        pub seq: i64,
    }

    impl AssertAssert {
        pub fn create_assertassert(
            AssertAssert {
                assertassertid,
                idlo,
                idhi,
                seq,
            }: AssertAssert,
        ) -> String {
            let parameters = format!(
                "INSERT INTO assertassert (assertassertid, idlo, idhi, seq) VALUES ({}, {}, {}, {})",
                assertassertid.to_string(),
                idlo.to_string(),
                idhi.to_string(),
                seq.to_string(),
            );
            // println!("This is create_assertassert: {}", parameters);
            parameters
        }

        pub fn read_assertassert(
            AssertAssert {
                assertassertid,
                idlo,
                idhi,
                seq,
            }: AssertAssert,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM assertassert WHERE assertassertid={}",
                assertassertid.to_string(),
                //                idlo.to_string(),
                //                idhi.to_string(),
                //                seq.to_string(),
            );
            // println!("This is read_assertassert: {}", parameters);
            parameters
        }

        pub fn update_assertassert(
            AssertAssert {
                assertassertid,
                idlo,
                idhi,
                seq,
            }: AssertAssert,
        ) -> String {
            let parameters = format!(
                "UPDATE assertassert SET assertassertid={}, idlo={}, idhi={}, seq={} WHERE assertassertid={}",
                assertassertid.to_string(),
                idlo.to_string(),
                idhi.to_string(),
                seq.to_string(),
                assertassertid.to_string(),
            );
            // println!("This is update_assertassert: {}", parameters);
            parameters
        }

        pub fn delete_assertassert(
            AssertAssert {
                assertassertid,
                idlo,
                idhi,
                seq,
            }: AssertAssert,
        ) -> String {
            let parameters = format!(
                "DELETE FROM assertassert WHERE assertassertid={}",
                assertassertid.to_string(),
            );
            // println!("This is delete_assertassert: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct ReprMediaType {
        pub reprmediaid: i64,
        pub reprmedianame: String,
    }

    impl ReprMediaType {
        pub fn create_reprmediatype(
            ReprMediaType {
                reprmediaid,
                reprmedianame,
            }: ReprMediaType,
        ) -> String {
            let parameters = format!(
                "INSERT INTO reprmediatype (reprmediaid, reprmedianame) VALUES ({}, \"{}\")",
                reprmediaid.to_string(),
                reprmedianame,
            );
            // println!("This is create_reprmediatype: {}", parameters);
            parameters
        }

        pub fn read_reprmediatype(
            ReprMediaType {
                reprmediaid,
                reprmedianame,
            }: ReprMediaType,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM reprmediatype WHERE reprmediaid={}",
                reprmediaid.to_string(),
            );
            // println!("This is read_reprmediatype: {}", parameters);
            parameters
        }

        pub fn update_reprmediatype(
            ReprMediaType {
                reprmediaid,
                reprmedianame,
            }: ReprMediaType,
        ) -> String {
            let parameters = format!(
                "UPDATE reprmediatype SET reprmediatypeid={}, reprmedianame=\"{}\" WHERE reprmediaid={}",
                reprmediaid.to_string(),
                reprmedianame,
                reprmediaid.to_string(),
            );
            // println!("This is update_reprmediatype: {}", parameters);
            parameters
        }

        pub fn delete_reprmediatype(
            ReprMediaType {
                reprmediaid,
                reprmedianame,
            }: ReprMediaType,
        ) -> String {
            let parameters = format!(
                "DELETE FROM reprmediatype WHERE reprmediaid={}",
                reprmediaid.to_string(),
            );
            // println!("This is delete_reprmediatype: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct RepresentType {
        pub reprtypeid: i64,
        pub name: String,
    }

    impl RepresentType {
        pub fn create_representtype(RepresentType { reprtypeid, name }: RepresentType) -> String {
            let parameters = format!(
                "INSERT INTO representtype (reprtypeid, name) VALUES ({}, \"{}\")",
                reprtypeid.to_string(),
                name,
            );
            // println!("This is create_representtype: {}", parameters);
            parameters
        }

        pub fn read_representtype(RepresentType { reprtypeid, name }: RepresentType) -> String {
            let parameters = format!(
                "SELECT * FROM representtype WHERE reprtypeid={}",
                reprtypeid.to_string(),
            );
            // println!("This is read_representtype: {}", parameters);
            parameters
        }

        pub fn update_representtype(RepresentType { reprtypeid, name }: RepresentType) -> String {
            let parameters = format!(
                "UPDATE representtype SET reprtypeid={}, name=\"{}\" WHERE reprtypeid={}",
                reprtypeid.to_string(),
                name,
                reprtypeid.to_string(),
            );
            // println!("This is update_representtype: {}", parameters);
            parameters
        }

        pub fn delete_representtype(RepresentType { reprtypeid, name }: RepresentType) -> String {
            let parameters = format!(
                "DELETE FROM representtype WHERE reprtypeid={}",
                reprtypeid.to_string(),
            );
            // println!("This is delete_representtype: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Representation {
        pub representationid: i64,
        pub sourceid: i64,
        pub reprtypeid: i64,
        pub reprmediaid: i64,
        pub physfilecode: String,
        pub comments: String,
        pub externallink: String,
    }

    impl Representation {
        pub fn create_representation(
            Representation {
                representationid,
                sourceid,
                reprtypeid,
                reprmediaid,
                physfilecode,
                comments,
                externallink,
            }: Representation,
        ) -> String {
            let parameters = format!(
                "INSERT INTO representation (representationid, sourceid, reprtypeid, reprmediaid, physfilecode, comments, externallink) VALUES ({}, {}, {}, {}, \"{}\", \"{}\", \"{}\")",
                representationid.to_string(),
                sourceid.to_string(),
                reprtypeid.to_string(),
                reprmediaid.to_string(),
                physfilecode,
                comments,
                externallink,
            );
            // println!("This is create_representation: {}", parameters);
            parameters
        }

        pub fn read_representation(
            Representation {
                representationid,
                sourceid,
                reprtypeid,
                reprmediaid,
                physfilecode,
                comments,
                externallink,
            }: Representation,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM representation WHERE representationid={}",
                representationid.to_string(),
            );
            // println!("This is read_representation: {}", parameters);
            parameters
        }

        pub fn update_representation(
            Representation {
                representationid,
                sourceid,
                reprtypeid,
                reprmediaid,
                physfilecode,
                comments,
                externallink,
            }: Representation,
        ) -> String {
            let parameters = format!(
                "UPDATE representation SET representationid={}, sourceid={}, reprtypeid={}, reprmediaid={}, physfilecode=\"{}\", comments=\"{}\", externallink=\"{}\" WHERE reprtypeid={}",
                representationid.to_string(),
                sourceid.to_string(),
                reprtypeid.to_string(),
                reprmediaid.to_string(),
                physfilecode,
                comments,
                externallink,
                representationid.to_string(),
            );
            // println!("This is update_representation: {}", parameters);
            parameters
        }

        pub fn delete_representation(
            Representation {
                representationid,
                sourceid,
                reprtypeid,
                reprmediaid,
                physfilecode,
                comments,
                externallink,
            }: Representation,
        ) -> String {
            let parameters = format!(
                "DELETE FROM representation WHERE representationid={}",
                representationid.to_string(),
            );
            // println!("This is delete_representation: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Search {
        pub searchid: i64,
        pub activityid: i64,
        pub sourceid: i64,
        pub repositoryid: i64,
        pub searchedfor: String,
    }

    impl Search {
        pub fn create_search(
            Search {
                searchid,
                activityid,
                sourceid,
                repositoryid,
                searchedfor,
            }: Search,
        ) -> String {
            let parameters = format!(
                "INSERT INTO search (searchid, activityid, sourceid, repositoryid, searchedfor) VALUES ({}, {}, {}, {}, \"{}\")",
                searchid.to_string(),
                activityid.to_string(),
                sourceid.to_string(),
                repositoryid.to_string(),
                searchedfor,
            );
            println!("This is create_search: {}", parameters);
            parameters
        }

        pub fn read_search(
            Search {
                searchid,
                activityid,
                sourceid,
                repositoryid,
                searchedfor,
            }: Search,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM search WHERE searchid={}",
                searchid.to_string(),
            );
            println!("This is read_search: {}", parameters);
            parameters
        }

        pub fn update_search(
            Search {
                searchid,
                activityid,
                sourceid,
                repositoryid,
                searchedfor,
            }: Search,
        ) -> String {
            let parameters = format!(
                "UPDATE search SET searchid={}, activityid={}, sourceid={}, repositoryid={}, searchedfor=\"{}\" WHERE searchid={}",
                searchid.to_string(),
                activityid.to_string(),
                sourceid.to_string(),
                repositoryid.to_string(),
                searchedfor,
                searchid.to_string(),
            );
            println!("This is update_search: {}", parameters);
            parameters
        }

        pub fn delete_search(
            Search {
                searchid,
                activityid,
                sourceid,
                repositoryid,
                searchedfor,
            }: Search,
        ) -> String {
            let parameters = format!("DELETE FROM search WHERE searchid={}", searchid.to_string(),);
            println!("This is delete_search: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct EventType {
        pub eventtypeid: i64,
        pub eventtypename: String,
        pub gedcomtag: String,
    }

    impl EventType {
        pub fn create_eventtype(
            EventType {
                eventtypeid,
                eventtypename,
                gedcomtag,
            }: EventType,
        ) -> String {
            let parameters = format!(
                "INSERT INTO eventtype (eventtypeid, eventtypename, gedcomtag) VALUES ({}, \"{}\", \"{}\")",
                eventtypeid.to_string(),
                eventtypename,
                gedcomtag,
            );
            // println!("This is create_eventtype: {}", parameters);
            parameters
        }

        pub fn read_eventtype(
            EventType {
                eventtypeid,
                eventtypename,
                gedcomtag,
            }: EventType,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM eventtype WHERE eventtypeid={}",
                eventtypeid.to_string(),
            );
            // println!("This is read_eventtype: {}", parameters);
            parameters
        }

        pub fn update_eventtype(
            EventType {
                eventtypeid,
                eventtypename,
                gedcomtag,
            }: EventType,
        ) -> String {
            let parameters = format!(
                "UPDATE eventtype SET eventtypeid={}, eventtypename=\"{}\", gedcomtag=\"{}\" WHERE eventtypeid={}",
                eventtypeid.to_string(),
                eventtypename,
                gedcomtag,
                eventtypeid.to_string(),
            );
            // println!("This is update_eventtype: {}", parameters);
            parameters
        }

        pub fn delete_eventtype(
            EventType {
                eventtypeid,
                eventtypename,
                gedcomtag,
            }: EventType,
        ) -> String {
            let parameters = format!(
                "DELETE FROM eventtype WHERE eventtypeid={}",
                eventtypeid.to_string(),
            );
            // println!("This is delete_eventtype: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct Event {
        pub eventid: i64,
        pub eventtypeid: i64,
        pub placeid: i64,
        pub eventdate: String,
        pub eventname: String,
    }

    impl Event {
        pub fn create_event(
            Event {
                eventid,
                eventtypeid,
                placeid,
                eventdate,
                eventname,
            }: Event,
        ) -> String {
            let parameters = format!(
                "INSERT INTO event (eventid, eventtypeid, placeid, eventdate, eventname) VALUES ({}, {}, {}, \"{}\", \"{}\")",
                eventid.to_string(),
                eventtypeid.to_string(),
                placeid.to_string(),
                eventdate,
                eventname,
            );
            // println!("This is create_event: {}", parameters);
            parameters
        }

        pub fn read_event(
            Event {
                eventid,
                eventtypeid,
                placeid,
                eventdate,
                eventname,
            }: Event,
        ) -> String {
            let parameters = format!("SELECT * FROM event WHERE eventid={}", eventid.to_string(),);
            // println!("This is read_event: {}", parameters);
            parameters
        }

        pub fn update_event(
            Event {
                eventid,
                eventtypeid,
                placeid,
                eventdate,
                eventname,
            }: Event,
        ) -> String {
            let parameters = format!(
                "UPDATE event SET eventid={}, eventtypeid={}, placeid={}, eventdate=\"{}\", eventname=\"{}\" WHERE eventid={}",
                eventid.to_string(),
                eventtypeid.to_string(),
                placeid.to_string(),
                eventdate,
                eventname,
                eventid.to_string(),
            );
            // println!("This is update_event: {}", parameters);
            parameters
        }

        pub fn delete_event(
            Event {
                eventid,
                eventtypeid,
                placeid,
                eventdate,
                eventname,
            }: Event,
        ) -> String {
            let parameters = format!("DELETE FROM event WHERE eventid={}", eventid.to_string(),);
            // println!("This is delete_event: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct EventTypeRole {
        pub eventtyperoleid: i64,
        pub eventtypeid: i64,
        pub eventtyperolename: String,
    }

    impl EventTypeRole {
        pub fn create_eventtyperole(
            EventTypeRole {
                eventtyperoleid,
                eventtypeid,
                eventtyperolename,
            }: EventTypeRole,
        ) -> String {
            let parameters = format!(
                "INSERT INTO eventtyperole (eventtyperoleid, eventtypeid, eventtyperolename) VALUES ({}, {}, \"{}\")",
                eventtyperoleid.to_string(),
                eventtypeid.to_string(),
                eventtyperolename,
            );
            println!("This is create_eventtyperole: {}", parameters);
            parameters
        }

        pub fn read_eventtyperole(
            EventTypeRole {
                eventtyperoleid,
                eventtypeid,
                eventtyperolename,
            }: EventTypeRole,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM eventtyperole WHERE eventtyperoleid={}",
                eventtyperoleid.to_string(),
            );
            println!("This is read_eventtyperole: {}", parameters);
            parameters
        }

        pub fn update_eventtyperole(
            EventTypeRole {
                eventtyperoleid,
                eventtypeid,
                eventtyperolename,
            }: EventTypeRole,
        ) -> String {
            let parameters = format!(
                "UPDATE eventtyperole SET eventtyperoleid={}, eventtypeid={}, eventtyperolename=\"{}\" WHERE eventtyperoleid={}",
                eventtyperoleid.to_string(),
                eventtypeid.to_string(),
                eventtyperolename,
                eventtyperoleid.to_string(),
            );
            println!("This is update_eventtyperole: {}", parameters);
            parameters
        }

        pub fn delete_eventtyperole(
            EventTypeRole {
                eventtyperoleid,
                eventtypeid,
                eventtyperolename,
            }: EventTypeRole,
        ) -> String {
            let parameters = format!(
                "DELETE FROM eventtyperole WHERE eventtyperoleid={}",
                eventtyperoleid.to_string(),
            );
            println!("This is delete_eventtyperole: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct GlGroupType {
        pub glgrouptypeid: i64,
        pub glgroupname: String,
        pub ascdescnone: String,
    }

    impl GlGroupType {
        pub fn create_glgrouptype(
            GlGroupType {
                glgrouptypeid,
                glgroupname,
                ascdescnone,
            }: GlGroupType,
        ) -> String {
            let parameters = format!(
            "INSERT INTO glgrouptype (glgrouptypeid, glgroupname, ascdescnone) VALUES ({}, \"{}\", \"{}\")",
            glgrouptypeid.to_string(),
            glgroupname,
            ascdescnone,
        );
            println!("This is create_glgrouptype: {}", parameters);
            parameters
        }

        pub fn read_glgrouptype(
            GlGroupType {
                glgrouptypeid,
                glgroupname,
                ascdescnone,
            }: GlGroupType,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM glgrouptype WHERE glgrouptypeid={}",
                glgrouptypeid.to_string(),
            );
            println!("This is read_glgrouptype: {}", parameters);
            parameters
        }

        pub fn update_glgrouptype(
            GlGroupType {
                glgrouptypeid,
                glgroupname,
                ascdescnone,
            }: GlGroupType,
        ) -> String {
            let parameters = format!(
            "UPDATE glgrouptype SET glgrouptypeid={}, glgroupname=\"{}\", ascdescnone=\"{}\" WHERE glgrouptypeid={}",
            glgrouptypeid.to_string(),
            glgroupname,
            ascdescnone,
            glgrouptypeid.to_string(),
        );
            println!("This is update_glgrouptype: {}", parameters);
            parameters
        }

        pub fn delete_glgrouptype(
            GlGroupType {
                glgrouptypeid,
                glgroupname,
                ascdescnone,
            }: GlGroupType,
        ) -> String {
            let parameters = format!(
                "DELETE FROM glgrouptype WHERE glgrouptypeid={}",
                glgrouptypeid.to_string(),
            );
            println!("This is delete_glgrouptype: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct GlGroup {
        pub glgroupid: i64,
        pub glgrouptypeid: i64,
        pub placeid: i64,
        pub glgroupdate: String,
        pub glgroupname: String,
        pub glgroupcriteria: String,
    }

    impl GlGroup {
        pub fn create_glgroup(
            GlGroup {
                glgroupid,
                glgrouptypeid,
                placeid,
                glgroupdate,
                glgroupname,
                glgroupcriteria,
            }: GlGroup,
        ) -> String {
            let parameters = format!(
            "INSERT INTO glgroup (glgroupid, glgrouptypeid, placeid, glgroupdate, glgroupname, glgroupcriteria) VALUES ({}, {}, {}, \"{}\",\"{}\", \"{}\")",
            glgroupid.to_string(),
            glgrouptypeid.to_string(),
            placeid.to_string(),
            glgroupdate,
            glgroupname,
            glgroupcriteria,
        );
            println!("This is create_glgroup: {}", parameters);
            parameters
        }

        pub fn read_glgroup(
            GlGroup {
                glgroupid,
                glgrouptypeid,
                placeid,
                glgroupdate,
                glgroupname,
                glgroupcriteria,
            }: GlGroup,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM glgroup WHERE glgroupid={}",
                glgroupid.to_string(),
            );
            println!("This is read_glgroup: {}", parameters);
            parameters
        }

        pub fn update_glgroup(
            GlGroup {
                glgroupid,
                glgrouptypeid,
                placeid,
                glgroupdate,
                glgroupname,
                glgroupcriteria,
            }: GlGroup,
        ) -> String {
            let parameters = format!(
            "UPDATE glgroup SET glgroupid={}, glgrouptypeid={}, placeid={}, glgroupdate=\"{}\", glgroupname=\"{}\", glgroupcriteria=\"{}\" WHERE glgroupid={}",
            glgroupid.to_string(),
            glgrouptypeid,
            placeid,
            glgroupdate,
            glgroupname,
            glgroupcriteria,
            glgroupid.to_string(),
        );
            println!("This is update_glgroup: {}", parameters);
            parameters
        }

        pub fn delete_glgroup(
            GlGroup {
                glgroupid,
                glgrouptypeid,
                placeid,
                glgroupdate,
                glgroupname,
                glgroupcriteria,
            }: GlGroup,
        ) -> String {
            let parameters = format!(
                "DELETE FROM glgroup WHERE glgroupid={}",
                glgroupid.to_string(),
            );
            println!("This is delete_glgroup: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct GlGroupTypeRole {
        pub glgrouptyperoleid: i64,
        pub glgrouptypeid: i64,
        pub glgrouptypename: String,
        pub sequencenumber: i64,
    }


    impl GlGroupTypeRole {
        pub fn create_glgrouptyperole(
            GlGroupTypeRole {
                glgrouptyperoleid,
                glgrouptypeid,
                glgrouptypename,
                sequencenumber,
            }: GlGroupTypeRole,
        ) -> String {
            let parameters = format!(
            "INSERT INTO glgrouptyperole (glgrouptyperoleid, glgrouptypeid, glgrouptypename, sequencenumber) VALUES ({}, {}, \"{}\", {})",
            glgrouptyperoleid.to_string(),
            glgrouptypeid.to_string(),
            glgrouptypename,
            sequencenumber.to_string(),
        );
            println!("This is create_glgrouptyperole: {}", parameters);
            parameters
        }

        pub fn read_glgrouptyperole(
            GlGroupTypeRole {
                glgrouptyperoleid,
                glgrouptypeid,
                glgrouptypename,
                sequencenumber,
            }: GlGroupTypeRole,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM glgrouptyperole WHERE glgrouptyperoleid={}",
                glgrouptyperoleid.to_string(),
            );
            println!("This is read_glgrouptyperole: {}", parameters);
            parameters
        }

        pub fn update_glgrouptyperole(
            GlGroupTypeRole {
                glgrouptyperoleid,
                glgrouptypeid,
                glgrouptypename,
                sequencenumber,
            }: GlGroupTypeRole,
        ) -> String {
            let parameters = format!(
            "UPDATE glgrouptyperole SET glgrouptyperoleid={}, glgrouptypeid={}, glgroupname=\"{}\", sequencenumber={} WHERE glgrouptyperoleid={}",
            glgrouptyperoleid.to_string(),
            glgrouptypeid.to_string(),
            glgrouptypename,
            sequencenumber.to_string(),
            glgrouptyperoleid.to_string(),
        );
            println!("This is update_glgrouptyperole: {}", parameters);
            parameters
        }

        pub fn delete_glgrouptyperole(
            GlGroupTypeRole {
                glgrouptyperoleid,
                glgrouptypeid,
                glgrouptypename,
                sequencenumber,
            }: GlGroupTypeRole,
        ) -> String {
            let parameters = format!(
                "DELETE FROM glgrouptyperole WHERE glgrouptyperoleid={}",
                glgrouptyperoleid.to_string(),
            );
            println!("This is delete_glgrouptyperole: {}", parameters);
            parameters
        }
    }

    #[derive(Debug)]
    pub(crate) struct RepoSource {
        pub reposourceid: i64,
        pub repositoryid: i64,
        pub sourceid: i64,
        pub rsactivityid: i64,
        pub callnumber: String,
        pub description: String,
    }

    impl RepoSource {
        pub fn create_reposource(
            RepoSource {
                reposourceid,
                repositoryid,
                sourceid,
                rsactivityid,
                callnumber,
                description,
            }: RepoSource,
        ) -> String {
            let parameters = format!(
            "INSERT INTO reposource (reposourceid, repositoryid, sourceid, rsactivityid, callnumber, description) VALUES ({}, {}, {}, {}, \"{}\", \"{}\")",
            reposourceid.to_string(),
            repositoryid.to_string(),
            sourceid.to_string(),
            rsactivityid.to_string(),
            callnumber,
            description,
        );
            println!("This is create_reposource: {}", parameters);
            parameters
        }

        pub fn read_reposource(
            RepoSource {
                reposourceid,
                repositoryid,
                sourceid,
                rsactivityid,
                callnumber,
                description,
            }: RepoSource,
        ) -> String {
            let parameters = format!(
                "SELECT * FROM reposource WHERE reposourceid={}",
                reposourceid.to_string(),
            );
            println!("This is read_reposource: {}", parameters);
            parameters
        }

        pub fn update_reposource(
            RepoSource {
                reposourceid,
                repositoryid,
                sourceid,
                rsactivityid,
                callnumber,
                description,
            }: RepoSource,
        ) -> String {
            let parameters = format!(
            "UPDATE reposource SET reposourceid={}, repositoryid={}, sourceid={}, rsactivityid={}, callnumber=\"{}\", description=\"{}\" WHERE reposourceid={}",
            reposourceid.to_string(),
            repositoryid.to_string(),
            sourceid.to_string(),
            rsactivityid.to_string(),
            callnumber,
            description,
            reposourceid.to_string(),
        );
            println!("This is update_reposource: {}", parameters);
            parameters
        }

        pub fn delete_reposource(
            RepoSource {
                reposourceid,
                repositoryid,
                sourceid,
                rsactivityid,
                callnumber,
                description,
            }: RepoSource,
        ) -> String {
            let parameters = format!(
                "DELETE FROM reposource WHERE reposourceid={}",
                reposourceid.to_string(),
            );
            println!("This is delete_reposource: {}", parameters);
            parameters
        }
    }

}
