use errors::Error;

pub struct LogtopusPort {}

impl LogtopusPort {
    pub fn new() -> LogtopusPort {
        LogtopusPort {}
    }

    pub fn search(&self, query: &String) -> Result<Vec<String>, Error> {
        let mut result = Vec::new();
        result.push(query.clone());
        Ok(result)
    }
}
