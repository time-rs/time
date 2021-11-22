use time::macros::declare_format_string;

declare_format_string!(); // unexpected end of input
declare_format_string!(my_format, "[year] [month]"); // missing formattable
declare_format_string!(OffsetDateTime, "[year] [month]"); // missing ident
declare_format_string!(my_format, OffsetDateTime); // missing string format
declare_format_string!(my_format, OffsetDateTime "[year] [month]"); // missing comma
declare_format_string!(my_format, OffsetDateTime : "[year] [month]"); // not a comma
declare_format_string!(my_format, OffsetDateTime, "[bad]"); // bad component name
declare_format_string!(my_format, OffsetDateTime, not_string); // string format wrong type
declare_format_string!(my_format, Date, "[year] [month]"); // formattable not in scope

fn main() {}
