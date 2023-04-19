#[derive(Debug)]
pub struct Characteristic {
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

