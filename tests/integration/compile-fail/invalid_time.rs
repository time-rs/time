use time::macros::time;

fn main() {
    let _ = time!(24:00); //~ERROR invalid component: hour was 24
    let _ = time!(0:60); //~ERROR invalid component: minute was 60
    let _ = time!(0:00:60); //~ERROR invalid component: second was 60
    let _ = time!(x); //~ERROR unexpected token: x
    let _ = time!(0:00:00 x); //~ERROR unexpected token: x
    let _ = time!(""); //~ERROR invalid component: hour was ""
    let _ = time!(0:); //~ERROR unexpected end of input
    let _ = time!(0,); //~ERROR unexpected token: ,
    let _ = time!(0:00:0a); //~ERROR invalid component: second was 0a
}
