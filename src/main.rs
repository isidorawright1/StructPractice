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