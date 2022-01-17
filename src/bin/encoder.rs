use std::env;
use std::error::Error;

use base64;

use encoder_decoder::ToItem;

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1).unwrap_or_default();

    let base64_item = encode_base64(&query)
        .unwrap_or_default()
        .to_item("base64 (standard)");
    let ascii_item = encode_ascii(&query)
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

fn encode_base64(s: &str) -> Result<String, Box<dyn Error>> {
    let base64_encoded = base64::encode(&s);
    Ok(base64_encoded)
}

fn encode_ascii(s: &str) -> Result<String, Box<dyn Error>> {
    match s.as_bytes().get(0) {
        Some(u) => Ok(u.to_string()),
        _ => Ok("".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_base64() -> Result<(), Box<dyn Error>> {
        let s = "hello";
        let e = encode_base64(s)?;
        assert_eq!(e.to_item("subtitle"),
                   powerpack::Item::new("aGVsbG8=")
                       .subtitle("subtitle")
                       .arg("aGVsbG8=")
        );
        Ok(())
    }

    #[test]
    fn test_encode_ascii() -> Result<(), Box<dyn Error>> {
        let s = "h";
        let e = encode_ascii(s)?;
        assert_eq!(e.to_item("ASCII"),
                   powerpack::Item::new("104")
                       .subtitle("ASCII")
                       .arg("104")
        );
        Ok(())
    }
}

