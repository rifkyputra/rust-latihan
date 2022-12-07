use rusqlite::{params, Connection, Error};

#[derive(Debug, Clone, Default)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub salary: f64,
}

pub fn create_table_employee(conn: &Connection) -> Result<usize, Error> {
    return conn.execute(
        "CREATE TABLE employee (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  salary          REAL
                  )",
        params![],
    );
}

pub fn insert_employee(conn: &Connection, employee: &Employee) -> Result<usize, Error> {
    conn.execute(
        "INSERT INTO employee (id, name, salary) VALUES (?1, ?2, ?3)",
        params![employee.id, employee.name, employee.salary],
    )
}

pub fn get_all_employee(conn: &Connection) -> Vec<Employee> {
    let mut stmt = conn.prepare("SELECT * FROM employee").unwrap();

    let employee_iter = stmt.query_map([], |row| {
        Ok(Employee {
            id: row.get(0)?,
            name: row.get(1)?,
            salary: row.get(2)?,
        })
    });

    let mut employees: Vec<Employee> = Vec::new();

    for employee in employee_iter.unwrap() {
        employees.push(employee.unwrap());
    }

    employees
}

pub fn delete_employee_by_id(conn: &Connection, id: i32) -> Result<usize, Error> {
    conn.execute("DELETE FROM employee WHERE id = ?1", params![id])
}
