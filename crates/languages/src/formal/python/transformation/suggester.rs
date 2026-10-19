use catena_suite::traits::transformation::{Edit, Suggester};

pub struct PythonSuggester;

impl Suggester for PythonSuggester {
    fn suggest(
        &self,
        text: &[u8]
    ) -> Vec<Edit> {
        Vec::new()
    }
}