CREATE TYPE implementation_status AS ENUM (
    'not_assessed',
    'none',
    'partial',
    'full',
    'redundant'
    );

CREATE TABLE risk_treatment_requirement_assessment (
                                                       id              UUID                  NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
                                                       risk_analysis   CHAR(6)               NOT NULL REFERENCES risk_analysis(code) ON DELETE CASCADE,
                                                       risk_treatment  UUID                  NOT NULL,
                                                       requirement     VARCHAR(20)           NOT NULL,
                                                       status          implementation_status NOT NULL DEFAULT 'not_assessed',
                                                       evaluation      TEXT                  NOT NULL DEFAULT '',
                                                       FOREIGN KEY (risk_treatment, requirement)
                                                           REFERENCES risk_treatment_requirement(risk_treatment, requirement) ON DELETE CASCADE
);

CREATE TABLE risk_treatment_security_measure_assessment (
                                                            id               UUID                  NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
                                                            risk_analysis    CHAR(6)               NOT NULL REFERENCES risk_analysis(code) ON DELETE CASCADE,
                                                            risk_treatment   UUID                  NOT NULL,
                                                            security_measure CHAR(8)               NOT NULL,
                                                            status           implementation_status NOT NULL DEFAULT 'not_assessed',
                                                            evaluation       TEXT                  NOT NULL DEFAULT '',
                                                            FOREIGN KEY (risk_treatment, security_measure)
                                                                REFERENCES risk_treatment_security_measure(risk_treatment, security_measure) ON DELETE CASCADE
);