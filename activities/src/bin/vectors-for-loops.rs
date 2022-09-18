struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![Test { score: 1 }, Test { score: 2 }, Test { score: 3 }];

    for test in my_scores {
        println!("{:?}", test.score);
    }
}
