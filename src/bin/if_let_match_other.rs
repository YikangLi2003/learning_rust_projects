use core::num;

fn main() {
    let n: u8 = 3;
    let m: u8 = 4;
    let l: u8 = 5;

    if_let(n);
    if_let(m);
    if_let(l);

    match_other(n);
    match_other(m);
    match_other(l);

    match__(n);
    match__(m);
    match__(l);
}

fn match_other(number: u8) {
    // Use keyword other to exhuast all cases.
    // Make sure other appears as the last match,
    // otherwise cases after 'other' will never be tested
    match number {
        3 => println!("three"),
        4 => println!("four"),
        other => println!("other: {}", other),
    }
}

fn match__(number: u8) {
    // _ works similar to 'other'
    // but _ cannot be used in the following statements
    match number {
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("other value _ that we don't not care"),
    }
}

fn if_let(number: u8) {
    // When there is only one match case
    // try 'if let' syntax
    // sometimes use else to handle _
    if let 3 = number {
        println!("three");
    } else {
        println!("other we don't care");
    }
}