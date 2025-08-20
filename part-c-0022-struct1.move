module mustafa::learn_move {

    use std::debug::print;
    use std::string::{String, utf8};

    struct Employee has drop {

        name: String,
        department: String,
        id: u16,
        salary_usd: u16,
        married: bool, 
    }

    fun lets_move() {

        let emp_ayhan: Employee = Employee {
            name: utf8(b"Ayhan Bilir"),
            department: utf8(b"Finance"),
            id: 4296,
            salary_usd: 4_250,
            married: true,
        };

        print(&emp_ayhan);
        print(&emp_ayhan.salary_usd);
    }
   
    
    #[test]
    fun testing() {

        lets_move(); 
    }

}

// [debug] 0xa3dd0715d0d4e18e91de3c4a13124c4b55283339cff3d2105f04b32ad04eb439::learn_move::Employee {
//   name: "Ayhan Bilir",
//   department: "Finance",
//   id: 4296,
//   salary_usd: 4250,
//   married: true
// }
// [debug] 4250

