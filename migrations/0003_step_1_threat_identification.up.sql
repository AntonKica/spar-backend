CREATE SEQUENCE risk_analysis_code_seq;
CREATE TYPE risk_analysis_state AS ENUM ('threat_identification');

CREATE TABLE risk_analysis
(
    code       CHAR(6)      NOT NULL PRIMARY KEY DEFAULT 'RA-' || LPAD(nextval('risk_analysis_code_seq')::TEXT, 3, '0'),
    state      risk_analysis_state NOT NULL,
    created_at DATE     NOT NULL DEFAULT now()
);
ALTER SEQUENCE risk_analysis_code_seq OWNED BY risk_analysis.code;

CREATE TABLE risk_analysis_asset (
                                     risk_analysis  CHAR(6)    NOT NULL REFERENCES risk_analysis(code) ON DELETE CASCADE,
                                     asset          CHAR(8) NOT NULL REFERENCES asset(code),
                                     PRIMARY KEY (risk_analysis, asset)
);

CREATE TABLE risk_analysis_module (
                                      risk_analysis  CHAR(6)         NOT NULL REFERENCES risk_analysis(code) ON DELETE CASCADE,
                                      module         VARCHAR(10)  NOT NULL REFERENCES it_grundschutz_module(code),
                                      PRIMARY KEY (risk_analysis, module)
);
CREATE TABLE risk_analysis_threat (
                                      risk_analysis  CHAR(6)         NOT NULL REFERENCES risk_analysis(code) ON DELETE CASCADE,
                                      module         VARCHAR(10)  NOT NULL REFERENCES it_grundschutz_module(code),
                                      threat         VARCHAR(20)  NOT NULL REFERENCES threat(code),
                                      stage          SMALLINT     NOT NULL CHECK (stage IN (1, 2, 3)),
                                      PRIMARY KEY (risk_analysis, module, threat)
);