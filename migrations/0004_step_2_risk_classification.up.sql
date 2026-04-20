CREATE TYPE likelihood AS ENUM (
    'rarely',
    'medium',
    'often',
    'very_often'
    );

CREATE TYPE impact AS ENUM (
    'negligible',
    'limited',
    'significant',
    'life_threatening'
    );
CREATE TYPE risk AS ENUM (
    'low',
    'medium',
    'high',
    'very_high'
    );

CREATE TABLE risk_classification (
                                     risk_analysis  CHAR(6)     NOT NULL REFERENCES risk_analysis(code) ON DELETE CASCADE,
                                     module         VARCHAR(10) NOT NULL,
                                     threat         VARCHAR(20) NOT NULL,
                                     likelihood     likelihood  NOT NULL DEFAULT 'rarely',
                                     impact         impact      NOT NULL DEFAULT 'negligible',
                                     evaluation     TEXT        NOT NULL DEFAULT '',
                                     PRIMARY KEY (risk_analysis, module, threat),
                                     FOREIGN KEY (risk_analysis, module, threat)
                                     REFERENCES risk_analysis_threat(risk_analysis, module, threat) ON DELETE CASCADE
);