CREATE TABLE asset(code VARCHAR(20) NOT NULL PRIMARY KEY,
                   name VARCHAR(80) NOT NULL,
                   description VARCHAR(400) NOT NULL,
                   responsible VARCHAR(40) NOT NULL
);

CREATE TABLE risk_analysis_process(code CHAR(8) NOT NULL PRIMARY KEY,
                                   created_on DATE
);

CREATE TABLE target_object_under_review(risk_analysis_process_code CHAR(8) REFERENCES risk_analysis_process(code),
                                        asset_code VARCHAR(20) REFERENCES asset(code)
);