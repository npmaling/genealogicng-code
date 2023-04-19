#[derive(Debug)]
pub struct SuretyScheme {
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
