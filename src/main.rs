//import libraries/modules
use std::io;
use titlecase::{titlecase, Titlecase};

//define struct
///Student Struct. Consists of student information
struct Student {
    name: String,
    grades: Vec<f64>,
    id: u64,
}

///Register struct. Consists of a vector of students
struct Register {
    students : Vec<Student>,
}

impl Student {
    //Construct to make a new student
    ///
    fn new(name: String, id: u64) -> Self {
        Student{
            name,
            grades: Vec::new(),
            id,
        }
    }

    fn add_grades(&mut self, grade: f64) {
        self.grades.push(grade);
    }

    fn average(&self) -> f64 {
        let mut sum : f64 = 0.0;
        for grade in &self.grades {
            sum += grade;
        }

        sum / self.grades.len() as f64
    }
    fn is_passing(&self) -> bool {
        self.average() > 70.0
    }

    //list student status
    fn list_stats(&self) {
        println!("Name: {} \n ID: {} \n Average: {}", self.name, self.id, self.average());
    }
}

impl Register {
    fn new() -> Self {
        Register {
            students: Vec::new(),
        }
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn remove_student(&mut self, id: usize) {
        //since id corresponds to the index-1 of the vector, use that to remove the student
        self.students.remove(id - 1);
    }

    fn view_all_students(&mut self)
    {
        if self.students.len() == 0
        {
            println!("There are no registered students at this time.");
        }
        else
        {
            for student in self.students.iter_mut()
            {
                println!("Name: {} \n ID: {} \n Average: {}", student.name, student.id, student.average());
            }
        }
    }
}

//get the name of the student from user input. Use this for most functions
fn get_name() -> String {
    let mut name = String::new();
    println!("Enter student name: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    name.trim().titlecase().to_string()
}

fn get_grade() -> f64 {
    let mut grade = String::new();
    println!("Enter a floating point number for grade (ex: 70.0): ");
    io::stdin().read_line(&mut grade).expect("Failed to read grade");

    //parse as f32
    match grade.trim().parse::<f64>() {
        Ok(g) => return g,
        Err(_) => {
            println!("Enter a valid floating point number.");
            0.0
        }
    }
}

//get id. this should happen automatically.
//find the id of the last student in the vector and iterate the count up one
fn get_id(vector: &mut Register) -> u64 {
    //get the stats of the last student
    let new_id : u64;
    if vector.students.len() == 0 {
        new_id = 1;
    }
    else {
        let last_stu = &vector.students[vector.students.len() - 1];
        new_id = last_stu.id + 1;
    }
    new_id
}

fn add_grade(vector: &mut Register){
    let name = get_name();
    let grade = get_grade();
    //match the name and add grade to it
    //what about students with the same name?? --maybe should do this by ID
    for student in vector.students.iter_mut() {
        if student.name == name {
            student.grades.push(grade);
            break;
        }
    }
}

/*fn add_student(vector: &mut Register) {
    let name = get_name();
    let id = get_id(vector);

    let student = Student::new(name, id);

    vector.students.push(student);
}*/

/*fn remove_student(vector: &mut Vec<Student>) {
    let name = get_name();

    if let Some(position) = vector.iter().position(|student| student.name == name) {
        vector.remove(position);
    }
    else {
        println!("No student found with that name");
    }
}*/

fn main() {
    //now have a vector of multiple students to compare and store and remove
    //let mut students = vec![new_stu, Student::new("second_stu".to_string(), 1433)];

    let mut registered_students = Register::new();

    //change students with user input
    //loop through until user breaks
    loop {
        println!("======== Menu ========: ");
        println!("1: Register a Student");
        println!("2: Remove a Student");
        println!("3: Add a Grade");
        println!("4: Change a Grade");
        println!("5: Remove a Grade");
        println!("6: View All Students");
        println!("7: Exit");

        let mut choice= String::new();
        println!("Please enter your choice (enter a number): ");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let name = get_name();
                let id = get_id(&mut registered_students);
                registered_students.add_student(Student::new(name, id));

                registered_students.students[registered_students.students.len() - 1].list_stats();
            },
            /*"2" => {
                let name = get_name();
                for student in registered_students.students.iter_mut() {
                    if student.name == name {
                        list_stats(&mut registered_students);
                    }
                }
                remove_student(&mut registered_students, )
            },*/
            "3" => add_grade(&mut registered_students),
            //"4" => change_grade(),
            //"5" => remove_grade(),
            //"6" => list_stats(&mut registered_students),
            "7" => break,
            _ => println!("Please enter valid choice"),
        }
    }
}

//Tests
//check failing grade
#[cfg(test)]

mod tests {
    use super::*;
    //test to add a new student
    #[test]
    fn test_new_student_constructor(){
        let s = Student::new(String::from("Isidora"), 1234);
        //Check name
        assert_eq!(s.name, "Isidora");
        //check id
        assert_eq!(s.id, 1234);
    }

    //check adding a grade
    #[test]
    fn test_add_grade_to_student(){
        let mut s = Student::new(String::from("Isidora"), 1234);
        s.add_grades(90.0);
        assert_eq!(s.grades, vec![90.0]);
    }

    #[test]
    fn test_add_many_grades_to_student(){
        let mut s = Student::new(String::from("Isidora"), 1234);
        s.add_grades(90.0);
        s.add_grades(80.5);
        s.add_grades(72.6);
        s.add_grades(60.0);
        s.add_grades(45.0);
        assert_eq!(s.grades, vec![90.0, 80.5, 72.6, 60.0, 45.0]);
    }

    #[test]
    //check passing grade
    fn test_student_passing(){
        let mut s = Student::new("Isidora".to_string(), 1234);
        s.add_grades(90.0);
        s.add_grades(90.0);
        s.add_grades(90.0);
        assert_eq!(s.is_passing(), true);
    }

    //check failing grade
    #[test]
    fn test_student_failing(){
        let mut s = Student::new("Isidora".to_string(), 1234);
        s.add_grades(20.0);
        s.add_grades(30.5);
        s.add_grades(35.0);
        assert_eq!(s.is_passing(), false);
    }

    //check average calculation
    #[test]
    fn test_student_average(){
        let mut s = Student::new("Isidora".to_string(), 1234);
        s.add_grades(90.0);
        s.add_grades(80.0);
        s.add_grades(70.0);
        s.add_grades(80.0);
        s.add_grades(90.0);
        assert_eq!(s.average(), 82.0);
    }

    //test for case sensitivity in name
    //#[test]
    /*fn test_titlecase(){
        let mut r = Register::new();
        let id = get_id(&mut r);
        let s = Student::new(String::from("isidora"), id);
    }*/

    #[test]
    fn test_add_student() {
        let mut r = Register::new();
        //id for students is now automatically assigned based on used ids
        let id = get_id(&mut r);
        //create a student with a name and id
        let s = Student::new("Isidora".to_string(), id);
        r.add_student(s);

        //make sure that the vector length has increased to the correct size
        assert_eq!(r.students.len(), 1);
        //ensure the name of the student added is correct
        assert_eq!(r.students[0].name, "Isidora".to_string());
        //since this is the first student in the registry, id should be 1
        assert_eq!(r.students[0].id, 1);
    }

}


//create a new student
/*let mut new_stu = Student::new("Name_Here".to_string(), 1432);

//check! -- test has passed! now print
println!("{} {}", new_stu.name, new_stu.id);

//now add a grade -- test has passed for this too
new_stu.add_grades(70.0);

//sanity check in console
for grade in &new_stu.grades {
    println!("{}", grade);
}

//now get average and passing
new_stu.add_grades(90.0);
new_stu.add_grades(46.5);
new_stu.add_grades(89.6);

//should be 74.025
println!("{}", new_stu.average());
//this is passing
if new_stu.is_passing() {
    println!("You are passing with a: {}", new_stu.average());
}
else {
    println!("You are failing with a: {}", new_stu.average());
}

students.push(Student::new("third".to_string(), 1434));
    students[1].add_grades(90.0);

    let mut many_students : Vec<Student> = Vec::new();

*/
/*
//get the name of the student from user input. Use this for most functions
fn get_name() -> String {
    let mut name = String::new();
    println!("Enter student name: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    name.trim().to_string()
}

fn get_grade() -> f32 {
    let mut grade = String::new();
    println!("Enter a floating point number for grade (ex: 70.0): ");
    io::stdin().read_line(&mut grade).expect("Failed to read grade");

    //parse as f32
    match grade.trim().parse::<f32>() {
        Ok(g) => return g,
        Err(_) => {
            println!("Enter a valid floating point number.");
            0.0
        }
    }
}

//get id. this should happen automatically.
//find the id of the last student in the vector and iterate the count up one
fn get_id(vector: &mut Vec<Student>) -> u32 {
    //get the stats of the last student
    let new_id : u32;
    if vector.len() == 0 {
        new_id = 00001;
    }
    else {
        let last_stu = &vector[vector.len() - 1];
        new_id = last_stu.id + 1;
    }
    new_id
}

fn add_grade(vector: &mut Vec<Student>){
    let name = get_name();
    let grade = get_grade();
    //match the name and add grade to it
    //what about students with the same name?? --maybe should do this by ID
    for student in vector {
        if student.name == name {
            student.grades.push(grade);
            break;
        }
    }
}

fn add_student(vector: &mut Vec<Student>) {
    let name = get_name();
    let id = get_id(vector);

    let student = Student::new(name, id);

    vector.push(student);
}

//list student status
fn list_stats(vector: &mut Vec<Student>) {
    let name = get_name();

    for student in vector {
        if student.name == name {
            println!("Name: {} \n ID: {} \n Average: {}", student.name, student.id, student.average());
            break;
        }
    }
    //if it gets this far, name no longer exists
    println!("No stats for the name inputted. The student has either been removed or it the wrong name.");
}

fn remove_student(vector: &mut Vec<Student>) {
    let name = get_name();

    if let Some(position) = vector.iter().position(|student| student.name == name) {
        vector.remove(position);
    }
    else {
        println!("No student found with that name");
    }
}

*/