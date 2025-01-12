use std::collections::HashMap;
use std::io;

fn main() {
    /*
     * Allow users to add Employees to a department in a company
     * "Add <name> to <department>
     */
    let mut department = String::new();
    let mut name = String::new();

    println!("Who would you like to add?");

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Which department do they belong to?");

    io::stdin()
        .read_line(&mut department)
        .expect("Failed to read line");

    let department = department.trim();
    let name = name.trim();

    //    If nanme doesn't exist, add it
    // If department doesn't exist, add it

    println!("Will add {name} to {department}");

    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    company.insert(
        "Engineering".to_string(),
        vec!["Aidan".to_string(), "John".to_string()],
    );

    // Check if the department already exists or not
    company
        .entry(department.to_string())
        .and_modify(|employees| employees.push(name.to_string()))
        .or_insert(vec![name.to_string()]);

    for (department, employees) in &company {
        println!("Department {department} has the following employees {:?}", employees);
    }
    println!("Current company structure is {:?}", company);

}
