CREATE TABLE business_process(code CHAR(7) NOT NULL PRIMARY KEY,
                              name VARCHAR(80) NOT NULL,
                              description VARCHAR(400) NOT NULL,
                              process_type VARCHAR(20) NOT NULL,
                              responsible VARCHAR(40) NOT NULL
);
CREATE TABLE staff(code CHAR(8) NOT NULL PRIMARY KEY,
                   name VARCHAR(80) NOT NULL,
                   description VARCHAR(400) NOT NULL
);

CREATE TABLE business_process__staff(
    business_process_code CHAR(7) REFERENCES business_process(code),
    staff_code CHAR(8) REFERENCES staff(code)
);