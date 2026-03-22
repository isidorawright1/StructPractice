//Tests
//check failing grade
mod stu_mods;

    use stu_mods::*;
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
        let mut r = Register {
            students: vec![Student::new(String::from("Isidora"), 1)],
        };
        r.add_grade(r.students[0].name.clone(), 90);
        assert_eq!(r.students[0].grades, vec![90]);
    }

    #[test]
    fn test_add_many_grades_to_student(){
        let mut r = Register {
            students: vec![Student::new(String::from("Isidora"), 1)],
        };

        r.add_grade(r.students[0].name.clone(), 90);
        r.add_grade(r.students[0].name.clone(), 80);
        r.add_grade(r.students[0].name.clone(), 72);
        r.add_grade(r.students[0].name.clone(), 60);
        r.add_grade(r.students[0].name.clone(), 45);
        assert_eq!(r.students[0].grades, vec![90, 80, 72, 60, 45]);
    }

    #[test]
    //check passing grade
    fn test_student_passing(){
        let mut r = Register {
            students: vec![Student::new(String::from("Isidora"), 1)],
        };

        r.add_grade(r.students[0].name.clone(), 90);
        r.add_grade(r.students[0].name.clone(), 90);
        r.add_grade(r.students[0].name.clone(), 90);
        assert_eq!(r.students[0].is_passing(), true);
    }

    //check failing grade
    #[test]
    fn test_student_failing(){
        let mut r = Register {
            students: vec![Student::new(String::from("Isidora"), 1)],
        };

        r.add_grade(r.students[0].name.clone(), 20);
        r.add_grade(r.students[0].name.clone(), 30);
        r.add_grade(r.students[0].name.clone(), 35);
        assert_eq!(r.students[0].is_passing(), false);
    }

    //check average calculation
    #[test]
    fn test_student_average(){
        let mut r = Register {
            students: vec![Student::new(String::from("Isidora"), 1)],
        };

        r.add_grade(r.students[0].name.clone(), 90);
        r.add_grade(r.students[0].name.clone(), 80);
        r.add_grade(r.students[0].name.clone(), 70);
        r.add_grade(r.students[0].name.clone(), 80);
        r.add_grade(r.students[0].name.clone(), 90);
        assert_eq!(r.students[0].average(), 82);
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