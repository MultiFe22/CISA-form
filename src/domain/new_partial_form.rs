use crate::domain::PartialFormComplete;
use crate::domain::PartialFormField;
use crate::domain::PartialFormSection;

pub struct NewPartialForm {
    pub field: PartialFormField,
    pub section: PartialFormSection,
    pub complete: PartialFormComplete,
}
