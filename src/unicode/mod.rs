
use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use regex::Regex;

pub struct UnicodeDB {
    descriptions: Vec<UnicodeDescription>,
}

struct UnicodeDescription {
    code:u32,
    description: String,
}

impl UnicodeDB {

    pub fn new() -> Self {
        Self {
            descriptions: vec!(),
        }
    }

    pub fn from_file(filename: &str) -> io::Result<Self> {
        let mut udb = UnicodeDB::new();
        udb.parse_database(filename)?;
        Ok(udb)
    }

    pub fn search(&self, s:&str) -> Result<Vec<char>, regex::Error> {
        
        // build user regexp
        let re = Regex::new(s)?;
            
        // iterate over unicode descriptions
        let mut vs: Vec<char> = Vec::new();
        for udesc in self.descriptions.iter() {
            // append if description match regexp
            if re.is_match(udesc.description.as_str()) {
                if let Some(c) = char::from_u32(udesc.code) {
                    vs.push(c);
                }
            }
        }

        Ok(vs)
    }

    fn parse_database(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;

        // iterate over file lines
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                // parse each line and push parsed structure to descriptions
                if let Some(ud) = UnicodeDB::parse_line(line.as_str()) {
                    self.descriptions.push(ud);
                }
            }
        }

        Ok(())
    }

    fn parse_line(line: &str) -> Option<UnicodeDescription> {
        
        // split line by ; and trim whitespaces
        let mut parts = line.split(';');

        let hexcode = parts.next()?;
        let line    = parts.next()?;

        // split remaining line by # and trim whitespaces
        let mut parts = line.split('#');
        
        let _           = parts.next()?;
        let description = parts.next()?;

        // parse hexcode to integer
        let code = u32::from_str_radix(hexcode.trim(), 16).ok()?;

        // trim description and lower case
        let description = description.trim().to_lowercase();

        Some(UnicodeDescription {
            code,
            description,
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line_0() {
        let dl = UnicodeDB::parse_line("1F9A6 ; [*180C.0020.0002] # OTTER");
        assert_eq!(dl.is_some(), true);
        if let Some(dl) = dl {
            assert_eq!(dl.code, 0x1F9A6);
            assert_eq!(dl.description, "otter");
        }
    }

    #[test]
    fn test_parse_line_1() {
        let dl = UnicodeDB::parse_line("1F3DD ; [*1544.0020.0002] # DESERT ISLAND");
        assert_eq!(dl.is_some(), true);
        if let Some(dl) = dl {
            assert_eq!(dl.code, 0x1F3DD);
            assert_eq!(dl.description, "desert island");
        }
    }

    #[test]
    fn test_parse_line_2() {
        let dl = UnicodeDB::parse_line("1F502 ; [*1669.0020.0002] # CLOCKWISE RIGHTWARDS AND LEFTWARDS OPEN CIRCLE ARROWS WITH CIRCLED ONE OVERLAY");
        assert_eq!(dl.is_some(), true);
        if let Some(dl) = dl {
            assert_eq!(dl.code, 0x1F502);
            assert_eq!(dl.description, "clockwise rightwards and leftwards open circle arrows with circled one overlay");
        }
    }

    #[test]
    fn test_from_file() {
        const UNICODE_ALL_KEYS: &str = "/usr/share/unicode/allkeys.txt";
        let rudb = UnicodeDB::from_file(UNICODE_ALL_KEYS);
        assert_eq!(rudb.is_ok(), true);
    }

}


