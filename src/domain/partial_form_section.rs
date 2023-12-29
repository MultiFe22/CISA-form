#[derive(Debug, PartialEq)]
pub enum PartialFormSection {
    Section1 = 1,
    Section2 = 2,
    Section3 = 3,
}

impl PartialFormSection {
    /// Converts an integer into a `PartialFormSection` enum variant.
    /// Returns `Ok` with the corresponding enum variant if the input is valid (1, 2, or 3).
    /// Returns `Err` with an error message if the input is not valid.
    pub fn from_int(value: i32) -> Result<Self, String> {
        match value {
            1 => Ok(PartialFormSection::Section1),
            2 => Ok(PartialFormSection::Section2),
            3 => Ok(PartialFormSection::Section3),
            _ => Err(format!("'{}' is not a valid form section.", value)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_section_1_is_converted_successfully() {
        assert_eq!(
            PartialFormSection::from_int(1),
            Ok(PartialFormSection::Section1)
        );
    }

    #[test]
    fn valid_section_2_is_converted_successfully() {
        assert_eq!(
            PartialFormSection::from_int(2),
            Ok(PartialFormSection::Section2)
        );
    }

    #[test]
    fn valid_section_3_is_converted_successfully() {
        assert_eq!(
            PartialFormSection::from_int(3),
            Ok(PartialFormSection::Section3)
        );
    }

    #[test]
    fn invalid_section_is_rejected() {
        assert!(PartialFormSection::from_int(4).is_err());
    }

    #[test]
    fn negative_section_is_rejected() {
        assert!(PartialFormSection::from_int(-1).is_err());
    }

    #[test]
    fn zero_section_is_rejected() {
        assert!(PartialFormSection::from_int(0).is_err());
    }

    #[test]
    fn section_1_is_converted_to_int() {
        assert_eq!(PartialFormSection::Section1 as i32, 1);
    }
}
