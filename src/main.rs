use ::derive_builder::Builder;
#[derive(Builder)]
struct Lorem {
    ipsum: u32,
}
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use ::std::error::Error;
    use super::{Lorem, LoremBuilder};
    #[test]
    fn test_lorem() -> Result<(), Box<dyn Error>> {
        let x: Lorem = LoremBuilder::default().ipsum(42).build()?;
        Ok(())
    }
}