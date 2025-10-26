CREATE TABLE role(code CHAR(7) NOT NULL PRIMARY KEY,
                   name VARCHAR(80) NOT NULL,
                   description VARCHAR(400) NOT NULL
);
CREATE TABLE business_process(code CHAR(7) NOT NULL PRIMARY KEY,
                              name VARCHAR(80) NOT NULL,
                              description VARCHAR(400) NOT NULL,
                              process_type INTEGER NOT NULL,
                              responsible CHAR(7) NULL REFERENCES role(code)
);

CREATE TABLE business_process__role(
    business_process_code CHAR(7) NOT NULL REFERENCES business_process(code),
    role_code CHAR(8) NOT NULL REFERENCES role(code)
);

CREATE TABLE application(code CHAR(9) NOT NULL PRIMARY KEY,
                         name VARCHAR(80) NOT NULL,
                         description VARCHAR(400) NOT NULL,
                         module_type INTEGER NOT NULL,
                         responsible CHAR(7) NOT NULL REFERENCES role(code),
                         application_user CHAR(7) NOT NULL REFERENCES role(code)
);

CREATE TABLE business_process__application(
                                       business_process_code CHAR(7) NOT NULL REFERENCES business_process(code),
                                       application_code CHAR(9) NOT NULL REFERENCES application(code)
);

CREATE TABLE it_system(code CHAR(9) NOT NULL PRIMARY KEY,
                         name VARCHAR(80) NOT NULL,
                         description VARCHAR(400) NOT NULL,
                         module_type INTEGER NOT NULL,
                         count INTEGER NOT NULL,
                         responsible CHAR(7) NOT NULL REFERENCES role(code),
                         application_user CHAR(7) NOT NULL REFERENCES role(code)
);