use lat_05_db::db;
use try_01_hello::*;

mod lat_03_io;
mod lat_05_db;
mod try_01_hello;
// use ::pokemon::start as poke;

fn main() {
    // hello::prompt();
    db::prompt();

    lat_03_io::start::execute();

    // poke::start();
}
