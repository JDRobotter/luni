
mod unicode;

use clap::{App,Arg};

const UNICODE_ALL_KEYS: &str = "/usr/share/unicode/allkeys.txt";

fn main() {

    let matches = App::new("luni")
        .arg(Arg::new("pattern")
            .about("match unicode characters which match this pattern")
            .required(true)
            )
        .get_matches();

    // open unicode definition file
    let udb = unicode::UnicodeDB::from_file(UNICODE_ALL_KEYS);
    if let Err(e) = udb {
        println!("Error while opening unicode definition: {}", e);
        return;
    } 
    let udb = udb.unwrap();

    let spattern = matches.value_of("pattern").unwrap();

    // search pattern in unicode definitions
    let chars = udb.search(spattern);
    if let Err(e) = chars {
        println!("Error with search pattern: {}", e);
        return;
    }
    let chars = chars.unwrap();

    // output chars to terminal
    println!("{}", chars.iter().map(|c| c.to_string()).collect::<Vec<String>>().join(" "));
}
