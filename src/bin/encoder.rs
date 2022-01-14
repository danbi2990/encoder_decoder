use std::env;
use std::error::Error;
use std::iter;

use base64;

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1).unwrap_or_default();

    let encoded = base64::encode(&query);

    // Create an item to show in the Alfred drop down.
    let item = powerpack::Item::new(&encoded)
        .subtitle("base64 (standard)")
        .arg(&encoded);

    // Output the item to Alfred!
    powerpack::output(iter::once(item))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let s = "hello";
        let e = base64::encode(s);
        assert_eq!(e, "aGVsbG8=");
    }
}

