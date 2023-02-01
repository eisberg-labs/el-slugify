//! URL slug generator utility. Your string will be transliterated and sanitized for use in URLs.
//! Created by Ana Bujan <ana@eisberg-labs.com>, MIT license
#![deny(missing_docs, rust_2018_idioms, elided_lifetimes_in_paths)]
#![crate_name = "el_slugify"]

use deunicode::deunicode_char;

/// Converts value to URL friendly slug. Default replacement is with "-"
///
/// # Examples
///
/// ```rust
/// use el_slugify::slugify;
///
/// slugify("Wait! Listen    \n\t!!!Runagalðr$ ~Wait! Listen    \n\t!!!Runagalðr$ ~Wait! Listen    \n\t!!!Runagalðr$ ~");
/// assert_eq!(slugify("di su ćevapi?"), "di-su-cevapi");
/// ```
pub fn slugify(value: &str) -> String {
    slugify_with_replacement(value, '-')
}

/// Converts value to URL friendly slug
pub fn slugify_with_replacement(value: &str, replacement: char) -> String {
    trim_trailing_space(sanitize(value, replacement).to_lowercase().as_str(), replacement)
}

/// Removes all non alphanumeric, substitutes to replacement character, without trailing replacement
fn sanitize(value: &str, replacement: char) -> String {
    let mut out = String::new();
    for elem in value.chars() {
        if is_contained_in_limited_set(elem) {
            out.push(elem)
        } else if elem.is_alphabetic() {
            // characters that need to be decoded should already be in the alphabetic range, everything else is for replacement
            let decoded_elem = deunicode_char(elem).map(|d| sanitize(d, replacement));
            if let Some(decoded) = decoded_elem {
                out.extend(decoded.chars());
            }
        } else {
            if !out.ends_with(replacement) {
                out.push(replacement)
            }
        }
    }


    out.to_string()
}

fn is_contained_in_limited_set(value: char) -> bool {
    match value {
        '0'..='9' | 'a'..='z' | 'A'..='Z' => true,
        _ => false
    }
}

fn trim_trailing_space(value: &str, replacement: char) -> String {
    let mut check_value = value.to_string();
    if check_value.starts_with(replacement) {
        check_value.remove(0);
    }
    if check_value.ends_with(replacement) {
        check_value.pop();
    }
    check_value.to_string()
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use slugify::slugify;
    use crate::{slugify as el_slugify};

    #[test]
    fn slugify_long_special_chars_wor2() {
        let binding = std::iter::repeat("#% MaČKA mački grize rep! (RIB-a) ~*")
            .take(10000).collect::<String>();

        let start = Instant::now();
        let _ = el_slugify(binding.as_str());
        let elapsed = start.elapsed();
        println!(">> El slugify took: {:?}", elapsed);

        let start = Instant::now();
        let _ = slugify!(binding.as_str());
        let elapsed = start.elapsed();
        println!(">> Slugify took: {:?}", elapsed);
    }

    #[test]
    fn test_slugify() {
        assert_eq!(el_slugify("Wait! Listen    \n\t!!!Runagalðr$ ~"), "wait-listen-runagaldr");
        assert_eq!(el_slugify("影 _ 師 嗎"), "ying-shi-ma");
        assert_eq!(el_slugify("影師嗎"), "ying-shi-ma");
        assert_eq!(el_slugify("di su ćevapi?"), "di-su-cevapi");
        assert_eq!(el_slugify("!-.#"), "");
        assert_eq!(el_slugify("!  kako  a je"), "kako-a-je");
        assert_eq!(el_slugify("iako **  .+a  na"), "iako-a-na");
        assert_eq!(el_slugify("   -!abc"), "abc");
        assert_eq!(el_slugify("iako *."), "iako");
        assert_eq!(el_slugify("dakako"), "dakako"); // unchanged
        assert_eq!(el_slugify("   -!kako tako _:+"), "kako-tako");
        assert_eq!(el_slugify(""), "");
        assert_eq!(el_slugify("!-.#"), "");
    }
}
