use chrono::NaiveDate;
use error::ParseError;
use std::str::FromStr;

// TODO:
// - function for savely getting range from str (when it is not filled till 80)
// - error handling (maybe thiserror/anyhow)
// - amino acid enum
mod error;

#[derive(Debug, PartialEq)]
pub enum Entry {
    Header(String, NaiveDate, String),
    Title(u8, String),
    Seqres(u32, Option<char>, u32, Vec<String>),
    Atom(
        u32,
        String,
        Option<char>,
        String,
        Option<char>,
        u32,
        Option<char>,
        f32, f32, f32,
        f32,
        f32,
        String,
        i8,
    ),
    Master(u32, u32, u32,
           u32, u32, u32,
           u32, u32, u32,
           u32, u32, u32),
}

impl FromStr for Entry {
    type Err = error::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        match &line[..6] {
            "HEADER" => Ok(Self::Header(
                get_string(line, 11, 50),
                NaiveDate::parse_from_str(line[50..59].trim(), "%d-%b-%y").unwrap(),
                get_string(line, 63, 66),
            )),
            "TITLE " => Ok(Self::Title(
                if line[8..10].trim().is_empty() {
                    1
                } else {
                    line[8..10].trim().parse().unwrap()
                },
                get_string(line, 11, 80),
            )),
            "SEQRES" => Ok(Self::Seqres(
                get_int(line, 8, 10)?,
                get_opt_char(line, 12),
                get_int(line, 14, 17)?,
                line[19..70]
                    .split_whitespace()
                    .map(|x| x.to_string())
                    .collect(),
            )),
            "ATOM  " => Ok(Self::Atom(
                get_int(line, 7, 11)?,
                get_string(line, 13, 16),
                get_opt_char(line, 17),
                get_string(line, 18, 20),
                get_opt_char(line, 22),
                get_int(line, 23, 26)?,
                get_opt_char(line, 27),
                get_float(line, 31, 38)?,
                get_float(line, 39, 46)?,
                get_float(line, 47, 54)?,
                get_float(line, 55, 60)?,
                get_float(line, 61, 66)?,
                get_string(line, 77, 78),
                get_charge(line, 79, 80)?,
            )),
            "MASTER" => Ok(Self::Master(
                    get_int(line, 11, 15)?, 
                    get_int(line, 16, 20)?,
                    get_int(line, 21, 25)?,
                    get_int(line, 26, 30)?,
                    get_int(line, 31, 35)?,
                    get_int(line, 36, 40)?,
                    get_int(line, 41, 45)?,
                    get_int(line, 46, 50)?,
                    get_int(line, 51, 55)?,
                    get_int(line, 56, 60)?,
                    get_int(line, 61, 65)?,
                    get_int(line, 66, 70)?,
                    )),
            x => Err(ParseError::UnknownEntry(x.to_string())),
        }
    }
}

fn get_charge(line: &str, start: usize, end: usize) -> Result<i8, ParseError> {
    match line.chars().last().unwrap() {
        ' ' => Ok(0),
        '+' => Ok(line[start..end - 1].trim().parse()?),
        '-' => Ok(-line[start..end - 1].trim().parse::<i8>()?),
        _ => Err(ParseError::InvalidCharge(line[start..end - 1].to_string())),
    }
}

fn get_opt_char(line: &str, pos: usize) -> Option<char> {
    match line.chars().nth(pos - 1).unwrap() {
        ' ' => None,
        x => Some(x),
    }
}

fn get_opt_string(line: &str, start: usize, end: usize) -> Option<String> {
    match get_string(line, start, end).as_str() {
        "" => None,
        x => Some(x.to_string()),
    }
}

fn get_string(line: &str, start: usize, end: usize) -> String {
    get_save_slice(line, start - 1, end).trim().to_string()
}

fn get_int(line: &str, start: usize, end: usize) -> Result<u32, ParseError> {
    Ok(get_save_slice(line, start - 1, end).trim().parse::<u32>()?)
}

fn get_float(line: &str, start: usize, end: usize) -> Result<f32, ParseError> {
    Ok(get_save_slice(line, start - 1, end).trim().parse::<f32>()?)
}

fn get_save_slice(line: &str, start: usize, end: usize) -> &str {
    match end >= line.len() {
        true => &line[start..],
        false => &line[start..end],
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
        let line = "TITLE    2 SMALL MOLECULE PHIKAN784";
        let result = Entry::Title(2, "SMALL MOLECULE PHIKAN784".to_string());
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_seqres_full() {
        let line =
            "SEQRES   2 A  219  TYR GLY PHE ARG LEU GLY PHE LEU HIS SER GLY THR ALA          ";
        let result = Entry::Seqres(
            2,
            Some('A'),
            219,
            vec![
                "TYR".to_string(),
                "GLY".to_string(),
                "PHE".to_string(),
                "ARG".to_string(),
                "LEU".to_string(),
                "GLY".to_string(),
                "PHE".to_string(),
                "LEU".to_string(),
                "HIS".to_string(),
                "SER".to_string(),
                "GLY".to_string(),
                "THR".to_string(),
                "ALA".to_string(),
            ],
        );
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_seqres_half() {
        let line =
            "SEQRES  17 B  219  GLY SER THR LYS ARG ALA LEU PRO ASN ASN THR                  ";
        let result = Entry::Seqres(
            17,
            Some('B'),
            219,
            vec![
                "GLY".to_string(),
                "SER".to_string(),
                "THR".to_string(),
                "LYS".to_string(),
                "ARG".to_string(),
                "ALA".to_string(),
                "LEU".to_string(),
                "PRO".to_string(),
                "ASN".to_string(),
                "ASN".to_string(),
                "THR".to_string(),
            ],
        );
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_seqres_one_chain() {
        let line =
            "SEQRES  17    219  GLY SER THR LYS ARG ALA LEU PRO ASN ASN THR                  ";
        let result = Entry::Seqres(
            17,
            None,
            219,
            vec![
                "GLY".to_string(),
                "SER".to_string(),
                "THR".to_string(),
                "LYS".to_string(),
                "ARG".to_string(),
                "ALA".to_string(),
                "LEU".to_string(),
                "PRO".to_string(),
                "ASN".to_string(),
                "ASN".to_string(),
                "THR".to_string(),
            ],
        );
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_atom() {
        let line = "ATOM     13  CG2 VAL A  97     114.726  77.558 -32.731  1.00 20.45           C  ";
        let result = Entry::Atom(13,
                                 "CG2".to_string(),
                                 None,
                                 "VAL".to_string(),
                                 Some('A'),
                                 97,
                                 None,
                                 114.726, 77.558, -32.731,
                                 1.00,
                                 20.45,
                                 "C".to_string(),
                                 0);
        assert_eq!(result, Entry::from_str(line).unwrap());
    }

    #[test]
    fn create_master() {
        let line = "MASTER      526    0    4    5   22    0    6    6 3738    2   38   34";
        let result = Entry::Master(526, 0, 4, 5, 22, 0, 6, 6, 3738, 2, 38, 34);
        assert_eq!(result, Entry::from_str(line).unwrap());
    }
}
