CREATE TABLE tour_specific_threat
(
    code                       CHAR(10)     NOT NULL PRIMARY KEY,
    risk_analysis_process_code CHAR(8)      NOT NULL REFERENCES risk_analysis_process (code),
    asset_code                 VARCHAR(20)  NOT NULL REFERENCES asset (code),
    name                       VARCHAR(80)  NOT NULL,
    description                VARCHAR(400) NOT NULL,
    confidentiality_impaired   BOOLEAN      NOT NULL,
    integrity_impaired         BOOLEAN      NOT NULL,
    availability_impaired      BOOLEAN      NOT NULL
);

CREATE TABLE tour_specific_threat_overview
(
    risk_analysis_process_code CHAR(8)     NOT NULL REFERENCES risk_analysis_process (code),
    asset_code                 VARCHAR(20) NOT NULL REFERENCES asset (code),
    reviewed                   BOOLEAN     NOT NULL
);
