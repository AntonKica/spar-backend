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
pub fn create_risk_analysis_process_workflow() -> Workflow {
    Workflow {
        code: "risk_analysis_workflow".to_owned(),
        name: "Proces analýzy rizík".to_owned(),
        tasks: vec![
            Task {
                code: "threat_overview".to_owned(),
                name: "Prehľad hrozieb".to_owned(),
                sub_tasks: vec![
                    Task {
                        code: "elementary_threat_overview".to_owned(),
                        name : "Prehľad základných hrozieb".to_owned(),
                        sub_tasks: vec![]
                    },
                    Task {
                        code: "supplementary_threat_overview".to_owned(),
                        name : "Prehľad doplnkových hrozieb".to_owned(),
                        sub_tasks: vec![]
                    }
                ]
            },
            Task {
                code: "risk_classification".to_owned(),
                name: "Klasifikácia rizika".to_owned(),
                sub_tasks: vec![
                    Task {
                        code: "risk_evaluation".to_owned(),
                        name : "Vyhodnotenie rizika".to_owned(),
                        sub_tasks: vec![]
                    }
                ]
            }
        ]
    }
}