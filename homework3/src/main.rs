use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();
    
    for book in books {
        writeln!(file, "{}|{}|{}", book.title, book.author, book.year).unwrap();
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut res: Vec<Book> = vec![];

    for line in reader.lines() {
        let mut index: usize = 0;
        let mut start: usize = 0;
        let mut title1: String = "".to_string();
        let mut author1: String = "".to_string();
        let mut year1: u16 = 0;

        let string = line.unwrap();
        for i in index..string.len() {
            if string.chars().nth(i).unwrap() == '|' {
                title1 = string[start..index].to_string();
                start = index + 1;
                break;
            }
            index += 1;
        }
        index += 1;
        for i in index..string.len() {
            if string.chars().nth(i).unwrap() == '|' {
                author1 = string[start..index].to_string();
                start = index + 1;
                break;
            }
            index += 1;
        }
        year1 = string[start..].parse::<u16>().expect("BIG ERROR");

        res.push(Book{ title: title1, author: author1, year: year1});
    }
    res
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}