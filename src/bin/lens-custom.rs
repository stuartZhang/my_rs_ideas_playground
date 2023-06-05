use ::std::error::Error;
use ::lens_rs::*;

fn main() -> Result<(), Box<dyn Error>> {
    // #[derive(Review, Prism)]
    // enum Either<L, R> {
    //     #[optic]
    //     Left(L), // generate optics::Left
    //     #[optic]
    //     Right(R), // generate optics::Right
    // }
    #[derive(Copy, Clone, Debug, Lens)]
    struct Tuple<A, B>(
        #[optic] A,
        #[optic] B
    );
    #[derive(Copy, Clone, Debug, Lens)]
    struct Foo<A, B> {
        #[optic]
        a: A, // generate optics::a
        #[optic]
        b: B, // generate optics::b
    }
    #[derive(Clone, Debug, Lens)]
    struct Bar {
        // #[optic]
        a: String, // generate optics::a, same as above
        // #[optic]
        c: i32,    // generate optics::c
    }
    Ok(())
}