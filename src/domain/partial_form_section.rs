#[derive(Debug, PartialEq)]
pub enum PartialFormSection {
    Section1,
    Section2,
    Section3,
}

impl PartialFormSection {
    /// Parses a string into a `PartialFormSection` enum variant.
    /// Returns `Ok` with the corresponding enum variant if the input is valid.
    /// Returns `Err` with an error message if the input is not valid.
    pub fn parse(s: &str) -> Result<Self, String> {
        match s.trim() {
            "1" => Ok(PartialFormSection::Section1),
            "2" => Ok(PartialFormSection::Section2),
            "3" => Ok(PartialFormSection::Section3),
            _ => Err(format!("'{}' is not a valid form section.", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_section_1_is_parsed_successfully() {
        assert_eq!(PartialFormSection::parse("1"), Ok(PartialFormSection::Section1));
    }

    #[test]
    fn valid_section_2_is_parsed_successfully() {
        assert_eq!(PartialFormSection::parse("2"), Ok(PartialFormSection::Section2));
    }

    #[test]
    fn valid_section_3_is_parsed_successfully() {
        assert_eq!(PartialFormSection::parse("3"), Ok(PartialFormSection::Section3));
    }

    #[test]
    fn invalid_section_is_rejected() {
        assert!(PartialFormSection::parse("4").is_err());
    }

    #[test]
    fn whitespace_section_is_rejected() {
        assert!(PartialFormSection::parse(" ").is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        assert!(PartialFormSection::parse("").is_err());
    }

    #[test]
    fn non_numeric_section_is_rejected() {
        assert!(PartialFormSection::parse("a").is_err());
    }
}
