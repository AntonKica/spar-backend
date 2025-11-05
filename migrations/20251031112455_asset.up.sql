CREATE TABLE asset(code VARCHAR(20) NOT NULL PRIMARY KEY,
                   name VARCHAR(80) NOT NULL,
                   confidentiality_protection_needs INTEGER NOT NULL,
                   integrity_protection_needs INTEGER NOT NULL,
                   availability_protection_needs INTEGER NOT NULL,
                   description VARCHAR(400) NOT NULL,
                   responsible VARCHAR(40) NOT NULL
);

CREATE TABLE risk_analysis_process(code CHAR(8) NOT NULL PRIMARY KEY,
                                   created_on DATE NOT NULL,
                                   workflow JSON NOT NULL
);

CREATE TABLE target_object_under_review(risk_analysis_process_code CHAR(8) NOT NULL REFERENCES risk_analysis_process(code),
                                        asset_code VARCHAR(20) NOT NULL REFERENCES asset(code)
);

CREATE TABLE tour_elementary_threat(risk_analysis_process_code CHAR(8) NOT NULL REFERENCES risk_analysis_process(code),
                                    asset_code VARCHAR(20) NOT NULL REFERENCES asset(code),
                                    it_grundschutz_elementary_threat_code VARCHAR(20) NOT NULL REFERENCES it_grundschutz_elementary_threat(code),
                                    relevance INTEGER NOT NULL,
                                    comment TEXT NOT NULL,
                                    reviewed BOOLEAN NOT NULL,
    PRIMARY KEY (risk_analysis_process_code, asset_code, it_grundschutz_elementary_threat_code)
);