use std::{io::prelude::*, io::stdin};

use pob_extractor::{extract_skill_tree_url, Error};
use structopt::StructOpt;
use url::Url;
use reqwest::blocking::get;

// take file option, open file and return url

// TODO: extract constants
// TODO: always take latest skill tree version (not hardcoded 3_13)

#[derive(Debug, StructOpt)]
#[structopt(
    name = "pob-skill-tree-extractor",
    about = "Extracts Path Of Exile passive skill tree url from the Path Of Building Community Edition exports."
)]
struct Opt {
    #[structopt(short = "p", long = "pastebin", parse(try_from_str = Url::parse))]
    pastebin: Option<Url>,
}

fn conver_pastebin_to_raw(mut pastebin: Url) -> Url {
    let mut path = "/raw".to_owned();
    path.push_str(pastebin.path());

    pastebin.set_path(path.as_ref());
    println!("URL path: {}", pastebin.as_str());

    pastebin
}

fn main() -> Result<(), Error> {
    let opt = Opt::from_args();
    let mut contents = String::new();

    if let Some(url) = opt.pastebin {
        let pastebin_raw = conver_pastebin_to_raw(url);
        contents = get(pastebin_raw)?.text()?;
    } else {
        let mut stdin = stdin();
        stdin.read_to_string(&mut contents)?;
    }

    let url = extract_skill_tree_url(contents)?;
    println!("{}", url);

    Ok(())
}
