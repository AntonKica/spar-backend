use spar_backend::workflow::{create_risk_analysis_process_workflow };
use spar_backend::workflow_model::create_workflow_model;

#[test]
fn test_workflow() {
    let workflow = create_risk_analysis_process_workflow();
    let workflow_model = create_workflow_model(&workflow);

    //println!("workflow model {}", workflow_model)
}
