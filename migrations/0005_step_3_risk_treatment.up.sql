CREATE TYPE risk_treatment_type AS ENUM (
    'avoid',
    'reduce',
    'transfer',
    'accept'
    );

CREATE SEQUENCE risk_treatment_code_seq;

CREATE TABLE risk_treatment
(
    code          CHAR(8)             NOT NULL PRIMARY KEY DEFAULT 'TRM-' || LPAD(nextval('risk_treatment_code_seq')::TEXT, 4, '0'),
    risk_analysis CHAR(6)             NOT NULL REFERENCES risk_analysis (code) ON DELETE CASCADE,
    module        VARCHAR(10) REFERENCES it_grundschutz_module (code),
    threat        VARCHAR(20) REFERENCES threat (code),
    treatment     risk_treatment_type NOT NULL,
    description   TEXT                NOT NULL             DEFAULT ''
);
ALTER SEQUENCE risk_treatment_code_seq OWNED BY risk_treatment.code;

CREATE TABLE security_measure
(
    code           CHAR(8) NOT NULL PRIMARY KEY,
    treatment      risk_treatment_type NOT NULL,
    description    TEXT    NOT NULL
);
CREATE SEQUENCE security_measure_avd_seq OWNED BY security_measure.code;
CREATE SEQUENCE security_measure_red_seq OWNED BY security_measure.code;
CREATE SEQUENCE security_measure_tsf_seq OWNED BY security_measure.code;
CREATE SEQUENCE security_measure_acp_seq OWNED BY security_measure.code;

CREATE TABLE risk_treatment_requirement
(
    risk_treatment CHAR(8)     NOT NULL REFERENCES risk_treatment (code) ON DELETE CASCADE,
    requirement    VARCHAR(20) NOT NULL REFERENCES it_grundschutz_module_requirement (code),
    PRIMARY KEY (risk_treatment, requirement)
);
CREATE TABLE risk_treatment_security_measure
(
    risk_treatment CHAR(8) NOT NULL REFERENCES risk_treatment (code) ON DELETE CASCADE,
    security_measure CHAR(8) NOT NULL REFERENCES security_measure (code),
    PRIMARY KEY (risk_treatment, security_measure)
);