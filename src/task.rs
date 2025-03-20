/* 
wyatt lamberth 3/17/2025
implementation of the task data structure
*/

use chrono::{DateTime, Utc};
use crate::storage;
use serde::{Serialize, Deserialize};

/* 
do these attributes need to be public?
they will only be touched using functions, not directly.

maybe add more attributes later, this is an okay start
*/ 
#[derive(Serialize, Deserialize)]
pub struct Task {
    name: String,
    date_created: DateTime<Utc>,
    due_date: DateTime<Utc>,
    description: String,
    category: String,
    task_id: i16,
}

impl Task {
    pub fn new(
        name: &str,  
        due: &DateTime<Utc>, 
        description: &String, 
        category: &String, 
    ) -> Self {
        let now = Utc::now();
        Self {
            name: name.to_string(),
            date_created: now,
            due_date: due.clone(),
            description: description.to_string(),
            category: category.to_string(),
            task_id: storage::get_latest_task_id() + 1,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn date_created(&self) -> &DateTime<Utc> {
        &self.date_created
    }

    pub fn due_date(&self) -> &DateTime<Utc> {
        &self.due_date
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn category(&self) -> &String {
        &self.category
    }

    pub fn task_id(&self) -> &i16 {
        &self.task_id
    }

    /* used later to change name if wanted */
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    /* used later to set a due date other than the default */
    pub fn set_due_date(&mut self, due: &DateTime<Utc>) {
        self.due_date = due.clone();
    }

    pub fn save(self) {
        storage::stash_task(self);
    }

}

