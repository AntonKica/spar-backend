use strum_macros::EnumIter;
use crate::response::EnumResponse;

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ProtectionNeeds {
    Normal,
    High,
    VeryHigh,
}

impl From<ProtectionNeeds> for EnumResponse {
    fn from(value: ProtectionNeeds) -> Self {
        match value {
            ProtectionNeeds::Normal => {
                EnumResponse {
                    code: ProtectionNeeds::Normal as i32,
                    name: "normálna".to_owned()
                }
            }
            ProtectionNeeds::High => {
                EnumResponse {
                    code: ProtectionNeeds::High as i32,
                    name: "vysoká".to_owned()
                }
            }
            ProtectionNeeds::VeryHigh => {
                EnumResponse {
                    code: ProtectionNeeds::VeryHigh as i32,
                    name: "veľmi vysoká".to_owned()
                }
            }
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ElementaryThreatRelevance {
    DIRECT,
    INDIRECT,
    IRRELEVANT,
}

impl From<ElementaryThreatRelevance> for EnumResponse {
    fn from(value: ElementaryThreatRelevance) -> Self {
        match value {
            ElementaryThreatRelevance::DIRECT => {
                EnumResponse {
                    code: ElementaryThreatRelevance::DIRECT as i32,
                    name: "priamy".to_owned()
                }
            }
            ElementaryThreatRelevance::INDIRECT => {
                EnumResponse {
                    code: ElementaryThreatRelevance::INDIRECT as i32,
                    name: "nepriamy".to_owned()
                }
            }
            ElementaryThreatRelevance::IRRELEVANT => {
                EnumResponse {
                    code: ElementaryThreatRelevance::IRRELEVANT as i32,
                    name: "irelevantný".to_owned()
                }
            }
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone)]
pub enum BusinessProcessType {
    UNKNOWN = -1,
    PRIMARY = 0,
    SUPPORT,
}

impl From<BusinessProcessType> for EnumResponse {
    fn from(value: BusinessProcessType) -> Self {
        match value {
            BusinessProcessType::SUPPORT => {
                EnumResponse {
                    code: BusinessProcessType::SUPPORT as i32,
                    name: "podporný".to_owned()
                }
            }
            BusinessProcessType::PRIMARY => {
                EnumResponse {
                    code: BusinessProcessType::PRIMARY as i32,
                    name: "primárny".to_owned()
                }
            }
            BusinessProcessType::UNKNOWN => {
                EnumResponse {
                    code: BusinessProcessType::UNKNOWN as i32,
                    name: "neznámy".to_owned()
                }
            }
        }
    }
}

impl From<i32> for BusinessProcessType {
    fn from(value: i32) -> Self {
        match value {
            0 => BusinessProcessType::PRIMARY,
            1 => BusinessProcessType::SUPPORT,
            _ => BusinessProcessType::UNKNOWN
        }
    }
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ProtectionRequirementType {
    UNKNOWN = -1,
    LOW = 0,
    MEDIUM,
    HIGH,
}

impl From<ProtectionRequirementType> for EnumResponse {
    fn from(value: ProtectionRequirementType) -> Self {
        match value {
            ProtectionRequirementType::LOW => {
                EnumResponse {
                    code: ProtectionRequirementType::LOW as i32,
                    name: "nízka".to_owned()
                }
            }
            ProtectionRequirementType::MEDIUM => {
                EnumResponse {
                    code: ProtectionRequirementType::MEDIUM as i32,
                    name: "stredná".to_owned()
                }
            }
            ProtectionRequirementType::HIGH => {
                EnumResponse {
                    code: BusinessProcessType::PRIMARY as i32,
                    name: "vysoká".to_owned()
                }
            }
            ProtectionRequirementType::UNKNOWN => {
                EnumResponse {
                    code: ProtectionRequirementType::UNKNOWN as i32,
                    name: "neznáma".to_owned()
                }
            }
        }
    }
}

impl From<i32> for ProtectionRequirementType {
    fn from(value: i32) -> Self {
        match value {
            0 => ProtectionRequirementType::LOW,
            1 => ProtectionRequirementType::MEDIUM,
            2 => ProtectionRequirementType::HIGH,
            _ => ProtectionRequirementType::UNKNOWN
        }
    }
}

#[repr(i32)]
#[derive(Debug, Copy, Clone, EnumIter)]
pub enum ModuleType {
    UNKNOWN = -1,
    ISMS_1_SECURITY_MANAGEMENT = 0,
    ORP_1_ORGANISATION,
    ORP_2_PERSONNEL,
    ORP_3_AWARENESS_AND_TRAINING_IN_INFORMATION_SECURITY,
    ORP_4_IDENTITY_AND_ACCESS_MANAGEMENT,
    ORP_5_COMPLIANCE_MANAGEMENT,
    CON_1_CRYPTO_CONCEPT,
    CON_2_DATA_PROTECTION,
    CON_3_BACKUP_CONCEPT,
    CON_6_DELETING_AND_DESTROYING_DATA_AND_DEVICES,
    CON_7_INFORMATION_SECURITY_ON_TRIPS_ABROAD,
    CON_8_SOFTWARE_DEVELOPMENT,
    CON_9_INFORMATION_EXCHANGE,
    CON_10_DEVELOPMENT_OF_WEB_APPLICATIONS,
    OPS_1_1_2_PROPER_IT_ADMINISTRATION,
    OPS_1_1_3_PATCH_AND_CHANGE_MANAGEMENT,
    OPS_1_1_4_PROTECTION_AGAINST_MALWARE,
    OPS_1_1_5_LOGGING,
    OPS_1_1_6_SOFTWARE_TESTS_AND_APPROVALS,
    OPS_1_1_7_SYSTEM_MANAGEMENT,
    OPS_1_2_2_ARCHIVING,
    OPS_1_2_4_TELEWORKING,
    OPS_1_2_5_REMOTE_MAINTENANCE,
    OPS_1_2_6_NTP_TIME_SYNCHRONISATION,
    OPS_2_1_OUTSOURCING_FOR_CUSTOMERS,
    OPS_2_2_CLOUD_USAGE,
    OPS_3_1_OUTSOURCING_FOR_SERVICE_PROVIDERS,
    DER_1_DETECTING_SECURITY_RELEVANT_EVENTS,
    DER_2_1_SECURITY_INCIDENT_HANDLING,
    DER_2_2_PROVISIONS_FOR_IT_FORENSICS,
    DER_2_3_CLEAN_UP_OF_EXTENSIVE_SECURITY_INCIDENTS,
    DER_3_1_AUDITS_AND_REVISIONS,
    DER_3_2_AUDITS_BASED_ON_THE_BSI_GUIDELINE_FOR_IS_AUDITS,
    DER_4_BUSINESS_CONTINUITY_MANAGEMENT,
    APP_1_1_OFFICE_PRODUCTS,
    APP_1_2_WEB_BROWSERS,
    APP_1_4_MOBILE_APPLICATIONS,
    APP_2_1_GENERAL_DIRECTORY_SERVICE,
    APP_2_2_ACTIVE_DIRECTORY,
    APP_2_3_OPENLDAP,
    APP_3_1_WEB_APPLICATIONS_AND_WEB_SERVICES,
    APP_3_2_WEB_SERVERS,
    APP_3_3_FILE_SERVERS,
    APP_3_4_SAMBA,
    APP_3_6_DNS_SERVERS,
    APP_4_2_SAP_ERP_SYSTEMS,
    APP_4_3_RELATIONAL_DATABASE_SYSTEMS,
    APP_4_4_KUBERNETES,
    APP_4_6_SAP_ABAP_PROGRAMMING,
    APP_5_2_MICROSOFT_EXCHANGE_AND_OUTLOOK,
    APP_5_3_GENERAL_E_MAIL_CLIENTS_AND_SERVERS,
    APP_6_GENERAL_SOFTWARE,
    APP_7_DEVELOPMENT_OF_INDIVIDUAL_SOFTWARE,
    SYS_1_1_GENERAL_SERVER,
    SYS_1_2_2_WINDOWS_SERVER_2012,
    SYS_1_3_LINUX_AND_UNIX_SERVERS,
    SYS_1_5_VIRTUALISATION,
    SYS_1_6_CONTAINERISATION,
    SYS_1_7_IBM_Z,
    SYS_1_8_STORAGE_SOLUTIONS,
    SYS_2_1_GENERAL_CLIENT,
    SYS_2_2_2_WINDOWS_8_1_CLIENTS,
    SYS_2_2_3_WINDOWS_10_CLIENTS,
    SYS_2_3_LINUX_AND_UNIX_CLIENTS,
    SYS_2_4_MACOS_CLIENTS,
    SYS_3_1_LAPTOPS,
    SYS_3_2_1_GENERAL_SMARTPHONES_AND_TABLETS,
    SYS_3_2_2_MOBILE_DEVICE_MANAGEMENT,
    SYS_3_2_3_IOS_FOR_ENTERPRISE,
    SYS_3_2_4_ANDROID,
    SYS_3_3_MOBILE_TELEPHONES,
    SYS_4_1_PRINTERS_COPIERS_AND_ALL_IN_ONE_DEVICES,
    SYS_4_3_EMBEDDED_SYSTEMS,
    SYS_4_4_GENERAL_IOT_DEVICES,
    SYS_4_5_REMOVABLE_MEDIA,
    IND_1_PROCESS_CONTROL_AND_AUTOMATION_TECHNOLOGY,
    IND_2_1_GENERAL_ICS_COMPONENTS,
    IND_2_2_PROGRAMMABLE_LOGIC_CONTROLLER,
    IND_2_3_SENSORS_AND_ACTUATORS,
    IND_2_4_MACHINE,
    IND_2_7_SAFETY_INSTRUMENTED_SYSTEMS,
    IND_3_2_REMOTE_MAINTENANCE_IN_INDUSTRY,
    NET_1_1_NETWORK_ARCHITECTURE_AND_DESIGN,
    NET_1_2_NETWORK_MANAGEMENT,
    NET_2_1_WLAN_OPERATION,
    NET_2_2_WLAN_USAGE,
    NET_3_1_ROUTERS_AND_SWITCHES,
    NET_3_2_FIREWALL,
    NET_3_3_VPN,
    NET_4_1_TELECOMMUNICATIONS_SYSTEMS,
    NET_4_2_VOIP,
    NET_4_3_FAX_MACHINES_AND_FAX_SERVERS,
    INF_1_GENERIC_BUILDING,
    INF_2_DATA_CENTRE_AND_SERVER_ROOM,
    INF_5_ROOM_OR_CABINET_FOR_TECHNICAL_INFRASTRUCTURE,
    INF_6_STORAGE_MEDIA_ARCHIVES,
    INF_7_OFFICE_WORKPLACE,
    INF_8_WORKING_FROM_HOME,
    INF_9_MOBILE_WORKPLACE,
    INF_10_MEETING_EVENT_AND_TRAINING_ROOMS,
    INF_11_GENERAL_VEHICLE,
    INF_12_CABLING,
    INF_13_TECHNICAL_BUILDING_MANAGEMENT,
    INF_14_BUILDING_AUTOMATION_AND_CONTROL_SYSTEMS,
}

impl From<ModuleType> for EnumResponse {
    fn from(value: ModuleType) -> Self {
        match value {
            ModuleType::ISMS_1_SECURITY_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::ISMS_1_SECURITY_MANAGEMENT as i32,
                    name: "ISMS.1 Security Management".to_owned()
                }
            }
            ModuleType::ORP_1_ORGANISATION => {
                EnumResponse {
                    code: ModuleType::ORP_1_ORGANISATION as i32,
                    name: "ORP.1 Organisation".to_owned()
                }
            }
            ModuleType::ORP_2_PERSONNEL => {
                EnumResponse {
                    code: ModuleType::ORP_2_PERSONNEL as i32,
                    name: "ORP.2 Personnel".to_owned()
                }
            }
            ModuleType::ORP_3_AWARENESS_AND_TRAINING_IN_INFORMATION_SECURITY => {
                EnumResponse {
                    code: ModuleType::ORP_3_AWARENESS_AND_TRAINING_IN_INFORMATION_SECURITY as i32,
                    name: "ORP.3 Awareness and Training in Information Security".to_owned()
                }
            }
            ModuleType::ORP_4_IDENTITY_AND_ACCESS_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::ORP_4_IDENTITY_AND_ACCESS_MANAGEMENT as i32,
                    name: "ORP.4 Identity and Access Management".to_owned()
                }
            }
            ModuleType::ORP_5_COMPLIANCE_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::ORP_5_COMPLIANCE_MANAGEMENT as i32,
                    name: "ORP.5 Compliance Management".to_owned()
                }
            }
            ModuleType::CON_1_CRYPTO_CONCEPT => {
                EnumResponse {
                    code: ModuleType::CON_1_CRYPTO_CONCEPT as i32,
                    name: "CON.1 Crypto Concept".to_owned()
                }
            }
            ModuleType::CON_2_DATA_PROTECTION => {
                EnumResponse {
                    code: ModuleType::CON_2_DATA_PROTECTION as i32,
                    name: "CON.2 Data Protection".to_owned()
                }
            }
            ModuleType::CON_3_BACKUP_CONCEPT => {
                EnumResponse {
                    code: ModuleType::CON_3_BACKUP_CONCEPT as i32,
                    name: "CON.3 Backup Concept".to_owned()
                }
            }
            ModuleType::CON_6_DELETING_AND_DESTROYING_DATA_AND_DEVICES => {
                EnumResponse {
                    code: ModuleType::CON_6_DELETING_AND_DESTROYING_DATA_AND_DEVICES as i32,
                    name: "CON.6 Deleting and Destroying Data and Devices".to_owned()
                }
            }
            ModuleType::CON_7_INFORMATION_SECURITY_ON_TRIPS_ABROAD => {
                EnumResponse {
                    code: ModuleType::CON_7_INFORMATION_SECURITY_ON_TRIPS_ABROAD as i32,
                    name: "CON.7 Information Security on Trips Abroad".to_owned()
                }
            }
            ModuleType::CON_8_SOFTWARE_DEVELOPMENT => {
                EnumResponse {
                    code: ModuleType::CON_8_SOFTWARE_DEVELOPMENT as i32,
                    name: "CON.8 Software Development".to_owned()
                }
            }
            ModuleType::CON_9_INFORMATION_EXCHANGE => {
                EnumResponse {
                    code: ModuleType::CON_9_INFORMATION_EXCHANGE as i32,
                    name: "CON.9 Information Exchange".to_owned()
                }
            }
            ModuleType::CON_10_DEVELOPMENT_OF_WEB_APPLICATIONS => {
                EnumResponse {
                    code: ModuleType::CON_10_DEVELOPMENT_OF_WEB_APPLICATIONS as i32,
                    name: "CON.10 Development of Web Applications".to_owned()
                }
            }
            ModuleType::OPS_1_1_2_PROPER_IT_ADMINISTRATION => {
                EnumResponse {
                    code: ModuleType::OPS_1_1_2_PROPER_IT_ADMINISTRATION as i32,
                    name: "OPS.1.1.2 Proper IT Administration".to_owned()
                }
            }
            ModuleType::OPS_1_1_3_PATCH_AND_CHANGE_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::OPS_1_1_3_PATCH_AND_CHANGE_MANAGEMENT as i32,
                    name: "OPS.1.1.3 Patch and Change Management".to_owned()
                }
            }
            ModuleType::OPS_1_1_4_PROTECTION_AGAINST_MALWARE => {
                EnumResponse {
                    code: ModuleType::OPS_1_1_4_PROTECTION_AGAINST_MALWARE as i32,
                    name: "OPS.1.1.4 Protection Against Malware".to_owned()
                }
            }
            ModuleType::OPS_1_1_5_LOGGING => {
                EnumResponse {
                    code: ModuleType::OPS_1_1_5_LOGGING as i32,
                    name: "OPS.1.1.5 Logging".to_owned()
                }
            }
            ModuleType::OPS_1_1_6_SOFTWARE_TESTS_AND_APPROVALS => {
                EnumResponse {
                    code: ModuleType::OPS_1_1_6_SOFTWARE_TESTS_AND_APPROVALS as i32,
                    name: "OPS.1.1.6 Software Tests and Approvals".to_owned()
                }
            }
            ModuleType::OPS_1_1_7_SYSTEM_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::OPS_1_1_7_SYSTEM_MANAGEMENT as i32,
                    name: "OPS.1.1.7 System Management".to_owned()
                }
            }
            ModuleType::OPS_1_2_2_ARCHIVING => {
                EnumResponse {
                    code: ModuleType::OPS_1_2_2_ARCHIVING as i32,
                    name: "OPS.1.2.2 Archiving".to_owned()
                }
            }
            ModuleType::OPS_1_2_4_TELEWORKING => {
                EnumResponse {
                    code: ModuleType::OPS_1_2_4_TELEWORKING as i32,
                    name: "OPS.1.2.4 Teleworking".to_owned()
                }
            }
            ModuleType::OPS_1_2_5_REMOTE_MAINTENANCE => {
                EnumResponse {
                    code: ModuleType::OPS_1_2_5_REMOTE_MAINTENANCE as i32,
                    name: "OPS.1.2.5 Remote Maintenance".to_owned()
                }
            }
            ModuleType::OPS_1_2_6_NTP_TIME_SYNCHRONISATION => {
                EnumResponse {
                    code: ModuleType::OPS_1_2_6_NTP_TIME_SYNCHRONISATION as i32,
                    name: "OPS.1.2.6 NTP Time Synchronisation".to_owned()
                }
            }
            ModuleType::OPS_2_1_OUTSOURCING_FOR_CUSTOMERS => {
                EnumResponse {
                    code: ModuleType::OPS_2_1_OUTSOURCING_FOR_CUSTOMERS as i32,
                    name: "OPS.2.1 Outsourcing for Customers".to_owned()
                }
            }
            ModuleType::OPS_2_2_CLOUD_USAGE => {
                EnumResponse {
                    code: ModuleType::OPS_2_2_CLOUD_USAGE as i32,
                    name: "OPS.2.2 Cloud Usage".to_owned()
                }
            }
            ModuleType::OPS_3_1_OUTSOURCING_FOR_SERVICE_PROVIDERS => {
                EnumResponse {
                    code: ModuleType::OPS_3_1_OUTSOURCING_FOR_SERVICE_PROVIDERS as i32,
                    name: "OPS.3.1 Outsourcing for Service Providers".to_owned()
                }
            }
            ModuleType::DER_1_DETECTING_SECURITY_RELEVANT_EVENTS => {
                EnumResponse {
                    code: ModuleType::DER_1_DETECTING_SECURITY_RELEVANT_EVENTS as i32,
                    name: "DER.1 Detecting Security-Relevant Events".to_owned()
                }
            }
            ModuleType::DER_2_1_SECURITY_INCIDENT_HANDLING => {
                EnumResponse {
                    code: ModuleType::DER_2_1_SECURITY_INCIDENT_HANDLING as i32,
                    name: "DER.2.1 Security Incident Handling".to_owned()
                }
            }
            ModuleType::DER_2_2_PROVISIONS_FOR_IT_FORENSICS => {
                EnumResponse {
                    code: ModuleType::DER_2_2_PROVISIONS_FOR_IT_FORENSICS as i32,
                    name: "DER.2.2 Provisions for IT Forensics".to_owned()
                }
            }
            ModuleType::DER_2_3_CLEAN_UP_OF_EXTENSIVE_SECURITY_INCIDENTS => {
                EnumResponse {
                    code: ModuleType::DER_2_3_CLEAN_UP_OF_EXTENSIVE_SECURITY_INCIDENTS as i32,
                    name: "DER.2.3 Clean-Up of Extensive Security Incidents".to_owned()
                }
            }
            ModuleType::DER_3_1_AUDITS_AND_REVISIONS => {
                EnumResponse {
                    code: ModuleType::DER_3_1_AUDITS_AND_REVISIONS as i32,
                    name: "DER.3.1 Audits and Revisions".to_owned()
                }
            }
            ModuleType::DER_3_2_AUDITS_BASED_ON_THE_BSI_GUIDELINE_FOR_IS_AUDITS => {
                EnumResponse {
                    code: ModuleType::DER_3_2_AUDITS_BASED_ON_THE_BSI_GUIDELINE_FOR_IS_AUDITS as i32,
                    name: "DER.3.2 Audits Based on the BSI \"Guideline for IS Audits\"".to_owned()
                }
            }
            ModuleType::DER_4_BUSINESS_CONTINUITY_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::DER_4_BUSINESS_CONTINUITY_MANAGEMENT as i32,
                    name: "DER.4 Business Continuity Management".to_owned()
                }
            }
            ModuleType::APP_1_1_OFFICE_PRODUCTS => {
                EnumResponse {
                    code: ModuleType::APP_1_1_OFFICE_PRODUCTS as i32,
                    name: "APP.1.1 Office Products".to_owned()
                }
            }
            ModuleType::APP_1_2_WEB_BROWSERS => {
                EnumResponse {
                    code: ModuleType::APP_1_2_WEB_BROWSERS as i32,
                    name: "APP.1.2 Web Browsers".to_owned()
                }
            }
            ModuleType::APP_1_4_MOBILE_APPLICATIONS => {
                EnumResponse {
                    code: ModuleType::APP_1_4_MOBILE_APPLICATIONS as i32,
                    name: "APP.1.4 Mobile Applications (Apps)".to_owned()
                }
            }
            ModuleType::APP_2_1_GENERAL_DIRECTORY_SERVICE => {
                EnumResponse {
                    code: ModuleType::APP_2_1_GENERAL_DIRECTORY_SERVICE as i32,
                    name: "APP.2.1 General Directory Service".to_owned()
                }
            }
            ModuleType::APP_2_2_ACTIVE_DIRECTORY => {
                EnumResponse {
                    code: ModuleType::APP_2_2_ACTIVE_DIRECTORY as i32,
                    name: "APP.2.2 Active Directory".to_owned()
                }
            }
            ModuleType::APP_2_3_OPENLDAP => {
                EnumResponse {
                    code: ModuleType::APP_2_3_OPENLDAP as i32,
                    name: "APP.2.3 OpenLDAP".to_owned()
                }
            }
            ModuleType::APP_3_1_WEB_APPLICATIONS_AND_WEB_SERVICES => {
                EnumResponse {
                    code: ModuleType::APP_3_1_WEB_APPLICATIONS_AND_WEB_SERVICES as i32,
                    name: "APP.3.1 Web Applications and Web Services".to_owned()
                }
            }
            ModuleType::APP_3_2_WEB_SERVERS => {
                EnumResponse {
                    code: ModuleType::APP_3_2_WEB_SERVERS as i32,
                    name: "APP.3.2 Web Servers".to_owned()
                }
            }
            ModuleType::APP_3_3_FILE_SERVERS => {
                EnumResponse {
                    code: ModuleType::APP_3_3_FILE_SERVERS as i32,
                    name: "APP.3.3 File Servers".to_owned()
                }
            }
            ModuleType::APP_3_4_SAMBA => {
                EnumResponse {
                    code: ModuleType::APP_3_4_SAMBA as i32,
                    name: "APP.3.4 Samba".to_owned()
                }
            }
            ModuleType::APP_3_6_DNS_SERVERS => {
                EnumResponse {
                    code: ModuleType::APP_3_6_DNS_SERVERS as i32,
                    name: "APP.3.6 DNS Servers".to_owned()
                }
            }
            ModuleType::APP_4_2_SAP_ERP_SYSTEMS => {
                EnumResponse {
                    code: ModuleType::APP_4_2_SAP_ERP_SYSTEMS as i32,
                    name: "APP.4.2 SAP ERP Systems".to_owned()
                }
            }
            ModuleType::APP_4_3_RELATIONAL_DATABASE_SYSTEMS => {
                EnumResponse {
                    code: ModuleType::APP_4_3_RELATIONAL_DATABASE_SYSTEMS as i32,
                    name: "APP.4.3 Relational Database Systems".to_owned()
                }
            }
            ModuleType::APP_4_4_KUBERNETES => {
                EnumResponse {
                    code: ModuleType::APP_4_4_KUBERNETES as i32,
                    name: "APP.4.4 Kubernetes".to_owned()
                }
            }
            ModuleType::APP_4_6_SAP_ABAP_PROGRAMMING => {
                EnumResponse {
                    code: ModuleType::APP_4_6_SAP_ABAP_PROGRAMMING as i32,
                    name: "APP.4.6 SAP ABAP Programming".to_owned()
                }
            }
            ModuleType::APP_5_2_MICROSOFT_EXCHANGE_AND_OUTLOOK => {
                EnumResponse {
                    code: ModuleType::APP_5_2_MICROSOFT_EXCHANGE_AND_OUTLOOK as i32,
                    name: "APP.5.2 Microsoft Exchange and Outlook".to_owned()
                }
            }
            ModuleType::APP_5_3_GENERAL_E_MAIL_CLIENTS_AND_SERVERS => {
                EnumResponse {
                    code: ModuleType::APP_5_3_GENERAL_E_MAIL_CLIENTS_AND_SERVERS as i32,
                    name: "APP.5.3 General E-Mail Clients and Servers".to_owned()
                }
            }
            ModuleType::APP_6_GENERAL_SOFTWARE => {
                EnumResponse {
                    code: ModuleType::APP_6_GENERAL_SOFTWARE as i32,
                    name: "APP.6 General Software".to_owned()
                }
            }
            ModuleType::APP_7_DEVELOPMENT_OF_INDIVIDUAL_SOFTWARE => {
                EnumResponse {
                    code: ModuleType::APP_7_DEVELOPMENT_OF_INDIVIDUAL_SOFTWARE as i32,
                    name: "APP.7 Development of Individual Software".to_owned()
                }
            }
            ModuleType::SYS_1_1_GENERAL_SERVER => {
                EnumResponse {
                    code: ModuleType::SYS_1_1_GENERAL_SERVER as i32,
                    name: "SYS.1.1 General Server".to_owned()
                }
            }
            ModuleType::SYS_1_2_2_WINDOWS_SERVER_2012 => {
                EnumResponse {
                    code: ModuleType::SYS_1_2_2_WINDOWS_SERVER_2012 as i32,
                    name: "SYS.1.2.2 Windows Server 2012".to_owned()
                }
            }
            ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS => {
                EnumResponse {
                    code: ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS as i32,
                    name: "SYS.1.3 Linux and Unix Servers".to_owned()
                }
            }
            ModuleType::SYS_1_5_VIRTUALISATION => {
                EnumResponse {
                    code: ModuleType::SYS_1_5_VIRTUALISATION as i32,
                    name: "SYS.1.5 Virtualisation".to_owned()
                }
            }
            ModuleType::SYS_1_6_CONTAINERISATION => {
                EnumResponse {
                    code: ModuleType::SYS_1_6_CONTAINERISATION as i32,
                    name: "SYS.1.6 Containerisation".to_owned()
                }
            }
            ModuleType::SYS_1_7_IBM_Z => {
                EnumResponse {
                    code: ModuleType::SYS_1_7_IBM_Z as i32,
                    name: "SYS.1.7 IBM Z".to_owned()
                }
            }
            ModuleType::SYS_1_8_STORAGE_SOLUTIONS => {
                EnumResponse {
                    code: ModuleType::SYS_1_8_STORAGE_SOLUTIONS as i32,
                    name: "SYS.1.8 Storage Solutions".to_owned()
                }
            }
            ModuleType::SYS_2_1_GENERAL_CLIENT => {
                EnumResponse {
                    code: ModuleType::SYS_2_1_GENERAL_CLIENT as i32,
                    name: "SYS.2.1 General Client".to_owned()
                }
            }
            ModuleType::SYS_2_2_2_WINDOWS_8_1_CLIENTS => {
                EnumResponse {
                    code: ModuleType::SYS_2_2_2_WINDOWS_8_1_CLIENTS as i32,
                    name: "SYS.2.2.2 Windows 8.1 Clients".to_owned()
                }
            }
            ModuleType::SYS_2_2_3_WINDOWS_10_CLIENTS => {
                EnumResponse {
                    code: ModuleType::SYS_2_2_3_WINDOWS_10_CLIENTS as i32,
                    name: "SYS.2.2.3 Windows 10 Clients".to_owned()
                }
            }
            ModuleType::SYS_2_3_LINUX_AND_UNIX_CLIENTS => {
                EnumResponse {
                    code: ModuleType::SYS_2_3_LINUX_AND_UNIX_CLIENTS as i32,
                    name: "SYS.2.3 Linux and Unix Clients".to_owned()
                }
            }
            ModuleType::SYS_2_4_MACOS_CLIENTS => {
                EnumResponse {
                    code: ModuleType::SYS_2_4_MACOS_CLIENTS as i32,
                    name: "SYS.2.4 macOS Clients".to_owned()
                }
            }
            ModuleType::SYS_3_1_LAPTOPS => {
                EnumResponse {
                    code: ModuleType::SYS_3_1_LAPTOPS as i32,
                    name: "SYS.3.1 Laptops".to_owned()
                }
            }
            ModuleType::SYS_3_2_1_GENERAL_SMARTPHONES_AND_TABLETS => {
                EnumResponse {
                    code: ModuleType::SYS_3_2_1_GENERAL_SMARTPHONES_AND_TABLETS as i32,
                    name: "SYS.3.2.1 General Smartphones and Tablets".to_owned()
                }
            }
            ModuleType::SYS_3_2_2_MOBILE_DEVICE_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::SYS_3_2_2_MOBILE_DEVICE_MANAGEMENT as i32,
                    name: "SYS.3.2.2 Mobile Device Management (MDM)".to_owned()
                }
            }
            ModuleType::SYS_3_2_3_IOS_FOR_ENTERPRISE => {
                EnumResponse {
                    code: ModuleType::SYS_3_2_3_IOS_FOR_ENTERPRISE as i32,
                    name: "SYS.3.2.3 iOS (for Enterprise)".to_owned()
                }
            }
            ModuleType::SYS_3_2_4_ANDROID => {
                EnumResponse {
                    code: ModuleType::SYS_3_2_4_ANDROID as i32,
                    name: "SYS.3.2.4 Android".to_owned()
                }
            }
            ModuleType::SYS_3_3_MOBILE_TELEPHONES => {
                EnumResponse {
                    code: ModuleType::SYS_3_3_MOBILE_TELEPHONES as i32,
                    name: "SYS.3.3 Mobile Telephones".to_owned()
                }
            }
            ModuleType::SYS_4_1_PRINTERS_COPIERS_AND_ALL_IN_ONE_DEVICES => {
                EnumResponse {
                    code: ModuleType::SYS_4_1_PRINTERS_COPIERS_AND_ALL_IN_ONE_DEVICES as i32,
                    name: "SYS.4.1 Printers, Copiers, and All-in-One Devices".to_owned()
                }
            }
            ModuleType::SYS_4_3_EMBEDDED_SYSTEMS => {
                EnumResponse {
                    code: ModuleType::SYS_4_3_EMBEDDED_SYSTEMS as i32,
                    name: "SYS.4.3 Embedded Systems".to_owned()
                }
            }
            ModuleType::SYS_4_4_GENERAL_IOT_DEVICES => {
                EnumResponse {
                    code: ModuleType::SYS_4_4_GENERAL_IOT_DEVICES as i32,
                    name: "SYS.4.4 General IoT Devices".to_owned()
                }
            }
            ModuleType::SYS_4_5_REMOVABLE_MEDIA => {
                EnumResponse {
                    code: ModuleType::SYS_4_5_REMOVABLE_MEDIA as i32,
                    name: "SYS.4.5 Removable Media".to_owned()
                }
            }
            ModuleType::IND_1_PROCESS_CONTROL_AND_AUTOMATION_TECHNOLOGY => {
                EnumResponse {
                    code: ModuleType::IND_1_PROCESS_CONTROL_AND_AUTOMATION_TECHNOLOGY as i32,
                    name: "IND.1 Process Control and Automation Technology".to_owned()
                }
            }
            ModuleType::IND_2_1_GENERAL_ICS_COMPONENTS => {
                EnumResponse {
                    code: ModuleType::IND_2_1_GENERAL_ICS_COMPONENTS as i32,
                    name: "IND.2.1 General ICS Components".to_owned()
                }
            }
            ModuleType::IND_2_2_PROGRAMMABLE_LOGIC_CONTROLLER => {
                EnumResponse {
                    code: ModuleType::IND_2_2_PROGRAMMABLE_LOGIC_CONTROLLER as i32,
                    name: "IND.2.2 Programmable Logic Controller (PLC)".to_owned()
                }
            }
            ModuleType::IND_2_3_SENSORS_AND_ACTUATORS => {
                EnumResponse {
                    code: ModuleType::IND_2_3_SENSORS_AND_ACTUATORS as i32,
                    name: "IND.2.3 Sensors and Actuators".to_owned()
                }
            }
            ModuleType::IND_2_4_MACHINE => {
                EnumResponse {
                    code: ModuleType::IND_2_4_MACHINE as i32,
                    name: "IND.2.4 Machine".to_owned()
                }
            }
            ModuleType::IND_2_7_SAFETY_INSTRUMENTED_SYSTEMS => {
                EnumResponse {
                    code: ModuleType::IND_2_7_SAFETY_INSTRUMENTED_SYSTEMS as i32,
                    name: "IND.2.7 Safety Instrumented Systems".to_owned()
                }
            }
            ModuleType::IND_3_2_REMOTE_MAINTENANCE_IN_INDUSTRY => {
                EnumResponse {
                    code: ModuleType::IND_3_2_REMOTE_MAINTENANCE_IN_INDUSTRY as i32,
                    name: "IND.3.2 Remote Maintenance in Industry".to_owned()
                }
            }
            ModuleType::NET_1_1_NETWORK_ARCHITECTURE_AND_DESIGN => {
                EnumResponse {
                    code: ModuleType::NET_1_1_NETWORK_ARCHITECTURE_AND_DESIGN as i32,
                    name: "NET.1.1 Network Architecture and Design".to_owned()
                }
            }
            ModuleType::NET_1_2_NETWORK_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::NET_1_2_NETWORK_MANAGEMENT as i32,
                    name: "NET.1.2 Network Management".to_owned()
                }
            }
            ModuleType::NET_2_1_WLAN_OPERATION => {
                EnumResponse {
                    code: ModuleType::NET_2_1_WLAN_OPERATION as i32,
                    name: "NET.2.1 WLAN Operation".to_owned()
                }
            }
            ModuleType::NET_2_2_WLAN_USAGE => {
                EnumResponse {
                    code: ModuleType::NET_2_2_WLAN_USAGE as i32,
                    name: "NET.2.2 WLAN Usage".to_owned()
                }
            }
            ModuleType::NET_3_1_ROUTERS_AND_SWITCHES => {
                EnumResponse {
                    code: ModuleType::NET_3_1_ROUTERS_AND_SWITCHES as i32,
                    name: "NET.3.1 Routers and Switches".to_owned()
                }
            }
            ModuleType::NET_3_2_FIREWALL => {
                EnumResponse {
                    code: ModuleType::NET_3_2_FIREWALL as i32,
                    name: "NET.3.2 Firewall".to_owned()
                }
            }
            ModuleType::NET_3_3_VPN => {
                EnumResponse {
                    code: ModuleType::NET_3_3_VPN as i32,
                    name: "NET.3.3 VPN".to_owned()
                }
            }
            ModuleType::NET_4_1_TELECOMMUNICATIONS_SYSTEMS => {
                EnumResponse {
                    code: ModuleType::NET_4_1_TELECOMMUNICATIONS_SYSTEMS as i32,
                    name: "NET.4.1 Telecommunications Systems".to_owned()
                }
            }
            ModuleType::NET_4_2_VOIP => {
                EnumResponse {
                    code: ModuleType::NET_4_2_VOIP as i32,
                    name: "NET.4.2 VoIP".to_owned()
                }
            }
            ModuleType::NET_4_3_FAX_MACHINES_AND_FAX_SERVERS => {
                EnumResponse {
                    code: ModuleType::NET_4_3_FAX_MACHINES_AND_FAX_SERVERS as i32,
                    name: "NET.4.3 Fax Machines and Fax Servers".to_owned()
                }
            }
            ModuleType::INF_1_GENERIC_BUILDING => {
                EnumResponse {
                    code: ModuleType::INF_1_GENERIC_BUILDING as i32,
                    name: "INF.1 Generic Building".to_owned()
                }
            }
            ModuleType::INF_2_DATA_CENTRE_AND_SERVER_ROOM => {
                EnumResponse {
                    code: ModuleType::INF_2_DATA_CENTRE_AND_SERVER_ROOM as i32,
                    name: "INF.2 Data Centre and Server Room".to_owned()
                }
            }
            ModuleType::INF_5_ROOM_OR_CABINET_FOR_TECHNICAL_INFRASTRUCTURE => {
                EnumResponse {
                    code: ModuleType::INF_5_ROOM_OR_CABINET_FOR_TECHNICAL_INFRASTRUCTURE as i32,
                    name: "INF.5 Room or Cabinet for Technical Infrastructure".to_owned()
                }
            }
            ModuleType::INF_6_STORAGE_MEDIA_ARCHIVES => {
                EnumResponse {
                    code: ModuleType::INF_6_STORAGE_MEDIA_ARCHIVES as i32,
                    name: "INF.6 Storage Media Archives".to_owned()
                }
            }
            ModuleType::INF_7_OFFICE_WORKPLACE => {
                EnumResponse {
                    code: ModuleType::INF_7_OFFICE_WORKPLACE as i32,
                    name: "INF.7 Office Workplace".to_owned()
                }
            }
            ModuleType::INF_8_WORKING_FROM_HOME => {
                EnumResponse {
                    code: ModuleType::INF_8_WORKING_FROM_HOME as i32,
                    name: "INF.8 Working from Home".to_owned()
                }
            }
            ModuleType::INF_9_MOBILE_WORKPLACE => {
                EnumResponse {
                    code: ModuleType::INF_9_MOBILE_WORKPLACE as i32,
                    name: "INF.9 Mobile Workplace".to_owned()
                }
            }
            ModuleType::INF_10_MEETING_EVENT_AND_TRAINING_ROOMS => {
                EnumResponse {
                    code: ModuleType::INF_10_MEETING_EVENT_AND_TRAINING_ROOMS as i32,
                    name: "INF.10 Meeting, Event, and Training Rooms".to_owned()
                }
            }
            ModuleType::INF_11_GENERAL_VEHICLE => {
                EnumResponse {
                    code: ModuleType::INF_11_GENERAL_VEHICLE as i32,
                    name: "INF.11 General Vehicle".to_owned()
                }
            }
            ModuleType::INF_12_CABLING => {
                EnumResponse {
                    code: ModuleType::INF_12_CABLING as i32,
                    name: "INF.12 Cabling".to_owned()
                }
            }
            ModuleType::INF_13_TECHNICAL_BUILDING_MANAGEMENT => {
                EnumResponse {
                    code: ModuleType::INF_13_TECHNICAL_BUILDING_MANAGEMENT as i32,
                    name: "INF.13 Technical Building Management (TBM)".to_owned()
                }
            }
            ModuleType::INF_14_BUILDING_AUTOMATION_AND_CONTROL_SYSTEMS => {
                EnumResponse {
                    code: ModuleType::INF_14_BUILDING_AUTOMATION_AND_CONTROL_SYSTEMS as i32,
                    name: "INF.14 Building Automation and Control Systems (BACS)".to_owned()
                }
            }
            ModuleType::UNKNOWN => {
                EnumResponse {
                    code: ModuleType::UNKNOWN as i32,
                    name: "neznámy".to_owned()
                }
            }
        }
    }
}

impl From<i32> for ModuleType {
    fn from(value: i32) -> Self {
        match value {
            0 => ModuleType::ISMS_1_SECURITY_MANAGEMENT,
            1 => ModuleType::ORP_1_ORGANISATION,
            2 => ModuleType::ORP_2_PERSONNEL,
            3 => ModuleType::ORP_3_AWARENESS_AND_TRAINING_IN_INFORMATION_SECURITY,
            4 => ModuleType::ORP_4_IDENTITY_AND_ACCESS_MANAGEMENT,
            5 => ModuleType::ORP_5_COMPLIANCE_MANAGEMENT,
            6 => ModuleType::CON_1_CRYPTO_CONCEPT,
            7 => ModuleType::CON_2_DATA_PROTECTION,
            8 => ModuleType::CON_3_BACKUP_CONCEPT,
            9 => ModuleType::CON_6_DELETING_AND_DESTROYING_DATA_AND_DEVICES,
            10 => ModuleType::CON_7_INFORMATION_SECURITY_ON_TRIPS_ABROAD,
            11 => ModuleType::CON_8_SOFTWARE_DEVELOPMENT,
            12 => ModuleType::CON_9_INFORMATION_EXCHANGE,
            13 => ModuleType::CON_10_DEVELOPMENT_OF_WEB_APPLICATIONS,
            14 => ModuleType::OPS_1_1_2_PROPER_IT_ADMINISTRATION,
            15 => ModuleType::OPS_1_1_3_PATCH_AND_CHANGE_MANAGEMENT,
            16 => ModuleType::OPS_1_1_4_PROTECTION_AGAINST_MALWARE,
            17 => ModuleType::OPS_1_1_5_LOGGING,
            18 => ModuleType::OPS_1_1_6_SOFTWARE_TESTS_AND_APPROVALS,
            19 => ModuleType::OPS_1_1_7_SYSTEM_MANAGEMENT,
            20 => ModuleType::OPS_1_2_2_ARCHIVING,
            21 => ModuleType::OPS_1_2_4_TELEWORKING,
            22 => ModuleType::OPS_1_2_5_REMOTE_MAINTENANCE,
            23 => ModuleType::OPS_1_2_6_NTP_TIME_SYNCHRONISATION,
            24 => ModuleType::OPS_2_1_OUTSOURCING_FOR_CUSTOMERS,
            25 => ModuleType::OPS_2_2_CLOUD_USAGE,
            26 => ModuleType::OPS_3_1_OUTSOURCING_FOR_SERVICE_PROVIDERS,
            27 => ModuleType::DER_1_DETECTING_SECURITY_RELEVANT_EVENTS,
            28 => ModuleType::DER_2_1_SECURITY_INCIDENT_HANDLING,
            29 => ModuleType::DER_2_2_PROVISIONS_FOR_IT_FORENSICS,
            30 => ModuleType::DER_2_3_CLEAN_UP_OF_EXTENSIVE_SECURITY_INCIDENTS,
            31 => ModuleType::DER_3_1_AUDITS_AND_REVISIONS,
            32 => ModuleType::DER_3_2_AUDITS_BASED_ON_THE_BSI_GUIDELINE_FOR_IS_AUDITS,
            33 => ModuleType::DER_4_BUSINESS_CONTINUITY_MANAGEMENT,
            34 => ModuleType::APP_1_1_OFFICE_PRODUCTS,
            35 => ModuleType::APP_1_2_WEB_BROWSERS,
            36 => ModuleType::APP_1_4_MOBILE_APPLICATIONS,
            37 => ModuleType::APP_2_1_GENERAL_DIRECTORY_SERVICE,
            38 => ModuleType::APP_2_2_ACTIVE_DIRECTORY,
            39 => ModuleType::APP_2_3_OPENLDAP,
            40 => ModuleType::APP_3_1_WEB_APPLICATIONS_AND_WEB_SERVICES,
            41 => ModuleType::APP_3_2_WEB_SERVERS,
            42 => ModuleType::APP_3_3_FILE_SERVERS,
            43 => ModuleType::APP_3_4_SAMBA,
            44 => ModuleType::APP_3_6_DNS_SERVERS,
            45 => ModuleType::APP_4_2_SAP_ERP_SYSTEMS,
            46 => ModuleType::APP_4_3_RELATIONAL_DATABASE_SYSTEMS,
            47 => ModuleType::APP_4_4_KUBERNETES,
            48 => ModuleType::APP_4_6_SAP_ABAP_PROGRAMMING,
            49 => ModuleType::APP_5_2_MICROSOFT_EXCHANGE_AND_OUTLOOK,
            50 => ModuleType::APP_5_3_GENERAL_E_MAIL_CLIENTS_AND_SERVERS,
            51 => ModuleType::APP_6_GENERAL_SOFTWARE,
            52 => ModuleType::APP_7_DEVELOPMENT_OF_INDIVIDUAL_SOFTWARE,
            53 => ModuleType::SYS_1_1_GENERAL_SERVER,
            54 => ModuleType::SYS_1_2_2_WINDOWS_SERVER_2012,
            55 => ModuleType::SYS_1_3_LINUX_AND_UNIX_SERVERS,
            56 => ModuleType::SYS_1_5_VIRTUALISATION,
            57 => ModuleType::SYS_1_6_CONTAINERISATION,
            58 => ModuleType::SYS_1_7_IBM_Z,
            59 => ModuleType::SYS_1_8_STORAGE_SOLUTIONS,
            60 => ModuleType::SYS_2_1_GENERAL_CLIENT,
            61 => ModuleType::SYS_2_2_2_WINDOWS_8_1_CLIENTS,
            62 => ModuleType::SYS_2_2_3_WINDOWS_10_CLIENTS,
            63 => ModuleType::SYS_2_3_LINUX_AND_UNIX_CLIENTS,
            64 => ModuleType::SYS_2_4_MACOS_CLIENTS,
            65 => ModuleType::SYS_3_1_LAPTOPS,
            66 => ModuleType::SYS_3_2_1_GENERAL_SMARTPHONES_AND_TABLETS,
            67 => ModuleType::SYS_3_2_2_MOBILE_DEVICE_MANAGEMENT,
            68 => ModuleType::SYS_3_2_3_IOS_FOR_ENTERPRISE,
            69 => ModuleType::SYS_3_2_4_ANDROID,
            70 => ModuleType::SYS_3_3_MOBILE_TELEPHONES,
            71 => ModuleType::SYS_4_1_PRINTERS_COPIERS_AND_ALL_IN_ONE_DEVICES,
            72 => ModuleType::SYS_4_3_EMBEDDED_SYSTEMS,
            73 => ModuleType::SYS_4_4_GENERAL_IOT_DEVICES,
            74 => ModuleType::SYS_4_5_REMOVABLE_MEDIA,
            75 => ModuleType::IND_1_PROCESS_CONTROL_AND_AUTOMATION_TECHNOLOGY,
            76 => ModuleType::IND_2_1_GENERAL_ICS_COMPONENTS,
            77 => ModuleType::IND_2_2_PROGRAMMABLE_LOGIC_CONTROLLER,
            78 => ModuleType::IND_2_3_SENSORS_AND_ACTUATORS,
            79 => ModuleType::IND_2_4_MACHINE,
            80 => ModuleType::IND_2_7_SAFETY_INSTRUMENTED_SYSTEMS,
            81 => ModuleType::IND_3_2_REMOTE_MAINTENANCE_IN_INDUSTRY,
            82 => ModuleType::NET_1_1_NETWORK_ARCHITECTURE_AND_DESIGN,
            83 => ModuleType::NET_1_2_NETWORK_MANAGEMENT,
            84 => ModuleType::NET_2_1_WLAN_OPERATION,
            85 => ModuleType::NET_2_2_WLAN_USAGE,
            86 => ModuleType::NET_3_1_ROUTERS_AND_SWITCHES,
            87 => ModuleType::NET_3_2_FIREWALL,
            88 => ModuleType::NET_3_3_VPN,
            89 => ModuleType::NET_4_1_TELECOMMUNICATIONS_SYSTEMS,
            90 => ModuleType::NET_4_2_VOIP,
            91 => ModuleType::NET_4_3_FAX_MACHINES_AND_FAX_SERVERS,
            92 => ModuleType::INF_1_GENERIC_BUILDING,
            93 => ModuleType::INF_2_DATA_CENTRE_AND_SERVER_ROOM,
            94 => ModuleType::INF_5_ROOM_OR_CABINET_FOR_TECHNICAL_INFRASTRUCTURE,
            95 => ModuleType::INF_6_STORAGE_MEDIA_ARCHIVES,
            96 => ModuleType::INF_7_OFFICE_WORKPLACE,
            97 => ModuleType::INF_8_WORKING_FROM_HOME,
            98 => ModuleType::INF_9_MOBILE_WORKPLACE,
            99 => ModuleType::INF_10_MEETING_EVENT_AND_TRAINING_ROOMS,
            100 => ModuleType::INF_11_GENERAL_VEHICLE,
            101 => ModuleType::INF_12_CABLING,
            102 => ModuleType::INF_13_TECHNICAL_BUILDING_MANAGEMENT,
            103 => ModuleType::INF_14_BUILDING_AUTOMATION_AND_CONTROL_SYSTEMS,
            _ => ModuleType::UNKNOWN,
        }
    }
}