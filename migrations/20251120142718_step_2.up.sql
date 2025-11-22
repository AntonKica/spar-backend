CREATE TABLE tour_et_list
(
    rap_code               CHAR(10) NOT NULL REFERENCES risk_analysis_process (code),
    tour_code             CHAR(10) NOT NULL REFERENCES asset (code),
    et_code                CHAR(4)  NOT NULL REFERENCES elementary_threat (code),
    relevance              INTEGER  NOT NULL,
    explanation            TEXT     NOT NULL
);

CREATE TABLE tour_st_list
(
    rap_code               CHAR(10) NOT NULL REFERENCES risk_analysis_process (code),
    tour_code             CHAR(10) NOT NULL REFERENCES asset (code),
    st_code                CHAR(10)  NOT NULL REFERENCES specific_threat (code),
    explanation            TEXT     NOT NULL
);
