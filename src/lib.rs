use base64::{decode, DecodeError};
use flate2::bufread::ZlibDecoder;
use minidom::Element;
use std::{io, io::Read};

mod error;

pub use error::Error;

pub fn extract_skill_tree_url(contents: String) -> Result<String, Error> {
    let decoded_contents = decode_base64(contents)?;
    let decoded_data = deflate(decoded_contents)?;

    // XML produced by POB is not valid, it's missing a namespace
    let fixed_xml = add_namespace(decoded_data);
    let root: Element = fixed_xml.parse()?;
    let url = extract_url(root).ok_or(Error::URLNotFoundError)?;

    Ok(url.trim().to_owned())
}

fn decode_base64(contents: String) -> Result<Vec<u8>, DecodeError> {
    let normalized = contents.replace("-", "+").replace("_", "/");
    decode(normalized)
}

fn deflate(decoded_contents: Vec<u8>) -> io::Result<String> {
    let mut decoder = ZlibDecoder::new(&decoded_contents[..]);
    let mut decoded_data = String::new();

    let _ = decoder.read_to_string(&mut decoded_data)?;

    Ok(decoded_data)
}

fn add_namespace(xml: String) -> String {
    xml.replacen("<PathOfBuilding>", "<PathOfBuilding xmlns=\"pob\">", 1)
}

fn extract_url(root: Element) -> Option<String> {
    for element in root.children() {
        if element.name() == "Tree" {
            for spec_element in element.children() {
                if let Some(version_attr) = spec_element.attr("treeVersion") {
                    if version_attr == "3_13" {
                        let xml = spec_element.get_child("URL", "pob").map(Element::text);

                        return xml;
                    }
                }
            }
        }
    }

    None
}
