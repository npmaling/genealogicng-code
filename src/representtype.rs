#[derive(Debug)]
pub struct RepresentType {
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
