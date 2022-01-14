use std::{env, iter, str};
use std::error::Error;

use base64;

fn main() -> Result<(), Box<dyn Error>> {
    // Alfred passes in a single argument for the user query.
    let query = env::args().nth(1).unwrap_or_default();

    let bytes = base64::decode(&query)
        .expect("Failed to base64 decoding");
    let decoded = str::from_utf8(&bytes)
        .expect("Failed to utf8 decoding");

    // Create an item to show in the Alfred drop down.
    let item = powerpack::Item::new(decoded)
        .subtitle("base64 (standard)")
        .arg(decoded);

    // Output the item to Alfred!
    powerpack::output(iter::once(item))?;

    Ok(())
}
