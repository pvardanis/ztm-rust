fn main() {
    let my_numbers = vec![1, 2, 3, 4, 5];

    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();

    println!("{:?}", my_numbers);
    println!("{:?}", my_numbers.len());
    println!("{:?}", my_numbers[1]);

    for num in my_numbers {
        println!("{:?}", num);
    }
}
