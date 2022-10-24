use ::derive_builder::Builder;
use ::std::error::Error;
#[derive(Builder)]
#[builder(pattern = "mutable")]
struct Lorem {
    ipsum: u32,
    #[builder(private, setter(into))]
    vis: String
}
fn main() -> Result<(), Box<dyn Error>> {
    let x: Lorem = LoremBuilder::default()
        .ipsum(42)
        .vis("123")
        .build()?;
    assert_eq!(x.ipsum, 42);
    assert_eq!(x.vis, "123");
    Ok(())
}
