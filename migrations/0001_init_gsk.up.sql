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
VALUES ('G-01', 'Požiar', '', FALSE, FALSE, TRUE, 'natural_threat'),
       ('G-02', 'Nepriaznivé klimatické podmienky', '', FALSE, TRUE, TRUE, 'natural_threat'),
       ('G-03', 'Voda', '', FALSE, TRUE, TRUE, 'natural_threat'),
       ('G-04', 'Znečistenie, prach, korózia', '', FALSE, TRUE, TRUE, 'natural_threat'),
       ('G-05', 'Prírodné katastrofy', '', FALSE, FALSE, TRUE, 'natural_threat'),
       ('G-06', 'Katastrofy v okolí', '', FALSE, FALSE, TRUE, 'natural_threat'),
       ('G-07', 'Významné udalosti v okolí', '', TRUE, TRUE, TRUE, 'natural_threat'),
       ('G-08', 'Zlyhanie alebo výpadok napájania', '', FALSE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-09', 'Zlyhanie alebo výpadok komunikačných sietí', '', FALSE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-10', 'Zlyhanie alebo výpadok zásobovacích sietí', '', FALSE, FALSE, TRUE, 'infrastructure_failure'),
       ('G-11', 'Zlyhanie alebo výpadok poskytovateľov služieb', '', TRUE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-12', 'Elektromagnetické rušenie', '', FALSE, TRUE, TRUE, 'infrastructure_failure'),
       ('G-13', 'Zachytenie kompromitujúcich rušivých signálov', '', TRUE, FALSE, FALSE, 'compromise_of_functions_and_services'),
       ('G-14', 'Zachytenie informácií / Špionáž', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-15', 'Odpočúvanie', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-16', 'Krádež zariadení, pamäťových médií a dokumentov', '', FALSE, TRUE, TRUE, 'physical_threats'),
       ('G-17', 'Strata zariadení, pamäťových médií a dokumentov', '', FALSE, TRUE, TRUE, 'physical_threats'),
       ('G-18', 'Zlé plánovanie alebo nedostatočná adaptácia', '', TRUE, TRUE, TRUE, 'organizational_threats'),
       ('G-19', 'Prezradenie citlivých informácií', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-20', 'Informácie alebo produkty z nespoľahlivého zdroja', '', TRUE, TRUE, TRUE, 'compromise_of_functions_and_services'),
       ('G-21', 'Manipulácia s hardvérom alebo softvérom', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-22', 'Manipulácia s informáciami', '', FALSE, TRUE, FALSE, 'human_actions'),
       ('G-23', 'Neoprávnený prístup k IT systémom', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-24', 'Zničenie zariadení alebo pamäťových médií', '', FALSE, FALSE, TRUE, 'physical_threats'),
       ('G-25', 'Zlyhanie zariadení alebo systémov', '', FALSE, FALSE, TRUE, 'technical_failures'),
       ('G-26', 'Porucha zariadení alebo systémov', '', TRUE, TRUE, TRUE, 'technical_failures'),
       ('G-27', 'Nedostatok zdrojov', '', FALSE, FALSE, TRUE, 'organizational_threats'),
       ('G-28', 'Zraniteľnosti alebo chyby softvéru', '', TRUE, TRUE, TRUE, 'technical_failures'),
       ('G-29', 'Porušenie zákonov alebo predpisov', '', TRUE, TRUE, TRUE, 'organizational_threats'),
       ('G-30', 'Neoprávnené používanie alebo správa zariadení a systémov', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-31', 'Nesprávne používanie alebo správa zariadení a systémov', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-32', 'Zneužitie oprávnení', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-33', 'Nedostatok personálu', '', FALSE, FALSE, TRUE, 'organizational_threats'),
       ('G-34', 'Napadnutie', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-35', 'Nátlak, vydieranie alebo korupcia', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-36', 'Krádež identity', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-37', 'Popretie vykonaných akcií', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-38', 'Zneužitie osobných údajov', '', TRUE, FALSE, FALSE, 'human_actions'),
       ('G-39', 'Malvér', '', TRUE, TRUE, TRUE, 'human_actions'),
       ('G-40', 'Odmietnutie služby', '', FALSE, FALSE, TRUE, 'human_actions'),
       ('G-41', 'Sabotáž', '', FALSE, FALSE, TRUE, 'human_actions'),
       ('G-42', 'Sociálne inžinierstvo', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-43', 'Útok pomocou špeciálne vytvorených správ', '', TRUE, TRUE, FALSE, 'human_actions'),
       ('G-44', 'Neoprávnený vstup do priestorov', '', TRUE, TRUE, TRUE, 'physical_threats'),
       ('G-45', 'Strata dát', '', FALSE, FALSE, TRUE, 'technical_failures'),
       ('G-46', 'Strata integrity citlivých informácií', '', FALSE, TRUE, FALSE, 'technical_failures'),
       ('G-47', 'Škodlivé vedľajšie účinky IT útokov', '', FALSE, FALSE, TRUE, 'human_actions')
;

INSERT INTO it_grundschutz_module
VALUES ('SYS-3-1', 'Notebooky', 'Notebook (tiež nazývaný laptop) je osobný počítač určený na mobilné použitie.'),
       ('ORP-2', 'Personál', 'Zamestnanci spoločnosti alebo verejného orgánu majú rozhodujúci vplyv na jej úspech alebo neúspech.')
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
    ('ORP-2-A1', 'ORP-2', 'Riadené oboznámenie nových zamestnancov'),
    ('ORP-2-A2', 'ORP-2', 'Riadený postup pri odchode zamestnancov z organizácie'),
    ('ORP-2-A3', 'ORP-2', 'Definovanie pravidiel zastupovania'),
    ('ORP-2-A4', 'ORP-2', 'Definovanie postupov pri využívaní externého personálu'),
    ('ORP-2-A5', 'ORP-2', 'Dohody o mlčanlivosti pre externý personál'),
    ('ORP-2-A7', 'ORP-2', 'Overovanie dôveryhodnosti zamestnancov'),
    ('ORP-2-A13', 'ORP-2', 'Bezpečnostná previerka'),
    ('ORP-2-A14', 'ORP-2', 'Úlohy a zodpovednosti zamestnancov'),
    ('ORP-2-A15', 'ORP-2', 'Kvalifikácia personálu'),
    ('SYS-3-1-A1', 'SYS-3-1', 'Pravidlá mobilného používania laptopov'),
    ('SYS-3-1-A3', 'SYS-3-1', 'Používanie osobných firewallov'),
    ('SYS-3-1-A6', 'SYS-3-1', 'Bezpečnostné smernice pre laptopy'),
    ('SYS-3-1-A7', 'SYS-3-1', 'Riadané vydávanie a vrátenie laptopov'),
    ('SYS-3-1-A8', 'SYS-3-1', 'Bezpečné pripojenie laptopov k dátovým sieťam'),
    ('SYS-3-1-A9', 'SYS-3-1', 'Bezpečný vzdialený prístup pomocou laptopov'),
    ('SYS-3-1-A10', 'SYS-3-1', 'Synchronizácia uložených dát na laptopoch'),
    ('SYS-3-1-A11', 'SYS-3-1', 'Zabezpečenie napájania laptopov'),
    ('SYS-3-1-A12', 'SYS-3-1', 'Hlásenie straty laptopov'),
    ('SYS-3-1-A13', 'SYS-3-1', 'Šifrovanie laptopov'),
    ('SYS-3-1-A14', 'SYS-3-1', 'Vhodné skladovanie laptopov'),
    ('SYS-3-1-A15', 'SYS-3-1', 'Vhodný výber laptopov'),
    ('SYS-3-1-A16', 'SYS-3-1', 'Centrálna správa a administrácia laptopov'),
    ('SYS-3-1-A17', 'SYS-3-1', 'Spoločné skladovanie laptopov'),
    ('SYS-3-1-A18', 'SYS-3-1', 'Používanie zariadení proti krádeži')
;