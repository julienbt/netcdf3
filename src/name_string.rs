mod tests;

/// Maximum size (number of bytes) allowed for the NetCDF names.
///
pub const NC_MAX_NAME_SIZE: usize = 256;

/// Checks that `name` follows the NetCDF-3 naming convention.
///
/// # Examples
///
/// ```
/// use netcdf3::{is_valid_name};
///
/// assert_eq!(true,    is_valid_name("title"));
/// assert_eq!(true,    is_valid_name("standard_name"));
/// assert_eq!(true,    is_valid_name("_FillValue"));
/// assert_eq!(true,    is_valid_name("café"));  // the UTF-8 encoded characters are supported
/// assert_eq!(true,    is_valid_name("A"));
///
/// assert_eq!(false,   is_valid_name(""));
/// assert_eq!(false,   is_valid_name("!invalid_name"));
/// ```
pub fn is_valid_name(name: &str) -> bool {
    // check the first character
    match name.chars().next() {
        None => {
            // then the name string is empty
            return false;
        }
        Some(c) => {
            if c.is_ascii() && !(c.is_alphanumeric() || c == '_') {
                return false;
            }
        }
    }
    if name.len() > NC_MAX_NAME_SIZE {
        return false;
    }
    for c in name.chars().skip(1) {
        if !(c.is_alphanumeric()) && c.is_ascii() && !(is_special_1(c) || is_special_2(c)) {
            return false;
        }
    }
    true
}

/// Returns `true` if the `char` is a NetCDF-3 special1 characters.
///
/// ``` text
/// special1     = '_''.''@''+''-'
/// ```
fn is_special_1(chr: char) -> bool {
    chr == '_' || chr == '.' || chr == '@' || chr == '+' || chr == '-'
}

/// Returns `true` if the `char` is a NetCDF-3 special2 characters.
///
/// ``` text
/// special2     = ' ' | '!' | '"' | '#' | '$' | '%' | '&' | '\'' |
/// '(' | ')' | '*' | ','  | ':' | ';' | '<' | '='  |
/// '>' | '?' | '[' | '\\' | ']' | '^' | '`' | '{'  |
/// '|' | '}' | '~'
/// ```
fn is_special_2(chr: char) -> bool {
    chr == ' '
        || chr == '!'
        || chr == '"'
        || chr == '#'
        || chr == '$'
        || chr == '%'
        || chr == '&'
        || chr == '\''
        || chr == '('
        || chr == ')'
        || chr == '*'
        || chr == ','
        || chr == ':'
        || chr == ';'
        || chr == '<'
        || chr == '='
        || chr == '>'
        || chr == '?'
        || chr == '['
        || chr == '\\'
        || chr == ']'
        || chr == '^'
        || chr == '`'
        || chr == '{'
        || chr == '|'
        || chr == '}'
        || chr == '~'
}
