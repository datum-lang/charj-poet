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
