use catena_suite::traits::transformation::Suggestions;

pub struct PythonSuggestions;

impl Suggestions for PythonSuggestions {
    fn suggestions(
        &self,
        text: &[u8]
    ) {
        
    }
}