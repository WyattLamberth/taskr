/* to start, i need a function that will read latest task id */

use std::fs;
use serde_json::json;
use crate::task::Task;

/* function to retrieve latest task_id */
pub fn get_latest_task_id() -> i16 {
    let raw_json_file = fs::read_to_string("./task_store/task_store.json").expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&raw_json_file).expect("JSON was not well-formatted");
    let latest_task_id = json["latest_task_id"].as_i64().unwrap();
    return latest_task_id as i16;
}

/* function to save Task  */
pub fn stash_task(task: Task) {
    // read existing JSON
    let raw_json_file = fs::read_to_string("./task_store/task_store.json").expect("Unable to read file");
    let mut json: serde_json::Value = serde_json::from_str(&raw_json_file).expect("JSON was not well-formatted");

    // create task JSON using Task functions
    let task_id = *task.task_id();
    let task_json = json!({
        "task_id": task_id,
        "name": task.name(),
        "date_created": task.date_created().to_rfc3339(),
        "due_date": task.due_date().to_rfc3339(),
        "description": task.description(),
        "category": task.category(),
    });

    // Add task to array and update latest ID
    json["tasks"].as_array_mut().unwrap().push(task_json);
    json["latest_task_id"] = json!(task_id);

    // Write back to file
    fs::write(
        "./task_store/task_store.json", 
        serde_json::to_string_pretty(&json).unwrap()
    ).expect("Unable to write file");

}