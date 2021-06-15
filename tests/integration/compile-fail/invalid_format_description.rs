use time::macros::format_description;

fn main() {
    let _ = format_description!(); //~ERROR expected string
    let _ = format_description!("[]"); //~ERROR missing component name at byte index 1
    let _ = format_description!("[foo]"); //~ERROR invalid component name `foo` at byte index 1
    let _ = format_description!("["); //~ERROR unclosed opening bracket at byte index 0
    let _ = format_description!("[hour foo]"); //~ERROR invalid modifier `foo` at byte index 6
    let _ = format_description!("" x); //~ERROR unexpected token: x
    let _ = format_description!(0); //~ERROR expected string
}
