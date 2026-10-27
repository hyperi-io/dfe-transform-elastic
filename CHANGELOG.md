# Changelog

Rendered by CI and committed back at the end of a release -- do not edit by
hand. Release notes also appear on the GitHub Releases page, one per tag.

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
