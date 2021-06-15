use time::macros::offset;

fn main() {
    let _ = offset!(+24); //~ERROR invalid component: hour was 24
    let _ = offset!(+0:60); //~ERROR invalid component: minute was 60
    let _ = offset!(+0:00:60); //~ERROR invalid component: second was 60
    let _ = offset!(0); //~ERROR unexpected token: 0
    let _ = offset!(); //~ERROR missing component: sign
    let _ = offset!(+0a); //~ERROR invalid component: hour was 0a
    let _ = offset!(+0:0a); //~ERROR invalid component: minute was 0a
    let _ = offset!(+0:00:0a); //~ERROR invalid component: second was 0a
}
