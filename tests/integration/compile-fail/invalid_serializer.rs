use time::macros::declare_format_string_offset_date_time;

declare_format_string_offset_date_time!("[year] [month]"); // missing ident
declare_format_string_offset_date_time!(my_format); // missing string format
declare_format_string_offset_date_time!(my_format "[year] [month]"); // missing comma
declare_format_string_offset_date_time!(my_format, "[bad]"); // bad component name
declare_format_string_offset_date_time!(my_format, not_string); // string format wrong type

fn main() {}
