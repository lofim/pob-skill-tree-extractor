use std::{io::prelude::*, io::stdin};
use pob_extractor::{extract_skill_tree_url, Error};

// take pastbin url, download the contents and return url
// take file option, open file and return url
// pipe into the program

// TODO: make it take parameters
// TODO: extract constants
// TODO: always take latest skill tree version (not hardcoded 3_13)

fn main() -> Result<(), Error> {
    // load POB export
    let mut stdin = stdin();
    let mut contents = String::new();
    stdin.read_to_string(&mut contents)?;

    let url = extract_skill_tree_url(contents)?;

    println!("{}", url);

    Ok(())
}
