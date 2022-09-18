struct Book {
    pages: i32,
    rating: i32,
}

fn display_page_count(book: &Book) {
    println!("The book has {} pages", book.pages);
}

fn display_rating(book: &Book) {
    println!("The book has a rating of {}", book.rating);
}

fn main() {
    let book = Book {
        pages: 200,
        rating: 5,
    };

    display_page_count(&book);
    display_rating(&book);
}
