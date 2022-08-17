mod run;

fn main() {
    let code: &str  = "\u{0080}\u{0002}\u{000C}Hello World!\u{001F}";

    use std::time::Instant;
    let now = Instant::now();
    {
        run::run_code(code);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}