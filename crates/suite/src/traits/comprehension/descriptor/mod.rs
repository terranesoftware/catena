mod description;
pub use description::Description;

pub trait Descriptor {
    fn describe(
        &self,
        text: &[u8]
    ) -> Description;
}