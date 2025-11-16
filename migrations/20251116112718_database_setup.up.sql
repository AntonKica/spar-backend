CREATE TABLE asset
(
    code                             CHAR(10)     NOT NULL PRIMARY KEY,
    name                             VARCHAR(200) NOT NULL,
    asset_type                       INTEGER      NOT NULL,
    confidentiality_protection_needs INTEGER      NOT NULL,
    integrity_protection_needs       INTEGER      NOT NULL,
    availability_protection_needs    INTEGER      NOT NULL,
    description                      TEXT         NOT NULL
);

CREATE TABLE security_measure
(
    code                      CHAR(10)     NOT NULL PRIMARY KEY,
    name                      VARCHAR(200) NOT NULL,
    description               TEXT         NOT NULL,
    confidentiality_protected BOOLEAN      NOT NULL,
    integrity_protected       BOOLEAN      NOT NULL,
    availability_protected    BOOLEAN      NOT NULL
);

CREATE TABLE asset_security_measure_list
(
    asset_code            CHAR(10) NOT NULL REFERENCES asset (code),
    security_measure_code CHAR(10) NOT NULL REFERENCES security_measure (code)
);

CREATE TABLE fulfilled_threat
(
    code                   CHAR(10) NOT NULL PRIMARY KEY,
    elementary_threat_code CHAR(4)  NULL REFERENCES elementary_threat (code),
    specific_threat_code   CHAR(10) NULL REFERENCES specific_threat (code),
    time_cost              INTEGER  NULL,
    time_cost_unit         INTEGER  NULL,
    monetary_cost          INTEGER  NULL,
    description            TEXT     NOT NULL,
    CHECK (elementary_threat_code IS NOT NULL OR specific_threat_code IS NOT NULL)
);

CREATE TABLE asset_fulfilled_threat_list
(
    asset_code            CHAR(10) NOT NULL REFERENCES asset (code),
    fulfilled_threat_code CHAR(10) NOT NULL REFERENCES fulfilled_threat (code)
);

CREATE TABLE risk_analysis_process
(
    code                                        CHAR(10) NOT NULL PRIMARY KEY,
    created_on                                  DATE     NOT NULL,
    process_status                              INTEGER  NOT NULL,
    step_1_select_tour_process_status           INTEGER  NOT NULL,
    step_2_threat_identification_process_status INTEGER  NOT NULL,
    step_3_risk_analysis_process_status         INTEGER  NOT NULL,
    step_4_risk_treatment_process_status        INTEGER  NOT NULL,
    step_5_risk_treatment_check_process_status  INTEGER  NOT NULL
);

CREATE TABLE risk_analysis_process_tour_list
(
    risk_analysis_process_code CHAR(10) NOT NULL REFERENCES risk_analysis_process(code),
    asset_code CHAR(10) NOT NULL REFERENCES asset(code)
);