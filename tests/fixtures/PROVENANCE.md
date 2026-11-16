# Where these fixtures come from

Two sets live here, and nothing in either comes from Elastic.

- `unencumbered/` holds log samples from CC0, MIT and Apache-2.0 projects. Its own [PROVENANCE.md](unencumbered/PROVENANCE.md) names each upstream and licence.
- `synthetic/` holds samples written for this repository, one file per log variant the floor tests cover. Every value is invented: addresses from the documentation ranges (192.0.2.0/24, 198.51.100.0/24, 203.0.113.0/24), names under example.com, made-up identifiers. Each file follows the layout the product's maker publishes, listed below. They are inputs only, with no expected output beside them.

The Elastic integration samples and the documents Elastic's engine produced for them are kept in the private dfe-transform-elastic-dev repository, which runs the parity tests against them. Never copy them back here: `the_fixture_tree_holds_no_elastic_material` in `crates/dfe-transforms/tests/unencumbered.rs` fails a tree holding anything but these two sets.

## The synthetic set

| Files | Format it follows |
|---|---|
| `synthetic/azure/activitylogs-*.log` | Microsoft Learn, Azure Monitor activity log event schema, the storage account and event hub `records` form |
| `synthetic/azure/auditlogs.log` | Microsoft Learn, Entra ID audit log schema as Azure Monitor streams it, with the Microsoft Graph `directoryAudit` properties |
| `synthetic/azure/signinlogs-*.log` | Microsoft Learn, Entra ID sign-in log schema as Azure Monitor streams it, with the Microsoft Graph `signIn` properties |
| `synthetic/crowdstrike/falcon-*.log` | CrowdStrike Falcon streaming API events as the Falcon SIEM Connector writes them: `metadata` plus `event` |
| `synthetic/okta/system-log.log` | Okta developer docs, System Log API `LogEvent` object |
| `synthetic/fortinet/fortigate*.log` | Fortinet FortiOS Log Reference, 6.2 and 7.4, and the 7.4 Administration Guide "Sample logs by log type" |
| `synthetic/cisco_ios/*.log` | Cisco IOS System Message Guide, message format with and without the syslog header |
| `synthetic/cisco_nexus/nexus.log` | Cisco NX-OS System Message Guide, remote syslog format with the origin id |
| `synthetic/cisco_meraki/*.log` | Cisco Meraki, Syslog Event Types and Log Samples |
| `synthetic/o365/*.log` | Microsoft Learn, Office 365 Management Activity API schema, nested under `o365audit` the way the filebeat o365 input delivers it |
| `synthetic/panw/*.log` | Palo Alto Networks PAN-OS Syslog Field Descriptions, traffic, threat and User-ID log fields in order |
