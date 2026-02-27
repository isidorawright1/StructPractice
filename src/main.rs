//import libraries/modules
use std::io;

//define struct
struct Student {
    name: String,
    grades: Vec<f32>,
    id: u32,
}

impl Student {
    //Construct to make a new student
    fn new(name: String, id: u32) -> Self {
        Student{
            name,
            grades: Vec::new(),
            id,
        }
    }

    fn add_grades(&mut self, grade: f32) {
        self.grades.push(grade);
    }

    fn average(&self) -> f32 {
        let mut sum : f32 = 0.0;
        for grade in &self.grades {
            sum += grade;
        }

        sum / self.grades.len() as f32
    }
    fn is_passing(&self) -> bool {
        self.average() > 70.0
    }
}

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
}

/*fn remove_student(){

}*/

fn main() {
    //create a new student
    let mut new_stu = Student::new("Name_Here".to_string(), 1432);

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

    //now have a vector of multiple students to compare and store and remove
    let mut students = vec![new_stu, Student::new("second_stu".to_string(), 1433)];

    students.push(Student::new("third".to_string(), 1434));
    students[1].add_grades(90.0);

    let mut many_students : Vec<Student> = Vec::new();

    //change students with user input
    //loop through until user breaks
    loop {
        println!("Here are your choices: ");
        println!("1. Add a Student");
        println!("2. Remove a Student");
        println!("3. Add a Grade");
        println!("4. Change a Grade");
        println!("5. Remove a Grade");
        println!("6. List a Student Stats");
        println!("7. Exit");

        let mut choice= String::new();
        println!("Please enter your choice (enter a number): ");
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => add_student(&mut many_students),
            //"2" => remove_student(),
            "3" => add_grade(&mut many_students),
            //"4" => change_grade(),
            //"5" => remove_grade(),
            "6" => list_stats(&mut many_students),
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

    #[test]
    //remove a student
    fn test_remove_student(){
        //create a student to remove
        //let s = Student::new(String::from("Isidora"), 1234);
    }
}