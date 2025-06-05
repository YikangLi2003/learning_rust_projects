use std::u64;

#[derive(Debug)]
enum State {
    Victoria,
    NewSouthWales,
    CapitalTerritory,
    NorthTerritory,
    WesternAustralia,
    SouthAustralia,
    Queensland,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

impl Coin {
    fn value_in_cents(self: &Self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
}

fn main() {
    let coins = [
        Coin::Nickel,
        Coin::Penny,
        Coin::Quarter(State::CapitalTerritory),
        Coin::Quarter(State::Victoria),
        Coin::Penny,
        Coin::Quarter(State::NewSouthWales),
        Coin::Quarter(State::WesternAustralia),
        Coin::Dime,
        Coin::Quarter(State::SouthAustralia),
        Coin::Quarter(State::NorthTerritory),
        Coin::Quarter(State::Queensland),
        Coin::Dime,
    ];

    let mut total_cents:u64 = 0;

    for coin in coins {
        total_cents = total_cents + coin.value_in_cents() as u64;
    }

    println!("Total value in cents: {}", total_cents);
}