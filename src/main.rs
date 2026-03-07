//import libraries/modules
use std::io;
use titlecase::{titlecase, Titlecase};

//define struct
///Student Struct. Consists of student information
struct Student {
    name: String,
    grades: Vec<u64>,
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

    fn add_grades(&mut self, grade: u64) {
        self.grades.push(grade);
    }

    fn average(&self) -> u64 {
        let mut sum : u64 = 0;
        if self.grades.len() == 0 {
            0
        }
        else {
            for grade in &self.grades {
                sum += grade;
            }

            sum / self.grades.len() as u64
        }
    }
    fn is_passing(&self) -> bool {
        self.average() > 70
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

fn get_grade() -> u64 {
    let mut grade = String::new();
    println!("Enter a floating point number for grade: ");
    io::stdin().read_line(&mut grade).expect("Failed to read grade");

    //parse as u64
    match grade.trim().parse::<u64>() {
        Ok(g) => return g,
        Err(_) => {
            println!("Enter a valid floating point number.");
            0
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

        //get user input about what kind of functionality they want to use
        let mut choice= String::new();
        println!("Please enter your choice (enter a number): ");
        //read in user input and store it in choice variable
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        //match the choice and do different thing depending on the choice
        match choice.trim() {
            //add a student to registry
            "1" => {
                let name = get_name();
                let id = get_id(&mut registered_students);
                registered_students.add_student(Student::new(name, id));

                registered_students.students[registered_students.students.len() - 1].list_stats();
            },
            "2" => {
                let name = get_name();
                let mut id = 0;
                for student in registered_students.students.iter_mut() {
                    //in the future, change this so that it lists the stats of all students with the same
                    //name and then remove the one with the id you want
                    if student.name == name {
                        id = student.id;
                        break;
                    }
                }
                registered_students.remove_student(id as usize);
            },
            "3" => add_grade(&mut registered_students),
            //"4" => change_grade(),
            //"5" => remove_grade(),
            "6" => {
                registered_students.view_all_students();
            },
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
        s.add_grades(90);
        assert_eq!(s.grades, vec![90]);
    }

    #[test]
    fn test_add_many_grades_to_student(){
        let mut s = Student::new(String::from("Isidora"), 1234);
        s.add_grades(90);
        s.add_grades(80);
        s.add_grades(72);
        s.add_grades(60);
        s.add_grades(45);
        assert_eq!(s.grades, vec![90, 80, 72, 60, 45]);
    }

    #[test]
    //check passing grade
    fn test_student_passing(){
        let mut s = Student::new("Isidora".to_string(), 1234);
        s.add_grades(90);
        s.add_grades(90);
        s.add_grades(90);
        assert_eq!(s.is_passing(), true);
    }

    //check failing grade
    #[test]
    fn test_student_failing(){
        let mut s = Student::new("Isidora".to_string(), 1234);
        s.add_grades(20);
        s.add_grades(30);
        s.add_grades(35);
        assert_eq!(s.is_passing(), false);
    }

    //check average calculation
    #[test]
    fn test_student_average(){
        let mut s = Student::new("Isidora".to_string(), 1234);
        s.add_grades(90);
        s.add_grades(80);
        s.add_grades(70);
        s.add_grades(80);
        s.add_grades(90);
        assert_eq!(s.average(), 82);
    }

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

    #[test]
    fn test_remove_student() {
        //create a register struct with 2 students and remove 1
        let mut r = Register {
            students: vec![Student::new(String::from("Isidora"), 1),
                           Student::new(String::from("Charles"), 2)],
        };

        r.remove_student(1);

        assert_eq!(r.students.len(), 1);
        assert_eq!(r.students[0].name, "Charles".to_string());
        assert_eq!(r.students[0].id, 2);
    }
}