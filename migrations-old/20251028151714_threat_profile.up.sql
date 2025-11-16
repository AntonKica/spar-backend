CREATE TABLE protection_profile(
                                   it_system_code CHAR(9) REFERENCES it_system(code),
                                   confidentiality_requirement INTEGER NOT NULL,
                                   integrity_requirement INTEGER NOT NULL,
                                   availability_requirement INTEGER NOT NULL
)