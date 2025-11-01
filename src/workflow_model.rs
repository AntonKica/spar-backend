use serde::{Deserialize, Serialize};
use crate::workflow::{Task, Workflow};
use sqlx::PgConnection;
use strum_macros::Display;

#[derive(Debug, Display, Serialize, Deserialize, Clone)]
enum TaskStatus {
    Initial,
    Started,
    Finished
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskModel {
    code: String,
    task_code: String,
    task_name: String,
    status: TaskStatus,
    sub_tasks: Vec<TaskModel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowModel {
    code: String,
    workflow_code: String,
    workflow_name: String,
    tasks: Vec<TaskModel>,
}

fn create_task_model(task: &Task) -> TaskModel {
    TaskModel {
        code: "TSK-0001".to_owned(),
        task_name: task.name.clone(),
        task_code: task.code.clone(),
        status: TaskStatus::Initial,
        sub_tasks: task.sub_tasks.iter().map(create_task_model).collect()
    }
}

fn set_initial(tasks: &mut Vec<TaskModel>) {
    match tasks.first_mut() {
        Some(first) => {
            first.status = TaskStatus::Started;
            set_initial(&mut first.sub_tasks)
        }
        None => {}
    }
}

pub fn create_workflow_model(workflow: &Workflow) -> WorkflowModel {
    let mut res = WorkflowModel {
        code: "WF-0001".to_owned(),
        workflow_code: workflow.code.clone(),
        workflow_name: workflow.name.clone(),
        tasks: workflow.tasks.iter().map(create_task_model).collect(),
    };
    set_initial(&mut res.tasks);
    
    res
}