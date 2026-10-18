pub trait Diagnostics {
    fn diagnostics(
        &self,
        text: &[u8]
    );
}