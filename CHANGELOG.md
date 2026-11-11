# Changelog

Rendered by CI and committed back at the end of a release -- do not edit by
hand. Release notes also appear on the GitHub Releases page, one per tag.

## [1.1.31](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.30...v1.1.31) (2026-11-11)

### Bug Fixes

* **detect:** generate labels and markers from pinned dfe-schemas ([d8d9a41](https://github.com/hyperi-io/dfe-transform-elastic/commit/d8d9a417d31cf0a8d64f52c9b6c67c344872816d))

## [1.1.30](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.29...v1.1.30) (2026-11-11)

### Bug Fixes

* **scalo:** rebuild on 2.14.29 ([1c45afb](https://github.com/hyperi-io/dfe-transform-elastic/commit/1c45afb8b1b0b45b9974e9c8665833ee7efc6519))

## [1.1.29](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.28...v1.1.29) (2026-11-10)

### Bug Fixes

* **ci:** drop Namespace runners, add test-tier ([842b0ed](https://github.com/hyperi-io/dfe-transform-elastic/commit/842b0ed54937be917ab352126e5eee859d276727))
* **transform-elastic:** rebuild on scalo 2.14.28 ([#166](https://github.com/hyperi-io/dfe-transform-elastic/issues/166)) ([5cd9562](https://github.com/hyperi-io/dfe-transform-elastic/commit/5cd956241562789b6949939dac7275bfdfb11d45))

## [1.1.28](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.27...v1.1.28) (2026-11-09)

### Bug Fixes

* **deps:** update redpandadata/redpanda Docker tag to v26.2.6 ([#159](https://github.com/hyperi-io/dfe-transform-elastic/issues/159)) ([f8d1b4d](https://github.com/hyperi-io/dfe-transform-elastic/commit/f8d1b4d3b347f865b08c5f2cbbb367d3e6794ae4))
* **deps:** update rust-dependencies ([#160](https://github.com/hyperi-io/dfe-transform-elastic/issues/160)) ([4a4e07d](https://github.com/hyperi-io/dfe-transform-elastic/commit/4a4e07dca866ebcd85852341acbef3f6ce4575f9))
* **tests:** gate the service-loop tests on a transport ([4ea4782](https://github.com/hyperi-io/dfe-transform-elastic/commit/4ea4782449947bfc358ba887a6611fe9c4beff80))

## [1.1.27](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.26...v1.1.27) (2026-11-03)

### Bug Fixes

* **ci:** drop the nightly full-tier run ([#155](https://github.com/hyperi-io/dfe-transform-elastic/issues/155)) ([4b8ccb0](https://github.com/hyperi-io/dfe-transform-elastic/commit/4b8ccb0d7465de1c928b176a206e50bd40755183))
* rebuild on scalo 2.14.26 ([bb3eef7](https://github.com/hyperi-io/dfe-transform-elastic/commit/bb3eef7f9c61a2fbf840164c33d7ac6d8d3ecabd))

## [1.1.26](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.25...v1.1.26) (2026-11-03)

### Bug Fixes

* **pipeline:** drop duplicate originals and restore unparsed lines ([#154](https://github.com/hyperi-io/dfe-transform-elastic/issues/154)) ([88934fb](https://github.com/hyperi-io/dfe-transform-elastic/commit/88934fb64ab456fcf625e0cd11f9d760b8996f3f))

## [1.1.25](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.24...v1.1.25) (2026-11-02)

### Bug Fixes

* **image:** restore curl for wait-for-engine ([03ac97b](https://github.com/hyperi-io/dfe-transform-elastic/commit/03ac97b26172ae672d660ad8bf4bd83a094ce57e))

## [1.1.24](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.23...v1.1.24) (2026-11-02)

### Bug Fixes

* **elastic:** bound NDJSON fan-out memory, update scalo 2.14.25 ([#152](https://github.com/hyperi-io/dfe-transform-elastic/issues/152)) ([e7bdc5a](https://github.com/hyperi-io/dfe-transform-elastic/commit/e7bdc5ae1d72bb59c244b94b872c22fa6f03ddd7))
* refuse dotted paths deeper than the decode ([#150](https://github.com/hyperi-io/dfe-transform-elastic/issues/150)) ([2485cbc](https://github.com/hyperi-io/dfe-transform-elastic/commit/2485cbc6cf6fc1503d0e8b942532bd4a1c44183f))

## [1.1.23](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.22...v1.1.23) (2026-10-30)

### Bug Fixes

* 2.2.2 round on scalo 2.14.22 ([#149](https://github.com/hyperi-io/dfe-transform-elastic/issues/149)) ([98c7fd3](https://github.com/hyperi-io/dfe-transform-elastic/commit/98c7fd32cdb2ea071c82c29cf01e98e5a5fb22e1))
* bring the clippy config to the baseline ([#147](https://github.com/hyperi-io/dfe-transform-elastic/issues/147)) ([37fe7c4](https://github.com/hyperi-io/dfe-transform-elastic/commit/37fe7c48ac381f9daa2ec0e42ac32532660af206))

## [1.1.22](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.21...v1.1.22) (2026-10-29)

### Bug Fixes

* cut the 302021 teardown line's peak memory by spelling IP digits [0-9] ([#141](https://github.com/hyperi-io/dfe-transform-elastic/issues/141)) ([d4cf7dc](https://github.com/hyperi-io/dfe-transform-elastic/commit/d4cf7dc57844e37888bef1ab3150686c26a51405))
* default the record budget to the DFE record cap of 8,000,000 bytes ([#140](https://github.com/hyperi-io/dfe-transform-elastic/issues/140)) ([1fc53ca](https://github.com/hyperi-io/dfe-transform-elastic/commit/1fc53cab082997f87f369ac310a5741a9b5106f4))
* dress Graph provisioning records as the Azure diagnostic log ([#143](https://github.com/hyperi-io/dfe-transform-elastic/issues/143)) ([d18ddb6](https://github.com/hyperi-io/dfe-transform-elastic/commit/d18ddb694e056eb9bdd4ec7c17c9f3dee2b57903))
* drop an invalid syslog tag instead of lifting it into log.syslog.appname ([#138](https://github.com/hyperi-io/dfe-transform-elastic/issues/138)) ([1c3caeb](https://github.com/hyperi-io/dfe-transform-elastic/commit/1c3caebafb4b83fcda95d746d2b29d276558da1a))
* rebuild on scalo 2.14.21 ([6ba1ffa](https://github.com/hyperi-io/dfe-transform-elastic/commit/6ba1ffaafef58dbfd70e6f5a92af4526d7b21708))
* resolve the 22 rustdoc link warnings ([#144](https://github.com/hyperi-io/dfe-transform-elastic/issues/144)) ([4c85b75](https://github.com/hyperi-io/dfe-transform-elastic/commit/4c85b75fe9b2e740d6be1b09a7bed5b04abd0be0))
* update the lockfile, pulldown-cmark 0.13.5 clears RUSTSEC-2026-0414 ([#145](https://github.com/hyperi-io/dfe-transform-elastic/issues/145)) ([1c9b7f7](https://github.com/hyperi-io/dfe-transform-elastic/commit/1c9b7f74143b3b4c4ed3016b805b13c5f16601c8))
* wrap Microsoft Graph audit records as the Azure diagnostic log the pipelines read ([#139](https://github.com/hyperi-io/dfe-transform-elastic/issues/139)) ([7f1d792](https://github.com/hyperi-io/dfe-transform-elastic/commit/7f1d792dcacb22567d62ab577dac97524f319c02))

## [1.1.21](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.20...v1.1.21) (2026-10-27)

### Bug Fixes

* unwrap a relabelled receiver syslog event and repair its split header ([e20400f](https://github.com/hyperi-io/dfe-transform-elastic/commit/e20400f1812fc5b280ca659e82115ce608e2e065))

## [1.1.20](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.19...v1.1.20) (2026-10-26)

### Bug Fixes

* take a body-framed syslog line's body from _raw when the receiver kept it ([e289f77](https://github.com/hyperi-io/dfe-transform-elastic/commit/e289f775be26832b2955222d71a743cfbca572c2))

## [1.1.19](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.18...v1.1.19) (2026-10-26)

### Bug Fixes

* detect the source of each syslog event in agentless mode ([8a47ea0](https://github.com/hyperi-io/dfe-transform-elastic/commit/8a47ea0f34db01c3e422a3012c146a51fa4c6db3))

## [1.1.18](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.17...v1.1.18) (2026-10-24)

### Bug Fixes

* keep the shipper's agent and namespace fields on a receiver delivery ([#125](https://github.com/hyperi-io/dfe-transform-elastic/issues/125)) ([9f5adbe](https://github.com/hyperi-io/dfe-transform-elastic/commit/9f5adbe13ebdb6d69508acdbd9e06c28709a9bb6))
* take scalo 2.14.20 ([#127](https://github.com/hyperi-io/dfe-transform-elastic/issues/127)) ([cecc086](https://github.com/hyperi-io/dfe-transform-elastic/commit/cecc08642f20f37fa60e66473c4981b0c4660fc2))

## [1.1.17](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.16...v1.1.17) (2026-10-21)

### Bug Fixes

* rebuild on scalo 2.14.18 ([#123](https://github.com/hyperi-io/dfe-transform-elastic/issues/123)) ([01d092a](https://github.com/hyperi-io/dfe-transform-elastic/commit/01d092a78a6bd39ce9096379d5bb2d5c1d31114d))

## [1.1.16](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.15...v1.1.16) (2026-10-20)

### Bug Fixes

* make source and sink config fields optional ([#122](https://github.com/hyperi-io/dfe-transform-elastic/issues/122)) ([0b4ead3](https://github.com/hyperi-io/dfe-transform-elastic/commit/0b4ead3a229cc8a05ec87cb07d7d00e4a23cfcf9))

## [1.1.15](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.14...v1.1.15) (2026-10-17)

### Bug Fixes

* rebuild on scalo 2.14.17 ([b3ba12b](https://github.com/hyperi-io/dfe-transform-elastic/commit/b3ba12b2db5df317070333c9c0b724ff09f89498))

## [1.1.14](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.13...v1.1.14) (2026-10-15)

### Bug Fixes

* rebuild on scalo 2.14.15 and rustls 0.23.46 ([a588298](https://github.com/hyperi-io/dfe-transform-elastic/commit/a5882984bf33610d1ad6b08828ede925f91542ff))

## [1.1.13](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.12...v1.1.13) (2026-10-14)

### Bug Fixes

* rebuild on scalo 2.14.13 and parse YAML through it ([#119](https://github.com/hyperi-io/dfe-transform-elastic/issues/119)) ([cd4eb28](https://github.com/hyperi-io/dfe-transform-elastic/commit/cd4eb2826a4aa95af7dc45c50cd42558a3813418))

## [1.1.12](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.11...v1.1.12) (2026-10-12)

### Bug Fixes

* rebuild on scalo 2.14.9 ([72c9a3a](https://github.com/hyperi-io/dfe-transform-elastic/commit/72c9a3a2b7385fd05e573b070c2f02c949d69219))

## [1.1.11](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.10...v1.1.11) (2026-10-11)

### Bug Fixes

* move to scalo 2.14.6, drop emit-chart ([a2aa95b](https://github.com/hyperi-io/dfe-transform-elastic/commit/a2aa95bba540a999b8431a555a5aab54a3f60280))
* pass the R2 credentials so a release publishes its binaries ([#113](https://github.com/hyperi-io/dfe-transform-elastic/issues/113)) ([25fe566](https://github.com/hyperi-io/dfe-transform-elastic/commit/25fe5663c83fd226fcbfffd634520d93825c73cd))

## [1.1.10](https://github.com/hyperi-io/dfe-transform-elastic/compare/v1.1.9...v1.1.10) (2026-10-09)

### Bug Fixes

* **ci:** give the optimised release build six hours ([#104](https://github.com/hyperi-io/dfe-transform-elastic/issues/104)) ([f92928d](https://github.com/hyperi-io/dfe-transform-elastic/commit/f92928dbc7df529ecef20d41c9533e60535852e1))
* emit deployment contract v4 ([#109](https://github.com/hyperi-io/dfe-transform-elastic/issues/109)) ([86c7fea](https://github.com/hyperi-io/dfe-transform-elastic/commit/86c7feaf5a1e33e7334854b1c3f622d79c01ae17))
* give the real reason a --config file misses scalo's cascade ([#110](https://github.com/hyperi-io/dfe-transform-elastic/issues/110)) ([94de0b4](https://github.com/hyperi-io/dfe-transform-elastic/commit/94de0b4551646ce3bd059ec417483d3f1b0b28b5))
* move Elastic test data out of the public repo ([#107](https://github.com/hyperi-io/dfe-transform-elastic/issues/107)) ([e1640e8](https://github.com/hyperi-io/dfe-transform-elastic/commit/e1640e8cdefdb04eef62b890dd34f291d294c182))
* pass the release app key to the release tail ([#112](https://github.com/hyperi-io/dfe-transform-elastic/issues/112)) ([cdb2ecb](https://github.com/hyperi-io/dfe-transform-elastic/commit/cdb2ecb1f70f179699c5d7099d5153ff9944af86))
* read Elastic test excerpts from external data ([#108](https://github.com/hyperi-io/dfe-transform-elastic/issues/108)) ([821a5fc](https://github.com/hyperi-io/dfe-transform-elastic/commit/821a5fcc8f1375b4366c2caa965100471388550b))

# 1.0.0 (2026-03-03)


### Features

* initial project scaffolding with architecture design ([ab3ada1](https://github.com/hyperi-io/dfe-transform-elastic/commit/ab3ada1feda86f44eb80e6294a6e27472f425a5e))
