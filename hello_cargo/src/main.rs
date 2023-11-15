#![allow(unused)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let four: IpAddrKind = IpAddrKind::V4(127,0,0,1);
    let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));

    let cents = value_in_cents(Coin::Penny);
    println!("Penny : {}", cents);
    
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn route(ip_kind: IpAddrKind) {

}