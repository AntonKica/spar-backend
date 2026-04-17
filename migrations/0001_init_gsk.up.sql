CREATE TYPE threat_category AS ENUM (
    'natural_threat',
    'infrastructure_failure',
    'compromise_of_functions_and_services',
    'human_actions',
    'physical_threats',
    'technical_failures',
    'organizational_threats',
    'other'
);

CREATE TABLE threat
(
    code                     VARCHAR(10)  NOT NULL PRIMARY KEY,
    name                     VARCHAR(160) NOT NULL,
    description              TEXT         NOT NULL,
    confidentiality_impaired BOOLEAN      NOT NULL,
    integrity_impaired       BOOLEAN      NOT NULL,
    availability_impaired    BOOLEAN      NOT NULL,
    category                 threat_category NOT NULL
);

CREATE SEQUENCE specific_threat_code_seq;

CREATE TABLE it_grundschutz_module
(
    code                     VARCHAR(10)  NOT NULL PRIMARY KEY,
    name                     VARCHAR(160) NOT NULL,
    description              TEXT         NOT NULL
);


CREATE TABLE it_grundschutz_module_threat
(
    it_grundschutz_module VARCHAR(10) NOT NULL REFERENCES it_grundschutz_module(code),
    threat VARCHAR(10) NOT NULL REFERENCES threat(code)
);

CREATE TABLE it_grundschutz_module_requirement
(
    code                     VARCHAR(20)  NOT NULL PRIMARY KEY,
    module                   VARCHAR(10) NOT NULL REFERENCES it_grundschutz_module(code),
    description              TEXT         NOT NULL
);

INSERT INTO threat
VALUES ('G-01', 'Fire', '', FALSE, FALSE, TRUE, 'natural_threat'),
       ('G-02', 'Unfavourable Climatic Conditions', '', FALSE, TRUE, TRUE, 'natural_threat'),
       ('G-03', 'Water', '', FALSE, TRUE, TRUE, 'natural_threat'),
       ('G-04', 'Pollution, Dust, Corrosion', '', FALSE, TRUE, TRUE, 'natural_threat'),
       ('G-05', 'Natural Disasters', '', FALSE, FALSE, TRUE, 'natural_threat'),
       ('G-06', 'Catastrophes in the Vicinity', '', FALSE, FALSE, TRUE, 'natural_threat'),
       ('G-07', 'Major Events in the Vicinity', '', TRUE, TRUE, TRUE, 'natural_threat'),
       ('G-08', 'Failure or Disruption of the Power Supply', '', FALSE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-09', 'Failure or Disruption of Communication Networks', '', FALSE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-10', 'Failure or Disruption of Supply Networks', '', FALSE, FALSE, TRUE, 'infrastructure_failure'),
       ('G-11', 'Failure or Disruption of Service Providers', '', TRUE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-12', 'Electromagnetic Interference', '', FALSE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-13', 'Interception of Compromising Interference Signals', '', TRUE, FALSE, FALSE, 'compromise_of_functions_and_services'),
       ('G-14', 'Interception of Information / Espionage', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-15', 'Eavesdropping', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-16', 'Theft of Devices, Storage Media and Documents', '', FALSE, TRUE, TRUE, 'physical_threats'),
       ('G-17', 'Loss of Devices, Storage Media and Documents', '', FALSE, TRUE, TRUE, 'physical_threats'),
       ('G-18', 'Poor Planning or Lack of Adaptation', '', TRUE, TRUE, TRUE, 'organizational_threats'),
       ('G-19', 'Disclosure of Sensitive Information', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-20', 'Information or Products from an Unreliable Source', '', TRUE, TRUE, TRUE, 'compromise_of_functions_and_services'),
       ('G-21', 'Manipulation with Hardware or Software', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-22', 'Manipulation of Information', '', FALSE, TRUE, FALSE, 'human_actions'),
       ('G-23', 'Unauthorised Access to IT Systems', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-24', 'Destruction of Devices or Storage Media', '', FALSE, FALSE, TRUE, 'physical_threats'),
       ('G-25', 'Failure of Devices or Systems', '', FALSE, FALSE, TRUE, 'technical_failures'),
       ('G-26', 'Malfunction of Devices or Systems', '', TRUE, TRUE, TRUE, 'technical_failures'),
       ('G-27', 'Lack of Resources', '', FALSE, FALSE, TRUE, 'organizational_threats'),
       ('G-28', 'Software Vulnerabilities or Errors', '', TRUE, TRUE, TRUE, 'technical_failures'),
       ('G-29', 'Violations of Laws or Regulations', '', TRUE, TRUE, TRUE, 'organizational_threats'),
       ('G-30', 'Unauthorised Use or Administration of Devices and Systems', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-31', 'Incorrect Use or Administration of Devices and Systems', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-32', 'Misuse of Authorisation', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-33', 'Shortage of Personnel', '', FALSE, FALSE, TRUE, 'organizational_threats'),
       ('G-34', 'Assault', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-35', 'Coercion, Blackmail or Corruption', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-36', 'Identity theft', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-37', 'Repudiation of Actions', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-38', 'Misuse of Personal Information', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-39', 'Malware', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-40', 'Denial of Service', '', FALSE, FALSE, TRUE, 'human_actions'),
       ('G-41', 'Sabotage', '', FALSE, FALSE, TRUE, 'human_actions'),
       ('G-42', 'Social Engineering', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-43', 'Attack with Specially Crafted Messages', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-44', 'Unauthorised Entry to Premises', '', TRUE, TRUE, TRUE, 'physical_threats'),
       ('G-45', 'Data Loss', '', FALSE, FALSE, TRUE, 'technical_failures'),
       ('G-46', 'Loss of Integrity of Sensitive Information', '', FALSE, TRUE, FALSE, 'technical_failures'),
       ('G-47', 'Harmful Side Effects of IT-Supported Attacks', '', FALSE, FALSE, TRUE, 'human_actions')
;

INSERT INTO it_grundschutz_module
VALUES ('SYS-3-1', 'Laptops', 'A laptop (also referred to as a notebook) is a PC designed for mobile use.'),
       ('ORP-2', 'Personnel', 'The staff of a company or public authority are crucial to its success or failure.')
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
