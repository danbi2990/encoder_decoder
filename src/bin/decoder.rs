use std::{env, str};
use std::error::Error;

use base64;

use encoder_decoder::ToItem;

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1).unwrap_or_default();

    let base64_item = decode_base64(&query)
        .unwrap_or_default()
        .to_item("base64 (standard)");
    let ascii_item = decode_ascii(&query)
        .unwrap_or_default()
        .to_item("ASCII");

    let items: Vec<powerpack::Item> = vec![
        base64_item,
        ascii_item,
    ];

    // Output the item to Alfred!
    powerpack::output(items)?;

    Ok(())
}

fn decode_base64(s: &str) -> Result<String, Box<dyn Error>> {
    let bytes = base64::decode(s)
        .unwrap_or_default();
    let decoded = str::from_utf8(&bytes)
        .unwrap_or_default()
        .to_owned();
    Ok(decoded)
}

fn decode_ascii(s: &str) -> Result<String, Box<dyn Error>> {
    match s.parse::<u8>() {
        Ok(u) => Ok((u as char).to_string()),
        _ => Ok("".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_decode_base64() -> Result<(), Box<dyn Error>> {
        let s = "h";
        let e = decode_base64(s)?;
        assert_eq!(e.to_item("subtitle"),
                   powerpack::Item::new("")
                       .subtitle("subtitle")
                       .arg("")
        );
        Ok(())
    }

    #[test]
    fn test_decode_base64() -> Result<(), Box<dyn Error>> {
        let s = "aGVsbG8=";
        let e = decode_base64(s)?;
        assert_eq!(e.to_item("subtitle"),
                   powerpack::Item::new("hello")
                       .subtitle("subtitle")
                       .arg("hello")
        );
        Ok(())
    }

    #[test]
    fn test_encode_ascii() -> Result<(), Box<dyn Error>> {
        let s = "97";
        let e = decode_ascii(s)?;
        assert_eq!(e.to_item("ASCII"),
                   powerpack::Item::new("a")
                       .subtitle("ASCII")
                       .arg("a")
        );
        Ok(())
    }
}

