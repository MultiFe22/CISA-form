pub struct PartialFormComplete(bool);

impl PartialFormComplete {
    /// Creates a new `PartialFormComplete` instance from a boolean value.
    pub fn new(value: bool) -> Self {
        PartialFormComplete(value)
    }
}

impl AsRef<bool> for PartialFormComplete {
    fn as_ref(&self) -> &bool {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::PartialFormComplete;

    #[test]
    fn true_value_is_handled_correctly() {
        let form_complete = PartialFormComplete::new(true);
        assert_eq!(*form_complete.as_ref(), true);
    }

    #[test]
    fn false_value_is_handled_correctly() {
        let form_complete = PartialFormComplete::new(false);
        assert_eq!(*form_complete.as_ref(), false);
    }
}
