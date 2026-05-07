use std::process;

pub fn error(user_input: &String, loc: &usize, fmt: &str) -> ! {
    eprintln!("{}", user_input);
    eprint!("{}", " ".repeat(*loc));
    eprintln!("^ {}", fmt);
    process::exit(1);
}
