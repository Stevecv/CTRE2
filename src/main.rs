mod run;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let code: &str  = "\u{0080}\u{0002}\u{00C8}aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\u{001F}";

    use std::time::Instant;
    let now = Instant::now();
    {
        run::run_code(code);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}