# Sample logs that can ship in a public repo

Vendor log samples for the sources this service transforms, taken from projects
whose licences permit redistribution. Nothing here comes from Elastic.

These are INPUTS ONLY. There is no expected output beside them, because the only
authority on what a source should produce is Elastic's own engine --
`scripts/compat.py` runs these through it and writes the confirmed documents to
`testdata/compat/`, which is not committed. See [docs/COMPAT.md](../../../docs/COMPAT.md).

The full corpus lives outside this repo, at `/projects/elastic-stuff/unencumbered`,
with a per-file event count and byte size in its own `MANIFEST.md`. Three fortinet
files are truncated to 5,000 lines here; the rest are copied whole.

## Licences and attribution

| Upstream | Licence | Files |
|---|---|---|
| [splunk/botsv1](https://github.com/splunk/botsv1), [splunk/botsv3](https://github.com/splunk/botsv3) | CC0 1.0 | `fortinet/*`, `o365/botsv3-*` |
| [Azure/Azure-Sentinel](https://github.com/Azure/Azure-Sentinel) | MIT, (c) Microsoft | `cisco_meraki/meraki-syslog.log` |
| [OTRF/Security-Datasets](https://github.com/OTRF/Security-Datasets) | MIT, (c) 2021 Open Threat Research Forge | `azure/otrf-*`, `o365/otrf-*` |
| [panther-labs/panther-analysis](https://github.com/panther-labs/panther-analysis) | Apache-2.0 | `azure/panther-*`, `crowdstrike/*`, `cisco_umbrella/*`, `o365/panther-*`, `okta/*` |
| [OpenSOC/opensoc](https://github.com/OpenSOC/opensoc) | Apache-2.0 | `panw/opensoc-*` |
| [apache/metron](https://github.com/apache/metron) | Apache-2.0 | `panw/metron-*` |
| [logstash-plugins/logstash-patterns-core](https://github.com/logstash-plugins/logstash-patterns-core) | Apache-2.0 | `cisco_asa/*` |
| [BenderScript/meraki_syslog_parser](https://github.com/BenderScript/meraki_syslog_parser) | Apache-2.0 | `cisco_meraki/meraki-events.log`, `cisco_meraki/meraki-urls.log`, `cisco_meraki/meraki-firewall.log` |
| [napalm-automation/napalm-logs](https://github.com/napalm-automation/napalm-logs) | Apache-2.0 | `cisco_ios/*`, `cisco_nexus/*` |

CC0 waives attribution; the MIT and Apache-2.0 material requires it, and this
table is the record. Whether a root `NOTICE` is also needed is a licensing call,
not an engineering one.

Provenance for each upstream, with the proof read from each repo's own licence
file, is in `/projects/elastic-stuff/unencumbered/MANIFEST.md` section 1.
