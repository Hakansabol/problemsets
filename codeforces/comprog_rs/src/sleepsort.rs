use std::{thread, time::Duration};

fn main() {
    let v = vec![6, 2, 5, 4];
    println!("good morning");

    let mut vh = vec![];
    for a in v {
        let x = a;
        let j = thread::spawn(move || {
            thread::sleep(Duration::from_secs(x));
            println!("{x}");
        });
        vh.push(j);
    }

    for a in vh {
        a.join().unwrap();
    }
}
