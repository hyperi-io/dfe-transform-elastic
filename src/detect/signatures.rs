// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED

//! The marker table: which text on a line means which source.
//!
//! One flat table. A marker is a literal, a pair of literals that must both
//! appear (a key=value pair or a JSON key set), or a CEF vendor and product.
//! The highest priority wins and the first entry wins a tie, so a broad
//! marker such as the IOS `%FAC-n-MNEM:` catch-all sits low and a specific one
//! sits high.
//!
//! Identical bytes go to the broader source: ASA over FTD, ADC over WAF, IOS
//! over the platforms that share its layout. A source that must win its own
//! lines carries a marker the broader one does not (`%FTD-`, a CEF header).
//! Excluding the broader source from `candidates` hands those lines to the
//! narrower one.

use super::verify;

/// Checks the bytes around a literal hit, for the few markers no literal pins.
pub(super) type Verify = fn(&[u8], usize) -> bool;

/// One marker and the source it names.
pub(super) struct Signature {
    /// Registry name of the transform to run.
    pub source: &'static str,
    /// Higher wins. The first entry wins a tie.
    pub priority: u8,
    /// Literals searched for in the line.
    pub markers: &'static [&'static str],
    /// Every marker must appear, rather than any one.
    pub all: bool,
    /// Run at the start of each hit, when `all` is false.
    pub verify: Option<Verify>,
}

/// A CEF or LEEF header: vendor and product name the source.
pub(super) struct CefMarker {
    /// Registry name of the transform to run.
    pub source: &'static str,
    /// Higher wins.
    pub priority: u8,
    /// Header vendor, compared without regard to ASCII case.
    pub vendor: &'static str,
    /// Header product, compared without regard to ASCII case.
    pub product: &'static str,
}

const fn one_of(source: &'static str, priority: u8, markers: &'static [&'static str]) -> Signature {
    Signature {
        source,
        priority,
        markers,
        all: false,
        verify: None,
    }
}

const fn every(source: &'static str, priority: u8, markers: &'static [&'static str]) -> Signature {
    Signature {
        source,
        priority,
        markers,
        all: true,
        verify: None,
    }
}

const fn checked(
    source: &'static str,
    priority: u8,
    markers: &'static [&'static str],
    verify: Verify,
) -> Signature {
    Signature {
        source,
        priority,
        markers,
        all: false,
        verify: Some(verify),
    }
}

const fn cef(
    source: &'static str,
    priority: u8,
    vendor: &'static str,
    product: &'static str,
) -> CefMarker {
    CefMarker {
        source,
        priority,
        vendor,
        product,
    }
}

/// The most `every` entries one detector may hold, which sizes its per-line
/// scratch. A test holds the table to it.
pub(super) const MAX_EVERY: usize = 32;

/// Every literal-marker entry, in tie-break order.
pub(super) static SIGNATURES: &[Signature] = &[
    // Untangle's uvm logger tag followed by a JSON object
    one_of(
        "filebeat.arista_ngfw_log.default",
        60,
        &["uvm[0]:  {", "uvm[0]: {"],
    ),
    // unit name then a log-type token (TR, NF, SYS, WF, AUDIT)
    one_of(
        "filebeat.barracuda_waf.default",
        50,
        &[" TR ", " NF ", " SYS ", " WF ", " AUDIT "],
    ),
    // Director's <<name.severity> marker
    one_of(
        "filebeat.bluecoat_director.default",
        40,
        &[".very-high> ", ".high> ", ".medium> ", ".low> "],
    ),
    // cb-event-forwarder keys
    one_of(
        "filebeat.carbonblack_edr_log.default",
        50,
        &["\"cb_server\":\"", "\"cb_version\":\"", "{\"docs\":[{"],
    ),
    // event-forwarder JSON opens with a 10-digit epoch and a fraction
    checked(
        "filebeat.carbonblack_edr_log.default",
        50,
        &["{\"event_timestamp\":"],
        verify::cb_timestamp,
    ),
    // any CEF header no vendor entry claims
    one_of("filebeat.cef_log.default", 5, &["CEF:0|", "CEF:1|"]),
    // Log Exporter app-name, or its SD id under Check Point's enterprise number
    one_of(
        "filebeat.checkpoint.default",
        55,
        &[" CheckPoint ", "Fields@1.3.6.1.4.1.2620", "[flags:\""],
    ),
    // AireOS `*Task:` prefix beside a %FACILITY
    every("filebeat.cisco_aironet_log.default", 52, &[": *", ": %"]),
    // 9800 chassis stamp and WLC-only facilities
    one_of(
        "filebeat.cisco_aironet_log.default",
        52,
        &[
            "Chassis 1 ",
            "Chassis 2 ",
            "%APMGR",
            "%CAPWAPAC",
            "%CLIENT_",
            "%RADIUS_AUDIT",
            "%MM_LOG",
            "%MM_INFRA_LOG",
            "%WPS-",
            "%LWAPP-",
            "%RRM-",
            "%RRM_",
            "%DTL-",
            "%APF-",
            "%LOADBALANCE",
            "%CW_LIC",
            "%SESSION_MGR",
            "%WIDS",
            "%DOT11-",
        ],
    ),
    // six-digit %ASA ids; byte-identical FTD lines stay here by design
    one_of("filebeat.cisco_asa.default", 50, &["%ASA-"]),
    // %FTD/%NGIPS ids, ASA ids 4300xx and the FMC process names
    one_of(
        "filebeat.cisco_ftd.default",
        55,
        &[
            "%FTD-",
            "%NGIPS-",
            "%ASA-4-4300",
            "%ASA-6-4300",
            "%ASA-3-4300",
            "%ASA-1-4300",
            "%ASA-2-4300",
            "%ASA-5-4300",
            "%ASA-7-4300",
            "ftd[",
            "mojo_server.pl: ",
            "sfdccsm: ",
            "ActionQueueScrape.pl: ",
            "platformSettingEdit.cgi: ",
            "ChangeReconciliation.cgi: ",
            "lights_out_mgmt.cgi: ",
        ],
    ),
    // %FAC-n-MNEM: catch-all, below every other %-style source
    checked(
        "filebeat.cisco_ios.default",
        10,
        &["%"],
        verify::ios_mnemonic,
    ),
    // CISE_<category> token
    one_of("filebeat.cisco_ise_log.default", 60, &["CISE_"]),
    // Meraki event types after the epoch.nanos header
    one_of(
        "filebeat.cisco_meraki.default",
        50,
        &[
            " airmarshal_events ",
            " events type=",
            " flows allow src=",
            " flows deny src=",
            " ip_flow_start src=",
            " ip_flow_end src=",
            " urls src=",
            " ids-alerts ",
            " security_event ",
            " firewall src=",
            "_firewall src=",
            " flows src=",
            " events Site-to-Site VPN:",
            " events dhcp ",
            " events client_vpn_connect ",
            " events carrier_change ",
            " events Blocked ARP Packet ",
        ],
    ),
    // NX-OS-only facilities
    one_of(
        "filebeat.cisco_nexus.default",
        53,
        &[
            "%ETHPORT-",
            "%L2FM-",
            "%VPC-",
            "%SYSMGR-",
            "%VSHD-",
            "%HMM-",
            "SYSTEM_MSG:",
            "AAA_ACCOUNTING_MESSAGE",
        ],
    ),
    // year-first stamp, which IOS does not write
    checked(
        "filebeat.cisco_nexus.default",
        53,
        &[
            " Jan ", " Feb ", " Mar ", " Apr ", " May ", " Jun ", " Jul ", " Aug ", " Sep ",
            " Oct ", " Nov ", " Dec ",
        ],
        verify::nexus_stamp,
    ),
    // AsyncOS log name as the syslog tag, level word after
    one_of(
        "filebeat.cisco_secure_email_gateway_log.default",
        55,
        &[
            "|ESA_CONSOLIDATED_LOG_EVENT|",
            " amp: ",
            " antispam: ",
            " antivirus: ",
            " consolidated_event: ",
            " content_scanner: ",
            " error_logs: ",
            " gui_logs: ",
            " mail_logs: ",
            " status: Info",
            " status: Warning",
            " status: Critical",
            " status: Error",
            " system: Info",
            " system: Warning",
            " system: Critical",
            " system: Error",
        ],
    ),
    // NetScaler packet-engine tag, also owns identical Citrix CEF
    one_of("filebeat.citrix_adc_log.default", 50, &["-PPE-"]),
    // Vault audit_record XML or JSON
    one_of(
        "filebeat.cyberarkpas_audit.default",
        60,
        &["audit_record\"", "audit_record>"],
    ),
    // CylancePROTECT product token
    one_of("filebeat.cylance_protect.default", 55, &["CylancePROTECT "]),
    // AI Analyst JSON keys
    one_of(
        "filebeat.darktrace_ai_analyst_alert.default",
        45,
        &[
            "{\"summariser\":\"",
            "\"attackPhases\":[",
            "\"groupCategory\":\"",
        ],
    ),
    // model-breach JSON keys
    one_of(
        "filebeat.darktrace_model_breach_alert.default",
        44,
        &[
            "{\"model\":{\"name\":\"",
            "\"breachUrl\":\"",
            "\"triggeredComponents\":[",
            "{\"commentCount\":",
        ],
    ),
    // system-status JSON keys
    one_of(
        "filebeat.darktrace_system_status_alert.default",
        45,
        &["\"priority_level\":\"", "\"child_id\":"],
    ),
    // ESET's misspelt `occured` key
    one_of(
        "filebeat.eset_protect_event.default",
        50,
        &["\"occured\":\"", "\"source_uuid\":\""],
    ),
    // Falco output keys
    every(
        "filebeat.falco_alerts.default",
        40,
        &["\",\"priority\":\"", "\"rule\":\""],
    ),
    // rawmsg envelope
    one_of("filebeat.fireeye_nx.default", 60, &["{\"rawmsg\":\""]),
    // appliance sudo line
    one_of(
        "filebeat.forescout_event.default",
        50,
        &["_fsservice : TTY="],
    ),
    // FortiOS logid beside type; FortiProxy and FortiManager outrank it
    every("filebeat.fortinet.default", 30, &["logid=", " type="]),
    // FPX serial prefix
    one_of(
        "filebeat.fortinet_fortiproxy_log.default",
        60,
        &["devid=\"FPX"],
    ),
    // FMG serial or forwarded-log prefix
    one_of(
        "filebeat.fortinet_fortimanager_log.default",
        50,
        &[" device_id=FMG", "logver=", "devid=\"FMG"],
    ),
    // quoted log_id beside type=
    every(
        "filebeat.fortinet_fortimanager_log.default",
        50,
        &[" log_id=\"", " type=\""],
    ),
    // comma-separated device_id=F.. and log_id=
    every(
        "filebeat.fortinet_fortimail_log.default",
        55,
        &[",device_id=F", ",log_id="],
    ),
    // traffic log key
    one_of(
        "filebeat.fortinet_forticlient_log.default",
        50,
        &["traff_direct="],
    ),
    // FortiEDR organisation fields
    one_of(
        "filebeat.fortinet_fortiedr_log.default",
        50,
        &["Raw Data ID:", "Organization ID:"],
    ),
    // vendor key
    one_of(
        "filebeat.gigamon_ami.default",
        50,
        &["\"vendor\":\"Gigamon\"", "\"vendor\": \"Gigamon\""],
    ),
    // audit line opens with time then request or response
    every(
        "filebeat.hashicorp_vault_audit.default",
        50,
        &["{\"time\":\"", "\",\"type\":\"re"],
    ),
    // Event|System, or Event|id|LOG_level|
    one_of("filebeat.hpe_aruba_cx_log.default", 50, &["Event|System "]),
    // Event| beside |LOG_
    every(
        "filebeat.hpe_aruba_cx_log.default",
        50,
        &["Event|", "|LOG_"],
    ),
    // audit log: Z [user]: action
    every("filebeat.infoblox_nios_log.default", 50, &["Z [", "]: "]),
    // DNS and DHCP extensions
    one_of(
        "filebeat.infoblox_nios_log.default",
        50,
        &[
            "UDP: query: ",
            "TCP: query: ",
            "lease-duration ",
            " TransID ",
            " apparently_via=",
        ],
    ),
    // netfilter LOG fields
    every(
        "filebeat.iptables_log.default",
        50,
        &["IN=", " OUT=", " SRC="],
    ),
    // _event_score key
    one_of(
        "filebeat.jamf_compliance_reporter_log.default",
        50,
        &["{\"_event_score\":"],
    ),
    // Junos message-id families
    one_of(
        "filebeat.juniper_junos_log.default",
        40,
        &[
            " UI_",
            " LOGIN_FAILED:",
            " LOGIN_PAM_",
            " LOGIN_INFORMATION",
            " SSHD_",
            " SNMP_TRAP_",
            " RPD_",
            " CHASSISD_",
            " SNMPD_",
            " NASD_",
            " PWC_",
            " SERVICED_",
            " FSAD_",
            " BOOTPD_",
            " TFTPD_",
            " RMOPD_",
            " MIB2D_",
            " ECCD_",
            " DCD_",
            " LIBJNX_",
            " LIBSERVICED_",
            " DFWD_",
        ],
    ),
    // ScreenOS device_id
    one_of(
        "filebeat.juniper_netscreen_log.default",
        50,
        &["NetScreen device_id="],
    ),
    // SRX app-names and the junos@ SD id
    one_of(
        "filebeat.juniper_srx_log.default",
        50,
        &[
            "junos@",
            "RT_FLOW",
            "RT_IDP",
            "RT_UTM",
            "RT_SCREEN",
            "RT_IDS",
            "RT_AAMW",
            "RT_ATP",
            "APPTRACK_SESSION",
            "PFE_FW_SYSLOG",
            "IKE negotiation ",
            "IPSec negotiation ",
        ],
    ),
    // pfsp tag and fixed message texts
    one_of(
        "filebeat.netscout_sightline.default",
        50,
        &[
            " pfsp:",
            "Change Log: Username:",
            "Changed protection mode to active for protection group",
            "Blocked Host: Blocked host",
            "anomaly: anomaly ",
            "instability router",
            "Test: Test syslog message",
        ],
    ),
    // templated alerts object
    one_of(
        "filebeat.netskope_alerts.default",
        50,
        &["\"netskope\":{\"alerts\":", "\"netskope\": {\"alerts\":"],
    ),
    // templated events object
    one_of(
        "filebeat.netskope_events.default",
        50,
        &["\"netskope\":{\"events\":", "\"netskope\": {\"events\":"],
    ),
    // PAN-OS log type, confirmed by the date field that follows
    checked(
        "filebeat.panw.default",
        50,
        &[
            ",TRAFFIC,",
            ",THREAT,",
            ",SYSTEM,",
            ",CONFIG,",
            ",GLOBALPROTECT,",
            ",HIPMATCH,",
            ",HIP-MATCH,",
            ",USERID,",
            ",DECRYPTION,",
            ",AUTHENTICATION,",
            ",AUTH,",
            ",CORRELATION,",
            ",GTP,",
            ",IPTAG,",
            ",SCTP,",
            ",START,",
            ",END,",
            ",AUDIT,",
        ],
        verify::panos_date,
    ),
    // filterlog, or the match action columns
    one_of(
        "filebeat.pfsense_log.default",
        50,
        &[
            "filterlog",
            ",match,block,",
            ",match,pass,",
            ",match,reject,",
            ",match,rdr,",
            ",match,nat,",
        ],
    ),
    // Pleasant Password Server tag
    one_of(
        "filebeat.pps_log.default",
        50,
        &["Pleasant Password Server:"],
    ),
    // device=SFW (older and 18.5+)
    one_of(
        "filebeat.sophos_xg.default",
        50,
        &["device=\"SFW\"", "device_name=\"SFW\""],
    ),
    // log_type beside log_component
    every(
        "filebeat.sophos_xg.default",
        45,
        &["log_type=\"", " log_component=\""],
    ),
    // id=firewall time=
    one_of(
        "filebeat.stormshield_log.default",
        50,
        &["id=firewall time=\""],
    ),
    // id=firewall sn=
    one_of(
        "filebeat.sonicwall_firewall_log.default",
        50,
        &["id=firewall sn="],
    ),
    // sn= beside time=
    every(
        "filebeat.sonicwall_firewall_log.default",
        48,
        &["sn=", " time=\""],
    ),
    // qulogd tag and the log banners
    one_of(
        "filebeat.qnap_nas_log.default",
        50,
        &["qulogd[", "conn log: Users: ", "event log: Users: "],
    ),
    // %APACHETOMCAT message id
    one_of("filebeat.tomcat_log.default", 50, &["%APACHETOMCAT-"]),
    // msg_id key
    one_of(
        "filebeat.watchguard_firebox_log.default",
        50,
        &["msg_id=\""],
    ),
    // vectra_json tag
    one_of("filebeat.vectra_detect_log.default", 50, &["vectra_json"]),
    // TCP_MISS/200 style action and status, which ProxySG spells with a space
    checked(
        "filebeat.squid_log.default",
        40,
        &["TCP_", "UDP_", "NONE/", "TAG_NONE/"],
        verify::squid_status,
    ),
    // alert_fast [**] [gid:sid:rev]
    one_of("filebeat.snort_log.default", 40, &["[**] ["]),
    // alert_csv
    checked("filebeat.snort_log.default", 40, &[" ,"], verify::snort_csv),
    // alert_json leading fields
    every(
        "filebeat.snort_log.default",
        40,
        &["{\"seconds\":", ",\"action\":\""],
    ),
    // alert_json key pair
    every(
        "filebeat.snort_log.default",
        40,
        &["\"pkt_gen\"", "\"pkt_num\""],
    ),
    // syslog form with the rule triple
    one_of("filebeat.snort_log.default", 40, &[" snort: ["]),
    // syslog form with a pid
    every("filebeat.snort_log.default", 40, &["snort[", "]: ["]),
    // NSS feed name in the SIEM template
    one_of(
        "filebeat.zscaler_zia_audit.default",
        55,
        &["zscalernss-audit\""],
    ),
    one_of(
        "filebeat.zscaler_zia_dns.default",
        55,
        &["zscalernss-dns\""],
    ),
    one_of(
        "filebeat.zscaler_zia_email_dlp.default",
        55,
        &["zscalernss-emaildlp\""],
    ),
    one_of(
        "filebeat.zscaler_zia_endpoint_dlp.default",
        55,
        &["zscalernss-edlp\""],
    ),
    one_of(
        "filebeat.zscaler_zia_firewall.default",
        55,
        &["zscalernss-fw\""],
    ),
    one_of(
        "filebeat.zscaler_zia_saas_security.default",
        55,
        &["zscalernss-saas_security\""],
    ),
    one_of(
        "filebeat.zscaler_zia_saas_security_activity.default",
        55,
        &["zscalernss-saas_security_activity\""],
    ),
    one_of(
        "filebeat.zscaler_zia_sandbox_verdict.default",
        55,
        &["zscalernss-sandbox_verdict\""],
    ),
    one_of(
        "filebeat.zscaler_zia_tunnel.default",
        55,
        &["zscalernss-tunnel\""],
    ),
    one_of(
        "filebeat.zscaler_zia_web.default",
        55,
        &["zscalernss-web\""],
    ),
    one_of("filebeat.zscaler_zia_alerts.default", 55, &["ZscalerNSS:"]),
    every(
        "filebeat.tanium_action_history.default",
        50,
        &["\"ActionName\"", "\"PackageName\"", "\"DistributeOver\""],
    ),
    every(
        "filebeat.tanium_client_status.default",
        50,
        &[
            "\"RegisteredWithTLS\"",
            "\"LastRegistration\"",
            "\"ServerNetworkLocation\"",
        ],
    ),
    every(
        "filebeat.tanium_discover.default",
        50,
        &[
            "\"MacAddress\"",
            "\"TaniumComputerId\"",
            "\"MacOrganization\"",
        ],
    ),
    every(
        "filebeat.tanium_endpoint_config.default",
        50,
        &["\"config_item\"", "\"manifest\""],
    ),
    every(
        "filebeat.tanium_reporting.default",
        50,
        &[
            "\"Computer Name\"",
            "\"OS Platform\"",
            "\"Virtual Platform\"",
            "\"Is Virtual\"",
        ],
    ),
    every(
        "filebeat.tanium_threat_response.default",
        50,
        &["\"Alert Id\"", "\"Intel Id\"", "\"Intel Type\""],
    ),
    every(
        "filebeat.tanium_threat_response.default",
        50,
        &["\"Event Id\"", "\"Event Name\"", "\"Computer IP\""],
    ),
    every(
        "filebeat.tanium_threat_response.default",
        50,
        &["\"rowId\"", "\"revision\"", "\"userName\""],
    ),
    every(
        "filebeat.zscaler_zpa_app_connector_status.default",
        50,
        &[
            "\"ConnectorGroup\"",
            "\"CPUUtilization\"",
            "\"SessionType\"",
        ],
    ),
    every(
        "filebeat.zscaler_zpa_audit.default",
        50,
        &[
            "\"AuditOperationType\"",
            "\"AuditNewValue\"",
            "\"ObjectType\"",
        ],
    ),
    every(
        "filebeat.zscaler_zpa_browser_access.default",
        50,
        &["\"Exporter\"", "\"TimestampRequestReceiveStart\""],
    ),
    every(
        "filebeat.zscaler_zpa_user_activity.default",
        50,
        &[
            "\"ConnectionSetupTime\"",
            "\"IPProtocol\"",
            "\"DoubleEncryption\"",
        ],
    ),
    every(
        "filebeat.zscaler_zpa_user_status.default",
        50,
        &["\"CertificateCN\"", "\"TrustedNetworks\""],
    ),
    every(
        "filebeat.prisma_cloud_host.default",
        50,
        &["\"binaries\"", "\"startupBinaries\"", "\"cloudMetadata\""],
    ),
    every(
        "filebeat.prisma_cloud_host_profile.default",
        50,
        &["\"startupProcess\"", "\"sshEvents\"", "\"hash\""],
    ),
    every(
        "filebeat.prisma_cloud_incident_audit.default",
        50,
        &[
            "\"acknowledged\"",
            "\"appID\"",
            "\"audits\"",
            "\"attackTechniques\"",
        ],
    ),
    // access-log action, below squid whose slash form it shares
    one_of("filebeat.proxysg_log.default", 35, &[" TCP_"]),
    // CONNECT rows with no action
    one_of("filebeat.proxysg_log.default", 45, &["TUNNELED "]),
    // YYYY:MM:DD-HH:MM:SS stamp
    checked(
        "filebeat.sophos_utm.default",
        50,
        &[
            "2018:", "2019:", "2020:", "2021:", "2022:", "2023:", "2024:", "2025:", "2026:",
            "2027:", "2028:", "2029:", "2030:",
        ],
        verify::utm_stamp,
    ),
    // PulseSecure tag
    one_of(
        "filebeat.pulse_connect_secure_log.default",
        55,
        &["PulseSecure:"],
    ),
    // SymantecServer tag
    one_of(
        "filebeat.symantec_endpoint_log.default",
        55,
        &["SymantecServer"],
    ),
    // client log columns
    one_of(
        "filebeat.symantec_endpoint_log.default",
        40,
        &[
            ",Event Description: ",
            ",Local Host IP: ",
            ",Computer name: ",
            ",Local Host: ",
        ],
    ),
    // server log columns
    every(
        "filebeat.symantec_endpoint_log.default",
        40,
        &["Site: ", ",Server"],
    ),
    // scan log columns
    every(
        "filebeat.symantec_endpoint_log.default",
        40,
        &["Scan ID: ", ",Begin: "],
    ),
    // behaviour rows
    one_of("filebeat.symantec_endpoint_log.default", 40, &[",[AC"]),
    // system and policy rows
    one_of(
        "filebeat.symantec_endpoint_log.default",
        40,
        &[",Category: "],
    ),
    // VCSA and ESXi program names, SD id and world id
    one_of(
        "filebeat.vsphere_log.default",
        40,
        &[
            " vmon ",
            " vpxd ",
            " vpxd-svcs ",
            " vpxd-main ",
            " vpxd-svcs-main ",
            " applmgmt-audit ",
            " applmgmt ",
            " vmcad ",
            " ssoadminserver ",
            " updatemgr ",
            " Hostd:",
            " Vpxa:",
            " Rhttpproxy:",
            " vmkernel:",
            " localcli[",
            " vcenter-server:",
            "[Originator@",
            "Z cpu",
        ],
    ),
    // DefensePro tag in both header forms
    one_of(
        "filebeat.radware_defensepro.default",
        55,
        &["DefensePro: ", "%DefensePro "],
    ),
];

/// Every CEF vendor and product entry.
pub(super) static CEF: &[CefMarker] = &[
    cef("filebeat.citrix_adc_log.default", 57, "Citrix", "NetScaler"),
    cef("filebeat.citrix_waf_log.default", 56, "Citrix", "NetScaler"),
    cef("filebeat.claroty_ctd_event.default", 60, "Claroty", "CTD"),
    cef(
        "filebeat.cyberark_pta_events.default",
        60,
        "CyberArk",
        "PTA",
    ),
    cef(
        "filebeat.imperva_securesphere.default",
        60,
        "Imperva Inc.",
        "SecureSphere",
    ),
    cef("filebeat.infoblox_nios_log.default", 60, "Infoblox", "NIOS"),
    cef(
        "filebeat.infoblox_threat_defense_event.default",
        60,
        "Infoblox",
        "Data Connector",
    ),
    cef(
        "filebeat.ping_federate_audit.default",
        60,
        "Ping Identity",
        "PingFederate",
    ),
    cef(
        "filebeat.prisma_access_event.default",
        60,
        "Palo Alto Networks",
        "LF",
    ),
    cef(
        "filebeat.thycotic_ss_logs.default",
        60,
        "Thycotic Software",
        "Secret Server",
    ),
    cef(
        "filebeat.trendmicro_deep_security.default",
        60,
        "Trend Micro",
        "Deep Security Agent",
    ),
    cef(
        "filebeat.trendmicro_deep_security.default",
        60,
        "Trend Micro",
        "Deep Security Manager",
    ),
    cef(
        "filebeat.varonis_logs.default",
        60,
        "Varonis Inc.",
        "DatAdvantage",
    ),
];
