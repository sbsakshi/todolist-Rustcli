use std::io;

#[derive(Debug)]
struct Task{
    heading:String,
    desc:String,
    priority:Priority,
    status:Status,
}

#[derive(Debug)]
enum Priority{
    High,
    Medium,
    Low,
}

#[derive(Debug)]
enum Status{
    Pending,
    Completed,
}

fn read_input( str:&mut String)->String{
    io::stdin().read_line(  str).expect("failed to read input");
    str.trim().to_string()
}

impl Task {
    fn create_task(v:&mut Vec<Task>){
    println!("Enter a Task");
    println!("Enter Task heading");
    let heading: String= read_input(& mut String::new());
    println!("Enter Task Description");
    let description: String= read_input(& mut String::new());
    println!("Set Priority");
    let priority:String= read_input(& mut String::new());
    
    let pri=match priority.trim(){
        "high"=>Priority::High,
        "low"=>Priority::Low,
        "medium"=>Priority::Medium,
        _ => return,

    };
    
    let task1=Task{
        heading:heading,
        desc:description,
        priority:pri,
        status:Status::Pending,
    };
    println!("{:?}",task1);

        v.push(task1);

    }
    fn check_task(v:&mut Vec<Task>,str:String ){
        if let Some(task)=v.iter_mut().find(|task| task.heading==str){
            task.status=Status::Completed;
        }
    }

    fn del_task(v:&mut Vec<Task>,str:String){
       v.retain(|task| task.heading!=str);
    }

    fn display_alltasks(v:&Vec<Task>){
        println!("{:?}",v);
    }

    fn display_completed_task(v:&Vec<Task>)->Vec<&Task>{
        let completed_tasks: Vec<&Task> = v
        .iter()
        .filter(|task| matches!(task.status, Status::Completed))
        .collect();
        completed_tasks
    }
    }





fn main() {
    let mut v:Vec<Task>=Vec::new();
    Task::create_task(&mut v);
    Task::create_task(&mut v);
}

