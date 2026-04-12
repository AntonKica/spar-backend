CREATE TABLE it_grundschutz_threat
(
    code                     VARCHAR(10)  NOT NULL PRIMARY KEY,
    name                     VARCHAR(160) NOT NULL,
    description              TEXT         NOT NULL,
    confidentiality_impaired BOOLEAN      NOT NULL,
    integrity_impaired       BOOLEAN      NOT NULL,
    availability_impaired    BOOLEAN      NOT NULL
);

CREATE TABLE it_grundschutz_module
(
    code                     VARCHAR(10)  NOT NULL PRIMARY KEY,
    name                     VARCHAR(160) NOT NULL,
    description              TEXT         NOT NULL
);


CREATE TABLE it_grundschutz_module_threat
(
    it_grundschutz_module VARCHAR(10) NOT NULL REFERENCES it_grundschutz_module(code),
    it_grundschutz_threat VARCHAR(10) NOT NULL REFERENCES it_grundschutz_threat(code)
);

CREATE TABLE it_grundschutz_module_requirement
(
    code                     VARCHAR(20)  NOT NULL PRIMARY KEY,
    module                   VARCHAR(10) NOT NULL REFERENCES it_grundschutz_module(code),
    description              TEXT         NOT NULL
);


INSERT INTO it_grundschutz_threat
VALUES ('G-01', 'G 0.1 Fire', '', FALSE, FALSE, TRUE),
       ('G-02', 'G 0.2 Unfavourable Climatic Conditions', '', FALSE, TRUE, TRUE),
       ('G-03', 'G 0.3 Water', '', FALSE, TRUE, TRUE),
       ('G-04', 'G 0.4 Pollution, Dust, Corrosion', '', FALSE, TRUE, TRUE),
       ('G-05', 'G 0.5 Natural Disasters', '', FALSE, FALSE, TRUE),
       ('G-06', 'G 0.6 Catastrophes in the Vicinity', '', FALSE, FALSE, TRUE),
       ('G-07', 'G 0.7 Major Events in the Vicinity', '', TRUE, TRUE, TRUE),
       ('G-08', 'G 0.8 Failure or Disruption of the Power Supply', '', FALSE, TRUE, TRUE),
       ('G-09', 'G 0.9 Failure or Disruption of Communication Networks', '', FALSE, TRUE, TRUE),
       ('G-10', 'G 0.10 Failure or Disruption of Supply Networks', '', FALSE, FALSE, TRUE),
       ('G-11', 'G 0.11 Failure or Disruption of Service Providers', '', TRUE, TRUE, TRUE),
       ('G-12', 'G 0.12 Electromagnetic Interference', '', FALSE, TRUE, TRUE),
       ('G-13', 'G 0.13 Interception of Compromising Interference Signals', '', TRUE, FALSE, FALSE),
       ('G-14', 'G 0.14 Interception of Information / Espionage', '', TRUE, FALSE, FALSE),
       ('G-15', 'G 0.15 Eavesdropping', '', TRUE, FALSE, FALSE),
       ('G-16', 'G 0.16 Theft of Devices, Storage Media and Documents', '', FALSE, TRUE, TRUE),
       ('G-17', 'G 0.17 Loss of Devices, Storage Media and Documents', '', FALSE, TRUE, TRUE),
       ('G-18', 'G 0.18 Poor Planning or Lack of Adaptation', '', TRUE, TRUE, TRUE),
       ('G-19', 'G 0.19 Disclosure of Sensitive Information', '', TRUE, FALSE, FALSE),
       ('G-20', 'G 0.20 Information or Products from an Unreliable Source', '', TRUE, TRUE, TRUE),
       ('G-21', 'G 0.21 Manipulation with Hardware or Software', '', TRUE, TRUE, TRUE),
       ('G-22', 'G 0.22 Manipulation of Information', '', FALSE, TRUE, FALSE),
       ('G-23', 'G 0.23 Unauthorised Access to IT Systems', '', TRUE, TRUE, FALSE),
       ('G-24', 'G 0.24 Destruction of Devices or Storage Media', '', FALSE, FALSE, TRUE),
       ('G-25', 'G 0.25 Failure of Devices or Systems', '', FALSE, FALSE, TRUE),
       ('G-26', 'G 0.26 Malfunction of Devices or Systems', '', TRUE, TRUE, TRUE),
       ('G-27', 'G 0.27 Lack of Resources', '', FALSE, FALSE, TRUE),
       ('G-28', 'G 0.28 Software Vulnerabilities or Errors', '', TRUE, TRUE, TRUE),
       ('G-29', 'G 0.29 Violations of Laws or Regulations', '', TRUE, TRUE, TRUE),
       ('G-30', 'G 0.30 Unauthorised Use or Administration of Devices and Systems', '', TRUE, TRUE, TRUE),
       ('G-31', 'G 0.31 Incorrect Use or Administration of Devices and Systems', '', TRUE, TRUE, TRUE),
       ('G-32', 'G 0.32 Misuse of Authorisation', '', TRUE, TRUE, TRUE),
       ('G-33', 'G 0.33 Shortage of Personnel', '', FALSE, FALSE, TRUE),
       ('G-34', 'G 0.34 Assault', '', TRUE, TRUE, TRUE),
       ('G-35', 'G 0.35 Coercion, Blackmail or Corruption', '', TRUE, TRUE, TRUE),
       ('G-36', 'G 0.36 Identity theft', '', TRUE, TRUE, TRUE),
       ('G-37', 'G 0.37 Repudiation of Actions', '', TRUE, TRUE, FALSE),
       ('G-38', 'G 0.38 Misuse of Personal Information', '', TRUE, FALSE, FALSE),
       ('G-39', 'G 0.39 Malware', '', TRUE, TRUE, TRUE),
       ('G-40', 'G 0.40 Denial of Service', '', FALSE, FALSE, TRUE),
       ('G-41', 'G 0.41 Sabotage', '', FALSE, FALSE, TRUE),
       ('G-42', 'G 0.42 Social Engineering', '', TRUE, TRUE, FALSE),
       ('G-43', 'G 0.43 Attack with Specially Crafted Messages', '', TRUE, TRUE, FALSE),
       ('G-44', 'G 0.44 Unauthorised Entry to Premises', '', TRUE, TRUE, TRUE),
       ('G-45', 'G 0.45 Data Loss', '', FALSE, FALSE, TRUE),
       ('G-46', 'G 0.46 Loss of Integrity of Sensitive Information', '', FALSE, TRUE, FALSE),
       ('G-47', 'G 0.47 Harmful Side Effects of IT-Supported Attacks', '', FALSE, FALSE, TRUE)
;

INSERT INTO it_grundschutz_module
VALUES ('SYS-3-1', 'SYS.3.1 Laptops', 'A laptop (also referred to as a notebook) is a PC designed for mobile use.'),
       ('ORP-2', 'ORP.2 Personnel', 'The staff of a company or public authority are crucial to its success or failure.')
;

INSERT INTO it_grundschutz_module_threat
VALUES
    ( 'SYS-3-1', 'G-04'),
    ( 'SYS-3-1', 'G-14'),
    ( 'SYS-3-1', 'G-16'),
    ( 'SYS-3-1', 'G-17'),
    ( 'SYS-3-1', 'G-18'),
    ( 'SYS-3-1', 'G-19'),
    ( 'SYS-3-1', 'G-22'),
    ( 'SYS-3-1', 'G-31'),
    ( 'SYS-3-1', 'G-39'),
    ( 'SYS-3-1', 'G-45'),

    ('ORP-2', 'G-14'),
    ('ORP-2', 'G-16'),
    ('ORP-2', 'G-17'),
    ('ORP-2', 'G-19'),
    ('ORP-2', 'G-22'),
    ('ORP-2', 'G-27'),
    ('ORP-2', 'G-29'),
    ('ORP-2', 'G-32'),
    ('ORP-2', 'G-33'),
    ('ORP-2', 'G-36'),
    ('ORP-2', 'G-37'),
    ('ORP-2', 'G-38'),
    ('ORP-2', 'G-41'),
    ('ORP-2', 'G-42'),
    ('ORP-2', 'G-44'),
    ('ORP-2', 'G-45'),
    ('ORP-2', 'G-46')
;

INSERT INTO it_grundschutz_module_requirement
VALUES
('ORP-2-A1', 'ORP-2', 'Well-Regulated Orientation of New Employees'),
('ORP-2-A2', 'ORP-2', 'Regulated Procedure for Employees Leaving the Organisation'),
('ORP-2-A3', 'ORP-2', 'Defining Deputising Rules'),
('ORP-2-A4', 'ORP-2', 'Defining Procedures for Using Third-Party Personnel'),
('ORP-2-A5', 'ORP-2', 'Confidentiality Agreements for Third-Party Personnel'),
('ORP-2-A7', 'ORP-2', 'Verifying the Trustworthiness of Employees'),
('ORP-2-A13', 'ORP-2', 'Security Vetting'),
('ORP-2-A14', 'ORP-2', 'Tasks and Responsibilities of Employees'),
('ORP-2-A15', 'ORP-2', 'Qualifications of Personnel'),
('SYS-3-1-A1', 'SYS-3-1', 'Rules for Mobile Laptop Use'),
('SYS-3-1-A3', 'SYS-3-1', 'Use of Personal Firewalls'),
('SYS-3-1-A6', 'SYS-3-1', 'Security Guidelines for Laptops'),
('SYS-3-1-A7', 'SYS-3-1', 'Orderly Issue and Return of Laptops'),
('SYS-3-1-A8', 'SYS-3-1', 'Secure Connection of Laptops to Data Networks'),
('SYS-3-1-A9', 'SYS-3-1', 'Secure Remote Access with Laptops'),
('SYS-3-1-A10', 'SYS-3-1', 'Synchronisation of Stored Data on Laptops'),
('SYS-3-1-A11', 'SYS-3-1', 'Securing the Power Supply for Laptops'),
('SYS-3-1-A12', 'SYS-3-1', 'Reporting the Loss of Laptops'),
('SYS-3-1-A13', 'SYS-3-1', 'Encryption of Laptops'),
('SYS-3-1-A14', 'SYS-3-1', 'Suitable Storage of Laptops'),
('SYS-3-1-A15', 'SYS-3-1', 'Appropriate Selection of Laptops'),
('SYS-3-1-A16', 'SYS-3-1', 'Central Administration and Administration of Laptops'),
('SYS-3-1-A17', 'SYS-3-1', 'Pooled Storage of Laptops'),
('SYS-3-1-A18', 'SYS-3-1', 'Use of Anti-Theft Devices')
;
