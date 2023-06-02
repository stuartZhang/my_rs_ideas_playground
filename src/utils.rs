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
macro_rules! compare_log {
    ($statement: stmt; $var: expr) => {
        println!("\n修改前{:?}", $var);
        $statement
        println!("修改后{:?}", $var);
    };
}
