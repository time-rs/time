use time::macros::date;

fn main() {
    let _ = date!(+1_000_000-01-01); //~ERROR invalid component: year was 1000000
    let _ = date!(10_000 - 01 - 01); //~ERROR years with more than four digits must have an explicit sign
    let _ = date!(2021-W 60-1); //~ERROR invalid component: week was 60
    let _ = date!(2021-W 01-0); //~ERROR invalid component: day was 0
    let _ = date!(2021-W 01-8); //~ERROR invalid component: day was 8
    let _ = date!(2021 - 00 - 01); //~ERROR invalid component: month was 0
    let _ = date!(2021 - 13 - 01); //~ERROR invalid component: month was 13
    let _ = date!(2021 - 01 - 00); //~ERROR invalid component: day was 0
    let _ = date!(2021 - 01 - 32); //~ERROR invalid component: day was 32
    let _ = date!(2021 - 000); //~ERROR invalid component: ordinal was 0
    let _ = date!(2021 - 366); //~ERROR invalid component: ordinal was 366
    let _ = date!(0a); //~ERROR invalid component: year was 0a
    let _ = date!(2021:); //~ERROR unexpected token: :
    let _ = date!(2021-W 0a); //~ERROR invalid component: week was 0a
    let _ = date!(2021-W 01:); //~ERROR unexpected token: :
    let _ = date!(2021-W 01-0a); //~ERROR invalid component: day was 0a
    let _ = date!(2021-0a); //~ERROR invalid component: month or ordinal was 0a
    let _ = date!(2021-01-0a); //~ERROR invalid component: day was 0a
}
