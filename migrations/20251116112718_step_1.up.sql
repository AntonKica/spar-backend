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

CREATE TABLE asset_sm_list
(
    asset_code CHAR(10) NOT NULL REFERENCES asset (code),
    sm_code    CHAR(10) NOT NULL REFERENCES security_measure (code)
);

CREATE TABLE risk_analysis_process
(
    code           CHAR(10) NOT NULL PRIMARY KEY,
    created_on     DATE     NOT NULL,
    process_status INTEGER  NOT NULL,
    process_step   INTEGER  NOT NULL
);

CREATE TABLE rap_tour_list
(
    rap_code   CHAR(10) NOT NULL REFERENCES risk_analysis_process (code),
    asset_code CHAR(10) NOT NULL REFERENCES asset (code)
);