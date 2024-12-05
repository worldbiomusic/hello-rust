fn main() {
    four()
}

fn a() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result is {result}");
}

fn four() {
    let a=  [1,2,3];
    for e in a {
        println!("{e}");
    }

    for e in (1..4).rev() { // (1..=4)하면 4포함
        println!("{e}");
    }
}