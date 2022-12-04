use std::fs;
use std::io;
use std::io::{BufRead, BufReader};

use crate::lat_03_io::write;
pub trait Writes {
    fn write(&self) -> ();
}

impl Writes for write::Article {
    fn write(&self) -> () {
        let content = format!(
            "# {} 
{}  
Category : {}
        ",
            self.title, self.body, self.category
        );

        fs::write("dist/foo.md", &content).expect("Unable to write file");
    }
}

pub fn execute() {
    let mut title = String::new();
    let mut isi = String::new();

    println!("Article Title");
    print!(" >> ");
    io::stdin().read_line(&mut title).unwrap();

    println!(" ");

    println!("Article Body : ");
    print!(" >> ");
    io::stdin().read_line(&mut isi).unwrap();

    let raw_categories = fs::File::open("categories.txt").expect("file not found");

    let mut raw_categories = BufReader::new(raw_categories);

    let c = &mut raw_categories;
    print_categories(c);

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let category = assign_category(
        &mut raw_categories,
        &mut input.trim().parse::<usize>().unwrap(),
    );

    let article = write::Article {
        title: title,
        body: isi,
        category: category,
        date: 8,
    };

    article.write();
}

fn print_categories(c: &mut BufReader<fs::File>) {
    for (num, item) in c.lines().enumerate() {
        println!("{}. {}", num, item.unwrap());
    }
}

fn assign_category(c: &mut BufReader<fs::File>, input: &usize) -> String {
    for (num, item) in c.lines().enumerate() {
        if num == *input {
            return String::from(item.unwrap());
        }
    }

    String::from("Uncategorized")
}
