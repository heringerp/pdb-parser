use crate::error::ParseError;

pub fn get_charge(line: &str, start: usize, end: usize) -> Result<i8, ParseError> {
    match line.chars().last().unwrap() {
        ' ' => Ok(0),
        '+' => Ok(line[start..end - 1].trim().parse()?),
        '-' => Ok(-line[start..end - 1].trim().parse::<i8>()?),
        _ => Err(ParseError::InvalidCharge(line[start..end - 1].to_string())),
    }
}

pub fn get_opt_char(line: &str, pos: usize) -> Option<char> {
    match line.chars().nth(pos - 1).unwrap() {
        ' ' => None,
        x => Some(x),
    }
}

pub fn get_opt_string(line: &str, start: usize, end: usize) -> Option<String> {
    match get_string(line, start, end).as_str() {
        "" => None,
        x => Some(x.to_string()),
    }
}

pub fn get_string(line: &str, start: usize, end: usize) -> String {
    get_save_slice(line, start - 1, end).trim().to_string()
}

pub fn get_int(line: &str, start: usize, end: usize) -> Result<u32, ParseError> {
    Ok(get_save_slice(line, start - 1, end).trim().parse::<u32>()?)
}

pub fn get_float(line: &str, start: usize, end: usize) -> Result<f32, ParseError> {
    Ok(get_save_slice(line, start - 1, end).trim().parse::<f32>()?)
}

pub fn get_save_slice(line: &str, start: usize, end: usize) -> &str {
    match end >= line.len() {
        true => &line[start..],
        false => &line[start..end],
    }
}
