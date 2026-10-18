use catena_suite::traits::transformation::{Edit, Suggestions};

pub struct PythonSuggestions;

impl Suggestions for PythonSuggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}