CREATE TABLE tour_threat_list
(
    rap_code    CHAR(10)    NOT NULL REFERENCES risk_analysis_process (code),
    tour_code   CHAR(10)    NOT NULL REFERENCES asset (code),
    threat_code VARCHAR(10) NOT NULL REFERENCES threat (code),
    relevance   INTEGER     NOT NULL,
    explanation TEXT        NOT NULL
);