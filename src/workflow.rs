use serde_json::Value;

#[derive(Debug, Clone)]
pub struct Task {
    pub code: String,
    pub name: String,
    pub sub_tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub code: String,
    pub name: String,
    pub tasks: Vec<Task>,
}

fn parse_task(task: Value) -> Result<Task, String> {
    let code = task["code"].as_str().ok_or("no code in task")?.to_owned();
    let name = task["name"].as_str().ok_or("no name in task")?.to_owned();
    let sub_tasks = match task["sub_tasks"].clone() {
        Value::Array(obj) => parse_tasks(obj)?,
        Value::Null => Vec::new(),
        _ => return Err("no/invalid tasks in workflow".to_owned()),
    };

    Ok(Task{code, name, sub_tasks})
}
fn parse_tasks(tasks: Vec<Value>) -> Result<Vec<Task>, String> {
    let mut res = Vec::<Task>::new();
    for task in tasks {
        match parse_task(task) {
            Ok(task) => res.push(task),
            Err(e) => return Err(e)
        }
    }

    Ok(res)
}

pub fn parse_workflow(raw: serde_json::Value) -> Result<Workflow, String> {
    let workflow = match raw {
        Value::Object(obj) => obj,
        _ => return Err("Invalid workflow object".to_owned()),
    };

    let code = workflow["code"].as_str().ok_or("no code in workflow")?.to_owned();
    let name = workflow["name"].as_str().ok_or("no name in workflow")?.to_owned();

    let tasks = match workflow["tasks"].clone() {
        Value::Array(obj) => parse_tasks(obj)?,
        Value::Null => Vec::new(),
        _ => return Err("no/invalid tasks in workflow".to_owned()),
    };


    Ok(Workflow{ code, name, tasks })
}