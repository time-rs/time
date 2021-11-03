use time::macros::declare_format_string;

declare_format_string!("[year] [month]"); // missing ident
declare_format_string!(my_format); // missing string format
declare_format_string!(my_format "[year] [month]"); // missing comma
declare_format_string!(my_format, "[bad]"); // bad component name
declare_format_string!(my_format, not_string); // string format wrong type

fn main() {}
