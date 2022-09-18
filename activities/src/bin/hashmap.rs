use std::collections::HashMap;

struct Contents {
    content: String,
}

fn main() {
    let mut lockers = HashMap::new();
    lockers.insert(
        1,
        Contents {
            content: "stuff".to_owned(),
        },
    );
    lockers.insert(
        2,
        Contents {
            content: "shorts".to_owned(),
        },
    );
    lockers.insert(
        3,
        Contents {
            content: "shirts".to_owned(),
        },
    );

    for (locker_number, content) in &lockers {
        println!("Locker {:?} contains {:?}", locker_number, content.content);
    }
}
