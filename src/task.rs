/* 
wyatt lamberth 3/17/2025
implementation of the task data structure
*/

/*
i need to define a task structure, probably a json - maybe move to sqlite in the future if i feel like it.

    struct task {
        name:
        date created:
        date due (optional):
        description:
    }
*/


use chrono::{DateTime, Utc};


/* 
do these attributes need to be public?
they will only be touched using functions, not directly.

maybe add more attributes later, this is an okay start
*/ 
pub struct Task {
    name: String,
    date_created: DateTime<Utc>,
    due_date: DateTime<Utc>,
    description: String,
    category: String,
    task_id: i16,
}

impl Task {
    pub fn new(name: &str, created: &DateTime<Utc>, due: &DateTime<Utc>, description: &String, category: &String, task_id: &i16) -> Self {
        /* 
        when we make a new Task, we will have to populate its values in memory, but also in the task_store
        this can be simplified with a function in storage.rs that will handle it
        something like -> stash_task(new_task)
        */
        Self {
            name: name.to_string(),
            date_created: created.clone(),
            due_date: due.clone();
            description: description.to_string(),
            category: category.to_string(),
            task_id
        }
    }
}


