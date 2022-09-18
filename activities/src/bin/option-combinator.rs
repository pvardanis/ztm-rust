fn main() {
    let a: Option<i32> = Some(1);
    dbg!(a);

    let a_is_some = a.is_some();
    dbg!(a_is_some);

    let a_is_none = a.is_none();
    dbg!(a_is_none);

    let a_mapped = a.map(|x| x + 1);
    dbg!(a_mapped);

    // it borrows the number, so it's not moved
    // if the value is throwed away, it will become None
    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    // it uses no arguments, if a is already data
    // then nothing is gonna happen, otherwise it will
    // return 2
    // it returns still optional data, we still need to
    // match on it later
    let a_or_else = a.or_else(|| Some(2));
    dbg!(a_or_else);

    // the same as or_else, but it will return the value
    // to the variable
    let unwrapped = a.unwrap_or_else(|| 0);
    dbg!(unwrapped);
}
