CREATE SEQUENCE asset_code_seq;
CREATE TABLE asset
(
    code        CHAR(8)      NOT NULL PRIMARY KEY DEFAULT 'AST-' || LPAD(nextval('asset_code_seq')::TEXT, 4, '0'),
    name        VARCHAR(200) NOT NULL,
    description TEXT         NOT NULL,
    module      VARCHAR(10)  NOT NULL REFERENCES it_grundschutz_module (code)
);

ALTER SEQUENCE asset_code_seq OWNED BY asset.code;