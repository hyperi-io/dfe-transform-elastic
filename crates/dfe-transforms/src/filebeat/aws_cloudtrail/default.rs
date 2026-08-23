// SPDX-License-Identifier: BUSL-1.1
// Copyright (c) 2026 HYPERI PTY LIMITED
//
// Generated file. Do not edit by hand.

use dfe_runtime::prelude::*;

/// Transform for the `default` pipeline.
pub struct Default;

impl Transform for Default {
    fn name(&self) -> &str {
        "default"
    }

    fn transform(&self, event: &mut dfe_runtime::Event) -> Result<TransformResult> {
        // A `drop` returns through here, so the closure carries the outcome.
        let outcome = (|event: &mut dfe_runtime::Event| -> Result<TransformResult> {
            let _cond = { !event.has_value("event.original") };
            if _cond {
                event.rename("message", "event.original")?;
            }

            let _cond = { event.has_value("event.original") };
            if _cond {
                event.remove("message");
            }

            // SKIPPED: condition not transpiled: ctx['@timestamp'] != null
            #[allow(unreachable_code, unused_variables)]
            if false {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
            }

            parse_json_field(event, "event.original", "json")?;

            event.set("ecs.version", json!("8.11.0"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.eventTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || v == '-' || v == 'none' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            let _cond = {
                event.has_value("json.eventName")
                    && event.has_value("tags")
                    && event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => {
                            a.iter().any(|x| x.as_str() == Some("actor_target_mapping"))
                        }
                        serde_json::Value::String(s) => s.contains("actor_target_mapping"),
                        _ => false,
                    })
            };
            if _cond {
                // on_failure: 2 handler(s)
                if let Err(err) = (|| -> Result<()> {
                    // Painless script
                    // Source: void addFields(Set entities, List fields) {\n  for (String field : fields) {\n    addField(entities, field);\n  }\n}\n\nvoid addField(Set entities, String fieldName) {\n  addValue(entities, $(fieldName, null));\n}\n\nboolean addValues(Set entities, List values) {\n  boolean addedAll = true;\n  for (String value : values) {\n    addedAll = addedAll && addValue(entities, value);\n  }\n  return addedAll;\n}\n\nboolean addValue(Set entities, String value) {\n  if (value == null || value == \"\") {\n    return false;\n  }\n  return entities.add(value);\n}\n\nvoid enrichCloudformation(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"cloudformation.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateStackSet\") {\n    addField(enrichCtx.target, \"json.responseElements.stackSetId\");\n  } else if (eventName == \"CreateStack\") {\n    addField(enrichCtx.target, \"json.responseElements.stackId\");\n  }\n}\n\nvoid enrichCloudtrail(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"cloudtrail.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.name\",\n    \"json.requestParameters.s3BucketName\",\n    \"json.responseElements.cloudWatchLogsLogGroupArn\",\n    \"json.responseElements.cloudWatchLogsRoleArn\",\n    \"json.responseElements.kmsKeyId\",\n    \"json.responseElements.snsTopicARN\",\n    \"json.responseElements.trailARN\",\n    \"json.responseElements.name\"\n  ]);\n\n  if (eventName == \"DeleteTrail\"\n    || eventName == \"StopLogging\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n  } else if (eventName == \"DescribeTrails\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichEc2InstanceConnect(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ec2-instance-connect.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"SendSSHPublicKey\"\n    || eventName == \"SendSerialConsoleSSHPublicKey\") {\n    addField(enrichCtx.target, \"json.requestParameters.instanceId\");\n  }\n}\n\nvoid enrichConfig(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"config.amazonaws.com\") {\n    return;\n  }\n\n  addField(enrichCtx.related, \"json.requestParameters.configurationRecorderName\");\n\n  if (eventName == \"StopConfigurationRecorder\"\n          || eventName == \"StartConfigurationRecorder\") {\n    addField(enrichCtx.target, \"json.requestParameters.configurationRecorderName\");\n  }\n}\n\nvoid enrichEc2(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ec2.amazonaws.com\") {\n      return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.groupId\",\n    \"json.requestParameters.groupName\",\n    \"json.requestParameters.roleName\",\n    \"json.requestParameters.subnetId\",\n    \"json.requestParameters.volumeId\",\n    \"json.requestParameters.networkInterfaceId\",\n    \"json.requestParameters.vpcId\",\n    \"json.requestParameters.snapshotId\",\n    \"json.responseElements.groupId\",\n    \"json.responseElements.reservationId\",\n    \"json.responseElements.vpc.vpcId\",\n    \"json.responseElements.vpc.dhcpOptionsId\",\n    \"json.responseElements.snapshotId\",\n    \"json.responseElements.volumeId\"\n  ]);\n\n  $(\"json.responseElements.securityGroupRuleSet.items\", []).stream().forEach(i -> {\n    addValues(enrichCtx.related, [\n      i.groupId,\n      i.referencedGroupInfo?.groupId,\n      i.securityGroupRuleId\n    ]);\n  });\n\n  $(\"json.responseElements.groupSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.groupId);\n  });\n\n  $(\"json.requestParameters.groupSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.groupId);\n  });\n\n  $(\"json.requestParameters.instancesSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.instanceId);\n  });\n\n  $(\"json.responseElements.instancesSet.items\", []).stream().forEach(instances -> {\n    addValues(enrichCtx.related, [\n      instances.subnetId,\n      instances.vpcId,\n      instances.instanceId,\n      instances.imageId,\n      instances.iamInstanceProfile?.arn\n    ]);\n\n    if (instances.networkInterfaceSet?.items == null) {\n      return true;\n    }\n\n    instances.networkInterfaceSet?.items?.stream().forEach(networks -> {\n      addValues(enrichCtx.related, [\n        networks.networkInterfaceId,\n        networks.vpcId,\n        networks.subnetId\n      ]);\n\n      if (networks.groupSet?.items == null) {\n        return true;\n      }\n\n      networks.groupSet?.items?.stream().forEach(group -> {\n        addValue(enrichCtx.related, group.groupId);\n      });\n    });\n  });\n\n  $(\"json.requestParameters.revokedSecurityGroupRuleSet.items\", []).stream().forEach(i -> {\n    addValues(enrichCtx.related, [\n      i.securityGroupRuleId,\n      i.groupId\n    ]);\n  });\n\n  if (eventName == \"AuthorizeSecurityGroupIngress\"\n      || eventName == \"AuthorizeSecurityGroupEgress\") {\n    addField(enrichCtx.target, \"json.requestParameters.groupId\");\n    $(\"json.responseElements.securityGroupRuleSet.items\", []).stream().forEach(f -> addValue(enrichCtx.target, f.securityGroupRuleId));\n\n  } else if (eventName == \"CreateTrafficMirrorFilter\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorFilterResponse.trafficMirrorFilter.trafficMirrorFilterId\");\n\n  } else if (eventName == \"CreateTrafficMirrorFilterRule\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorFilterRuleResponse.trafficMirrorFilterRule.trafficMirrorFilterRuleId\");\n\n  } else if (eventName == \"CreateTrafficMirrorSession\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorSessionResponse.trafficMirrorSession.trafficMirrorSessionId\");\n\n  } else if (eventName == \"CreateTrafficMirrorTarget\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorTargetResponse.trafficMirrorTarget.trafficMirrorTargetId\");\n\n  } else if (eventName == \"DeleteFlowLogs\") {\n    addField(enrichCtx.target, \"json.requestParameters.DeleteFlowLogsRequest.FlowLogId.content\");\n\n  } else if (eventName == \"DeleteNetworkAcl\") {\n    addField(enrichCtx.target, \"json.requestParameters.networkAclId\");\n\n  } else if (eventName == \"DeleteNetworkAclEntry\") {\n    addField(enrichCtx.target, \"json.requestParameters.networkAclId\");\n    def ruleNumber = $(\"json.requestParameters.ruleNumber\", null);\n    if (ruleNumber != null) {\n      addValue(enrichCtx.target, String.valueOf(ruleNumber));\n    }\n\n  } else if (eventName == \"GetPasswordData\") {\n    addField(enrichCtx.target, \"json.requestParameters.instanceId\");\n\n  } else if (eventName == \"ModifyImageAttribute\") {\n    addField(enrichCtx.target, \"json.requestParameters.imageId\");\n\n  } else if (eventName == \"ModifySnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.requestParameters.snapshotId\");\n\n  } else if (eventName == \"DescribeSecurityGroups\"\n            || eventName == \"DescribeNetworkInterfaces\"\n            || eventName == \"DescribeRegions\"\n            || eventName == \"DescribeVpcs\"\n            || eventName == \"DescribeNetworkAcls\"\n            || eventName == \"DescribeVolumes\"\n  ) {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichElasticFileSystem(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"elasticfilesystem.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteFileSystem\") {\n    addField(enrichCtx.target, \"json.requestParameters.fileSystemId\");\n  } else if (eventName == \"DeleteMountTarget\") {\n    addField(enrichCtx.target, \"json.requestParameters.mountTargetId\");\n  }\n}\n\nvoid enrichEvents(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"events.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteRule\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n  }\n}\n\nvoid enrichGuardDuty(def enrichCtx, def eventSource, def eventName)  {\n  if (eventSource != \"guardduty.amazonaws.com\") {\n      return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.detectorId\",\n    \"json.responseElements.detectorId\"\n  ]);\n\n  if (eventName == \"CreateDetector\") {\n    addField(enrichCtx.target, \"json.responseElements.detectorId\");\n  } else if (eventName == \"DeleteDetector\") {\n    addField(enrichCtx.target, \"json.requestParameters.detectorId\");\n  }\n}\n\nvoid enrichIam(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"iam.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.userName\",\n    \"json.requestParameters.serialNumber\",\n    \"json.requestParameters.accessKeyId\",\n    \"json.requestParameters.policyArn\",\n    \"json.requestParameters.roleName\",\n    \"json.requestParameters.policyName\",\n    \"json.requestParameters.serialNumber\",\n    \"json.responseElements.accessKey.userName\",\n    \"json.responseElements.accessKey.accessKeyId\",\n    \"json.responseElements.user.arn\",\n    \"json.responseElements.user.userId\",\n    \"json.responseElements.user.userName\",\n    \"json.responseElements.userId\",\n    \"json.responseElements.role.arn\",\n    \"json.responseElements.serialNumber\"\n  ]);\n\n  if (eventName == \"AttachGroupPolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.groupName\");\n\n  } else if (eventName == \"AttachRolePolicy\"\n            || eventName == \"ListAttachedRolePolicies\"\n            || eventName == \"UpdateAssumeRolePolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.roleName\");\n\n  } else if (eventName == \"AttachUserPolicy\") {\n    addFields(enrichCtx.target, [\n      \"json.requestParameters.policyArn\",\n      \"json.requestParameters.userName\"\n    ]);\n\n  } else if (eventName == \"CreateAccessKey\") {\n    addFields(enrichCtx.target, [\n      \"json.responseElements.accessKey.accessKeyId\",\n      \"json.requestParameters.userName\"\n    ]);\n\n  } else if (eventName == \"CreateUser\"\n            || eventName == \"DeactivateMFADevice\") {\n    addField(enrichCtx.target, \"json.requestParameters.userName\");\n\n  } else if (eventName == \"DeleteVirtualMFADevice\") {\n    addField(enrichCtx.target, \"json.requestParameters.serialNumber\");\n\n  } else if (eventName == \"GetPolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.policyArn\");\n\n  } else if (eventName == \"CreatePolicy\") {\n    addField(enrichCtx.target, \"json.responseElements.policy.arn\");\n\n  } else if (eventName == \"ListRoles\"\n            || eventName == \"ListUsers\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichKms(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"kms.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.keyId\",\n    \"json.responseElements.keyId\",\n    \"json.responseElements.keyMetadata.arn\",\n    \"json.responseElements.keyMetadata.keyId\"\n  ]);\n\n  if (eventName == \"DisableKey\"\n    || eventName == \"ScheduleKeyDeletion\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n  }\n}\n\nvoid enrichLambda(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"lambda.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.functionName\",\n    \"json.responseElements.functionArn\",\n    \"json.responseElements.functionName\",\n    \"json.responseElements.role\"\n  ]);\n\n  addValues(enrichCtx.related, $(\"json.responseElements.vpcConfig.securityGroupIds\", []));\n  addValues(enrichCtx.related, $(\"json.responseElements.vpcConfig.subnetIds\", []));\n\n  if (eventName == null) {\n    return;\n  }\n\n  if (eventName.contains(\"AddPermission\")) { // needs to be contains because lambda event names are versioned on the name\n    addField(enrichCtx.target, \"json.requestParameters.functionName\");\n\n  } else if (eventName.contains(\"ListFunctions\")) {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichLogs(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"logs.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteLogGroup\") {\n    addField(enrichCtx.target, \"json.requestParameters.logGroupName\");\n\n  } else if (eventName == \"DeleteLogStream\") {\n    addField(enrichCtx.target, \"json.requestParameters.logStreamName\");\n\n  }\n}\n\nvoid enrichMonitoring(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"monitoring.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteAlarms\") {\n    $(\"json.requestParameters.alarmNames\", []).stream().forEach(f -> addValue(enrichCtx.target, f));\n\n  }\n}\n\nvoid enrichRds(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"rds.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.dBInstanceIdentifier\",\n    \"json.requestParameters.dBInstanceArn\",\n    \"json.responseElements.dBInstanceIdentifier\",\n    \"json.responseElements.dbInstanceArn\",\n    \"json.responseElements.dBSubnetGroup.vpcId\",\n    \"json.responseElements.vpcSecurityGroups.vpcSecurityGroupId\"\n  ]);\n\n  $(\"json.responseElements.dBSubnetGroup.subnets\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.subnetIdentifier);\n  });\n\n  $(\"json.responseElements.vpcSecurityGroups\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.vpcSecurityGroupId);\n  });\n\n  if (eventName == \"DeleteDBCluster\"\n    || eventName == \"ModifyDBCluster\"\n    || eventName == \"StopDBCluster\") {\n    addField(enrichCtx.target, \"json.responseElements.dBClusterArn\");\n\n  } else if (eventName == \"DeleteDBInstance\"\n          || eventName == \"ModifyDBInstance\"\n          || eventName == \"RestoreDBInstanceFromDBSnapshot\"\n          || eventName == \"RestoreDBInstanceFromS3\"\n          || eventName == \"StopDBInstance\") {\n    addField(enrichCtx.target, \"json.responseElements.dBInstanceArn\");\n\n  } else if (eventName == \"DeleteGlobalCluster\") {\n    addField(enrichCtx.target, \"json.responseElements.globalClusterArn\");\n\n  } else if (eventName == \"ModifyDBClusterSnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.responseElements.dBClusterSnapshotIdentifier\");\n\n  } else if (eventName == \"ModifyDBSnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.responseElements.dBSnapshotIdentifier\");\n\n  } else if (eventName == \"DescribeDBInstances\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichRolesAnywhere(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"rolesanywhere.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateTrustAnchor\") {\n    addField(enrichCtx.target, \"json.responseElements.trustAnchor.trustAnchorArn\");\n\n  }\n}\n\nvoid enrichRoute53Resolver(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"route53resolver.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteResolverQueryLogConfig\") {\n    addField(enrichCtx.target, \"json.responseElements.resolverQueryLogConfig.arn\");\n  }\n}\n\nvoid enrichS3(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"s3.amazonaws.com\") {\n    return;\n  }\n\n  addField(enrichCtx.related, \"json.requestParameters.bucketName\");\n\n  if (eventName == \"CopyObject\"\n    || eventName == \"PutBucketLogging\"\n    || eventName == \"PutBucketVersioning\"\n    || eventName == \"PutObject\"\n    || eventName == \"GetBucketPolicy\"\n    || eventName == \"ListObjects\"\n    || eventName == \"HeadObject\"\n    || eventName == \"GetObject\"\n    || eventName == \"DeleteObject\"\n    || eventName == \"DeleteBucket\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n\n  } else if (eventName == \"PutBucketReplication\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n    addField(enrichCtx.target, \"json.requestParameters.ReplicationConfiguration.Rule.Destination.Bucket\");\n\n  } else if (eventName == \"ListBuckets\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichSecretsManager(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"secretsmanager.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"BatchGetSecretValue\") {\n    $(\"json.requestParameters.secretIdList\", []).stream().forEach(f -> addValue(enrichCtx.target, f));\n\n  } else if (eventName == \"GetSecretValue\") {\n    addField(enrichCtx.target, \"json.requestParameters.secretId\");\n\n  }\n}\n\nvoid enrichSignin(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"signin.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"ConsoleLogin\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichSsm(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ssm.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"GetParameter\"\n    || eventName == \"GetParameters\"\n    || eventName == \"CreateControlChannel\"\n    || eventName == \"OpenControlChannel\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n\n  } else if (eventName == \"StartSession\") {\n    addField(enrichCtx.target, \"json.requestParameters.target\");\n\n  } else if (eventName == \"CreateDocument\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n\n  } else if (eventName == \"TerminateSession\"\n            || eventName == \"OpenDataChannel\") {\n    addField(enrichCtx.target, \"json.requestParameters.sessionId\");\n\n  } else if (eventName == \"SendCommand\") {\n    List instanceIds = $(\"json.requestParameters.instanceIds\", []);\n\n    if (instanceIds.isEmpty()) {\n      instanceIds = $(\"json.requestParameters.targets\", []).stream().flatMap(target -> target.values.stream()).collect(Collectors.toList());\n    }\n\n    if (instanceIds.size() == 1 && instanceIds.get(0) == \"*\") {\n      instanceIds = [ $(\"json.recipientAccountId\", null) ]; // if all instances, point to full account\n    }\n\n    addValues(enrichCtx.target, instanceIds);\n\n  } else if (eventName == \"ListInstanceAssociations\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichSts(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"sts.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.roleArn\",\n    \"json.sourceIdentity\",\n    \"json.additionalEventData.MFAIdentifier\",\n    \"json.responseElements.assumedRoleUser.arn\",\n    \"json.requestParameters.roleSessionName\",\n    \"json.responseElements.accessKeyId\"\n  ]);\n\n  if (eventName == \"AssumeRole\") {\n    def userType = $(\"json.userIdentity.type\", null);\n\n    if (userType == \"AWSService\") {\n      enrichCtx.actor = $(\"json.userIdentity.invokedBy\", null);\n    } else if (userType == \"AssumedRole\") {\n      enrichCtx.actor = $(\"json.userIdentity.sessionContext.sessionIssuer.arn\", null);\n    } else {\n      enrichCtx.actor = $(\"json.userIdentity.arn\", null);\n    }\n\n    addField(enrichCtx.target, \"json.requestParameters.roleArn\");\n\n  } else if (eventName == \"GetCallerIdentity\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n    enrichCtx.actor = $(\"json.userIdentity.arn\", null);\n\n  }\n}\n\nvoid enrichWafv2(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"wafv2.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.id\",\n    \"json.responseElements.summary\"\n  ]);\n\n  if (eventName == \"DeleteRuleGroup\"\n      || eventName == \"DeleteWebACL\") {\n    addField(enrichCtx.target, \"json.requestParameters.id\");\n  }\n}\n\nvoid enrichSns(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"sns.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateTopic\") {\n    addField(enrichCtx.target, \"json.responseElements.topicArn\");\n  } else if (eventName == \"Subscribe\"\n            || eventName == \"Publish\") {\n    addField(enrichCtx.target, \"json.requestParameters.topicArn\");\n  }\n\n}\n\nvoid enrichBedrock(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"bedrock.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"Converse\") {\n    addField(enrichCtx.target, \"json.requestParameters.modelId\");\n\n  }\n}\n\nvoid enrichElasticLoadBalancing(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"elasticloadbalancing.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DescribeLoadBalancers\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichDynamoDB(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"dynamodb.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"ListTables\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nMap enrichCtx = [:];\nenrichCtx.related = new TreeSet();\nenrichCtx.target = new TreeSet();\n\nenrichCtx.actor = $(\"json.userIdentity.arn\", null); // default actor value\nif (enrichCtx.actor == null) {\n  enrichCtx.actor = $(\"json.userIdentity.onBehalfOf.userId\", null);\n}\n\naddFields(enrichCtx.related, [\n  \"json.userIdentity.accessKeyId\",\n  \"json.userIdentity.arn\",\n  \"json.userIdentity.userName\",\n  \"json.userIdentity.sessionContext.sessionIssuer.arn\",\n  \"json.userIdentity.sessionContext.sessionIssuer.userName\"\n]);\n\nif ($(\"json.userIdentity.type\", null) == \"IdentityCenterUser\") {\n  addField(enrichCtx.related, \"json.userIdentity.onBehalfOf.identityStoreArn\");\n}\n\n$(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.related, f.ARN));\n\nString eventSource = $(\"json.eventSource\", null);\nString eventName = $(\"json.eventName\", null);\n\nenrichCloudformation(enrichCtx, eventSource, eventName);\nenrichCloudtrail(enrichCtx, eventSource, eventName);\nenrichConfig(enrichCtx, eventSource, eventName);\nenrichEc2InstanceConnect(enrichCtx, eventSource, eventName);\nenrichEc2(enrichCtx, eventSource, eventName);\nenrichElasticFileSystem(enrichCtx, eventSource, eventName);\nenrichEvents(enrichCtx, eventSource, eventName);\nenrichGuardDuty(enrichCtx, eventSource, eventName);\nenrichIam(enrichCtx, eventSource, eventName);\nenrichKms(enrichCtx, eventSource, eventName);\nenrichLambda(enrichCtx, eventSource, eventName);\nenrichLogs(enrichCtx, eventSource, eventName);\nenrichMonitoring(enrichCtx, eventSource, eventName);\nenrichRds(enrichCtx, eventSource, eventName);\nenrichRolesAnywhere(enrichCtx, eventSource, eventName);\nenrichRoute53Resolver(enrichCtx, eventSource, eventName);\nenrichS3(enrichCtx, eventSource, eventName);\nenrichSecretsManager(enrichCtx, eventSource, eventName);\nenrichSignin(enrichCtx, eventSource, eventName);\nenrichSsm(enrichCtx, eventSource, eventName);\nenrichSts(enrichCtx, eventSource, eventName);\nenrichWafv2(enrichCtx, eventSource, eventName);\nenrichSns(enrichCtx, eventSource, eventName);\nenrichBedrock(enrichCtx, eventSource, eventName);\nenrichElasticLoadBalancing(enrichCtx, eventSource, eventName);\nenrichDynamoDB(enrichCtx, eventSource, eventName);\n\n// Classify target entities by type\nif (!enrichCtx.target.isEmpty()) {\n  // Always set the legacy target.entity.id for backward compatibility\n  field(\"target.entity.id\").set(enrichCtx.target);\n  \n  def userTargets = new TreeSet();\n  def hostTargets = new TreeSet();\n  def serviceTargets = new TreeSet();\n  def genericTargets = new TreeSet();\n\n  for (def targetValue : enrichCtx.target) {\n    if (targetValue == null || targetValue == \"\") {\n      continue;\n    }\n\n    String target = targetValue.toString();\n    boolean classified = false;\n\n    // Handle ARN format: arn:aws:service:region:account-id:resource-type/resource-id\n    if (target.startsWith(\"arn:\")) {\n      def parts = target.splitOnToken(\":\");\n      if (parts.length >= 6) {\n        // Extract resource type (last part before /)\n        String resourcePart = parts[5];\n        String resourceType = resourcePart;\n        \n        // Handle resource-type/resource-id format\n        int slashIdx = resourcePart.indexOf(\"/\");\n        if (slashIdx > 0) {\n          resourceType = resourcePart.substring(0, slashIdx);\n        }\n        \n        // Check for service-linked roles first (AWSServiceRoleFor*) - these are service resources, not users\n        if (target.contains(\"/AWSServiceRoleFor\") || target.contains(\"/aws-service-role/\")) {\n          serviceTargets.add(target);\n          classified = true;\n        }\n        // Classify based on resource type\n        else if (params.userResourceTypes.containsKey(resourceType)) {\n          userTargets.add(target);\n          classified = true;\n        } else if (params.hostResourceTypes.containsKey(resourceType)) {\n          hostTargets.add(target);\n          classified = true;\n        } else if (params.serviceResourceTypes.containsKey(resourceType)) {\n          serviceTargets.add(target);\n          classified = true;\n        } else if (parts[2] == \"s3\") {\n          // Special case: S3 buckets/objects are service resources\n          serviceTargets.add(target);\n          classified = true;\n        }\n      }\n    }\n    // Handle simple resource IDs with prefixes\n    else if (!classified) {\n      // Access Key ID pattern (AKIA...)\n      if (target.length() == 20 && target.startsWith(\"AKIA\")) {\n        userTargets.add(target);\n        classified = true;\n      } else {\n        // Check host ID prefixes\n        for (def prefix : params.hostIdPrefixes) {\n          if (target.startsWith(prefix)) {\n            hostTargets.add(target);\n            classified = true;\n            break;\n          }\n        }\n        \n        // Check service ID prefixes if not yet classified\n        if (!classified) {\n          for (def prefix : params.serviceIdPrefixes) {\n            if (target.startsWith(prefix)) {\n              serviceTargets.add(target);\n              classified = true;\n              break;\n            }\n          }\n        }\n      }\n    }\n\n    // Generic fallback for everything else (including account IDs)\n    if (!classified) {\n      genericTargets.add(target);\n    }\n  }\n\n  // Set the appropriate type-specific entity fields\n  if (!userTargets.isEmpty()) {\n    field(\"user.target.entity.id\").set(userTargets);\n  }\n  if (!hostTargets.isEmpty()) {\n    field(\"host.target.entity.id\").set(hostTargets);\n  }\n  if (!serviceTargets.isEmpty()) {\n    field(\"service.target.entity.id\").set(serviceTargets);\n  }\n  if (!genericTargets.isEmpty()) {\n    field(\"entity.target.id\").set(genericTargets);\n  }\n\n  enrichCtx.related.addAll(enrichCtx.target);\n}\n\n// Map actor to appropriate entity type based on identity\nif (enrichCtx.actor != null) {\n  // Always set the legacy actor.entity.id for backward compatibility\n  field(\"actor.entity.id\").set([ enrichCtx.actor ]);\n  \n  def userType = $(\"json.userIdentity.type\", null);\n  String actorStr = enrichCtx.actor.toString();\n  boolean classified = false;\n  \n  // Check userType first (most reliable indicator)\n  if (userType == \"AWSService\") {\n    field(\"service.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  // Check for service-linked roles (AWSServiceRoleFor*) - these are AWS services, not users\n  else if (actorStr.contains(\"/AWSServiceRoleFor\") || actorStr.contains(\"/aws-service-role/\")) {\n    field(\"service.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  else if (params.userIdentityTypes.containsKey(userType)) {\n    field(\"user.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  \n  // If not classified by userType, parse ARN structure\n  if (!classified && actorStr.startsWith(\"arn:\")) {\n    def parts = actorStr.splitOnToken(\":\");\n    if (parts.length >= 6) {\n      String resourcePart = parts[5];\n      String resourceType = resourcePart;\n      \n      // Handle resource-type/resource-id format\n      int slashIdx = resourcePart.indexOf(\"/\");\n      if (slashIdx > 0) {\n        resourceType = resourcePart.substring(0, slashIdx);\n      }\n      \n      // Classify based on resource type\n      if (resourceType == \"user\" || resourceType == \"role\" || \n          resourceType == \"assumed-role\" || resourceType == \"federated-user\") {\n        field(\"user.entity.id\").set([ enrichCtx.actor ]);\n        classified = true;\n      } else if (resourceType == \"instance\") {\n        field(\"host.entity.id\").set([ enrichCtx.actor ]);\n        classified = true;\n      }\n    }\n  }\n  \n  // Check simple instance ID format\n  if (!classified && actorStr.startsWith(\"i-\")) {\n    field(\"host.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  \n  // Fallback for other types -> entity.id\n  if (!classified) {\n    field(\"entity.id\").set([ enrichCtx.actor ]);\n  }\n}\n\nfield(\"related.entity\").set(enrichCtx.related);\n
                    // TODO: Transpile Painless to Rust (2.2.3)
                    painless_exec_plan_params(
                        event,
                        cached_painless!(
                            r#"void addFields(Set entities, List fields) {\n  for (String field : fields) {\n    addField(entities, field);\n  }\n}\n\nvoid addField(Set entities, String fieldName) {\n  addValue(entities, $(fieldName, null));\n}\n\nboolean addValues(Set entities, List values) {\n  boolean addedAll = true;\n  for (String value : values) {\n    addedAll = addedAll && addValue(entities, value);\n  }\n  return addedAll;\n}\n\nboolean addValue(Set entities, String value) {\n  if (value == null || value == \"\") {\n    return false;\n  }\n  return entities.add(value);\n}\n\nvoid enrichCloudformation(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"cloudformation.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateStackSet\") {\n    addField(enrichCtx.target, \"json.responseElements.stackSetId\");\n  } else if (eventName == \"CreateStack\") {\n    addField(enrichCtx.target, \"json.responseElements.stackId\");\n  }\n}\n\nvoid enrichCloudtrail(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"cloudtrail.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.name\",\n    \"json.requestParameters.s3BucketName\",\n    \"json.responseElements.cloudWatchLogsLogGroupArn\",\n    \"json.responseElements.cloudWatchLogsRoleArn\",\n    \"json.responseElements.kmsKeyId\",\n    \"json.responseElements.snsTopicARN\",\n    \"json.responseElements.trailARN\",\n    \"json.responseElements.name\"\n  ]);\n\n  if (eventName == \"DeleteTrail\"\n    || eventName == \"StopLogging\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n  } else if (eventName == \"DescribeTrails\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichEc2InstanceConnect(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ec2-instance-connect.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"SendSSHPublicKey\"\n    || eventName == \"SendSerialConsoleSSHPublicKey\") {\n    addField(enrichCtx.target, \"json.requestParameters.instanceId\");\n  }\n}\n\nvoid enrichConfig(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"config.amazonaws.com\") {\n    return;\n  }\n\n  addField(enrichCtx.related, \"json.requestParameters.configurationRecorderName\");\n\n  if (eventName == \"StopConfigurationRecorder\"\n          || eventName == \"StartConfigurationRecorder\") {\n    addField(enrichCtx.target, \"json.requestParameters.configurationRecorderName\");\n  }\n}\n\nvoid enrichEc2(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ec2.amazonaws.com\") {\n      return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.groupId\",\n    \"json.requestParameters.groupName\",\n    \"json.requestParameters.roleName\",\n    \"json.requestParameters.subnetId\",\n    \"json.requestParameters.volumeId\",\n    \"json.requestParameters.networkInterfaceId\",\n    \"json.requestParameters.vpcId\",\n    \"json.requestParameters.snapshotId\",\n    \"json.responseElements.groupId\",\n    \"json.responseElements.reservationId\",\n    \"json.responseElements.vpc.vpcId\",\n    \"json.responseElements.vpc.dhcpOptionsId\",\n    \"json.responseElements.snapshotId\",\n    \"json.responseElements.volumeId\"\n  ]);\n\n  $(\"json.responseElements.securityGroupRuleSet.items\", []).stream().forEach(i -> {\n    addValues(enrichCtx.related, [\n      i.groupId,\n      i.referencedGroupInfo?.groupId,\n      i.securityGroupRuleId\n    ]);\n  });\n\n  $(\"json.responseElements.groupSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.groupId);\n  });\n\n  $(\"json.requestParameters.groupSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.groupId);\n  });\n\n  $(\"json.requestParameters.instancesSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.instanceId);\n  });\n\n  $(\"json.responseElements.instancesSet.items\", []).stream().forEach(instances -> {\n    addValues(enrichCtx.related, [\n      instances.subnetId,\n      instances.vpcId,\n      instances.instanceId,\n      instances.imageId,\n      instances.iamInstanceProfile?.arn\n    ]);\n\n    if (instances.networkInterfaceSet?.items == null) {\n      return true;\n    }\n\n    instances.networkInterfaceSet?.items?.stream().forEach(networks -> {\n      addValues(enrichCtx.related, [\n        networks.networkInterfaceId,\n        networks.vpcId,\n        networks.subnetId\n      ]);\n\n      if (networks.groupSet?.items == null) {\n        return true;\n      }\n\n      networks.groupSet?.items?.stream().forEach(group -> {\n        addValue(enrichCtx.related, group.groupId);\n      });\n    });\n  });\n\n  $(\"json.requestParameters.revokedSecurityGroupRuleSet.items\", []).stream().forEach(i -> {\n    addValues(enrichCtx.related, [\n      i.securityGroupRuleId,\n      i.groupId\n    ]);\n  });\n\n  if (eventName == \"AuthorizeSecurityGroupIngress\"\n      || eventName == \"AuthorizeSecurityGroupEgress\") {\n    addField(enrichCtx.target, \"json.requestParameters.groupId\");\n    $(\"json.responseElements.securityGroupRuleSet.items\", []).stream().forEach(f -> addValue(enrichCtx.target, f.securityGroupRuleId));\n\n  } else if (eventName == \"CreateTrafficMirrorFilter\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorFilterResponse.trafficMirrorFilter.trafficMirrorFilterId\");\n\n  } else if (eventName == \"CreateTrafficMirrorFilterRule\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorFilterRuleResponse.trafficMirrorFilterRule.trafficMirrorFilterRuleId\");\n\n  } else if (eventName == \"CreateTrafficMirrorSession\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorSessionResponse.trafficMirrorSession.trafficMirrorSessionId\");\n\n  } else if (eventName == \"CreateTrafficMirrorTarget\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorTargetResponse.trafficMirrorTarget.trafficMirrorTargetId\");\n\n  } else if (eventName == \"DeleteFlowLogs\") {\n    addField(enrichCtx.target, \"json.requestParameters.DeleteFlowLogsRequest.FlowLogId.content\");\n\n  } else if (eventName == \"DeleteNetworkAcl\") {\n    addField(enrichCtx.target, \"json.requestParameters.networkAclId\");\n\n  } else if (eventName == \"DeleteNetworkAclEntry\") {\n    addField(enrichCtx.target, \"json.requestParameters.networkAclId\");\n    def ruleNumber = $(\"json.requestParameters.ruleNumber\", null);\n    if (ruleNumber != null) {\n      addValue(enrichCtx.target, String.valueOf(ruleNumber));\n    }\n\n  } else if (eventName == \"GetPasswordData\") {\n    addField(enrichCtx.target, \"json.requestParameters.instanceId\");\n\n  } else if (eventName == \"ModifyImageAttribute\") {\n    addField(enrichCtx.target, \"json.requestParameters.imageId\");\n\n  } else if (eventName == \"ModifySnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.requestParameters.snapshotId\");\n\n  } else if (eventName == \"DescribeSecurityGroups\"\n            || eventName == \"DescribeNetworkInterfaces\"\n            || eventName == \"DescribeRegions\"\n            || eventName == \"DescribeVpcs\"\n            || eventName == \"DescribeNetworkAcls\"\n            || eventName == \"DescribeVolumes\"\n  ) {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichElasticFileSystem(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"elasticfilesystem.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteFileSystem\") {\n    addField(enrichCtx.target, \"json.requestParameters.fileSystemId\");\n  } else if (eventName == \"DeleteMountTarget\") {\n    addField(enrichCtx.target, \"json.requestParameters.mountTargetId\");\n  }\n}\n\nvoid enrichEvents(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"events.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteRule\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n  }\n}\n\nvoid enrichGuardDuty(def enrichCtx, def eventSource, def eventName)  {\n  if (eventSource != \"guardduty.amazonaws.com\") {\n      return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.detectorId\",\n    \"json.responseElements.detectorId\"\n  ]);\n\n  if (eventName == \"CreateDetector\") {\n    addField(enrichCtx.target, \"json.responseElements.detectorId\");\n  } else if (eventName == \"DeleteDetector\") {\n    addField(enrichCtx.target, \"json.requestParameters.detectorId\");\n  }\n}\n\nvoid enrichIam(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"iam.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.userName\",\n    \"json.requestParameters.serialNumber\",\n    \"json.requestParameters.accessKeyId\",\n    \"json.requestParameters.policyArn\",\n    \"json.requestParameters.roleName\",\n    \"json.requestParameters.policyName\",\n    \"json.requestParameters.serialNumber\",\n    \"json.responseElements.accessKey.userName\",\n    \"json.responseElements.accessKey.accessKeyId\",\n    \"json.responseElements.user.arn\",\n    \"json.responseElements.user.userId\",\n    \"json.responseElements.user.userName\",\n    \"json.responseElements.userId\",\n    \"json.responseElements.role.arn\",\n    \"json.responseElements.serialNumber\"\n  ]);\n\n  if (eventName == \"AttachGroupPolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.groupName\");\n\n  } else if (eventName == \"AttachRolePolicy\"\n            || eventName == \"ListAttachedRolePolicies\"\n            || eventName == \"UpdateAssumeRolePolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.roleName\");\n\n  } else if (eventName == \"AttachUserPolicy\") {\n    addFields(enrichCtx.target, [\n      \"json.requestParameters.policyArn\",\n      \"json.requestParameters.userName\"\n    ]);\n\n  } else if (eventName == \"CreateAccessKey\") {\n    addFields(enrichCtx.target, [\n      \"json.responseElements.accessKey.accessKeyId\",\n      \"json.requestParameters.userName\"\n    ]);\n\n  } else if (eventName == \"CreateUser\"\n            || eventName == \"DeactivateMFADevice\") {\n    addField(enrichCtx.target, \"json.requestParameters.userName\");\n\n  } else if (eventName == \"DeleteVirtualMFADevice\") {\n    addField(enrichCtx.target, \"json.requestParameters.serialNumber\");\n\n  } else if (eventName == \"GetPolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.policyArn\");\n\n  } else if (eventName == \"CreatePolicy\") {\n    addField(enrichCtx.target, \"json.responseElements.policy.arn\");\n\n  } else if (eventName == \"ListRoles\"\n            || eventName == \"ListUsers\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichKms(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"kms.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.keyId\",\n    \"json.responseElements.keyId\",\n    \"json.responseElements.keyMetadata.arn\",\n    \"json.responseElements.keyMetadata.keyId\"\n  ]);\n\n  if (eventName == \"DisableKey\"\n    || eventName == \"ScheduleKeyDeletion\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n  }\n}\n\nvoid enrichLambda(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"lambda.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.functionName\",\n    \"json.responseElements.functionArn\",\n    \"json.responseElements.functionName\",\n    \"json.responseElements.role\"\n  ]);\n\n  addValues(enrichCtx.related, $(\"json.responseElements.vpcConfig.securityGroupIds\", []));\n  addValues(enrichCtx.related, $(\"json.responseElements.vpcConfig.subnetIds\", []));\n\n  if (eventName == null) {\n    return;\n  }\n\n  if (eventName.contains(\"AddPermission\")) { // needs to be contains because lambda event names are versioned on the name\n    addField(enrichCtx.target, \"json.requestParameters.functionName\");\n\n  } else if (eventName.contains(\"ListFunctions\")) {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichLogs(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"logs.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteLogGroup\") {\n    addField(enrichCtx.target, \"json.requestParameters.logGroupName\");\n\n  } else if (eventName == \"DeleteLogStream\") {\n    addField(enrichCtx.target, \"json.requestParameters.logStreamName\");\n\n  }\n}\n\nvoid enrichMonitoring(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"monitoring.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteAlarms\") {\n    $(\"json.requestParameters.alarmNames\", []).stream().forEach(f -> addValue(enrichCtx.target, f));\n\n  }\n}\n\nvoid enrichRds(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"rds.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.dBInstanceIdentifier\",\n    \"json.requestParameters.dBInstanceArn\",\n    \"json.responseElements.dBInstanceIdentifier\",\n    \"json.responseElements.dbInstanceArn\",\n    \"json.responseElements.dBSubnetGroup.vpcId\",\n    \"json.responseElements.vpcSecurityGroups.vpcSecurityGroupId\"\n  ]);\n\n  $(\"json.responseElements.dBSubnetGroup.subnets\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.subnetIdentifier);\n  });\n\n  $(\"json.responseElements.vpcSecurityGroups\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.vpcSecurityGroupId);\n  });\n\n  if (eventName == \"DeleteDBCluster\"\n    || eventName == \"ModifyDBCluster\"\n    || eventName == \"StopDBCluster\") {\n    addField(enrichCtx.target, \"json.responseElements.dBClusterArn\");\n\n  } else if (eventName == \"DeleteDBInstance\"\n          || eventName == \"ModifyDBInstance\"\n          || eventName == \"RestoreDBInstanceFromDBSnapshot\"\n          || eventName == \"RestoreDBInstanceFromS3\"\n          || eventName == \"StopDBInstance\") {\n    addField(enrichCtx.target, \"json.responseElements.dBInstanceArn\");\n\n  } else if (eventName == \"DeleteGlobalCluster\") {\n    addField(enrichCtx.target, \"json.responseElements.globalClusterArn\");\n\n  } else if (eventName == \"ModifyDBClusterSnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.responseElements.dBClusterSnapshotIdentifier\");\n\n  } else if (eventName == \"ModifyDBSnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.responseElements.dBSnapshotIdentifier\");\n\n  } else if (eventName == \"DescribeDBInstances\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichRolesAnywhere(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"rolesanywhere.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateTrustAnchor\") {\n    addField(enrichCtx.target, \"json.responseElements.trustAnchor.trustAnchorArn\");\n\n  }\n}\n\nvoid enrichRoute53Resolver(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"route53resolver.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteResolverQueryLogConfig\") {\n    addField(enrichCtx.target, \"json.responseElements.resolverQueryLogConfig.arn\");\n  }\n}\n\nvoid enrichS3(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"s3.amazonaws.com\") {\n    return;\n  }\n\n  addField(enrichCtx.related, \"json.requestParameters.bucketName\");\n\n  if (eventName == \"CopyObject\"\n    || eventName == \"PutBucketLogging\"\n    || eventName == \"PutBucketVersioning\"\n    || eventName == \"PutObject\"\n    || eventName == \"GetBucketPolicy\"\n    || eventName == \"ListObjects\"\n    || eventName == \"HeadObject\"\n    || eventName == \"GetObject\"\n    || eventName == \"DeleteObject\"\n    || eventName == \"DeleteBucket\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n\n  } else if (eventName == \"PutBucketReplication\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n    addField(enrichCtx.target, \"json.requestParameters.ReplicationConfiguration.Rule.Destination.Bucket\");\n\n  } else if (eventName == \"ListBuckets\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichSecretsManager(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"secretsmanager.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"BatchGetSecretValue\") {\n    $(\"json.requestParameters.secretIdList\", []).stream().forEach(f -> addValue(enrichCtx.target, f));\n\n  } else if (eventName == \"GetSecretValue\") {\n    addField(enrichCtx.target, \"json.requestParameters.secretId\");\n\n  }\n}\n\nvoid enrichSignin(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"signin.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"ConsoleLogin\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichSsm(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ssm.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"GetParameter\"\n    || eventName == \"GetParameters\"\n    || eventName == \"CreateControlChannel\"\n    || eventName == \"OpenControlChannel\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n\n  } else if (eventName == \"StartSession\") {\n    addField(enrichCtx.target, \"json.requestParameters.target\");\n\n  } else if (eventName == \"CreateDocument\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n\n  } else if (eventName == \"TerminateSession\"\n            || eventName == \"OpenDataChannel\") {\n    addField(enrichCtx.target, \"json.requestParameters.sessionId\");\n\n  } else if (eventName == \"SendCommand\") {\n    List instanceIds = $(\"json.requestParameters.instanceIds\", []);\n\n    if (instanceIds.isEmpty()) {\n      instanceIds = $(\"json.requestParameters.targets\", []).stream().flatMap(target -> target.values.stream()).collect(Collectors.toList());\n    }\n\n    if (instanceIds.size() == 1 && instanceIds.get(0) == \"*\") {\n      instanceIds = [ $(\"json.recipientAccountId\", null) ]; // if all instances, point to full account\n    }\n\n    addValues(enrichCtx.target, instanceIds);\n\n  } else if (eventName == \"ListInstanceAssociations\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichSts(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"sts.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.roleArn\",\n    \"json.sourceIdentity\",\n    \"json.additionalEventData.MFAIdentifier\",\n    \"json.responseElements.assumedRoleUser.arn\",\n    \"json.requestParameters.roleSessionName\",\n    \"json.responseElements.accessKeyId\"\n  ]);\n\n  if (eventName == \"AssumeRole\") {\n    def userType = $(\"json.userIdentity.type\", null);\n\n    if (userType == \"AWSService\") {\n      enrichCtx.actor = $(\"json.userIdentity.invokedBy\", null);\n    } else if (userType == \"AssumedRole\") {\n      enrichCtx.actor = $(\"json.userIdentity.sessionContext.sessionIssuer.arn\", null);\n    } else {\n      enrichCtx.actor = $(\"json.userIdentity.arn\", null);\n    }\n\n    addField(enrichCtx.target, \"json.requestParameters.roleArn\");\n\n  } else if (eventName == \"GetCallerIdentity\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n    enrichCtx.actor = $(\"json.userIdentity.arn\", null);\n\n  }\n}\n\nvoid enrichWafv2(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"wafv2.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.id\",\n    \"json.responseElements.summary\"\n  ]);\n\n  if (eventName == \"DeleteRuleGroup\"\n      || eventName == \"DeleteWebACL\") {\n    addField(enrichCtx.target, \"json.requestParameters.id\");\n  }\n}\n\nvoid enrichSns(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"sns.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateTopic\") {\n    addField(enrichCtx.target, \"json.responseElements.topicArn\");\n  } else if (eventName == \"Subscribe\"\n            || eventName == \"Publish\") {\n    addField(enrichCtx.target, \"json.requestParameters.topicArn\");\n  }\n\n}\n\nvoid enrichBedrock(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"bedrock.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"Converse\") {\n    addField(enrichCtx.target, \"json.requestParameters.modelId\");\n\n  }\n}\n\nvoid enrichElasticLoadBalancing(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"elasticloadbalancing.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DescribeLoadBalancers\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichDynamoDB(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"dynamodb.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"ListTables\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nMap enrichCtx = [:];\nenrichCtx.related = new TreeSet();\nenrichCtx.target = new TreeSet();\n\nenrichCtx.actor = $(\"json.userIdentity.arn\", null); // default actor value\nif (enrichCtx.actor == null) {\n  enrichCtx.actor = $(\"json.userIdentity.onBehalfOf.userId\", null);\n}\n\naddFields(enrichCtx.related, [\n  \"json.userIdentity.accessKeyId\",\n  \"json.userIdentity.arn\",\n  \"json.userIdentity.userName\",\n  \"json.userIdentity.sessionContext.sessionIssuer.arn\",\n  \"json.userIdentity.sessionContext.sessionIssuer.userName\"\n]);\n\nif ($(\"json.userIdentity.type\", null) == \"IdentityCenterUser\") {\n  addField(enrichCtx.related, \"json.userIdentity.onBehalfOf.identityStoreArn\");\n}\n\n$(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.related, f.ARN));\n\nString eventSource = $(\"json.eventSource\", null);\nString eventName = $(\"json.eventName\", null);\n\nenrichCloudformation(enrichCtx, eventSource, eventName);\nenrichCloudtrail(enrichCtx, eventSource, eventName);\nenrichConfig(enrichCtx, eventSource, eventName);\nenrichEc2InstanceConnect(enrichCtx, eventSource, eventName);\nenrichEc2(enrichCtx, eventSource, eventName);\nenrichElasticFileSystem(enrichCtx, eventSource, eventName);\nenrichEvents(enrichCtx, eventSource, eventName);\nenrichGuardDuty(enrichCtx, eventSource, eventName);\nenrichIam(enrichCtx, eventSource, eventName);\nenrichKms(enrichCtx, eventSource, eventName);\nenrichLambda(enrichCtx, eventSource, eventName);\nenrichLogs(enrichCtx, eventSource, eventName);\nenrichMonitoring(enrichCtx, eventSource, eventName);\nenrichRds(enrichCtx, eventSource, eventName);\nenrichRolesAnywhere(enrichCtx, eventSource, eventName);\nenrichRoute53Resolver(enrichCtx, eventSource, eventName);\nenrichS3(enrichCtx, eventSource, eventName);\nenrichSecretsManager(enrichCtx, eventSource, eventName);\nenrichSignin(enrichCtx, eventSource, eventName);\nenrichSsm(enrichCtx, eventSource, eventName);\nenrichSts(enrichCtx, eventSource, eventName);\nenrichWafv2(enrichCtx, eventSource, eventName);\nenrichSns(enrichCtx, eventSource, eventName);\nenrichBedrock(enrichCtx, eventSource, eventName);\nenrichElasticLoadBalancing(enrichCtx, eventSource, eventName);\nenrichDynamoDB(enrichCtx, eventSource, eventName);\n\n// Classify target entities by type\nif (!enrichCtx.target.isEmpty()) {\n  // Always set the legacy target.entity.id for backward compatibility\n  field(\"target.entity.id\").set(enrichCtx.target);\n  \n  def userTargets = new TreeSet();\n  def hostTargets = new TreeSet();\n  def serviceTargets = new TreeSet();\n  def genericTargets = new TreeSet();\n\n  for (def targetValue : enrichCtx.target) {\n    if (targetValue == null || targetValue == \"\") {\n      continue;\n    }\n\n    String target = targetValue.toString();\n    boolean classified = false;\n\n    // Handle ARN format: arn:aws:service:region:account-id:resource-type/resource-id\n    if (target.startsWith(\"arn:\")) {\n      def parts = target.splitOnToken(\":\");\n      if (parts.length >= 6) {\n        // Extract resource type (last part before /)\n        String resourcePart = parts[5];\n        String resourceType = resourcePart;\n        \n        // Handle resource-type/resource-id format\n        int slashIdx = resourcePart.indexOf(\"/\");\n        if (slashIdx > 0) {\n          resourceType = resourcePart.substring(0, slashIdx);\n        }\n        \n        // Check for service-linked roles first (AWSServiceRoleFor*) - these are service resources, not users\n        if (target.contains(\"/AWSServiceRoleFor\") || target.contains(\"/aws-service-role/\")) {\n          serviceTargets.add(target);\n          classified = true;\n        }\n        // Classify based on resource type\n        else if (params.userResourceTypes.containsKey(resourceType)) {\n          userTargets.add(target);\n          classified = true;\n        } else if (params.hostResourceTypes.containsKey(resourceType)) {\n          hostTargets.add(target);\n          classified = true;\n        } else if (params.serviceResourceTypes.containsKey(resourceType)) {\n          serviceTargets.add(target);\n          classified = true;\n        } else if (parts[2] == \"s3\") {\n          // Special case: S3 buckets/objects are service resources\n          serviceTargets.add(target);\n          classified = true;\n        }\n      }\n    }\n    // Handle simple resource IDs with prefixes\n    else if (!classified) {\n      // Access Key ID pattern (AKIA...)\n      if (target.length() == 20 && target.startsWith(\"AKIA\")) {\n        userTargets.add(target);\n        classified = true;\n      } else {\n        // Check host ID prefixes\n        for (def prefix : params.hostIdPrefixes) {\n          if (target.startsWith(prefix)) {\n            hostTargets.add(target);\n            classified = true;\n            break;\n          }\n        }\n        \n        // Check service ID prefixes if not yet classified\n        if (!classified) {\n          for (def prefix : params.serviceIdPrefixes) {\n            if (target.startsWith(prefix)) {\n              serviceTargets.add(target);\n              classified = true;\n              break;\n            }\n          }\n        }\n      }\n    }\n\n    // Generic fallback for everything else (including account IDs)\n    if (!classified) {\n      genericTargets.add(target);\n    }\n  }\n\n  // Set the appropriate type-specific entity fields\n  if (!userTargets.isEmpty()) {\n    field(\"user.target.entity.id\").set(userTargets);\n  }\n  if (!hostTargets.isEmpty()) {\n    field(\"host.target.entity.id\").set(hostTargets);\n  }\n  if (!serviceTargets.isEmpty()) {\n    field(\"service.target.entity.id\").set(serviceTargets);\n  }\n  if (!genericTargets.isEmpty()) {\n    field(\"entity.target.id\").set(genericTargets);\n  }\n\n  enrichCtx.related.addAll(enrichCtx.target);\n}\n\n// Map actor to appropriate entity type based on identity\nif (enrichCtx.actor != null) {\n  // Always set the legacy actor.entity.id for backward compatibility\n  field(\"actor.entity.id\").set([ enrichCtx.actor ]);\n  \n  def userType = $(\"json.userIdentity.type\", null);\n  String actorStr = enrichCtx.actor.toString();\n  boolean classified = false;\n  \n  // Check userType first (most reliable indicator)\n  if (userType == \"AWSService\") {\n    field(\"service.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  // Check for service-linked roles (AWSServiceRoleFor*) - these are AWS services, not users\n  else if (actorStr.contains(\"/AWSServiceRoleFor\") || actorStr.contains(\"/aws-service-role/\")) {\n    field(\"service.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  else if (params.userIdentityTypes.containsKey(userType)) {\n    field(\"user.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  \n  // If not classified by userType, parse ARN structure\n  if (!classified && actorStr.startsWith(\"arn:\")) {\n    def parts = actorStr.splitOnToken(\":\");\n    if (parts.length >= 6) {\n      String resourcePart = parts[5];\n      String resourceType = resourcePart;\n      \n      // Handle resource-type/resource-id format\n      int slashIdx = resourcePart.indexOf(\"/\");\n      if (slashIdx > 0) {\n        resourceType = resourcePart.substring(0, slashIdx);\n      }\n      \n      // Classify based on resource type\n      if (resourceType == \"user\" || resourceType == \"role\" || \n          resourceType == \"assumed-role\" || resourceType == \"federated-user\") {\n        field(\"user.entity.id\").set([ enrichCtx.actor ]);\n        classified = true;\n      } else if (resourceType == \"instance\") {\n        field(\"host.entity.id\").set([ enrichCtx.actor ]);\n        classified = true;\n      }\n    }\n  }\n  \n  // Check simple instance ID format\n  if (!classified && actorStr.startsWith(\"i-\")) {\n    field(\"host.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  \n  // Fallback for other types -> entity.id\n  if (!classified) {\n    field(\"entity.id\").set([ enrichCtx.actor ]);\n  }\n}\n\nfield(\"related.entity\").set(enrichCtx.related);\n"#
                        ),
                        cached_params!(
                            "{\"hostIdPrefixes\":[\"i-\"],\"hostResourceTypes\":{\"instance\":true},\"serviceIdPrefixes\":[\"sg-\",\"sgr-\",\"eni-\",\"vpc-\",\"subnet-\",\"acl-\",\"rtb-\",\"igw-\",\"nat-\",\"vpce-\",\"tgw-\",\"pcx-\"],\"serviceResourceTypes\":{\"alarm\":true,\"cluster\":true,\"configuration-recorder\":true,\"db\":true,\"detector\":true,\"dhcp-options\":true,\"document\":true,\"file-system\":true,\"function\":true,\"global-cluster\":true,\"group\":true,\"ipset\":true,\"key\":true,\"loadbalancer\":true,\"log-group\":true,\"log-stream\":true,\"model\":true,\"mount-target\":true,\"network-acl\":true,\"network-interface\":true,\"parameter\":true,\"policy\":true,\"resolver-query-log-config\":true,\"rule\":true,\"rulegroup\":true,\"secret\":true,\"security-group\":true,\"session\":true,\"stack\":true,\"stack-set\":true,\"subnet\":true,\"table\":true,\"topic\":true,\"trail\":true,\"trust-anchor\":true,\"vpc\":true,\"webacl\":true},\"userIdentityTypes\":{\"AssumedRole\":true,\"IAMUser\":true,\"Root\":true},\"userResourceTypes\":{\"assumed-role\":true,\"federated-user\":true,\"instance-profile\":true,\"mfa\":true,\"role\":true,\"serialNumber\":true,\"user\":true}}"
                        ),
                    )?;
                    Ok(())
                })() {
                    event.set("_ingest.on_failure_message", err.to_string())?;
                    event.set("_ingest.on_failure_processor_type", "script")?;
                    event.set(
                        "_ingest.on_failure_processor_tag",
                        "script_append_related_entity",
                    )?;
                    event.append_unique("tags", json!("preserve_original_event"))?;
                    event.set(
                        "error.message",
                        json!(format!(
                            "Processor '{}'\n{}with tag '{}'\n{}failed with message '{}'\n",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("#_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("/_ingest.on_failure_processor_tag")
                                .map_or_else(String::new, template_to_string),
                            event
                                .get("_ingest.on_failure_message")
                                .map_or_else(String::new, template_to_string)
                        )),
                    )?;
                    event.remove("_ingest.on_failure_message");
                    event.remove("_ingest.on_failure_processor_type");
                    event.remove("_ingest.on_failure_processor_tag");
                    if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                        event.remove("_ingest");
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.eventVersion", "aws.cloudtrail.event_version")?;
                Ok(())
            })();

            if event.has("json.userIdentity.type") {
                event.rename(
                    "json.userIdentity.type",
                    "aws.cloudtrail.user_identity.type",
                )?;
            }

            if event.has("json.userIdentity.userName") {
                event.rename("json.userIdentity.userName", "user.name")?;
            }

            if event.has("json.userIdentity.principalId") {
                event.rename("json.userIdentity.principalId", "user.id")?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.userIdentity.arn", "aws.cloudtrail.user_identity.arn")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.userIdentity.accountId", "cloud.account.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.userIdentity.accessKeyId",
                    "aws.cloudtrail.user_identity.access_key_id",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.userIdentity.sessionContext.attributes.mfaAuthenticated",
                    "aws.cloudtrail.user_identity.session_context.mfa_authenticated",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) =
                    event.get_as_string("json.userIdentity.sessionContext.attributes.creationDate")
                {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set(
                            "aws.cloudtrail.user_identity.session_context.creation_date",
                            parsed,
                        )?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.userIdentity.sessionContext.sessionIssuer.type",
                    "aws.cloudtrail.user_identity.session_context.session_issuer.type",
                )?;
                Ok(())
            })();

            let _cond = {
                event.get_str("aws.cloudtrail.user_identity.type") == Some("AssumedRole")
                    || event.get_str("aws.cloudtrail.user_identity.type") == Some("FederatedUser")
            };
            if _cond {
                if event.has("json.userIdentity.sessionContext.sessionIssuer.userName") {
                    event.rename(
                        "json.userIdentity.sessionContext.sessionIssuer.userName",
                        "user.name",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.userIdentity.sessionContext.sessionIssuer.principalId") {
                    event.rename(
                        "json.userIdentity.sessionContext.sessionIssuer.principalId",
                        "aws.cloudtrail.user_identity.session_context.session_issuer.principal_id",
                    )?;
                }
                Ok(())
            })();

            let _cond = {
                (event.get_str("aws.cloudtrail.user_identity.type") == Some("AssumedRole")
                    || event.get_str("aws.cloudtrail.user_identity.type") == Some("FederatedUser"))
                    && event.has_value("aws.cloudtrail.user_identity.arn")
            };
            if _cond {
                if event.has_value("aws.cloudtrail.user_identity.arn") {
                    if let Some(input) = event.get_string("aws.cloudtrail.user_identity.arn") {
                        // Grok pattern: arn:(aws|aws-us-gov):sts:.*/%{GREEDYDATA:_tmp.session_name}$
                        let _ = cached_grok!(
                            "arn:(aws|aws-us-gov):sts:.*/%{GREEDYDATA:_tmp.session_name}$"
                        )
                        .extract_into(&input, event)?;
                    }
                }
            }

            let _cond = { event.has_value("_tmp.session_name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_tmp.session_name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.has_value("_tmp.session_name")
                    && event.get("_tmp.session_name").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a.iter().any(|x| x.as_str() == Some("@")),
                        serde_json::Value::String(s) => s.contains("@"),
                        _ => false,
                    })
            };
            if _cond {
                if let Some(input) = event.get_string("_tmp.session_name") {
                    let mut remaining: &str = &input;
                    let mut captured: Vec<(&str, &str)> = Vec::new();
                    let matched = 'dissect: {
                        let Some(pos) = remaining.find("@") else {
                            break 'dissect false;
                        };
                        captured.push(("_tmp.session_name_prefix", &remaining[..pos]));
                        remaining = &remaining[pos..];
                        let Some(rest) = remaining.strip_prefix("@") else {
                            break 'dissect false;
                        };
                        remaining = rest;
                        true
                    };
                    if matched {
                        for (path, value) in captured {
                            event.set(path, value)?;
                        }
                    } else {
                        return Err(TransformError::ParseError {
                            path: "_tmp.session_name".into(),
                            message: "dissect pattern did not match".into(),
                        });
                    }
                }
            }

            let _cond = { event.has_value("_tmp.session_name_prefix") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("_tmp.session_name_prefix")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond =
                { !event.has_value("user.email") && event.has_value("_tmp.session_name_prefix") };
            if _cond {
                if let Some(v) = event
                    .get("_tmp.session_name")
                    .filter(|v| !painless_is_empty_value(v))
                    .cloned()
                {
                    event.set("user.email", v)?;
                }
            }

            let _cond = {
                event.get_str("aws.cloudtrail.user_identity.type") == Some("IdentityCenterUser")
                    && !event.has_value("user.id")
            };
            if _cond {
                if event.has("json.userIdentity.onBehalfOf.userId") {
                    event.rename("json.userIdentity.onBehalfOf.userId", "user.id")?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.userIdentity.sessionContext.sessionIssuer.arn",
                    "aws.cloudtrail.user_identity.session_context.session_issuer.arn",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.userIdentity.sessionContext.sessionIssuer.accountId",
                    "aws.cloudtrail.user_identity.session_context.session_issuer.account_id",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has("json.sessionCredentialFromConsole") {
                    event.rename(
                        "json.sessionCredentialFromConsole",
                        "aws.cloudtrail.session_credential_from_console",
                    )?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.userIdentity.invokedBy",
                    "aws.cloudtrail.user_identity.invoked_by",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.eventSource", "event.provider")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("json.eventName")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.action", v)?;
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.eventCategory", "aws.cloudtrail.event_category")?;
                Ok(())
            })();

            let _cond = {
                event.get_str("event.action") == Some("UserAuthentication")
                    && !event.has_value("user.name")
                    && event.has_value("json.additionalEventData.UserName")
            };
            if _cond {
                event.rename("json.additionalEventData.UserName", "user.name")?;
            }

            if let Some(v) = event
                .get("json.awsRegion")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("cloud.region", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.sourceIPAddress", "source.address")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(input) = event.get_string("source.address") {
                    // Grok pattern: ^%{IP:source.ip}$
                    let _ = cached_grok!("^%{IP:source.ip}$").extract_into(&input, event)?;
                }
                Ok(())
            })();

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-City.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_city", &ip_str) {
                        if let Some(v) = geo.get("country_iso_code") {
                            event.set("source.geo.country_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("country_name") {
                            event.set("source.geo.country_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("continent_name") {
                            event.set("source.geo.continent_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_iso_code") {
                            event.set("source.geo.region_iso_code", v.clone())?;
                        }
                        if let Some(v) = geo.get("region_name") {
                            event.set("source.geo.region_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("city_name") {
                            event.set("source.geo.city_name", v.clone())?;
                        }
                        if let Some(v) = geo.get("timezone") {
                            event.set("source.geo.timezone", v.clone())?;
                        }
                        if let Some(v) = geo.get("location") {
                            event.set("source.geo.location", v.clone())?;
                        }
                    }
                }
            }

            if event.has_value("source.ip") {
                if let Some(ip_str) = event.get_string("source.ip") {
                    let ip_str = ip_str.to_string();
                    // GeoIP enrichment (GeoLite2-ASN.mmdb)
                    if let Ok(geo) = geoip_lookup("geoip_asn", &ip_str) {
                        if let Some(v) = geo.get("asn") {
                            event.set("source.as.asn", v.clone())?;
                        }
                        if let Some(v) = geo.get("organization_name") {
                            event.set("source.as.organization_name", v.clone())?;
                        }
                    }
                }
            }

            if event.has("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if let Some(ua_str) = event.get_string("json.userAgent") {
                let ua_str = ua_str.to_string();
                // User agent parsing
                if let Ok(ua) = parse_user_agent(&ua_str) {
                    event.set("user_agent.original", json!(ua_str))?;
                    if let Some(name) = ua.name {
                        event.set("user_agent.name", json!(name))?;
                    }
                    if let Some(version) = ua.version {
                        event.set("user_agent.version", json!(version))?;
                    }
                    if let Some(os_name) = ua.os_name {
                        event.set("user_agent.os.name", json!(os_name))?;
                        if let Some(os_version) = ua.os_version {
                            event.set("user_agent.os.version", json!(os_version))?;
                            event.set(
                                "user_agent.os.full",
                                json!(format!("{} {}", os_name, os_version)),
                            )?;
                        }
                    }
                    if let Some(device) = ua.device {
                        event.set("user_agent.device.name", json!(device))?;
                    }
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.errorCode", "aws.cloudtrail.error_code")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.errorMessage", "aws.cloudtrail.error_message")?;
                Ok(())
            })();

            // Painless script
            // Source: ctx._conf = ctx._conf ?: [:];\nctx._conf.keep_flattened_duplicates = ctx._conf.retain == null ||\n  ctx._conf.retain.contains('all') ||\n  ctx._conf.retain.contains('flattened') ||\n  ctx._conf.retain.contains('minimal');
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"ctx._conf = ctx._conf ?: [:];\nctx._conf.keep_flattened_duplicates = ctx._conf.retain == null ||\n  ctx._conf.retain.contains('all') ||\n  ctx._conf.retain.contains('flattened') ||\n  ctx._conf.retain.contains('minimal');"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail?.flattened == null) {\n  ctx.aws.cloudtrail.flattened = [:];\n}\nif (ctx.json?.requestParameters != null) {\n  ctx.aws.cloudtrail.request_parameters = ctx.json.requestParameters.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.request_parameters.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.request_parameters = ctx.json.requestParameters;\n  }\n}\nif (ctx.json?.responseElements != null) {\n  ctx.aws.cloudtrail.response_elements = ctx.json.responseElements.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.response_elements.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.response_elements = ctx.json.responseElements;\n  }\n}\nif (ctx.json?.additionalEventData != null) {\n  ctx.aws.cloudtrail.additional_eventdata = ctx.json.additionalEventData.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.additional_eventdata.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.additional_eventdata = ctx.json.additionalEventData;\n  }\n}\nif (ctx.json?.serviceEventDetails != null) {\n  ctx.aws.cloudtrail.service_event_details = ctx.json.serviceEventDetails.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.service_event_details.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.service_event_details = ctx.json.serviceEventDetails;\n  }\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail?.flattened == null) {\n  ctx.aws.cloudtrail.flattened = [:];\n}\nif (ctx.json?.requestParameters != null) {\n  ctx.aws.cloudtrail.request_parameters = ctx.json.requestParameters.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.request_parameters.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.request_parameters = ctx.json.requestParameters;\n  }\n}\nif (ctx.json?.responseElements != null) {\n  ctx.aws.cloudtrail.response_elements = ctx.json.responseElements.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.response_elements.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.response_elements = ctx.json.responseElements;\n  }\n}\nif (ctx.json?.additionalEventData != null) {\n  ctx.aws.cloudtrail.additional_eventdata = ctx.json.additionalEventData.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.additional_eventdata.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.additional_eventdata = ctx.json.additionalEventData;\n  }\n}\nif (ctx.json?.serviceEventDetails != null) {\n  ctx.aws.cloudtrail.service_event_details = ctx.json.serviceEventDetails.toString();\n  if (ctx._conf.keep_flattened_duplicates && ctx.aws.cloudtrail.service_event_details.length() < 32766) {\n    ctx.aws.cloudtrail.flattened.service_event_details = ctx.json.serviceEventDetails;\n  }\n}"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.requestID", "aws.cloudtrail.request_id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.eventID", "event.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.eventType", "aws.cloudtrail.event_type")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.apiVersion", "aws.cloudtrail.api_version")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.managementEvent != null) {\n  ctx.aws.cloudtrail.management_event = String.valueOf(ctx.json.managementEvent);\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.managementEvent != null) {\n  ctx.aws.cloudtrail.management_event = String.valueOf(ctx.json.managementEvent);\n}\n"#
                    ),
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.readOnly", "aws.cloudtrail.read_only")?;
                Ok(())
            })();

            // Painless script
            // Source: if (ctx.json?.resources instanceof List) {\n  def resources = ctx.json.resources;\n\n  Map uniqueResources = [:];\n  for (def originalResource : resources) {\n    if (originalResource instanceof Map) {\n      Map resource = [:];\n      resource.putAll(originalResource);\n      if (resource.containsKey('ARN')) {\n        resource.arn = resource.remove('ARN');\n      }\n      if (resource.containsKey('accountId')) {\n        resource.account_id = resource.remove('accountId');\n      }\n      String key = (resource.containsKey('arn') ? resource.arn : '') + '_' +\n                   (resource.containsKey('account_id') ? resource.account_id : '') + '_' +\n                   (resource.containsKey('type') ? resource.type : '');\n      uniqueResources[key] = resource;\n    }\n  }\n  ctx.json.resources = new ArrayList(uniqueResources.values());\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.json?.resources instanceof List) {\n  def resources = ctx.json.resources;\n\n  Map uniqueResources = [:];\n  for (def originalResource : resources) {\n    if (originalResource instanceof Map) {\n      Map resource = [:];\n      resource.putAll(originalResource);\n      if (resource.containsKey('ARN')) {\n        resource.arn = resource.remove('ARN');\n      }\n      if (resource.containsKey('accountId')) {\n        resource.account_id = resource.remove('accountId');\n      }\n      String key = (resource.containsKey('arn') ? resource.arn : '') + '_' +\n                   (resource.containsKey('account_id') ? resource.account_id : '') + '_' +\n                   (resource.containsKey('type') ? resource.type : '');\n      uniqueResources[key] = resource;\n    }\n  }\n  ctx.json.resources = new ArrayList(uniqueResources.values());\n}\n"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.resources", "aws.cloudtrail.resources")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.recipientAccountId",
                    "aws.cloudtrail.recipient_account_id",
                )?;
                Ok(())
            })();

            if let Some(v) = event
                .get("aws.cloudtrail.recipient_account_id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                if !event.has("cloud.account.id") {
                    event.set("cloud.account.id", v)?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.sharedEventId", "aws.cloudtrail.shared_event_id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.vpcEndpointId", "aws.cloudtrail.vpc_endpoint_id")?;
                Ok(())
            })();

            let _cond = { event.has_value("json.requestParameters.userName") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.requestParameters.userName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.requestParameters.newUserName") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.requestParameters.newUserName")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = { event.has_value("json.responseElements.user.userId") };
            if _cond {
                event.append_unique(
                    "related.user",
                    json!(
                        event
                            .get("json.responseElements.user.userId")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // Painless script
                // Source: if (ctx.json?.eventName != 'ConsoleLogin') {\n  return;\n} Map aed_map = [:]; if (ctx.json?.additionalEventData?.MobileVersion != null) {\n  aed_map.mobile_version = ctx.json.additionalEventData.MobileVersion != 'No';\n} if (ctx.json?.additionalEventData?.LoginTo != null) {\n  aed_map.login_to = ctx.json.additionalEventData.LoginTo;\n} if (ctx.json?.additionalEventData?.MFAUsed != null) {\n  aed_map.mfa_used = ctx.json.additionalEventData.MFAUsed != 'No';\n} if (aed_map.size() > 0) {\n  ctx.aws.cloudtrail.console_login = [:];\n  ctx.aws.cloudtrail.console_login.additional_eventdata = aed_map;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"if (ctx.json?.eventName != 'ConsoleLogin') {\n  return;\n} Map aed_map = [:]; if (ctx.json?.additionalEventData?.MobileVersion != null) {\n  aed_map.mobile_version = ctx.json.additionalEventData.MobileVersion != 'No';\n} if (ctx.json?.additionalEventData?.LoginTo != null) {\n  aed_map.login_to = ctx.json.additionalEventData.LoginTo;\n} if (ctx.json?.additionalEventData?.MFAUsed != null) {\n  aed_map.mfa_used = ctx.json.additionalEventData.MFAUsed != 'No';\n} if (aed_map.size() > 0) {\n  ctx.aws.cloudtrail.console_login = [:];\n  ctx.aws.cloudtrail.console_login.additional_eventdata = aed_map;\n}"#
                    ),
                )?;
                Ok(())
            })();

            // Painless script
            // Source: ctx.event.kind = 'event'; ctx.event.type = ['info'];\nif (ctx.aws?.cloudtrail?.error_code != null || ctx.aws?.cloudtrail?.error_message != null) {\n  ctx.event.outcome = 'failure'\n} else {\n  ctx.event.outcome = 'success'\n}\nif (ctx.event?.action == null) {\n  return;\n}\nif (ctx.event.action == 'ConsoleLogin' && ctx.json?.responseElements?.ConsoleLogin != null) {\n  ctx.event.outcome = Processors.lowercase(ctx.json.responseElements.ConsoleLogin);\n}\nif (params.get(ctx.event.action) == null) {\n  return;\n}\ndef hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan_params(
                event,
                cached_painless!(
                    r#"ctx.event.kind = 'event'; ctx.event.type = ['info'];\nif (ctx.aws?.cloudtrail?.error_code != null || ctx.aws?.cloudtrail?.error_message != null) {\n  ctx.event.outcome = 'failure'\n} else {\n  ctx.event.outcome = 'success'\n}\nif (ctx.event?.action == null) {\n  return;\n}\nif (ctx.event.action == 'ConsoleLogin' && ctx.json?.responseElements?.ConsoleLogin != null) {\n  ctx.event.outcome = Processors.lowercase(ctx.json.responseElements.ConsoleLogin);\n}\nif (params.get(ctx.event.action) == null) {\n  return;\n}\ndef hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"AddUserToGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"AssumeRole\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"AttachGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"AttachRolePolicy\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"AttachUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"AuthorizeSecurityGroupIngress\":{\"category\":[\"network\"],\"type\":[\"access\"]},\"ChangePassword\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"ConsoleLogin\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"Converse\":{\"category\":[\"api\"],\"type\":[]},\"CreateAccessKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"CreateBucket\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"CreateControlChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"CreateDataChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"CreateDocument\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"CreateGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"CreateKeyPair\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"creation\"]},\"CreatePolicy\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"CreateTopic\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"CreateUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"CreateVirtualMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeactivateMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteAccessKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteBucket\":{\"category\":[\"file\"],\"type\":[\"deletion\"]},\"DeleteGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"DeleteGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"DeleteObject\":{\"category\":[\"file\"],\"type\":[\"delete\"]},\"DeleteSSHPublicKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteTrail\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"DeleteUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"DeleteUserPermissionsBoundary\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteVirtualMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DescribeDBInstances\":{\"category\":[\"database\"],\"type\":[\"info\"]},\"DescribeInstances\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"DescribeLoadBalancers\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeNetworkAcls\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeNetworkInterfaces\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeRegions\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"DescribeSecurityGroups\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeTrails\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"DescribeVolumes\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"DescribeVpcs\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DetachGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"DetachUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"EnableMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"GetCallerIdentity\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"GetGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"GetGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"GetObject\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"GetPolicy\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"GetUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"GetUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"HeadObject\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"ListAttachedGroupPolicies\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"ListAttachedRolePolicies\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"ListAttachedUserPolicies\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListBuckets\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"ListFunctions\":{\"category\":[\"package\"],\"type\":[\"info\"]},\"ListGroupPolicies\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"ListGroups\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"ListGroupsForUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListInstanceAssociations\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"ListObjects\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"ListRoles\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"ListTables\":{\"category\":[\"database\"],\"type\":[\"info\"]},\"ListUserPolicies\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListUserTags\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListUsers\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"OpenControlChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"OpenDataChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"Publish\":{\"category\":[\"api\"],\"type\":[]},\"PutGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"PutObject\":{\"category\":[\"file\"],\"type\":[\"change\"]},\"PutUserPermissionsBoundary\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"PutUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"RemoveUserFromGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"SendCommand\":{\"category\":[\"process\"],\"type\":[]},\"SetDefaultPolicyVersion\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"SetSecurityTokenServicePreferences\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"StartSession\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"Subscribe\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"TagUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"TerminateSession\":{\"category\":[\"session\"],\"type\":[\"end\"]},\"UntagUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateAccessKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateAccountPasswordPolicy\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"UpdateGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"UpdateLoginProfile\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateRole\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"UpdateSSHPublicKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]}}"
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.awsAccountId", "cloud.account.id")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.digestS3Object", "file.path")?;
                Ok(())
            })();

            let _cond = {
                event.has_value("json.previousDigestHashAlgorithm")
                    && event.get_str("json.previousDigestHashAlgorithm") == Some("SHA-256")
            };
            if _cond {
                event.rename("json.previousDigestSignature", "file.hash.sha256")?;
            }

            let _cond = { event.has_value("file.hash.sha256") };
            if _cond {
                event.append(
                    "related.hash",
                    json!(
                        event
                            .get("file.hash.sha256")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.logFiles", "aws.cloudtrail.digest.log_files")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.digestStartTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.cloudtrail.digest.start_time", parsed)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.digestEndTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("@timestamp", parsed)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.digestEndTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.cloudtrail.digest.end_time", parsed)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename("json.digestS3Bucket", "aws.cloudtrail.digest.s3_bucket")?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.newestEventTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.cloudtrail.digest.newest_event_time", parsed)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.oldestEventTime") {
                    if let Some(parsed) = parse_date_out(&date_str, &["ISO8601"], None, None) {
                        event.set("aws.cloudtrail.digest.oldest_event_time", parsed)?;
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.previousDigestS3Bucket",
                    "aws.cloudtrail.digest.previous_s3_bucket",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.previousDigestHashAlgorithm",
                    "aws.cloudtrail.digest.previous_hash_algorithm",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.publicKeyFingerprint",
                    "aws.cloudtrail.digest.public_key_fingerprint",
                )?;
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                event.rename(
                    "json.digestSignatureAlgorithm",
                    "aws.cloudtrail.digest.signature_algorithm",
                )?;
                Ok(())
            })();

            if let Some(v) = event
                .get("json.responseElements.group.groupId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.id", v)?;
            }

            if let Some(v) = event
                .get("json.responseElements.user.userId")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.id", v)?;
            }

            if let Some(v) = event
                .get("json.requestParameters.newUserName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.changes.name", v)?;
            }

            if let Some(v) = event
                .get("json.requestParameters.groupName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("group.name", v)?;
            }

            if let Some(v) = event
                .get("json.requestParameters.userName")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.target.name", v)?;
            }

            let _cond = { !(event.get_bool("_conf.keep_flattened_duplicates") == Some(true)) };
            if _cond {
                event.remove("aws.cloudtrail.digest");
                event.remove("json.insightDetails");
            }

            if event.has("aws.cloudtrail.digest") {
                event.rename("aws.cloudtrail.digest", "aws.cloudtrail.flattened.digest")?;
            }

            if event.has("json.insightDetails") {
                event.rename(
                    "json.insightDetails",
                    "aws.cloudtrail.flattened.insight_details",
                )?;
            }

            let _cond = { event.get_str("json.tlsDetails.tlsVersion") == Some("tlsVersion") };
            if _cond {
                if event.remove("json.tlsDetails").is_none() {
                    return Err(TransformError::FieldNotFound {
                        path: "json.tlsDetails".into(),
                    });
                }
            }

            // on_failure: 1 handler(s)
            if let Err(err) = (|| -> Result<()> {
                if event.has_value("json.tlsDetails.tlsVersion") {
                    if let Some(input) = event.get_string("json.tlsDetails.tlsVersion") {
                        let mut remaining: &str = &input;
                        let mut captured: Vec<(&str, &str)> = Vec::new();
                        let matched = 'dissect: {
                            let Some(pos) = remaining.find("v") else {
                                break 'dissect false;
                            };
                            captured.push(("tls.version_protocol", &remaining[..pos]));
                            remaining = &remaining[pos..];
                            let Some(rest) = remaining.strip_prefix("v") else {
                                break 'dissect false;
                            };
                            remaining = rest;
                            captured.push(("tls.version", remaining));
                            true
                        };
                        if matched {
                            for (path, value) in captured {
                                event.set(path, value)?;
                            }
                        }
                    }
                }
                Ok(())
            })() {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("_ingest.on_failure_processor_type", "dissect")?;
                event.set(
                    "_ingest.on_failure_processor_tag",
                    "dissect_json_tlsDetails_tlsVersion_7a65d5ad",
                )?;
                event.rename("json.tlsDetails.tlsVersion", "tls.version")?;
                event.remove("_ingest.on_failure_message");
                event.remove("_ingest.on_failure_processor_type");
                event.remove("_ingest.on_failure_processor_tag");
                if event.get_object("_ingest").is_some_and(|m| m.is_empty()) {
                    event.remove("_ingest");
                }
            }

            if event.has_value("tls.version_protocol") {
                if let Some(s) = event.get_string("tls.version_protocol") {
                    let lowered = s.to_lowercase();
                    event.set("tls.version_protocol", lowered)?;
                }
            }

            if event.has("json.tlsDetails.cipherSuite") {
                event.rename("json.tlsDetails.cipherSuite", "tls.cipher")?;
            }

            if event.has("json.tlsDetails.clientProvidedHostHeader") {
                event.rename(
                    "json.tlsDetails.clientProvidedHostHeader",
                    "tls.client.server_name",
                )?;
            }

            let _cond = {
                !event.has_value("user.email")
                    && event.has_value("user.name")
                    && event
                        .get_str("user.name")
                        .map(|s| s.find("@").map(|b| s[..b].chars().count()))
                        .is_some_and(|i| i.is_some_and(|i| i > 0))
            };
            if _cond {
                if let Some(v) = event.get("user.name").cloned() {
                    event.set("user.email", v)?;
                }
            }

            let _cond = { event.has_value("user.id") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.id")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = { event.has_value("user.changes.name") };
            if _cond {
                // ignore_failure: true
                let _ = (|| -> Result<()> {
                    event.append_unique(
                        "related.user",
                        json!(
                            event
                                .get("user.changes.name")
                                .map_or_else(String::new, template_to_string)
                        ),
                    )?;
                    Ok(())
                })();
            }

            let _cond = {
                event.get_str("_conf.retain") == Some("minimal")
                    && event.has_value("aws.cloudtrail.flattened")
            };
            if _cond {
                // Painless script
                // Source: def set(Map root, String path, def v) {\n  String[] elems = path.splitOnToken('.');\n  def dst = root;\n  for (int i = 0; i < elems.length-1; i++) {\n    dst = dst.computeIfAbsent(elems[i], _ -> [:]);\n  }\n  dst[elems[elems.length-1]] = v;\n}\nMap flattened = [:];\nint prefix = \"aws.cloudtrail.flattened.\".length();\nfor (String f: params.required_flattened_fields) {\n  def v = $(f, null);\n  if (v == null) {\n    continue;\n  }\n  set(flattened, f.substring(prefix), v);\n}\nctx.aws.cloudtrail.flattened = flattened;
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def set(Map root, String path, def v) {\n  String[] elems = path.splitOnToken('.');\n  def dst = root;\n  for (int i = 0; i < elems.length-1; i++) {\n    dst = dst.computeIfAbsent(elems[i], _ -> [:]);\n  }\n  dst[elems[elems.length-1]] = v;\n}\nMap flattened = [:];\nint prefix = \"aws.cloudtrail.flattened.\".length();\nfor (String f: params.required_flattened_fields) {\n  def v = $(f, null);\n  if (v == null) {\n    continue;\n  }\n  set(flattened, f.substring(prefix), v);\n}\nctx.aws.cloudtrail.flattened = flattened;"#
                    ),
                    cached_params!(
                        "{\"required_flattened_fields\":[\"aws.cloudtrail.flattened.additional_eventdata.SSEApplied\",\"aws.cloudtrail.flattened.request_parameters.cidrIp\",\"aws.cloudtrail.flattened.request_parameters.dryRun\",\"aws.cloudtrail.flattened.request_parameters.fromPort\",\"aws.cloudtrail.flattened.request_parameters.includeDeprecated\",\"aws.cloudtrail.flattened.request_parameters.policyArn\",\"aws.cloudtrail.flattened.request_parameters.serialNumber\",\"aws.cloudtrail.flattened.request_parameters.withDecryption\",\"aws.cloudtrail.flattened.request_parameters.x-amz-server-side-encryption-customer-algorithm\"]}"
                    ),
                )?;
            }

            let _cond = {
                event.has_value("_conf.retain")
                    && event.get_str("_conf.retain") != Some("")
                    && event.get_str("_conf.retain") != Some("all")
                    && event.get_str("_conf.retain") != Some("keyword")
                    && event.get_str("_conf.retain") != Some("minimal")
            };
            if _cond {
                event.remove("aws.cloudtrail.response_elements");
                event.remove("aws.cloudtrail.request_parameters");
                event.remove("aws.cloudtrail.additional_eventdata");
            }

            event.remove("json");
            event.remove("_conf");
            event.remove("_tmp");

            // Painless script
            // Source: void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"void handleMap(Map map) {\n  map.values().removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nvoid handleList(List list) {\n  list.removeIf(v -> {\n    if (v instanceof Map) {\n        handleMap(v);\n    } else if (v instanceof List) {\n        handleList(v);\n    }\n    return v == null || v == '' || (v instanceof Map && v.size() == 0) || (v instanceof List && v.size() == 0)\n  });\n}\nhandleMap(ctx);\n"#
                ),
            )?;

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append("error.message", json!(format!("Processor '{}' {}with tag '{}' {}in pipeline '{}' failed with message '{}'", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("#_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("/_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.remove("json");
                event.remove("_conf");
                event.remove("_tmp");
                event.remove("_ingest.on_failure_message");
            }
        }

        // --- Post-processing (codegen-emitted) ---
        // Dedup related.* arrays (same value can be appended multiple times)
        if let Some(Value::Array(mut arr)) = event.get("related.ip").cloned() {
            dedup_array(&mut arr);
            event.set("related.ip", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.user").cloned() {
            dedup_array(&mut arr);
            event.set("related.user", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hash").cloned() {
            dedup_array(&mut arr);
            event.set("related.hash", Value::Array(arr))?;
        }
        if let Some(Value::Array(mut arr)) = event.get("related.hosts").cloned() {
            dedup_array(&mut arr);
            event.set("related.hosts", Value::Array(arr))?;
        }
        Ok(TransformResult::Continue)
    }
}
