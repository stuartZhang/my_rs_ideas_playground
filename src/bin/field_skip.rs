use ::derive_builder::Builder;
use ::std::error::Error;
#[derive(Builder)]
#[builder(pattern = "mutable")]
struct Lorem {
    ipsum: u32,
    #[builder(setter(skip = true), default = r#""123".to_string()"#)]
    skip: String
}
fn main() -> Result<(), Box<dyn Error>> {
    let x: Lorem = LoremBuilder::default()
        .ipsum(42)
        .build()?;
    assert_eq!(x.ipsum, 42);
    assert_eq!(x.skip, "123");
    Ok(())
}
