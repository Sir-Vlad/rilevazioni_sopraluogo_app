use std::fs::File;

use serde::de::DeserializeOwned;

use crate::test::ResultTest;

#[macro_export]
macro_rules! path_data_fake {
    ($filename:expr) => {
        format!("../dataFake/{}.json", $filename)
    };
}

pub fn read_json_file<T: DeserializeOwned>(file_path: &str) -> ResultTest<Vec<T>> {
    let file = File::open(file_path)?;
    let content = std::io::read_to_string(file)?;

    match serde_json::from_str(&content) {
        Ok(data) => Ok(data),
        Err(e) => Err(Box::new(e)),
    }
}
