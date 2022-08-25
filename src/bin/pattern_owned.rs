use ::derive_builder::Builder;
use ::std::{fs::File, error::Error};
#[derive(Builder)]
#[builder(pattern = "owned")]
struct Lorem {
    ipsum: u32,
    file: File,
}
fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./Cargo.toml")?;
    let x: Lorem = LoremBuilder::default()
        .ipsum(42)
        .file(file)
        .build()?;
    assert_eq!(x.ipsum, 42);
    Ok(())
}
