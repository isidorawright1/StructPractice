//import libraries/modules
use std::io;
use titlecase::{titlecase, Titlecase};
use std::collections::HashMap;

//define struct
///Student Struct. Consists of student information
pub struct Student {
    name: String,
    grades: Vec<i64>, //using the Option enum, the type can be any
    id: i64,
}


impl Student {
    //Construct to make a new student
    ///
    fn new(name: String, id: i64) -> Self {
        Student{
            name,
            grades: Vec::new(),
            id,
        }
    }

    fn view_grades(&self) {
        println!("{:#?}", self.grades);
    }

    fn average(&self) -> i64 {
        let mut sum : i64 = 0;
        if self.grades.len() == 0 {
            return 0
        }

        for grade in &self.grades {
            sum += grade;
        }

        sum / self.grades.len() as i64
    }
    fn is_passing(&self) -> bool {
        self.average() > 70
    }

    //list student status
    //return a string and change println to format macro
    fn list_stats(&self) -> String{
        format!("Name: {}\nID: {}\nAverage: {avg:.2}\nPassing: {}",
                self.name, self.id, self.is_passing(), avg=self.average())
        //println!("\n Name: {} \n ID: {} \n Average: {} \n Passing: {} \n", self.name, self.id, self.average(), self.is_passing());
    }
}

///Register struct. Consists of a vector of students
struct Register {
    students : Vec<Student>,
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
        //use a hash map instead of a vector and remove by key!!
        self.students.remove(id - 1);
    }

    fn view_all_students(&mut self)
    {
        if self.students.len() == 0
        {
            println!("There are no registered students at this time.\n");
        }
        else
        {
            for student in self.students.iter() //no need to iterate mutable!!
            {
                //use format macro!
                println!("Name: {} \n ID: {} \n Average: {} \n", student.name, student.id, student.average());
            }
        }
    }

    fn add_grade(&mut self, name: String, grade: i64){
        //match the name and add grade to it
        //what about students with the same name?? --maybe should do this by ID
        for student in self.students.iter_mut() {
            //FIX SO THAT I CAN HAVE DUPLICATE NAMES!
            if student.name == name {
                student.grades.push(grade);
                student.list_stats();
                student.view_grades();
                break;
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

fn get_grade() -> i64 {
    let mut grade = String::new();
    println!("Enter a grade: ");
    io::stdin().read_line(&mut grade).expect("Failed to read grade");

    //parse as i64
    match grade.trim().parse::<i64>() {
        Ok(g) => return g,
        Err(_) => {
            println!("Enter a valid number.");
            //do a negative 1 here
            -1
        }
    }
}

//get id. this should happen automatically.
//find the id of the last student in the vector and iterate the count up one
fn get_id(vector: &mut Register) -> i64 {
    //get the stats of the last student
    //best practice is to initialize immediately
    if vector.students.len() == 0 {
        return 1;
    }
    //condense to be one line of code
    &vector.students[vector.students.len() - 1].id + 1
}

//look into setting up dir structure to move test cases to a test dir -- integration tests
//refactor current code
//keep going with rustlings
//look at persistent storage
//figure out gpu issue
//go over and get proficient in the linux commands in discord
//make own config in tmux -- install nerd font --must have symbols

fn main() {
    //create a new struct for registered students and manipulate by user choice
    let mut registered_students = Register::new();

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
            "3" => {
                let name = get_name();
                //in the future, add loop to add many grades at once
                let grade = get_grade();
                registered_students.add_grade(name, grade);
            },
            //maybe have a nested hash map -> quiz/homework/test, etc. and then date
            //figure out prototype for persistent storage!! --DBs!
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

