use std::env::args;

fn main() {
    let files = args().skip(1).collect::<Vec<String>>();

    for file in files {
        match std::fs::read_to_string(&file) {
            Ok(content) => {
                println!("FILE: {}", file);
                match edn_format::parse_str(&content) {
                    Ok(value) => println!("EDN: {:#?}", value),
                    Err(e) => eprintln!("ERROR: during {:?} at {}", e.context, e),
                }
            },
            Err(e) => eprintln!("{}: {}", file, e),
        }
    }
}
