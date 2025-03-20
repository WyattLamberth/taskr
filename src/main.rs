 
mod task;
use task::Task;

fn main() {
    let task = Task::new(
        "Task 1",
        &chrono::Utc::now(),
        &String::from("This is a task"),
        &String::from("General"),
    );
    task.save();
    println!("Task saved!");
}
