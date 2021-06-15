use time::macros::datetime;

fn main() {
    let _ = datetime!(2021 - 000 0:00); //~ERROR invalid component: ordinal was 0
    let _ = datetime!(2021 - 001 24:00); //~ERROR invalid component: hour was 24
    let _ = datetime!(2021 - 001 0:00 0); //~ERROR unexpected token: 0
    let _ = datetime!(2021 - 001 0:00 UTC x); //~ERROR unexpected token: x
}
