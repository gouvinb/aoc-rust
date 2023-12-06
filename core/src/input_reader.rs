use std::{env, fmt, fs};

pub enum AOCEdition {
    Y2023,
    Y2022,
    Y2021,
    Y2020,
    Y2019,
    Y2018,
    Y2017,
    Y2016,
    Y2015,
}

impl fmt::Display for AOCEdition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AOCEdition::Y2023 => write!(f, "2023"),
            AOCEdition::Y2022 => write!(f, "2022"),
            AOCEdition::Y2021 => write!(f, "2021"),
            AOCEdition::Y2020 => write!(f, "2020"),
            AOCEdition::Y2019 => write!(f, "2019"),
            AOCEdition::Y2018 => write!(f, "2018"),
            AOCEdition::Y2017 => write!(f, "2017"),
            AOCEdition::Y2016 => write!(f, "2016"),
            AOCEdition::Y2015 => write!(f, "2015"),
        }
    }
}

pub enum AOCDay {
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
}

impl fmt::Display for AOCDay {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            AOCDay::D01 => write!(f, "01"),
            AOCDay::D02 => write!(f, "02"),
            AOCDay::D03 => write!(f, "03"),
            AOCDay::D04 => write!(f, "04"),
            AOCDay::D05 => write!(f, "05"),
            AOCDay::D06 => write!(f, "06"),
            AOCDay::D07 => write!(f, "07"),
            AOCDay::D08 => write!(f, "08"),
            AOCDay::D09 => write!(f, "09"),
            AOCDay::D10 => write!(f, "10"),
            AOCDay::D11 => write!(f, "11"),
            AOCDay::D12 => write!(f, "12"),
            AOCDay::D13 => write!(f, "13"),
            AOCDay::D14 => write!(f, "14"),
            AOCDay::D15 => write!(f, "15"),
            AOCDay::D16 => write!(f, "16"),
            AOCDay::D17 => write!(f, "17"),
            AOCDay::D18 => write!(f, "18"),
            AOCDay::D19 => write!(f, "19"),
            AOCDay::D20 => write!(f, "20"),
            AOCDay::D21 => write!(f, "21"),
            AOCDay::D22 => write!(f, "22"),
            AOCDay::D23 => write!(f, "23"),
            AOCDay::D24 => write!(f, "24"),
            AOCDay::D25 => write!(f, "25"),
        }
    }
}

pub fn parse_file(dir_str: String, aoc_edition: AOCEdition, aoc_day: AOCDay, suffix: Option<String>) -> String {
    let current_path_buf = env::current_dir().unwrap();
    let current_path_str = current_path_buf.to_str().unwrap();

    let mut path_str = match fs::metadata(&dir_str) {
        Ok(metadata) if metadata.is_dir() => format!("{}/", dir_str),
        _ => match fs::metadata(&format!("../{}/", dir_str)) {
            Ok(metadata) if metadata.is_dir() => format!("../{}/", dir_str),
            _ => panic!("Couldn't find `{}` directory.", dir_str),
        },
    };

    path_str.push_str(format!("{}/", aoc_edition).as_str());
    match suffix {
        None => path_str.push_str(format!("{}.txt", aoc_day).as_str()),
        Some(suffix) => path_str.push_str(format!("{}_{}.txt", aoc_day, suffix).as_str()),
    }

    let path_str = path_str.as_str();

    return fs::read_to_string(path_str).expect(format!("Could not read file {} (current path: {})", path_str, current_path_str).as_str());
}

pub fn parse_input(aoc_edition: AOCEdition, aoc_day: AOCDay, suffix: Option<String>) -> String {
    return parse_file("inputs".to_string(), aoc_edition, aoc_day, suffix);
}

pub fn parse_response(aoc_edition: AOCEdition, aoc_day: AOCDay, suffix: Option<String>) -> String {
    return parse_file("responses".to_string(), aoc_edition, aoc_day, suffix);
}
