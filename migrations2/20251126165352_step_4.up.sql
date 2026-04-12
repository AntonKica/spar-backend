CREATE TABLE risk_treatment_code(code CHAR(10) NOT NULL PRIMARY KEY);
CREATE TABLE risk_treatment
(
    rap_code    CHAR(10)    NOT NULL REFERENCES risk_analysis_process (code),
    tour_code   CHAR(10)    NOT NULL REFERENCES asset (code),
    threat_code VARCHAR(10) NOT NULL REFERENCES threat (code),
    treatment_type INTEGER NOT NULL,
    treatment_code  CHAR(10) NOT NULL REFERENCES risk_treatment_code(code),
    PRIMARY KEY (rap_code, tour_code, threat_code, treatment_code)
);

CREATE TABLE risk_acceptance
(
    code CHAR(10) NOT NULL PRIMARY KEY REFERENCES risk_treatment_code(code),
    name VARCHAR(200) NOT NULL,
    explanation TEXT NOT NULL
);

CREATE TABLE risk_avoidance
(
    code CHAR(10) NOT NULL PRIMARY KEY REFERENCES risk_treatment_code(code),
    name VARCHAR(200) NOT NULL,
    explanation TEXT NOT NULL
);

CREATE TABLE risk_transfer
(
    code CHAR(10) NOT NULL PRIMARY KEY REFERENCES risk_treatment_code(code),
    name VARCHAR(200) NOT NULL,
    risk_transfer_type INTEGER NOT NULL,
    checklist VARCHAR(100)[] NOT NULL,
    explanation TEXT NOT NULL
);

CREATE TABLE risk_reduction
(
    code CHAR(10) NOT NULL PRIMARY KEY REFERENCES risk_treatment_code(code),
    name VARCHAR(200) NOT NULL,
    confidentiality_protected BOOLEAN      NOT NULL,
    integrity_protected       BOOLEAN      NOT NULL,
    availability_protected    BOOLEAN      NOT NULL,
    explanation TEXT NOT NULL
);