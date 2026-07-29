// trait Iterator {
//     type: Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

struct Employee {
    name: String,
    salary: u16,
}

struct EmployeeRecords {
    emloyee_db: Vec<Employee>,
}

impl Iterator for EmployeeRecords {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if self.emloyee_db.len() != 0 {
            let result = self.emloyee_db[0].name.clone();
            self.emloyee_db.remove(0);
            Some(result)
        } else {
            None
        }
    }
}

fn main() {
    let mut emp_1 = Employee {
        name: String::from("John"),
        salary: 40000,
    };
    let mut emp_2 = Employee {
        name: String::from("Joseph"),
        salary: 30000,
    };

    let mut emp_db = EmployeeRecords {
        emloyee_db: vec![emp_1, emp_2],
    };

    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    // println!("{:?}", emp_db.next());
    for employee in emp_db {
        println!("{employee}");
    }
}
