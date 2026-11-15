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

            let _cond = { event.has_value("@timestamp") };
            if _cond {
                if let Some(v) = event.get("@timestamp").cloned() {
                    event.set("event.created", v)?;
                }
            }

            parse_json_field(event, "event.original", "json")?;

            event.set("ecs.version", json!("8.11.0"))?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.eventTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.eventTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    sentinels: vec!["-".into(), "none".into()],
                    ..DropPolicy::none()
                },
                None,
            );

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
                    // The pipeline's script, run by the runtime's equivalent
                    painless_exec_plan_params(
                        event,
                        compiled_painless!(
                            "c5ebe0676e6d13bca9205d0df59eb52f3e0dc23dfc285c3aa1a7e72446059465",
                            r#"{"params":{"AwsEntity":{"enrich":[{"source":"cloudformation.amazonaws.com","body":[{"IfEvent":{"names":["CreateStackSet"],"then":[{"AddField":["Target","json.responseElements.stackSetId"]}],"otherwise":[{"IfEvent":{"names":["CreateStack"],"then":[{"AddField":["Target","json.responseElements.stackId"]}],"otherwise":[]}}]}}]},{"source":"cloudtrail.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.name","json.requestParameters.s3BucketName","json.responseElements.cloudWatchLogsLogGroupArn","json.responseElements.cloudWatchLogsRoleArn","json.responseElements.kmsKeyId","json.responseElements.snsTopicARN","json.responseElements.trailARN","json.responseElements.name"]]},{"IfEvent":{"names":["DeleteTrail","StopLogging"],"then":[{"AddField":["Target","json.requestParameters.name"]}],"otherwise":[{"IfEvent":{"names":["DescribeTrails"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]},{"source":"ec2-instance-connect.amazonaws.com","body":[{"IfEvent":{"names":["SendSSHPublicKey","SendSerialConsoleSSHPublicKey"],"then":[{"AddField":["Target","json.requestParameters.instanceId"]}],"otherwise":[]}}]},{"source":"config.amazonaws.com","body":[{"AddField":["Related","json.requestParameters.configurationRecorderName"]},{"IfEvent":{"names":["StopConfigurationRecorder","StartConfigurationRecorder"],"then":[{"AddField":["Target","json.requestParameters.configurationRecorderName"]}],"otherwise":[]}}]},{"source":"ec2.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.groupId","json.requestParameters.groupName","json.requestParameters.roleName","json.requestParameters.subnetId","json.requestParameters.volumeId","json.requestParameters.networkInterfaceId","json.requestParameters.vpcId","json.requestParameters.snapshotId","json.responseElements.groupId","json.responseElements.reservationId","json.responseElements.vpc.vpcId","json.responseElements.vpc.dhcpOptionsId","json.responseElements.snapshotId","json.responseElements.volumeId"]]},{"ForEach":{"var":"i","source":{"Event":"json.responseElements.securityGroupRuleSet.items"},"body":[{"AddMembers":["Related",["groupId","referencedGroupInfo.groupId","securityGroupRuleId"]]}]}},{"ForEach":{"var":"i","source":{"Event":"json.responseElements.groupSet.items"},"body":[{"AddMember":["Related","groupId"]}]}},{"ForEach":{"var":"i","source":{"Event":"json.requestParameters.groupSet.items"},"body":[{"AddMember":["Related","groupId"]}]}},{"ForEach":{"var":"i","source":{"Event":"json.requestParameters.instancesSet.items"},"body":[{"AddMember":["Related","instanceId"]}]}},{"ForEach":{"var":"instances","source":{"Event":"json.responseElements.instancesSet.items"},"body":[{"AddMembers":["Related",["subnetId","vpcId","instanceId","imageId","iamInstanceProfile.arn"]]},{"ForEach":{"var":"networks","source":{"Member":"networkInterfaceSet.items"},"body":[{"AddMembers":["Related",["networkInterfaceId","vpcId","subnetId"]]},{"ForEach":{"var":"group","source":{"Member":"groupSet.items"},"body":[{"AddMember":["Related","groupId"]}]}}]}}]}},{"ForEach":{"var":"i","source":{"Event":"json.requestParameters.revokedSecurityGroupRuleSet.items"},"body":[{"AddMembers":["Related",["securityGroupRuleId","groupId"]]}]}},{"IfEvent":{"names":["AuthorizeSecurityGroupIngress","AuthorizeSecurityGroupEgress"],"then":[{"AddField":["Target","json.requestParameters.groupId"]},{"ForEach":{"var":"f","source":{"Event":"json.responseElements.securityGroupRuleSet.items"},"body":[{"AddMember":["Target","securityGroupRuleId"]}]}}],"otherwise":[{"IfEvent":{"names":["CreateTrafficMirrorFilter"],"then":[{"AddField":["Target","json.responseElements.CreateTrafficMirrorFilterResponse.trafficMirrorFilter.trafficMirrorFilterId"]}],"otherwise":[{"IfEvent":{"names":["CreateTrafficMirrorFilterRule"],"then":[{"AddField":["Target","json.responseElements.CreateTrafficMirrorFilterRuleResponse.trafficMirrorFilterRule.trafficMirrorFilterRuleId"]}],"otherwise":[{"IfEvent":{"names":["CreateTrafficMirrorSession"],"then":[{"AddField":["Target","json.responseElements.CreateTrafficMirrorSessionResponse.trafficMirrorSession.trafficMirrorSessionId"]}],"otherwise":[{"IfEvent":{"names":["CreateTrafficMirrorTarget"],"then":[{"AddField":["Target","json.responseElements.CreateTrafficMirrorTargetResponse.trafficMirrorTarget.trafficMirrorTargetId"]}],"otherwise":[{"IfEvent":{"names":["DeleteFlowLogs"],"then":[{"AddField":["Target","json.requestParameters.DeleteFlowLogsRequest.FlowLogId.content"]}],"otherwise":[{"IfEvent":{"names":["DeleteNetworkAcl"],"then":[{"AddField":["Target","json.requestParameters.networkAclId"]}],"otherwise":[{"IfEvent":{"names":["DeleteNetworkAclEntry"],"then":[{"AddField":["Target","json.requestParameters.networkAclId"]},{"IfPresent":{"path":"json.requestParameters.ruleNumber","then":[{"AddFieldAsString":["Target","json.requestParameters.ruleNumber"]}]}}],"otherwise":[{"IfEvent":{"names":["GetPasswordData"],"then":[{"AddField":["Target","json.requestParameters.instanceId"]}],"otherwise":[{"IfEvent":{"names":["ModifyImageAttribute"],"then":[{"AddField":["Target","json.requestParameters.imageId"]}],"otherwise":[{"IfEvent":{"names":["ModifySnapshotAttribute"],"then":[{"AddField":["Target","json.requestParameters.snapshotId"]}],"otherwise":[{"IfEvent":{"names":["DescribeSecurityGroups","DescribeNetworkInterfaces","DescribeRegions","DescribeVpcs","DescribeNetworkAcls","DescribeVolumes"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]},{"source":"elasticfilesystem.amazonaws.com","body":[{"IfEvent":{"names":["DeleteFileSystem"],"then":[{"AddField":["Target","json.requestParameters.fileSystemId"]}],"otherwise":[{"IfEvent":{"names":["DeleteMountTarget"],"then":[{"AddField":["Target","json.requestParameters.mountTargetId"]}],"otherwise":[]}}]}}]},{"source":"events.amazonaws.com","body":[{"IfEvent":{"names":["DeleteRule"],"then":[{"AddField":["Target","json.requestParameters.name"]}],"otherwise":[]}}]},{"source":"guardduty.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.detectorId","json.responseElements.detectorId"]]},{"IfEvent":{"names":["CreateDetector"],"then":[{"AddField":["Target","json.responseElements.detectorId"]}],"otherwise":[{"IfEvent":{"names":["DeleteDetector"],"then":[{"AddField":["Target","json.requestParameters.detectorId"]}],"otherwise":[]}}]}}]},{"source":"iam.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.userName","json.requestParameters.serialNumber","json.requestParameters.accessKeyId","json.requestParameters.policyArn","json.requestParameters.roleName","json.requestParameters.policyName","json.requestParameters.serialNumber","json.responseElements.accessKey.userName","json.responseElements.accessKey.accessKeyId","json.responseElements.user.arn","json.responseElements.user.userId","json.responseElements.user.userName","json.responseElements.userId","json.responseElements.role.arn","json.responseElements.serialNumber"]]},{"IfEvent":{"names":["AttachGroupPolicy"],"then":[{"AddField":["Target","json.requestParameters.groupName"]}],"otherwise":[{"IfEvent":{"names":["AttachRolePolicy","ListAttachedRolePolicies","UpdateAssumeRolePolicy"],"then":[{"AddField":["Target","json.requestParameters.roleName"]}],"otherwise":[{"IfEvent":{"names":["AttachUserPolicy"],"then":[{"AddFields":["Target",["json.requestParameters.policyArn","json.requestParameters.userName"]]}],"otherwise":[{"IfEvent":{"names":["CreateAccessKey"],"then":[{"AddFields":["Target",["json.responseElements.accessKey.accessKeyId","json.requestParameters.userName"]]}],"otherwise":[{"IfEvent":{"names":["CreateUser","DeactivateMFADevice"],"then":[{"AddField":["Target","json.requestParameters.userName"]}],"otherwise":[{"IfEvent":{"names":["DeleteVirtualMFADevice"],"then":[{"AddField":["Target","json.requestParameters.serialNumber"]}],"otherwise":[{"IfEvent":{"names":["GetPolicy"],"then":[{"AddField":["Target","json.requestParameters.policyArn"]}],"otherwise":[{"IfEvent":{"names":["CreatePolicy"],"then":[{"AddField":["Target","json.responseElements.policy.arn"]}],"otherwise":[{"IfEvent":{"names":["ListRoles","ListUsers"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]}}]}}]}}]}}]}}]}}]}}]},{"source":"kms.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.keyId","json.responseElements.keyId","json.responseElements.keyMetadata.arn","json.responseElements.keyMetadata.keyId"]]},{"IfEvent":{"names":["DisableKey","ScheduleKeyDeletion"],"then":[{"ForEach":{"var":"f","source":{"Event":"json.resources"},"body":[{"AddMember":["Target","ARN"]}]}}],"otherwise":[]}}]},{"source":"lambda.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.functionName","json.responseElements.functionArn","json.responseElements.functionName","json.responseElements.role"]]},{"AddArray":["Related","json.responseElements.vpcConfig.securityGroupIds"]},{"AddArray":["Related","json.responseElements.vpcConfig.subnetIds"]},{"IfEventContains":{"needles":["AddPermission"],"then":[{"AddField":["Target","json.requestParameters.functionName"]}],"otherwise":[{"IfEventContains":{"needles":["ListFunctions"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]},{"source":"logs.amazonaws.com","body":[{"IfEvent":{"names":["DeleteLogGroup"],"then":[{"AddField":["Target","json.requestParameters.logGroupName"]}],"otherwise":[{"IfEvent":{"names":["DeleteLogStream"],"then":[{"AddField":["Target","json.requestParameters.logStreamName"]}],"otherwise":[]}}]}}]},{"source":"monitoring.amazonaws.com","body":[{"IfEvent":{"names":["DeleteAlarms"],"then":[{"ForEach":{"var":"f","source":{"Event":"json.requestParameters.alarmNames"},"body":[{"AddMember":["Target",""]}]}}],"otherwise":[]}}]},{"source":"rds.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.dBInstanceIdentifier","json.requestParameters.dBInstanceArn","json.responseElements.dBInstanceIdentifier","json.responseElements.dbInstanceArn","json.responseElements.dBSubnetGroup.vpcId","json.responseElements.vpcSecurityGroups.vpcSecurityGroupId"]]},{"ForEach":{"var":"i","source":{"Event":"json.responseElements.dBSubnetGroup.subnets"},"body":[{"AddMember":["Related","subnetIdentifier"]}]}},{"ForEach":{"var":"i","source":{"Event":"json.responseElements.vpcSecurityGroups"},"body":[{"AddMember":["Related","vpcSecurityGroupId"]}]}},{"IfEvent":{"names":["DeleteDBCluster","ModifyDBCluster","StopDBCluster"],"then":[{"AddField":["Target","json.responseElements.dBClusterArn"]}],"otherwise":[{"IfEvent":{"names":["DeleteDBInstance","ModifyDBInstance","RestoreDBInstanceFromDBSnapshot","RestoreDBInstanceFromS3","StopDBInstance"],"then":[{"AddField":["Target","json.responseElements.dBInstanceArn"]}],"otherwise":[{"IfEvent":{"names":["DeleteGlobalCluster"],"then":[{"AddField":["Target","json.responseElements.globalClusterArn"]}],"otherwise":[{"IfEvent":{"names":["ModifyDBClusterSnapshotAttribute"],"then":[{"AddField":["Target","json.responseElements.dBClusterSnapshotIdentifier"]}],"otherwise":[{"IfEvent":{"names":["ModifyDBSnapshotAttribute"],"then":[{"AddField":["Target","json.responseElements.dBSnapshotIdentifier"]}],"otherwise":[{"IfEvent":{"names":["DescribeDBInstances"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]}}]}}]}}]}}]},{"source":"rolesanywhere.amazonaws.com","body":[{"IfEvent":{"names":["CreateTrustAnchor"],"then":[{"AddField":["Target","json.responseElements.trustAnchor.trustAnchorArn"]}],"otherwise":[]}}]},{"source":"route53resolver.amazonaws.com","body":[{"IfEvent":{"names":["DeleteResolverQueryLogConfig"],"then":[{"AddField":["Target","json.responseElements.resolverQueryLogConfig.arn"]}],"otherwise":[]}}]},{"source":"s3.amazonaws.com","body":[{"AddField":["Related","json.requestParameters.bucketName"]},{"IfEvent":{"names":["CopyObject","PutBucketLogging","PutBucketVersioning","PutObject","GetBucketPolicy","ListObjects","HeadObject","GetObject","DeleteObject","DeleteBucket"],"then":[{"ForEach":{"var":"f","source":{"Event":"json.resources"},"body":[{"AddMember":["Target","ARN"]}]}}],"otherwise":[{"IfEvent":{"names":["PutBucketReplication"],"then":[{"ForEach":{"var":"f","source":{"Event":"json.resources"},"body":[{"AddMember":["Target","ARN"]}]}},{"AddField":["Target","json.requestParameters.ReplicationConfiguration.Rule.Destination.Bucket"]}],"otherwise":[{"IfEvent":{"names":["ListBuckets"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]}}]},{"source":"secretsmanager.amazonaws.com","body":[{"IfEvent":{"names":["BatchGetSecretValue"],"then":[{"ForEach":{"var":"f","source":{"Event":"json.requestParameters.secretIdList"},"body":[{"AddMember":["Target",""]}]}}],"otherwise":[{"IfEvent":{"names":["GetSecretValue"],"then":[{"AddField":["Target","json.requestParameters.secretId"]}],"otherwise":[]}}]}}]},{"source":"signin.amazonaws.com","body":[{"IfEvent":{"names":["ConsoleLogin"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]},{"source":"ssm.amazonaws.com","body":[{"IfEvent":{"names":["GetParameter","GetParameters","CreateControlChannel","OpenControlChannel"],"then":[{"ForEach":{"var":"f","source":{"Event":"json.resources"},"body":[{"AddMember":["Target","ARN"]}]}}],"otherwise":[{"IfEvent":{"names":["StartSession"],"then":[{"AddField":["Target","json.requestParameters.target"]}],"otherwise":[{"IfEvent":{"names":["CreateDocument"],"then":[{"AddField":["Target","json.requestParameters.name"]}],"otherwise":[{"IfEvent":{"names":["TerminateSession","OpenDataChannel"],"then":[{"AddField":["Target","json.requestParameters.sessionId"]}],"otherwise":[{"IfEvent":{"names":["SendCommand"],"then":["SendCommand"],"otherwise":[{"IfEvent":{"names":["ListInstanceAssociations"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}}]}}]}}]}}]}}]},{"source":"sts.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.roleArn","json.sourceIdentity","json.additionalEventData.MFAIdentifier","json.responseElements.assumedRoleUser.arn","json.requestParameters.roleSessionName","json.responseElements.accessKeyId"]]},{"IfEvent":{"names":["AssumeRole"],"then":[{"IfRead":{"path":"json.userIdentity.type","values":["AWSService"],"then":[{"SetActor":"json.userIdentity.invokedBy"}],"otherwise":[{"IfRead":{"path":"json.userIdentity.type","values":["AssumedRole"],"then":[{"SetActor":"json.userIdentity.sessionContext.sessionIssuer.arn"}],"otherwise":[{"SetActor":"json.userIdentity.arn"}]}}]}},{"AddField":["Target","json.requestParameters.roleArn"]}],"otherwise":[{"IfEvent":{"names":["GetCallerIdentity"],"then":[{"AddField":["Target","json.recipientAccountId"]},{"SetActor":"json.userIdentity.arn"}],"otherwise":[]}}]}}]},{"source":"wafv2.amazonaws.com","body":[{"AddFields":["Related",["json.requestParameters.id","json.responseElements.summary"]]},{"IfEvent":{"names":["DeleteRuleGroup","DeleteWebACL"],"then":[{"AddField":["Target","json.requestParameters.id"]}],"otherwise":[]}}]},{"source":"sns.amazonaws.com","body":[{"IfEvent":{"names":["CreateTopic"],"then":[{"AddField":["Target","json.responseElements.topicArn"]}],"otherwise":[{"IfEvent":{"names":["Subscribe","Publish"],"then":[{"AddField":["Target","json.requestParameters.topicArn"]}],"otherwise":[]}}]}}]},{"source":"bedrock.amazonaws.com","body":[{"IfEvent":{"names":["Converse"],"then":[{"AddField":["Target","json.requestParameters.modelId"]}],"otherwise":[]}}]},{"source":"elasticloadbalancing.amazonaws.com","body":[{"IfEvent":{"names":["DescribeLoadBalancers"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]},{"source":"dynamodb.amazonaws.com","body":[{"IfEvent":{"names":["ListTables"],"then":[{"AddField":["Target","json.recipientAccountId"]}],"otherwise":[]}}]}],"preamble":[{"SetActor":"json.userIdentity.arn"},{"SetActorIfAbsent":"json.userIdentity.onBehalfOf.userId"},{"AddFields":["Related",["json.userIdentity.accessKeyId","json.userIdentity.arn","json.userIdentity.userName","json.userIdentity.sessionContext.sessionIssuer.arn","json.userIdentity.sessionContext.sessionIssuer.userName"]]},{"IfRead":{"path":"json.userIdentity.type","values":["IdentityCenterUser"],"then":[{"AddField":["Related","json.userIdentity.onBehalfOf.identityStoreArn"]}],"otherwise":[]}},{"ForEach":{"var":"f","source":{"Event":"json.resources"},"body":[{"AddMember":["Related","ARN"]}]}}]}},"known":["CollectingLadder"]}"#,
                            r#"void addFields(Set entities, List fields) {\n  for (String field : fields) {\n    addField(entities, field);\n  }\n}\n\nvoid addField(Set entities, String fieldName) {\n  addValue(entities, $(fieldName, null));\n}\n\nboolean addValues(Set entities, List values) {\n  boolean addedAll = true;\n  for (String value : values) {\n    addedAll = addedAll && addValue(entities, value);\n  }\n  return addedAll;\n}\n\nboolean addValue(Set entities, String value) {\n  if (value == null || value == \"\") {\n    return false;\n  }\n  return entities.add(value);\n}\n\nvoid enrichCloudformation(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"cloudformation.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateStackSet\") {\n    addField(enrichCtx.target, \"json.responseElements.stackSetId\");\n  } else if (eventName == \"CreateStack\") {\n    addField(enrichCtx.target, \"json.responseElements.stackId\");\n  }\n}\n\nvoid enrichCloudtrail(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"cloudtrail.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.name\",\n    \"json.requestParameters.s3BucketName\",\n    \"json.responseElements.cloudWatchLogsLogGroupArn\",\n    \"json.responseElements.cloudWatchLogsRoleArn\",\n    \"json.responseElements.kmsKeyId\",\n    \"json.responseElements.snsTopicARN\",\n    \"json.responseElements.trailARN\",\n    \"json.responseElements.name\"\n  ]);\n\n  if (eventName == \"DeleteTrail\"\n    || eventName == \"StopLogging\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n  } else if (eventName == \"DescribeTrails\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichEc2InstanceConnect(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ec2-instance-connect.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"SendSSHPublicKey\"\n    || eventName == \"SendSerialConsoleSSHPublicKey\") {\n    addField(enrichCtx.target, \"json.requestParameters.instanceId\");\n  }\n}\n\nvoid enrichConfig(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"config.amazonaws.com\") {\n    return;\n  }\n\n  addField(enrichCtx.related, \"json.requestParameters.configurationRecorderName\");\n\n  if (eventName == \"StopConfigurationRecorder\"\n          || eventName == \"StartConfigurationRecorder\") {\n    addField(enrichCtx.target, \"json.requestParameters.configurationRecorderName\");\n  }\n}\n\nvoid enrichEc2(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ec2.amazonaws.com\") {\n      return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.groupId\",\n    \"json.requestParameters.groupName\",\n    \"json.requestParameters.roleName\",\n    \"json.requestParameters.subnetId\",\n    \"json.requestParameters.volumeId\",\n    \"json.requestParameters.networkInterfaceId\",\n    \"json.requestParameters.vpcId\",\n    \"json.requestParameters.snapshotId\",\n    \"json.responseElements.groupId\",\n    \"json.responseElements.reservationId\",\n    \"json.responseElements.vpc.vpcId\",\n    \"json.responseElements.vpc.dhcpOptionsId\",\n    \"json.responseElements.snapshotId\",\n    \"json.responseElements.volumeId\"\n  ]);\n\n  $(\"json.responseElements.securityGroupRuleSet.items\", []).stream().forEach(i -> {\n    addValues(enrichCtx.related, [\n      i.groupId,\n      i.referencedGroupInfo?.groupId,\n      i.securityGroupRuleId\n    ]);\n  });\n\n  $(\"json.responseElements.groupSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.groupId);\n  });\n\n  $(\"json.requestParameters.groupSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.groupId);\n  });\n\n  $(\"json.requestParameters.instancesSet.items\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.instanceId);\n  });\n\n  $(\"json.responseElements.instancesSet.items\", []).stream().forEach(instances -> {\n    addValues(enrichCtx.related, [\n      instances.subnetId,\n      instances.vpcId,\n      instances.instanceId,\n      instances.imageId,\n      instances.iamInstanceProfile?.arn\n    ]);\n\n    if (instances.networkInterfaceSet?.items == null) {\n      return true;\n    }\n\n    instances.networkInterfaceSet?.items?.stream().forEach(networks -> {\n      addValues(enrichCtx.related, [\n        networks.networkInterfaceId,\n        networks.vpcId,\n        networks.subnetId\n      ]);\n\n      if (networks.groupSet?.items == null) {\n        return true;\n      }\n\n      networks.groupSet?.items?.stream().forEach(group -> {\n        addValue(enrichCtx.related, group.groupId);\n      });\n    });\n  });\n\n  $(\"json.requestParameters.revokedSecurityGroupRuleSet.items\", []).stream().forEach(i -> {\n    addValues(enrichCtx.related, [\n      i.securityGroupRuleId,\n      i.groupId\n    ]);\n  });\n\n  if (eventName == \"AuthorizeSecurityGroupIngress\"\n      || eventName == \"AuthorizeSecurityGroupEgress\") {\n    addField(enrichCtx.target, \"json.requestParameters.groupId\");\n    $(\"json.responseElements.securityGroupRuleSet.items\", []).stream().forEach(f -> addValue(enrichCtx.target, f.securityGroupRuleId));\n\n  } else if (eventName == \"CreateTrafficMirrorFilter\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorFilterResponse.trafficMirrorFilter.trafficMirrorFilterId\");\n\n  } else if (eventName == \"CreateTrafficMirrorFilterRule\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorFilterRuleResponse.trafficMirrorFilterRule.trafficMirrorFilterRuleId\");\n\n  } else if (eventName == \"CreateTrafficMirrorSession\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorSessionResponse.trafficMirrorSession.trafficMirrorSessionId\");\n\n  } else if (eventName == \"CreateTrafficMirrorTarget\") {\n    addField(enrichCtx.target, \"json.responseElements.CreateTrafficMirrorTargetResponse.trafficMirrorTarget.trafficMirrorTargetId\");\n\n  } else if (eventName == \"DeleteFlowLogs\") {\n    addField(enrichCtx.target, \"json.requestParameters.DeleteFlowLogsRequest.FlowLogId.content\");\n\n  } else if (eventName == \"DeleteNetworkAcl\") {\n    addField(enrichCtx.target, \"json.requestParameters.networkAclId\");\n\n  } else if (eventName == \"DeleteNetworkAclEntry\") {\n    addField(enrichCtx.target, \"json.requestParameters.networkAclId\");\n    def ruleNumber = $(\"json.requestParameters.ruleNumber\", null);\n    if (ruleNumber != null) {\n      addValue(enrichCtx.target, String.valueOf(ruleNumber));\n    }\n\n  } else if (eventName == \"GetPasswordData\") {\n    addField(enrichCtx.target, \"json.requestParameters.instanceId\");\n\n  } else if (eventName == \"ModifyImageAttribute\") {\n    addField(enrichCtx.target, \"json.requestParameters.imageId\");\n\n  } else if (eventName == \"ModifySnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.requestParameters.snapshotId\");\n\n  } else if (eventName == \"DescribeSecurityGroups\"\n            || eventName == \"DescribeNetworkInterfaces\"\n            || eventName == \"DescribeRegions\"\n            || eventName == \"DescribeVpcs\"\n            || eventName == \"DescribeNetworkAcls\"\n            || eventName == \"DescribeVolumes\"\n  ) {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichElasticFileSystem(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"elasticfilesystem.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteFileSystem\") {\n    addField(enrichCtx.target, \"json.requestParameters.fileSystemId\");\n  } else if (eventName == \"DeleteMountTarget\") {\n    addField(enrichCtx.target, \"json.requestParameters.mountTargetId\");\n  }\n}\n\nvoid enrichEvents(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"events.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteRule\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n  }\n}\n\nvoid enrichGuardDuty(def enrichCtx, def eventSource, def eventName)  {\n  if (eventSource != \"guardduty.amazonaws.com\") {\n      return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.detectorId\",\n    \"json.responseElements.detectorId\"\n  ]);\n\n  if (eventName == \"CreateDetector\") {\n    addField(enrichCtx.target, \"json.responseElements.detectorId\");\n  } else if (eventName == \"DeleteDetector\") {\n    addField(enrichCtx.target, \"json.requestParameters.detectorId\");\n  }\n}\n\nvoid enrichIam(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"iam.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.userName\",\n    \"json.requestParameters.serialNumber\",\n    \"json.requestParameters.accessKeyId\",\n    \"json.requestParameters.policyArn\",\n    \"json.requestParameters.roleName\",\n    \"json.requestParameters.policyName\",\n    \"json.requestParameters.serialNumber\",\n    \"json.responseElements.accessKey.userName\",\n    \"json.responseElements.accessKey.accessKeyId\",\n    \"json.responseElements.user.arn\",\n    \"json.responseElements.user.userId\",\n    \"json.responseElements.user.userName\",\n    \"json.responseElements.userId\",\n    \"json.responseElements.role.arn\",\n    \"json.responseElements.serialNumber\"\n  ]);\n\n  if (eventName == \"AttachGroupPolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.groupName\");\n\n  } else if (eventName == \"AttachRolePolicy\"\n            || eventName == \"ListAttachedRolePolicies\"\n            || eventName == \"UpdateAssumeRolePolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.roleName\");\n\n  } else if (eventName == \"AttachUserPolicy\") {\n    addFields(enrichCtx.target, [\n      \"json.requestParameters.policyArn\",\n      \"json.requestParameters.userName\"\n    ]);\n\n  } else if (eventName == \"CreateAccessKey\") {\n    addFields(enrichCtx.target, [\n      \"json.responseElements.accessKey.accessKeyId\",\n      \"json.requestParameters.userName\"\n    ]);\n\n  } else if (eventName == \"CreateUser\"\n            || eventName == \"DeactivateMFADevice\") {\n    addField(enrichCtx.target, \"json.requestParameters.userName\");\n\n  } else if (eventName == \"DeleteVirtualMFADevice\") {\n    addField(enrichCtx.target, \"json.requestParameters.serialNumber\");\n\n  } else if (eventName == \"GetPolicy\") {\n    addField(enrichCtx.target, \"json.requestParameters.policyArn\");\n\n  } else if (eventName == \"CreatePolicy\") {\n    addField(enrichCtx.target, \"json.responseElements.policy.arn\");\n\n  } else if (eventName == \"ListRoles\"\n            || eventName == \"ListUsers\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichKms(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"kms.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.keyId\",\n    \"json.responseElements.keyId\",\n    \"json.responseElements.keyMetadata.arn\",\n    \"json.responseElements.keyMetadata.keyId\"\n  ]);\n\n  if (eventName == \"DisableKey\"\n    || eventName == \"ScheduleKeyDeletion\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n  }\n}\n\nvoid enrichLambda(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"lambda.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.functionName\",\n    \"json.responseElements.functionArn\",\n    \"json.responseElements.functionName\",\n    \"json.responseElements.role\"\n  ]);\n\n  addValues(enrichCtx.related, $(\"json.responseElements.vpcConfig.securityGroupIds\", []));\n  addValues(enrichCtx.related, $(\"json.responseElements.vpcConfig.subnetIds\", []));\n\n  if (eventName == null) {\n    return;\n  }\n\n  if (eventName.contains(\"AddPermission\")) { // needs to be contains because lambda event names are versioned on the name\n    addField(enrichCtx.target, \"json.requestParameters.functionName\");\n\n  } else if (eventName.contains(\"ListFunctions\")) {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichLogs(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"logs.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteLogGroup\") {\n    addField(enrichCtx.target, \"json.requestParameters.logGroupName\");\n\n  } else if (eventName == \"DeleteLogStream\") {\n    addField(enrichCtx.target, \"json.requestParameters.logStreamName\");\n\n  }\n}\n\nvoid enrichMonitoring(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"monitoring.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteAlarms\") {\n    $(\"json.requestParameters.alarmNames\", []).stream().forEach(f -> addValue(enrichCtx.target, f));\n\n  }\n}\n\nvoid enrichRds(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"rds.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.dBInstanceIdentifier\",\n    \"json.requestParameters.dBInstanceArn\",\n    \"json.responseElements.dBInstanceIdentifier\",\n    \"json.responseElements.dbInstanceArn\",\n    \"json.responseElements.dBSubnetGroup.vpcId\",\n    \"json.responseElements.vpcSecurityGroups.vpcSecurityGroupId\"\n  ]);\n\n  $(\"json.responseElements.dBSubnetGroup.subnets\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.subnetIdentifier);\n  });\n\n  $(\"json.responseElements.vpcSecurityGroups\", []).stream().forEach(i -> {\n    addValue(enrichCtx.related, i.vpcSecurityGroupId);\n  });\n\n  if (eventName == \"DeleteDBCluster\"\n    || eventName == \"ModifyDBCluster\"\n    || eventName == \"StopDBCluster\") {\n    addField(enrichCtx.target, \"json.responseElements.dBClusterArn\");\n\n  } else if (eventName == \"DeleteDBInstance\"\n          || eventName == \"ModifyDBInstance\"\n          || eventName == \"RestoreDBInstanceFromDBSnapshot\"\n          || eventName == \"RestoreDBInstanceFromS3\"\n          || eventName == \"StopDBInstance\") {\n    addField(enrichCtx.target, \"json.responseElements.dBInstanceArn\");\n\n  } else if (eventName == \"DeleteGlobalCluster\") {\n    addField(enrichCtx.target, \"json.responseElements.globalClusterArn\");\n\n  } else if (eventName == \"ModifyDBClusterSnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.responseElements.dBClusterSnapshotIdentifier\");\n\n  } else if (eventName == \"ModifyDBSnapshotAttribute\") {\n    addField(enrichCtx.target, \"json.responseElements.dBSnapshotIdentifier\");\n\n  } else if (eventName == \"DescribeDBInstances\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichRolesAnywhere(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"rolesanywhere.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateTrustAnchor\") {\n    addField(enrichCtx.target, \"json.responseElements.trustAnchor.trustAnchorArn\");\n\n  }\n}\n\nvoid enrichRoute53Resolver(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"route53resolver.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DeleteResolverQueryLogConfig\") {\n    addField(enrichCtx.target, \"json.responseElements.resolverQueryLogConfig.arn\");\n  }\n}\n\nvoid enrichS3(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"s3.amazonaws.com\") {\n    return;\n  }\n\n  addField(enrichCtx.related, \"json.requestParameters.bucketName\");\n\n  if (eventName == \"CopyObject\"\n    || eventName == \"PutBucketLogging\"\n    || eventName == \"PutBucketVersioning\"\n    || eventName == \"PutObject\"\n    || eventName == \"GetBucketPolicy\"\n    || eventName == \"ListObjects\"\n    || eventName == \"HeadObject\"\n    || eventName == \"GetObject\"\n    || eventName == \"DeleteObject\"\n    || eventName == \"DeleteBucket\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n\n  } else if (eventName == \"PutBucketReplication\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n    addField(enrichCtx.target, \"json.requestParameters.ReplicationConfiguration.Rule.Destination.Bucket\");\n\n  } else if (eventName == \"ListBuckets\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichSecretsManager(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"secretsmanager.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"BatchGetSecretValue\") {\n    $(\"json.requestParameters.secretIdList\", []).stream().forEach(f -> addValue(enrichCtx.target, f));\n\n  } else if (eventName == \"GetSecretValue\") {\n    addField(enrichCtx.target, \"json.requestParameters.secretId\");\n\n  }\n}\n\nvoid enrichSignin(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"signin.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"ConsoleLogin\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichSsm(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"ssm.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"GetParameter\"\n    || eventName == \"GetParameters\"\n    || eventName == \"CreateControlChannel\"\n    || eventName == \"OpenControlChannel\") {\n    $(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.target, f.ARN));\n\n  } else if (eventName == \"StartSession\") {\n    addField(enrichCtx.target, \"json.requestParameters.target\");\n\n  } else if (eventName == \"CreateDocument\") {\n    addField(enrichCtx.target, \"json.requestParameters.name\");\n\n  } else if (eventName == \"TerminateSession\"\n            || eventName == \"OpenDataChannel\") {\n    addField(enrichCtx.target, \"json.requestParameters.sessionId\");\n\n  } else if (eventName == \"SendCommand\") {\n    List instanceIds = $(\"json.requestParameters.instanceIds\", []);\n\n    if (instanceIds.isEmpty()) {\n      instanceIds = $(\"json.requestParameters.targets\", []).stream().flatMap(target -> target.values.stream()).collect(Collectors.toList());\n    }\n\n    if (instanceIds.size() == 1 && instanceIds.get(0) == \"*\") {\n      instanceIds = [ $(\"json.recipientAccountId\", null) ]; // if all instances, point to full account\n    }\n\n    addValues(enrichCtx.target, instanceIds);\n\n  } else if (eventName == \"ListInstanceAssociations\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n  }\n}\n\nvoid enrichSts(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"sts.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.roleArn\",\n    \"json.sourceIdentity\",\n    \"json.additionalEventData.MFAIdentifier\",\n    \"json.responseElements.assumedRoleUser.arn\",\n    \"json.requestParameters.roleSessionName\",\n    \"json.responseElements.accessKeyId\"\n  ]);\n\n  if (eventName == \"AssumeRole\") {\n    def userType = $(\"json.userIdentity.type\", null);\n\n    if (userType == \"AWSService\") {\n      enrichCtx.actor = $(\"json.userIdentity.invokedBy\", null);\n    } else if (userType == \"AssumedRole\") {\n      enrichCtx.actor = $(\"json.userIdentity.sessionContext.sessionIssuer.arn\", null);\n    } else {\n      enrichCtx.actor = $(\"json.userIdentity.arn\", null);\n    }\n\n    addField(enrichCtx.target, \"json.requestParameters.roleArn\");\n\n  } else if (eventName == \"GetCallerIdentity\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n    enrichCtx.actor = $(\"json.userIdentity.arn\", null);\n\n  }\n}\n\nvoid enrichWafv2(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"wafv2.amazonaws.com\") {\n    return;\n  }\n\n  addFields(enrichCtx.related, [\n    \"json.requestParameters.id\",\n    \"json.responseElements.summary\"\n  ]);\n\n  if (eventName == \"DeleteRuleGroup\"\n      || eventName == \"DeleteWebACL\") {\n    addField(enrichCtx.target, \"json.requestParameters.id\");\n  }\n}\n\nvoid enrichSns(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"sns.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"CreateTopic\") {\n    addField(enrichCtx.target, \"json.responseElements.topicArn\");\n  } else if (eventName == \"Subscribe\"\n            || eventName == \"Publish\") {\n    addField(enrichCtx.target, \"json.requestParameters.topicArn\");\n  }\n\n}\n\nvoid enrichBedrock(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"bedrock.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"Converse\") {\n    addField(enrichCtx.target, \"json.requestParameters.modelId\");\n\n  }\n}\n\nvoid enrichElasticLoadBalancing(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"elasticloadbalancing.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"DescribeLoadBalancers\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\nvoid enrichDynamoDB(def enrichCtx, def eventSource, def eventName) {\n  if (eventSource != \"dynamodb.amazonaws.com\") {\n    return;\n  }\n\n  if (eventName == \"ListTables\") {\n    addField(enrichCtx.target, \"json.recipientAccountId\");\n\n  }\n}\n\n// Using tree set to ensure a sorting is kept (testing purposes)\nMap enrichCtx = [:];\nenrichCtx.related = new TreeSet();\nenrichCtx.target = new TreeSet();\n\nenrichCtx.actor = $(\"json.userIdentity.arn\", null); // default actor value\nif (enrichCtx.actor == null) {\n  enrichCtx.actor = $(\"json.userIdentity.onBehalfOf.userId\", null);\n}\n\naddFields(enrichCtx.related, [\n  \"json.userIdentity.accessKeyId\",\n  \"json.userIdentity.arn\",\n  \"json.userIdentity.userName\",\n  \"json.userIdentity.sessionContext.sessionIssuer.arn\",\n  \"json.userIdentity.sessionContext.sessionIssuer.userName\"\n]);\n\nif ($(\"json.userIdentity.type\", null) == \"IdentityCenterUser\") {\n  addField(enrichCtx.related, \"json.userIdentity.onBehalfOf.identityStoreArn\");\n}\n\n$(\"json.resources\", []).stream().forEach(f -> addValue(enrichCtx.related, f.ARN));\n\nString eventSource = $(\"json.eventSource\", null);\nString eventName = $(\"json.eventName\", null);\n\nenrichCloudformation(enrichCtx, eventSource, eventName);\nenrichCloudtrail(enrichCtx, eventSource, eventName);\nenrichConfig(enrichCtx, eventSource, eventName);\nenrichEc2InstanceConnect(enrichCtx, eventSource, eventName);\nenrichEc2(enrichCtx, eventSource, eventName);\nenrichElasticFileSystem(enrichCtx, eventSource, eventName);\nenrichEvents(enrichCtx, eventSource, eventName);\nenrichGuardDuty(enrichCtx, eventSource, eventName);\nenrichIam(enrichCtx, eventSource, eventName);\nenrichKms(enrichCtx, eventSource, eventName);\nenrichLambda(enrichCtx, eventSource, eventName);\nenrichLogs(enrichCtx, eventSource, eventName);\nenrichMonitoring(enrichCtx, eventSource, eventName);\nenrichRds(enrichCtx, eventSource, eventName);\nenrichRolesAnywhere(enrichCtx, eventSource, eventName);\nenrichRoute53Resolver(enrichCtx, eventSource, eventName);\nenrichS3(enrichCtx, eventSource, eventName);\nenrichSecretsManager(enrichCtx, eventSource, eventName);\nenrichSignin(enrichCtx, eventSource, eventName);\nenrichSsm(enrichCtx, eventSource, eventName);\nenrichSts(enrichCtx, eventSource, eventName);\nenrichWafv2(enrichCtx, eventSource, eventName);\nenrichSns(enrichCtx, eventSource, eventName);\nenrichBedrock(enrichCtx, eventSource, eventName);\nenrichElasticLoadBalancing(enrichCtx, eventSource, eventName);\nenrichDynamoDB(enrichCtx, eventSource, eventName);\n\n// Classify target entities by type\nif (!enrichCtx.target.isEmpty()) {\n  // Always set the legacy target.entity.id for backward compatibility\n  field(\"target.entity.id\").set(enrichCtx.target);\n  \n  def userTargets = new TreeSet();\n  def hostTargets = new TreeSet();\n  def serviceTargets = new TreeSet();\n  def genericTargets = new TreeSet();\n\n  for (def targetValue : enrichCtx.target) {\n    if (targetValue == null || targetValue == \"\") {\n      continue;\n    }\n\n    String target = targetValue.toString();\n    boolean classified = false;\n\n    // Handle ARN format: arn:aws:service:region:account-id:resource-type/resource-id\n    if (target.startsWith(\"arn:\")) {\n      def parts = target.splitOnToken(\":\");\n      if (parts.length >= 6) {\n        // Extract resource type (last part before /)\n        String resourcePart = parts[5];\n        String resourceType = resourcePart;\n        \n        // Handle resource-type/resource-id format\n        int slashIdx = resourcePart.indexOf(\"/\");\n        if (slashIdx > 0) {\n          resourceType = resourcePart.substring(0, slashIdx);\n        }\n        \n        // Check for service-linked roles first (AWSServiceRoleFor*) - these are service resources, not users\n        if (target.contains(\"/AWSServiceRoleFor\") || target.contains(\"/aws-service-role/\")) {\n          serviceTargets.add(target);\n          classified = true;\n        }\n        // Classify based on resource type\n        else if (params.userResourceTypes.containsKey(resourceType)) {\n          userTargets.add(target);\n          classified = true;\n        } else if (params.hostResourceTypes.containsKey(resourceType)) {\n          hostTargets.add(target);\n          classified = true;\n        } else if (params.serviceResourceTypes.containsKey(resourceType)) {\n          serviceTargets.add(target);\n          classified = true;\n        } else if (parts[2] == \"s3\") {\n          // Special case: S3 buckets/objects are service resources\n          serviceTargets.add(target);\n          classified = true;\n        }\n      }\n    }\n    // Handle simple resource IDs with prefixes\n    else if (!classified) {\n      // Access Key ID pattern (AKIA...)\n      if (target.length() == 20 && target.startsWith(\"AKIA\")) {\n        userTargets.add(target);\n        classified = true;\n      } else {\n        // Check host ID prefixes\n        for (def prefix : params.hostIdPrefixes) {\n          if (target.startsWith(prefix)) {\n            hostTargets.add(target);\n            classified = true;\n            break;\n          }\n        }\n        \n        // Check service ID prefixes if not yet classified\n        if (!classified) {\n          for (def prefix : params.serviceIdPrefixes) {\n            if (target.startsWith(prefix)) {\n              serviceTargets.add(target);\n              classified = true;\n              break;\n            }\n          }\n        }\n      }\n    }\n\n    // Generic fallback for everything else (including account IDs)\n    if (!classified) {\n      genericTargets.add(target);\n    }\n  }\n\n  // Set the appropriate type-specific entity fields\n  if (!userTargets.isEmpty()) {\n    field(\"user.target.entity.id\").set(userTargets);\n  }\n  if (!hostTargets.isEmpty()) {\n    field(\"host.target.entity.id\").set(hostTargets);\n  }\n  if (!serviceTargets.isEmpty()) {\n    field(\"service.target.entity.id\").set(serviceTargets);\n  }\n  if (!genericTargets.isEmpty()) {\n    field(\"entity.target.id\").set(genericTargets);\n  }\n\n  enrichCtx.related.addAll(enrichCtx.target);\n}\n\n// Map actor to appropriate entity type based on identity\nif (enrichCtx.actor != null) {\n  // Always set the legacy actor.entity.id for backward compatibility\n  field(\"actor.entity.id\").set([ enrichCtx.actor ]);\n  \n  def userType = $(\"json.userIdentity.type\", null);\n  String actorStr = enrichCtx.actor.toString();\n  boolean classified = false;\n  \n  // Check userType first (most reliable indicator)\n  if (userType == \"AWSService\") {\n    field(\"service.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  // Check for service-linked roles (AWSServiceRoleFor*) - these are AWS services, not users\n  else if (actorStr.contains(\"/AWSServiceRoleFor\") || actorStr.contains(\"/aws-service-role/\")) {\n    field(\"service.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  else if (params.userIdentityTypes.containsKey(userType)) {\n    field(\"user.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  \n  // If not classified by userType, parse ARN structure\n  if (!classified && actorStr.startsWith(\"arn:\")) {\n    def parts = actorStr.splitOnToken(\":\");\n    if (parts.length >= 6) {\n      String resourcePart = parts[5];\n      String resourceType = resourcePart;\n      \n      // Handle resource-type/resource-id format\n      int slashIdx = resourcePart.indexOf(\"/\");\n      if (slashIdx > 0) {\n        resourceType = resourcePart.substring(0, slashIdx);\n      }\n      \n      // Classify based on resource type\n      if (resourceType == \"user\" || resourceType == \"role\" || \n          resourceType == \"assumed-role\" || resourceType == \"federated-user\") {\n        field(\"user.entity.id\").set([ enrichCtx.actor ]);\n        classified = true;\n      } else if (resourceType == \"instance\") {\n        field(\"host.entity.id\").set([ enrichCtx.actor ]);\n        classified = true;\n      }\n    }\n  }\n  \n  // Check simple instance ID format\n  if (!classified && actorStr.startsWith(\"i-\")) {\n    field(\"host.entity.id\").set([ enrichCtx.actor ]);\n    classified = true;\n  }\n  \n  // Fallback for other types -> entity.id\n  if (!classified) {\n    field(\"entity.id\").set([ enrichCtx.actor ]);\n  }\n}\n\nfield(\"related.entity\").set(enrichCtx.related);\n"#
                        ),
                        cached_params!(
                            "{\"userResourceTypes\":{\"user\":true,\"role\":true,\"assumed-role\":true,\"federated-user\":true,\"instance-profile\":true,\"mfa\":true,\"serialNumber\":true},\"hostResourceTypes\":{\"instance\":true},\"serviceResourceTypes\":{\"stack\":true,\"stack-set\":true,\"trail\":true,\"configuration-recorder\":true,\"file-system\":true,\"mount-target\":true,\"rule\":true,\"detector\":true,\"key\":true,\"function\":true,\"log-group\":true,\"log-stream\":true,\"alarm\":true,\"db\":true,\"cluster\":true,\"global-cluster\":true,\"trust-anchor\":true,\"resolver-query-log-config\":true,\"secret\":true,\"parameter\":true,\"document\":true,\"session\":true,\"webacl\":true,\"rulegroup\":true,\"ipset\":true,\"topic\":true,\"model\":true,\"loadbalancer\":true,\"table\":true,\"group\":true,\"policy\":true,\"security-group\":true,\"network-interface\":true,\"vpc\":true,\"subnet\":true,\"network-acl\":true,\"dhcp-options\":true},\"hostIdPrefixes\":[\"i-\"],\"serviceIdPrefixes\":[\"sg-\",\"sgr-\",\"eni-\",\"vpc-\",\"subnet-\",\"acl-\",\"rtb-\",\"igw-\",\"nat-\",\"vpce-\",\"tgw-\",\"pcx-\"],\"userIdentityTypes\":{\"IAMUser\":true,\"Root\":true,\"AssumedRole\":true}}"
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
                            "Processor '{}'\n{}failed with message '{}'\n",
                            event
                                .get("_ingest.on_failure_processor_type")
                                .map_or_else(String::new, template_to_string),
                            if event
                                .get("_ingest.on_failure_processor_tag")
                                .is_some_and(|v| !v.is_null()
                                    && v.as_str() != Some("")
                                    && !matches!(v, Value::Bool(false))
                                    && !v.as_array().is_some_and(Vec::is_empty))
                            {
                                format!(
                                    "with tag '{}'\n",
                                    event
                                        .get("_ingest.on_failure_processor_tag")
                                        .map_or_else(String::new, template_to_string)
                                )
                            } else {
                                String::new()
                            },
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

            if event.has_value("json.userIdentity.type") {
                event.rename(
                    "json.userIdentity.type",
                    "aws.cloudtrail.user_identity.type",
                )?;
            }

            if event.has_value("json.userIdentity.userName") {
                event.rename("json.userIdentity.userName", "user.name")?;
            }

            if event.has_value("json.userIdentity.principalId") {
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
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set(
                            "aws.cloudtrail.user_identity.session_context.creation_date",
                            parsed,
                        )?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.userIdentity.sessionContext.attributes.creationDate"
                                    .into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                if event.has_value("json.userIdentity.sessionContext.sessionIssuer.userName") {
                    event.rename_over(
                        "json.userIdentity.sessionContext.sessionIssuer.userName",
                        "user.name",
                    )?;
                }
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("json.userIdentity.sessionContext.sessionIssuer.principalId") {
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
                        if !cached_grok!(
                            "arn:(aws|aws-us-gov):sts:.*/%{GREEDYDATA:_tmp.session_name}$"
                        )
                        .extract_into(&input, event)?
                        {
                            return Err(TransformError::GrokNoMatch { value: input });
                        }
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
                if event.has_value("json.userIdentity.onBehalfOf.userId") {
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
                if event.has_value("json.sessionCredentialFromConsole") {
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
                    if !cached_grok!("^%{IP:source.ip}$").extract_into(&input, event)? {
                        return Err(TransformError::GrokNoMatch { value: input });
                    }
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

            if event.has_value("source.as.asn") {
                event.rename("source.as.asn", "source.as.number")?;
            }

            if event.has_value("source.as.organization_name") {
                event.rename("source.as.organization_name", "source.as.organization.name")?;
            }

            if let Some(ua_str) = event.get_string("json.userAgent") {
                let ua_str = ua_str.to_string();
                // User agent parsing
                if let Ok(ua) = parse_user_agent(&ua_str) {
                    event.remove("user_agent");
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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "2497d2dda68b294a0d3efc8568b81181ebd72492d31c0b2a26aebf74058c3be4",
                    r#"{"params":null,"known":["FlattenedDuplicates"]}"#,
                    r#"ctx._conf = ctx._conf ?: [:];\nctx._conf.keep_flattened_duplicates = ctx._conf.retain == null ||\n  ctx._conf.retain.contains('all') ||\n  ctx._conf.retain.contains('flattened') ||\n  ctx._conf.retain.contains('minimal');"#
                ),
            )?;

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "eb7dcf86eb859267bdd353d7abd4c675febedf759de966c25e687e01ebe20953",
                        r#"{"params":null,"known":["FlattenedDuplicates"]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "1e749abb5deabc9017fd872701938604196548198f858fac7bf4989bf43cce35",
                        r#"{"params":null,"known":[{"GuardedCopy":{"statements":[{"If":{"test":[[{"Compare":{"path":"json.managementEvent","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"aws.cloudtrail.management_event","value":{"Text":"json.managementEvent"}}}}],"alt":[]}}],"whole":true}}]}"#,
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

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "155f88f304ebec73bfa7a3aadd3a13328bb48108b3dedad6cbb4a69504eedc33",
                    r#"{"params":null,"known":[{"ResourcesRenameDedup":"json.resources"}]}"#
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "be020fa3e16ab48f4088361bb91512d633897ce2ccfa4c5a5cc8c67133ed9a6a",
                        r#"{"params":null,"known":["ConsoleLoginEventData"]}"#
                    ),
                )?;
                Ok(())
            })();

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan_params(
                event,
                compiled_painless!(
                    "711940065d1051a2efba95179267448dc97acc3b423b7664e1f205c78efc264e",
                    r#"{"params":{"LookupMerge":{"statements":[{"Literal":{"Set":{"path":"event.kind","value":{"Literal":"event"}}}},{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}},{"If":{"test":[[{"Compare":{"path":"aws.cloudtrail.error_code","wanted":"Null","negated":true,"lowered":false}}],[{"Compare":{"path":"aws.cloudtrail.error_message","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.outcome","value":{"Literal":"failure"}}}}],"alt":[{"Literal":{"Set":{"path":"event.outcome","value":{"Literal":"success"}}}}]}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":"Null","negated":false,"lowered":false}}]],"then":["Return"],"alt":[]}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":{"Text":"ConsoleLogin"},"negated":false,"lowered":false}},{"Compare":{"path":"json.responseElements.ConsoleLogin","wanted":"Null","negated":true,"lowered":false}}]],"then":[],"alt":[]}},{"If":{"test":[["Never"]],"then":["Return"],"alt":[]}}],"whole":false}},"known":[{"GuardedCopy":{"statements":[{"Literal":{"Set":{"path":"event.kind","value":{"Literal":"event"}}}},{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}},{"If":{"test":[[{"Compare":{"path":"aws.cloudtrail.error_code","wanted":"Null","negated":true,"lowered":false}}],[{"Compare":{"path":"aws.cloudtrail.error_message","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.outcome","value":{"Literal":"failure"}}}}],"alt":[{"Literal":{"Set":{"path":"event.outcome","value":{"Literal":"success"}}}}]}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":"Null","negated":false,"lowered":false}}]],"then":["Return"],"alt":[]}},{"If":{"test":[[{"Compare":{"path":"event.action","wanted":{"Text":"ConsoleLogin"},"negated":false,"lowered":false}},{"Compare":{"path":"json.responseElements.ConsoleLogin","wanted":"Null","negated":true,"lowered":false}}]],"then":[],"alt":[]}},{"If":{"test":[["Never"]],"then":["Return"],"alt":[]}}],"whole":false}}]}"#,
                    r#"ctx.event.kind = 'event'; ctx.event.type = ['info'];\nif (ctx.aws?.cloudtrail?.error_code != null || ctx.aws?.cloudtrail?.error_message != null) {\n  ctx.event.outcome = 'failure'\n} else {\n  ctx.event.outcome = 'success'\n}\nif (ctx.event?.action == null) {\n  return;\n}\nif (ctx.event.action == 'ConsoleLogin' && ctx.json?.responseElements?.ConsoleLogin != null) {\n  ctx.event.outcome = Processors.lowercase(ctx.json.responseElements.ConsoleLogin);\n}\nif (params.get(ctx.event.action) == null) {\n  return;\n}\ndef hm = new HashMap(params.get(ctx.event.action)); hm.forEach((k, v) -> ctx.event[k] = v);"#
                ),
                cached_params!(
                    "{\"AddUserToGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"AssumeRole\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"AttachGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"AttachUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"ChangePassword\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"ConsoleLogin\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"CreateAccessKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"CreateBucket\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"CreateGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"creation\"]},\"CreateKeyPair\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"creation\"]},\"CreateUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"creation\"]},\"CreateVirtualMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeactivateMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteAccessKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteBucket\":{\"category\":[\"file\"],\"type\":[\"deletion\"]},\"DeleteGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"deletion\"]},\"DeleteGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"DeleteSSHPublicKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"deletion\"]},\"DeleteUserPermissionsBoundary\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DeleteVirtualMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"DetachGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"DetachUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"EnableMFADevice\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"GetGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"GetGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"GetUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"GetUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListAttachedGroupPolicies\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"ListAttachedUserPolicies\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListGroupPolicies\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"ListGroups\":{\"category\":[\"iam\"],\"type\":[\"group\",\"info\"]},\"ListGroupsForUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListUserPolicies\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListUsers\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"ListUserTags\":{\"category\":[\"iam\"],\"type\":[\"user\",\"info\"]},\"PutGroupPolicy\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"PutUserPermissionsBoundary\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"PutUserPolicy\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"RemoveUserFromGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"SetDefaultPolicyVersion\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"SetSecurityTokenServicePreferences\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"TagUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UntagUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateAccessKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateAccountPasswordPolicy\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"UpdateGroup\":{\"category\":[\"iam\"],\"type\":[\"group\",\"change\"]},\"UpdateLoginProfile\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateRole\":{\"category\":[\"iam\"],\"type\":[\"admin\",\"change\"]},\"UpdateSSHPublicKey\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"UpdateUser\":{\"category\":[\"iam\"],\"type\":[\"user\",\"change\"]},\"GetCallerIdentity\":{\"category\":[\"authentication\"],\"type\":[\"info\"]},\"DescribeInstances\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"DescribeSecurityGroups\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"ListBuckets\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"DescribeNetworkInterfaces\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeRegions\":{\"category\":[\"api\"],\"type\":[\"info\"]},\"ListRoles\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"DescribeVpcs\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeNetworkAcls\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeLoadBalancers\":{\"category\":[\"network\"],\"type\":[\"info\"]},\"DescribeVolumes\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"DescribeTrails\":{\"category\":[\"configuration\"],\"type\":[\"info\"]},\"ListFunctions\":{\"category\":[\"package\"],\"type\":[\"info\"]},\"ListTables\":{\"category\":[\"database\"],\"type\":[\"info\"]},\"ListInstanceAssociations\":{\"category\":[\"host\"],\"type\":[\"info\"]},\"DescribeDBInstances\":{\"category\":[\"database\"],\"type\":[\"info\"]},\"AuthorizeSecurityGroupIngress\":{\"category\":[\"network\"],\"type\":[\"access\"]},\"CreateTopic\":{\"category\":[\"configuration\"],\"type\":[\"creation\"]},\"Subscribe\":{\"category\":[\"configuration\"],\"type\":[\"change\"]},\"Publish\":{\"category\":[\"api\"],\"type\":[]},\"SendCommand\":{\"category\":[\"process\"],\"type\":[]},\"GetPolicy\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"CreatePolicy\":{\"category\":[\"iam\"],\"type\":[\"creation\"]},\"ListAttachedRolePolicies\":{\"category\":[\"iam\"],\"type\":[\"info\"]},\"AttachRolePolicy\":{\"category\":[\"iam\"],\"type\":[\"change\"]},\"Converse\":{\"category\":[\"api\"],\"type\":[]},\"CreateDocument\":{\"category\":[\"file\"],\"type\":[\"creation\"]},\"StartSession\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"CreateControlChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"OpenControlChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"CreateDataChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"OpenDataChannel\":{\"category\":[\"session\"],\"type\":[\"start\"]},\"TerminateSession\":{\"category\":[\"session\"],\"type\":[\"end\"]},\"ListObjects\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"HeadObject\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"GetObject\":{\"category\":[\"file\"],\"type\":[\"info\"]},\"PutObject\":{\"category\":[\"file\"],\"type\":[\"change\"]},\"DeleteTrail\":{\"category\":[\"configuration\"],\"type\":[\"deletion\"]},\"DeleteObject\":{\"category\":[\"file\"],\"type\":[\"delete\"]}}"
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
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("aws.cloudtrail.digest.start_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.digestStartTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.digestEndTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("@timestamp", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.digestEndTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.digestEndTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => event.set("aws.cloudtrail.digest.end_time", parsed)?,
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.digestEndTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("aws.cloudtrail.digest.newest_event_time", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.newestEventTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
                    }
                }
                Ok(())
            })();

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if let Some(date_str) = event.get_as_string("json.oldestEventTime") {
                    match parse_date_out(&date_str, &["ISO8601"], None, None) {
                        Some(parsed) => {
                            event.set("aws.cloudtrail.digest.oldest_event_time", parsed)?
                        }
                        None => {
                            return Err(TransformError::ParseError {
                                path: "json.oldestEventTime".into(),
                                message: format!("unable to parse date [{date_str}]"),
                            });
                        }
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

            if event.has_value("aws.cloudtrail.digest") {
                event.rename("aws.cloudtrail.digest", "aws.cloudtrail.flattened.digest")?;
            }

            if event.has_value("json.insightDetails") {
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
                        } else {
                            return Err(TransformError::ParseError {
                                path: "json.tlsDetails.tlsVersion".into(),
                                message: "dissect pattern did not match".into(),
                            });
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
                map_strings(
                    event,
                    "tls.version_protocol",
                    "tls.version_protocol",
                    str::to_lowercase,
                )?;
            }

            if event.has_value("json.tlsDetails.cipherSuite") {
                event.rename("json.tlsDetails.cipherSuite", "tls.cipher")?;
            }

            if event.has_value("json.tlsDetails.clientProvidedHostHeader") {
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
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "9e00a0490f712c778819d8572239a79e8db4e395d8638129578818a150e20993",
                        r#"{"params":null,"known":[]}"#
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

            // The pipeline's script, emitted as the runtime's equivalent
            drop_empty(
                event,
                &DropPolicy {
                    nulls: true,
                    empty_strings: true,
                    empty_collections: true,
                    prune_lists: true,
                    ..DropPolicy::none()
                },
                None,
            );

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append(
                    "error.message",
                    json!(format!(
                        "Processor '{}' {}in pipeline '{}' failed with message '{}'",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        if event
                            .get("_ingest.on_failure_processor_tag")
                            .is_some_and(|v| !v.is_null()
                                && v.as_str() != Some("")
                                && !matches!(v, Value::Bool(false))
                                && !v.as_array().is_some_and(Vec::is_empty))
                        {
                            format!(
                                "with tag '{}' ",
                                event
                                    .get("_ingest.on_failure_processor_tag")
                                    .map_or_else(String::new, template_to_string)
                            )
                        } else {
                            String::new()
                        },
                        event
                            .get("_ingest.pipeline")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("json");
                event.remove("_conf");
                event.remove("_tmp");
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
