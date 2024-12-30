fn main() {
  
    let book_deep_learning = Book {
        name: "Derin Öğrenme".to_string(),
        author: "Yalçın Özkan".to_string(),
        publisher: "Papatya Bilim".to_string(),
        pages: 272,
        price: 283.5,
    };

    book_deep_learning.book_info(); // Book name: Derin Öğrenme - author: Yalçın Özkan - publisher: Papatya Bilim - page number: 272- price: 283.5

    // ! error -> we'll cover later

    //println!("Book price: {}", book_deep_learning.price); // borrow of moved value: `book_deep_learning`

    //println!("Book author: {}", book_deep_learning.author); // borrow of moved value: `book_deep_learning`
}

#[derive(Debug)]
struct Book {
    name: String,
    author: String,
    publisher: String,
    pages: u16,
    price: f32,
}

impl Book {
    fn book_info(self) {
        println!(
            "Book name: {} - author: {} - publisher: {} - page number: {}- price: {}",
            self.name, self.author, self.publisher, self.pages, self.price
        )
    }
}
