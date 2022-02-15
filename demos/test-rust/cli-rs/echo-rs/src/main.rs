use clap::{arg, App};

fn main() {
    let matches = App::new("echo-rs")
        .version("0.1.0")
        .author("author")
        .about("Rust echo")
        // name
        // .arg(arg!([NAME]))
        // .arg(arg!([NAME]).default_value("alice"))
        // ... 支持多个
        .arg(arg!(<TEXT> ... "Input text"))
        // option name
        // .arg(arg!(-n --name <NAME>).required(false))
        // bool 值. 也可以这么写 `-n - -omit_newline`
        .arg(arg!(omit_newline: -n "Do not print newline"))
        .get_matches();
    let omit_newline = matches.is_present("omit_newline");
    let texts: Vec<&str> = matches.values_of("TEXT").unwrap().collect();
    print!(
        "{}{}",
        texts.join(" "),
        if omit_newline { "" } else { "\n" }
    );
}
