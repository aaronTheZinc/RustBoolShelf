mod api;
mod other;

struct Book {
    title: String,
    author_first_name: String,
    author_last_name: String,
    rating: i32,
}

struct BookShelf {
    pub book_count: u32,
    books: Vec<Book>,
}

impl BookShelf {
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
        self.book_count = self.book_count + 1;
        return;
    }
    pub fn read_all_authors(&self) -> Vec<String> {
        let mut names = Vec::new();
        for book in &self.books {
            let full_name = format!("{} {}", book.author_first_name, book.author_last_name);
            names.push(full_name)
        }
        return names;
    }
}

fn main() {
    api::get_books().await;
    println!("dkdk");
    let book = Book {
        title: String::from("The Great Gatsby"),
        author_first_name: String::from("Aaron"),
        author_last_name: String::from("Marsh"),
        rating: 10,
    };

    let mut shelf = BookShelf {
        book_count: 0,
        books: Vec::new(),
    };
    shelf.add_book(book);

    let authors = shelf.read_all_authors();
    for name in authors {
        println!("{}", name)
    }
}
