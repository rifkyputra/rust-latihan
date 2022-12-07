use std::io;

use super::connection::*;
use super::employee::*;

pub fn prompt() -> () {
    let mut conn = open_connection_in_memory();
    match create_table_employee(&conn) {
        Ok(_) => println!("Table employee created"),
        _ => println!("Table employee already exists"),
    };

    loop {
        println!("0. Open Database");
        println!("1. Insert Employee");
        println!("2. Show All Employee");
        println!("3. Delete Employee");
        println!("4. Backup Database");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let number = input.trim().parse::<i32>().unwrap();

        match number {
            0 => {
                println!("Opening Database");
                // prompt file name
                let mut file_name: String = String::new();
                io::stdin().read_line(&mut file_name).unwrap();
                let file_name = file_name.trim();

                match load_db(file_name) {
                    Ok(new_conn) => conn = new_conn,
                    _ => println!("Database not found"),
                }
            }
            1 => {
                let mut id: String = String::new();
                let mut name: String = String::new();
                let mut salary: String = String::new();

                println!("Employee ID");
                print!(" >> ");
                io::stdin().read_line(&mut id).unwrap();

                println!("Employee Name");
                print!(" >> ");
                io::stdin().read_line(&mut name).unwrap();

                println!("Employee Salary");
                print!(" >> ");
                io::stdin().read_line(&mut salary).unwrap();

                let employee = Employee {
                    id: id.trim().parse::<i32>().unwrap(),
                    name: name,
                    salary: salary.trim().parse::<f64>().unwrap(),
                };

                match insert_employee(&conn, &employee) {
                    Ok(_) => println!("Employee inserted"),
                    _ => println!("Cannot Insert Employee"),
                };
            }
            2 => {
                let employees = get_all_employee(&conn);
                println!(" ");
                println!("========");

                for employee in employees {
                    println!("Name : {}", employee.name);
                    println!("ID : {}", employee.id);
                    println!("Salary : {}", employee.salary);
                    println!("========");
                }
                println!(" ");
            }
            3 => {
                let mut id: String = String::new();
                println!("Employee ID");
                print!(" >> ");
                io::stdin().read_line(&mut id).unwrap();

                match delete_employee_by_id(&conn, id.trim().parse::<i32>().unwrap()) {
                    Ok(_) => println!("Employee deleted"),
                    Err(e) => println!("Cannot delete employee {:?}", e),
                }
            }
            4 => {
                backup_db(&conn, "db.sqlite", |progress| {
                    println!(
                        "Backup progress: {} of {} pages",
                        progress.pagecount, progress.remaining
                    );
                })
                .unwrap();
            }
            _ => {
                println!("Exiting...");

                break;
            }
        }
    }
}
