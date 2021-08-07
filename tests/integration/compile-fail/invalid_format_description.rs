use time::macros::format_description;

fn main() {
    let _ = format_description!();
    let _ = format_description!("[]");
    let _ = format_description!("[foo]");
    let _ = format_description!("[");
    let _ = format_description!("[hour foo]");
    let _ = format_description!("" x);
    let _ = format_description!(x);
    let _ = format_description!(0);
}
