use crate::structs::dbtables::Place;
use super::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_place() {
        let place = Place {
            placeid: 1,
            startdate: "2022-01-01".to_string(),
            enddate: "2022-01-31".to_string(),
            ascdescnone: "ASC".to_string(),
            placecomment: "Test place".to_string(),
        };
        let result = create_place(place);
        assert_eq!(
            result,
            "INSERT INTO place (placeid, startdate, enddate, ascdescnone, placecomment) VALUES (1, \"2022-01-01\", \"2022-01-31\", \"ASC\", \"Test place\")"
        );
    }

    #[test]
    fn test_read_place() {
        let place = Place {
            placeid: 1,
            startdate: "".to_string(),
            enddate: "".to_string(),
            ascdescnone: "".to_string(),
            placecomment: "".to_string(),
        };
        let result = read_place(place);
        assert_eq!(result, "SELECT * FROM place WHERE placeid=1");
    }
}
