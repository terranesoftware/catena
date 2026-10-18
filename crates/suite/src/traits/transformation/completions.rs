pub trait Completions {
    fn completions(
        &self,
        at: usize,
        text: &[u8]
    );
}