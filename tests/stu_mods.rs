//import libraries/modules
use std::io;
use titlecase::{titlecase, Titlecase};
use std::collections::HashMap;

//define struct
///Student Struct. Consists of student information
pub struct Student {
    pub name: String,
    pub grades: Vec<i64>, //using the Option enum, the type can be any
    pub id: i64,
}


impl Student {
    //Construct to make a new student
    ///
    pub fn new(name: String, id: i64) -> Self {
        Student{
            name,
            grades: Vec::new(),
            id,
        }
    }

    pub fn view_grades(&self) {
        println!("{:#?}", self.grades);
    }

    pub fn average(&self) -> i64 {
        let mut sum : i64 = 0;
        if self.grades.len() == 0 {
            return 0
        }

        for grade in &self.grades {
            sum += grade;
        }

        sum / self.grades.len() as i64
    }
    pub fn is_passing(&self) -> bool {
        self.average() > 70
    }

    //list student status
    //return a string and change println to format macro
    pub fn list_stats(&self) -> String{
        format!("Name: {}\nID: {}\nAverage: {avg:.2}\nPassing: {}",
                self.name, self.id, self.is_passing(), avg=self.average())
        //println!("\n Name: {} \n ID: {} \n Average: {} \n Passing: {} \n", self.name, self.id, self.average(), self.is_passing());
    }
}

///Register struct. Consists of a vector of students
pub struct Register {
    pub students : Vec<Student>,
}

impl Register {
    pub fn new() -> Self {
        Register {
            students: Vec::new(),
        }
    }

    pub fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    pub fn remove_student(&mut self, id: usize) {
        //since id corresponds to the index-1 of the vector, use that to remove the student
        //use a hash map instead of a vector and remove by key!!
        self.students.remove(id - 1);
    }

    pub fn view_all_students(&mut self)
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

    pub fn add_grade(&mut self, name: String, grade: i64){
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
pub fn get_name() -> String {
    let mut name = String::new();
    println!("Enter student name: ");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    name.trim().titlecase().to_string()
}

pub fn get_grade() -> i64 {
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
pub fn get_id(vector: &mut Register) -> i64 {
    //get the stats of the last student
    //best practice is to initialize immediately
    if vector.students.len() == 0 {
        return 1;
    }
    //condense to be one line of code
    &vector.students[vector.students.len() - 1].id + 1
}