macro_rules! main {
    ($body: block) => {
        main!(pub(self), $body);
    };
    ($v: vis, $body: block) => {
        $v fn main() -> Result<(), Box<dyn std::error::Error>> {
            $body
            Ok(())
        }
    };
}