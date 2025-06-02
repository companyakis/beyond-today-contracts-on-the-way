fn main() {

    let todo_result1 = Project::Completed;

    println!("{}", todo_result1.give_project_info()); // We did! Team, thanks!

    let todo_result2 = Project::InProgress;

    println!("{}", todo_result2.give_project_info()); // Everything is ok. We have time. We'll do our best!

    let todo_result3 = Project::Failed("Unfortunately, we failed! We have to check what happened. We'll prepare a detailed report. I believe this result is an important lesson for us.".to_string());

    println!("{}", todo_result3.give_project_info());

}

enum Project {

    Completed,
    InProgress,
    Failed(String)
}

impl Project {
    
    fn give_project_info(&self) -> String {

        match self {
            Project::Completed => String::from("We did! Team, thanks!"),
            Project::InProgress => String::from("Everything is ok. We have time. We'll do our best!"),
            Project::Failed(e) => e.to_string()
        }
    }
}



