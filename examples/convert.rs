use case_style::CaseStyle;

fn main() {
    let mut arguments = std::env::args();
    arguments.next().unwrap();

    if let Some(first_arg) = arguments.next() {
        let input: String = arguments.collect::<Vec<String>>().join(" ");

        if is_help(&first_arg) || input == "" {
            print_help()
        } else {
            match CaseStyle::guess(&input)
                .unwrap_or(CaseStyle::from_sentencecase(&input))
                .to_case(first_arg)
            {
                Ok(x) => println!("{}", x),
                Err(x) => println!("{}", x),
            }
        }
    } else {
        print_help()
    }
}

fn is_help(input: &str) -> bool {
    match input {
        "-h" | "--help" | "help" => true,
        _ => false,
    }
}

fn print_help() {
    println!(
        "
    convert ARG... to STYLE

    USAGE: convert [STYLE] [ARG...]

    examples:
        camel snake_case
        kebab this is a sentence
    "
    )
}
