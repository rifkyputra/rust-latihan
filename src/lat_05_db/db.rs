use std::io;

use crate::lat_05_db::connection;

use super::connection::*;
use super::employee::*;

pub fn prompt() -> () {
    let conn = openConnectionInMemory();
    match create_table_employee(&conn) {
        Ok(_) => println!("Table employee created"),
        _ => println!("Table employee already exists"),
    };

    loop {
        println!("1. Insert Employee");
        println!("2. Show All Employee");
        println!("3. Delete Employee");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let number = input.trim().parse::<i32>().unwrap();

        match number {
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

                insert_employee(&conn, &employee);
            }
            2 => {
                let employees = get_all_employee(&conn);
                for employee in employees {
                    println!("ID : {}", employee.id);
                    println!("Name : {}", employee.name);
                    println!("Salary : {}", employee.salary);
                    println!(" ");
                }
            }
            3 => {
                let mut id: String = String::new();
                println!("Employee ID");
                print!(" >> ");
                io::stdin().read_line(&mut id).unwrap();

                let conn = connection::openConnectionInMemory();
                deleteEmployeeById(&conn, id.trim().parse::<i32>().unwrap());
            }
            _ => {
                break;
                println!("Invalid Input");
            }
        }
    }
}
