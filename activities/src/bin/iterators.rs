fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // old way
    // let mut plus_one = vec![];
    // for num in numbers {
    //     plus_one.push(num + 1);
    // }

    let plus_one: Vec<_> = numbers.iter().map(|num| num + 1).collect();
    println!("{:?}", plus_one);
}
