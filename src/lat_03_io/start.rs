use std::io;
use std::fs;

use crate::lat_03_io::write as write;
pub trait Writes {
    fn write(&self) -> ();
}

impl Writes for write::Article {
    fn write(&self) -> () {


        fs::write("dist/foo", &self.body )
        .expect("Unable to write file");




    }
}

pub fn execute() {
    println!("---> File : ");


    let mut isi = String::new();


    io::stdin().read_line(&mut isi).unwrap();


    let write = write::Article{
        title: String::from("new article"),
        body: isi,
        category: vec![],
        date: 8,
    };

    write.write();


}