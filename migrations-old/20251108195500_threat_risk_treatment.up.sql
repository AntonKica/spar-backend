CREATE TABLE tour_elementary_threat_risk_treatment
(
    risk_analysis_process_code         CHAR(8)     NOT NULL REFERENCES risk_analysis_process (code),
    asset_code                         VARCHAR(20) NOT NULL REFERENCES asset (code),
    tour_elementary_threat_code        VARCHAR(20) NOT NULL REFERENCES it_grundschutz_elementary_threat(code),
    potential_risk                     INTEGER     NOT NULL,
    remaining_risk                     INTEGER     NOT NULL,
    risk_treatment                     INTEGER     NOT NULL,
    description                        TEXT        NOT NULL,
    PRIMARY KEY (risk_analysis_process_code, asset_code, tour_elementary_threat_code)
);

CREATE TABLE tour_specific_threat_risk_treatment
(
    risk_analysis_process_code         CHAR(8)     NOT NULL REFERENCES risk_analysis_process (code),
    asset_code                         VARCHAR(20) NOT NULL REFERENCES asset (code),
    tour_specific_threat_code          VARCHAR(20) NOT NULL REFERENCES tour_specific_threat(code),
    potential_risk                     INTEGER     NOT NULL,
    remaining_risk                     INTEGER     NOT NULL,
    risk_treatment                     INTEGER     NOT NULL,
    description                        TEXT        NOT NULL,
    PRIMARY KEY (risk_analysis_process_code, asset_code, tour_specific_threat_code)
);

