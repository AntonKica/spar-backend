CREATE TABLE risk_classification
(
    rap_code    CHAR(10)    NOT NULL REFERENCES risk_analysis_process (code),
    tour_code   CHAR(10)    NOT NULL REFERENCES asset (code),
    threat_code VARCHAR(10) NOT NULL REFERENCES threat (code),
    probability INTEGER  NOT NULL,
    impact      INTEGER  NOT NULL,
    evaluation  TEXT     NOT NULL,
    PRIMARY KEY (rap_code, tour_code, threat_code)
);