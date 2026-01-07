//! FFXIV Share Parser
//! 
//! A Rust implementation for parsing FFXIV gearset share strings.
//! This library can decode base62-encoded gearset data used by FFXIV gearing tools.

use wasm_bindgen::prelude::*;

pub mod base62;
pub mod types;
pub mod parser;

pub use types::*;
pub use parser::parse;

/// Parse a gearset share string and return the decoded gearset data
/// 
/// # Arguments
/// * `s` - The base62-encoded share string
/// 
/// # Returns
/// * `Ok(ParseResult::Gearset(gearset))` - Successfully parsed gearset
/// * `Ok(ParseResult::Shb)` - Old Shadowbringers format (version < 4)
/// * `Ok(ParseResult::Ew)` - Old Endwalker format (version < 5)
/// * `Err(String)` - Parse error with description
/// 
/// # Example
/// ```
/// use ffxiv_share_parser::parse;
/// 
/// let share_string = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we";
/// match parse(share_string) {
///     Ok(result) => println!("Parsed: {:?}", result),
///     Err(e) => println!("Error: {}", e),
/// }
/// ```
pub fn parse_gearset(s: &str) -> Result<ParseResult, String> {
    parser::parse(s)
}

/// WASM-bindgen wrapper for parsing gearset share strings
/// Returns a JsValue that can be used directly in JavaScript
#[wasm_bindgen]
pub fn parse_gearset_wasm(share_string: &str) -> Result<JsValue, JsValue> {
    match parser::parse(share_string) {
        Ok(result) => {
            serde_wasm_bindgen::to_value(&result)
                .map_err(|e| JsValue::from_str(&format!("Serialization error: {}", e)))
        }
        Err(e) => Err(JsValue::from_str(&e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api() {
        let s = "45WGpd4LvOX9v3JkHJrIG8hUEncvc0s0we";
        let result = parse_gearset(s);
        assert!(result.is_ok());
        println!("{:?}", result.unwrap());
    }
}