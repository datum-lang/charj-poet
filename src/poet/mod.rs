pub const DEFAULT_INDENT: &str = "    ";

pub mod annotation_spec;
pub mod class_name;
pub mod code_block;
pub mod code_writer;
pub mod file_spec;
pub mod import;
pub mod line_wrapper;
pub mod member_name;
pub mod method_spec;
pub mod modifier;
pub mod name_allocator;
pub mod operator;
pub mod parameter_spec;
pub mod parameterized_type_name;
pub mod property_spec;
pub mod type_alias_spec;
pub mod type_name;
pub mod type_spec;
pub mod type_variable_name;
pub mod wildcard_type_name;

pub fn index_of(chars: &Vec<char>, p: usize) -> i32 {
    let mut index: i32 = -1;
    for i in p + 1..chars.len() {
        if chars[i] == '$' {
            index = i as i32;
        }
    }

    index
}

pub fn index_of_any(chars: &[char], special_chars: [char; 3], start_index: usize) -> i32 {
    for index in start_index..chars.len() {
        let ch: char = chars[index];
        for schar in special_chars.iter() {
            if *schar == ch {
                return index as i32;
            }
        }
    }

    return -1;
}

pub fn string_literal_with_quotes(
    value: &str,
    indent: &str,
    // _escape_dollar_sign: Option<bool>,
    // _is_constant_context: Option<bool>,
) -> String {
    // let escape_dollar_sign: bool;
    // match _escape_dollar_sign {
    //     None => escape_dollar_sign = true,
    //     Some(value) => escape_dollar_sign = value,
    // }
    //
    // let is_constant_context: bool;
    // match _is_constant_context {
    //     None => is_constant_context = false,
    //     Some(value) => is_constant_context = value,
    // }

    // if !is_constant_context && value.contains("\n") {
    let mut result = String::new();
    result.push_str("\"");

    let chars: Vec<char> = value.chars().collect();
    let mut i = 0;
    for c in chars {
        if c == '\'' {
            result.push_str("'");
            continue;
        }
        if c == '\"' {
            result.push_str("\\\"");
            continue;
        }

        result.push_str(&*character_literal_without_single_quotes(c));
        if c == '\n' && i + 1 < value.len() {
            result.push_str("\"\n");
            result.push_str(&*indent);
            result.push_str(&*indent);
            result.push_str("+ \"");
        }
        i = i + 1;
    }

    result.push_str("\"");
    // }
    return result;
}

pub fn character_literal_without_single_quotes(c: char) -> String {
    match c {
        '\x08' => String::from("\\b"), /* \u0008: backspace (BS) */
        '\t' => String::from("\\t"),   /* \u0009: horizontal tab (HT) */
        '\n' => String::from("\\n"),   /* \u000a: linefeed (LF) */
        '\x0c' => String::from("\\f"), /* \u000c: form feed (FF) */
        '\r' => String::from("\\r"),   /* \u000d: carriage return (CR) */
        '\"' => String::from("\""),    /* \u0022: double quote (") */
        '\'' => String::from("\\'"),   /* \u0027: single quote (') */
        '\\' => String::from("\\\\"),  /* \u005c: backslash (\) */
        _ => {
            if c.is_ascii() {
                return c.to_string();
            } else {
                return c.escape_unicode().to_string();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::poet::{character_literal_without_single_quotes, string_literal_with_quotes};

    #[test]
    #[rustfmt::skip]
    fn string_literal() {
        assert_eq!("\"abc\"", string_literal_with_quotes("abc", " "));
        assert_eq!("\"\\u{2666}\\u{2665}\\u{2660}\\u{2663}\"", string_literal_with_quotes("♦♥♠♣", " "));
        assert_eq!("\"\\u{20ac}\\t@\\t$\"", string_literal_with_quotes("€\t@\t$", " "));
        assert_eq!("\"abc();\\n\"\n  + \"def();\"", string_literal_with_quotes("abc();\ndef();", " "));
        assert_eq!("\"This is \\\"quoted\\\"!\"", string_literal_with_quotes("This is \"quoted\"!", " "));
        assert_eq!("\"e^{i\\\\pi}+1=0\"", string_literal_with_quotes("e^{i\\pi}+1=0", " "));
    }

    #[test]
    #[rustfmt::skip]
    fn character_literal() {
        assert_eq!("a", character_literal_without_single_quotes('a'));
        // common escapes

        // unicode escapes
        assert_eq!("\u{0000}", character_literal_without_single_quotes('\u{0000}'));

        // unicode escapes
        assert_eq!("\\u{20ac}", character_literal_without_single_quotes('\u{20AC}'));
        // assert_eq!("€", character_literal_without_single_quotes('\u{20AC}'));
    }
}
