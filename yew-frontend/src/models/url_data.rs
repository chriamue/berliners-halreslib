use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct UrlData {
    pub catchdate: NaiveDate,
    pub url_raw: String,
}

impl UrlData {
    pub fn urls_from_string(input: &str) -> Result<Vec<UrlData>, Box<dyn Error>> {
        let mut rdr = csv::Reader::from_reader(input.as_bytes());
        let mut urls = Vec::new();

        for result in rdr.records() {
            let record = result?;
            let catchdate = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;
            let url_raw = record[1].to_string();
            urls.push(UrlData { catchdate, url_raw });
        }
        Ok(urls)
    }

    pub fn default_urls() -> Result<Vec<UrlData>, Box<dyn Error>> {
        //let data = include_str!("../../../_work/data/urls.csv");
        //UrlData::urls_from_string(data)
        Ok(vec![
            UrlData {
                catchdate: NaiveDate::from_ymd(2021, 10, 1),
                url_raw: "https://www.rust-lang.org/".to_string(),
            },
            UrlData {
                catchdate: NaiveDate::from_ymd(2024, 8, 29),
                url_raw: "https://corrode.dev/blog/rust-option-handling-best-practices/"
                    .to_string(),
            },
            UrlData {
                catchdate: NaiveDate::from_ymd(2024, 8, 29),
                url_raw: "https://github.com/radumarias/aws-lambda-axum-dynamodb-template"
                    .to_string(),
            },
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_urls() {
        let urls = UrlData::default_urls().unwrap();
        assert_eq!(urls.len(), 3);
    }
}
