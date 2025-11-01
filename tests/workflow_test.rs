use spar_backend::workflow::parse_workflow;
use spar_backend::workflow_model::create_workflow_model;

#[test]
fn test_workflow() {
    let file = std::fs::File::open("risk_analysis_workflow.json").unwrap();
    let json = serde_json::from_reader(file).unwrap();
    let workflow = parse_workflow(json).unwrap();
    let workflow_model = create_workflow_model(&workflow);

    //println!("workflow model {}", workflow_model)
}
