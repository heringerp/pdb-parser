use chrono::NaiveDate;
use std::str::FromStr;

// TODO:
// - function for savely getting range from str (when it is not filled till 80)
// - error handling (maybe thiserror/anyhow)

#[derive(Debug, PartialEq, Eq)]
pub enum Entry {
    Header(String, NaiveDate, String),
    Title(u8, String),
    Compound,
    Source,
    Remark,
}

impl FromStr for Entry {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match &line[..6] {
            "HEADER" => Ok(Self::Header(
                line[10..50].trim().to_string(),
                NaiveDate::parse_from_str(line[50..59].trim(), "%d-%b-%y").unwrap(),
                line[62..66].trim().to_string(),
            )),
            "TITLE " => Ok(Self::Title(
                if line[8..10].trim().is_empty() {
                    1
                } else {
                    line[8..10].trim().parse().unwrap()
                },
                line[10..80].trim().to_string(),
            )),
            x => Err(format!("Unknown entry {}", x)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_header() {
        let line =
            "HEADER    CELL CYCLE                              30-JAN-12   4AGL              ";
        let result = Entry::Header(
            "CELL CYCLE".to_string(),
            NaiveDate::from_ymd_opt(2012, 1, 30).unwrap(),
            "4AGL".to_string(),
        );
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_title() {
        let line =
            "TITLE     STRUCTURE OF THE P53 CORE DOMAIN MUTANT Y220C BOUND TO THE STABILIZING";
        let result = Entry::Title(
            1,
            "STRUCTURE OF THE P53 CORE DOMAIN MUTANT Y220C BOUND TO THE STABILIZING".to_string(),
        );
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_title_continuation() {
        let line =
            "TITLE    2 SMALL MOLECULE PHIKAN784                                             ";
        let result = Entry::Title(2, "SMALL MOLECULE PHIKAN784".to_string());
        assert_eq!(result, Entry::from_str(line).unwrap());
    }
}
