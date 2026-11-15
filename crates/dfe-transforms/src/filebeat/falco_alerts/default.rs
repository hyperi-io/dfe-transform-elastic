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
            event.set("event.kind", json!("alert"))?;

            if let Some(v) = event
                .get("message")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.original", v)?;
            }

            let _cond = { event.has_value("message") };
            if _cond {
                parse_json_field(event, "message", "falco")?;
            }

            let _cond = { event.has_value("output_fields") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "0310232932adb4be61ba44792fae7f9726c51b72dd231c3a34e73b3774e1e60e",
                        r#"{"params":null,"known":[]}"#
                    ),
                    cached_params!(
                        "{\"fields\":[\"uuid\",\"output\",\"priority\",\"rule\",\"time\",\"output_fields\",\"source\",\"tags\",\"hostname\"]}"
                    ),
                )?;
            }

            dot_expand(event, "falco.output_fields", "container.image")?;

            dot_expand(event, "falco.output_fields", "fd.cip")?;

            dot_expand(event, "falco.output_fields", "fd.sip")?;

            dot_expand(event, "falco.output_fields", "fd.lip")?;

            dot_expand(event, "falco.output_fields", "fd.rip")?;

            dot_expand(event, "falco.output_fields", "proc.ppid")?;

            dot_expand(event, "falco.output_fields", "proc.pid")?;

            dot_expand(event, "falco.output_fields", "proc.sid")?;

            dot_expand(event, "falco.output_fields", "proc.vpgid")?;

            dot_expand(event, "falco.output_fields", "evt.time")?;

            dot_expand(event, "falco.output_fields", "syslog.facility")?;

            dot_expand(event, "falco.output_fields", "syslog.severity")?;

            if event.has_value("falco.output_fields.container.image") {
                event.rename(
                    "falco.output_fields.container.image",
                    "falco.output_fields.container.image.name",
                )?;
            }

            if event.has_value("falco.output_fields.fd.cip") {
                event.rename(
                    "falco.output_fields.fd.cip",
                    "falco.output_fields.client.ip",
                )?;
            }

            if event.has_value("falco.output_fields.fd.sip") {
                event.rename(
                    "falco.output_fields.fd.sip",
                    "falco.output_fields.server.ip",
                )?;
            }

            if event.has_value("falco.output_fields.fd.lip") {
                event.rename(
                    "falco.output_fields.fd.lip",
                    "falco.output_fields.source.ip",
                )?;
            }

            if event.has_value("falco.output_fields.fd.rip") {
                event.rename(
                    "falco.output_fields.fd.rip",
                    "falco.output_fields.destination.ip",
                )?;
            }

            if event.has_value("falco.output_fields.proc.ppid") {
                event.rename(
                    "falco.output_fields.proc.ppid",
                    "falco.output_fields.process.parent.pid",
                )?;
            }

            if event.has_value("falco.output_fields.proc.pid") {
                event.rename(
                    "falco.output_fields.proc.pid",
                    "falco.output_fields.process.pid",
                )?;
            }

            if event.has_value("falco.output_fields.proc.sid") {
                event.rename(
                    "falco.output_fields.proc.sid",
                    "falco.output_fields.process.session_leader.pid",
                )?;
            }

            if event.has_value("falco.output_fields.proc.vpgid") {
                event.rename(
                    "falco.output_fields.proc.vpgid",
                    "falco.output_fields.process.group_leader.vpid",
                )?;
            }

            if event.has_value("falco.output_fields.syslog.facility") {
                event.rename(
                    "falco.output_fields.syslog.facility",
                    "falco.output_fields.syslog_facility",
                )?;
            }

            if event.has_value("falco.output_fields.evt.time") {
                event.rename(
                    "falco.output_fields.evt.time",
                    "falco.output_fields.event.time",
                )?;
            }

            let _cond = { !event.has_value("falco.output_fields.event.time") };
            if _cond {
                if event.has_value("falco.output_fields.evt_time") {
                    event.rename(
                        "falco.output_fields.evt_time",
                        "falco.output_fields.event.time",
                    )?;
                }
            }

            if event.has_value("falco.output_fields.syslog.severity") {
                event.rename(
                    "falco.output_fields.syslog.severity",
                    "falco.output_fields.syslog_severity",
                )?;
            }

            dot_expand(event, "falco.output_fields", "*")?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "4956173aa1caf44ea3c8c3fec1bbfdc2f823a441ff6243bc7efd3b580239dd8d",
                    r#"{"params":null,"known":[{"GuardedCopy":{"statements":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt","wanted":"Null","negated":true,"lowered":false}},{"Compare":{"path":"falco.output_fields.evt.category","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"If":{"test":[[{"ListContains":{"members":["api","authentication","configuration","database","driver","email","file","host","iam","intrusion_detection","library","malware","network","package","process","registry","session","threat","vulnerability","web"],"argument":{"LoweredField":"falco.output_fields.evt.category"}}}]],"then":[{"Literal":{"Set":{"path":"event.category","value":{"FieldInList":"falco.output_fields.evt.category"}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"time"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"scheduler"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["configuration"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"system"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"memory"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"sleep"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"wait"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"internal"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["host"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"ipc"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"net"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"signal"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["network"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"processing"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"process"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"io_read"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"io_write"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"io_other"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["process"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.category","wanted":{"Text":"user"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["session"]}}}}],"alt":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["process"]}}}}]}}]}}]}}]}}]}}]}}],"alt":[{"Literal":{"Set":{"path":"event.category","value":{"Literal":["process"]}}}}]}}],"whole":false}}]}"#,
                    r#"def allowedValues = [\n    'api',\n    'authentication',\n    'configuration',\n    'database',\n    'driver',\n    'email',\n    'file',\n    'host',\n    'iam',\n    'intrusion_detection',\n    'library',\n    'malware',\n    'network',\n    'package',\n    'process',\n    'registry',\n    'session',\n    'threat',\n    'vulnerability',\n    'web'\n];\n\nif (ctx?.falco?.output_fields?.evt != null && ctx?.falco?.output_fields?.evt?.category != null) {\n    def inputCategory = ctx?.falco?.output_fields?.evt?.category;\n    def lowercaseCategory = inputCategory.toLowerCase();\n    if (allowedValues.contains(lowercaseCategory)) {\n        ctx.event.category = [inputCategory];\n    } else if (inputCategory == 'time' || inputCategory == 'scheduler') {\n        ctx.event.category = ['configuration'];\n    } else if (inputCategory == 'system' || inputCategory == 'memory' || inputCategory == 'sleep' || inputCategory == 'wait' || inputCategory == 'internal') {\n        ctx.event.category = ['host'];\n    } else if (inputCategory == 'ipc' || inputCategory == 'net' || inputCategory == 'signal') {\n        ctx.event.category = ['network'];\n    } else if (inputCategory == 'processing' || inputCategory == 'process' || inputCategory == 'io_read' || inputCategory == 'io_write' || inputCategory == 'io_other') {\n        ctx.event.category = ['process'];\n    } else if (inputCategory == 'user') {\n        ctx.event.category = ['session'];\n    } else {\n        ctx.event.category = ['process'];\n    }\n} else {\n    ctx.event.category = ['process'];\n}\n"#
                ),
            )?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "34ebc68776c4cff801ba324154b6750d022c094be566f1a05930f155329b8df8",
                    r#"{"params":null,"known":[{"GuardedCopy":{"statements":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt","wanted":"Null","negated":true,"lowered":false}},{"Compare":{"path":"falco.output_fields.evt.type","wanted":"Null","negated":true,"lowered":false}}]],"then":[{"If":{"test":[[{"ListContains":{"members":["access","admin","allowed","change","connection","creation","deletion","denied","end","error","group","indicator","info","installation","protocol","start","user"],"argument":{"LoweredField":"falco.output_fields.evt.type"}}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"FieldInList":"falco.output_fields.evt.type"}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"faccessat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"faccessat2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fsopen"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"name_to_handle_at"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"nfsservctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"open"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"open_by_handle_at"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"open_tree"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"openat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"openat2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pciconfig_read"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pidfd_open"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"read"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"readahead"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"readdir"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"readlink"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"readlinkat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"readv"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"s390_pci_mmio_read"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"uselib"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"vm86old"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"vm86"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["access"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"bdflush"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"membarrier"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ptrace"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"reboot"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"restart_syscall"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["admin"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fallocate"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"finit_module"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["allowed"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"llseek"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sysctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"acct"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"adjtimex"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"alarm"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"arch_prctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"bind"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"bpf"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"brk"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"capset"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"chdir"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"chmod"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"chown"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"chown32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"chroot"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clock_adjtime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clock_nanosleep"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clock_settime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"close"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"close_range"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"epoll_ctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fchdir"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fchmod"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fchmodat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fchown"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fchown32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fchownat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fcntl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fcntl64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"flock"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"free_hugepages"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fsetxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fsconfig"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ftruncate"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ftruncate64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"futimesat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_setup"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_uring_setup"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ioctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ioperm"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"iopl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ioprio_set"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"keyctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"landlock_restrict_self"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lchown"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lchown32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"madvise"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mbind"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"memory_ordering"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"migrate_pages"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mlock"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mlock2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mlockall"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mmap"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mmap2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"modify_ldt"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"move_mount"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"move_pages"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mprotect"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mq_getsetattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mremap"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"msgctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"munlock"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"munlockall"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"munmap"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"nanosleep"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"nice"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"old_adjtimex"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pause"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pciconfig_write"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"personality"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"perfctr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"perfmonctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pivot_root"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pkey_alloc"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pkey_free"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pkey_mprotect"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"prctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"prlimit64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"process_madvise"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"process_vm_writev"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pwrite64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pwritev"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pwritev2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"quotactl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"quotactl_fd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rename"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"renameat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"renameat2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rseq"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigaction"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigpending"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigprocmask"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigqueueinfo"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigreturn"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rtas"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"s390_pci_mmio_write"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"s390_guarded_storage"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_setaffinity"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_setattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_setparam"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_setscheduler"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"seccomp"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"semctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"semop"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"semtimedop"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_mempolicy"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_robust_list"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_thread_area"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_tid_address"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_tls"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_domainname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"set_fsgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setfsgid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setfsuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setfsuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setgid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sethae"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sethostimer"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setitimer"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setns"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setpgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setpgrp"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setpriority"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setregid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setregid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setresgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setresgid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setresuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setresuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setreuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setreuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setrlimit"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setsid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setsockopt"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"settimeofday"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setup"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"shmat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"shmctl"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"shmdt"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sigaction"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sigaltstack"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"subpage_prot"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"swapcontext"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"switch_endian"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sys_debug_setcontext"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timer_settime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timerfd_settime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"truncate"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"truncate64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"umask"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"utime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"utimesat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"utimes"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"write"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"writev"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"xtensa"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["change"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"accept"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"accept4"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"connect"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mq_timedreceive"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mq_timedsend"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"msgrcv"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"msgsnd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pidfd_send_signal"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"recv"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"recvfrom"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"recvmsg"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"recvmmsg"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigqueueinfo"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_tgsigqueueinfo"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"send"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sendfile"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sendfile64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sendmmsg"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sendmsg"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sendto"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"signal"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"signalfd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"signalfd4"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"socket"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"socketcall"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"socketpair"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"syscall"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["connection"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"add_key"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clone"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clone2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clone3"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"copy_file_range"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"creat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"create_module"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"dup"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"dup2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"dup3"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"epoll_create"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"epoll_create1"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"eventfd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"eventfd2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fdatasync"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fork"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fsmount"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fsync"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"init_module"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"inotify_add_watch"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"inotify_init"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"inotify_init1"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"kexec_file_load"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"kexec_load"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"landlock_add_rule"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"landlock_create_ruleset"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"link"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"linkat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"memfd_create"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"memfd_secret"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mkdir"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mkdirat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mknod"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mknodat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mount"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mq_notify"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mq_open"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"msync"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pidfd_getfd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pipe"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pipe2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"remap_file_pages"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"splice"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"spu_create"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"symlink"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"symlinkat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sync"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sync_file_range"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sync_file_range2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"syncfs"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"tee"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timer_create"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timerfd_create"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"vfork"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"vmsplice"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["creation"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"cacheflush"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"delete_module"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fremovexattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"inotify_rm_watch"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_destroy"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lremovexattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mq_unlink"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"oldumount"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"removexattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"riscv_flush_icache"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rmdir"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"spill"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timer_delete"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"umount"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"unlink"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"unlinkat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"unshare"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["deletion"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"exit"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"exit_group"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_cancel"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"kill"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"shutdown"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"swapoff"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"tgkill"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"tkill"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"vhangup"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["end"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fanotify_init"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fanotify_mark"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setgroups"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"setgroups32"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["group"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"alloc_hugepages"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"capget"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clock_getres"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"clock_gettime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"epoll_pwait"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"epoll_pwait2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"epoll_wait"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fadvise64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fadvise64_64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fgetxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"flistxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fspick"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fstat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fstat64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fstatat64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fstatfs"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"fstatfs64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"futex"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"get_kernel_syms"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"get_mempolicy"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"get_robust_list"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"get_thread_area"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"get_tls"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getcpu"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getcwd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getdents"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getdents64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getdomainname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getdtablesize"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getegid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getegid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"geteuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"geteuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getgid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getgroups"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getgroups32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"gethostname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getitimer"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getpeername"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getpagesize"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getpgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getpgrp"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getpid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getppid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getpriority"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getrandom"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getresgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getresgid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getresuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getresuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getrlimit"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getrusage"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getsid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getsockname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getsockopt"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"gettid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"gettimeofday"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getuid32"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getunwind"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getxgid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getxpid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"getxuid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_getevents"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_pgetevents"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_submit"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_uring_register"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ioprio_get"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"kcmp"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"kern_features"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lgetxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"listen"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"listxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"llistxattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lookup_dcookie"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lstat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"lstat64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"mincore"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"msgget"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"newfstatat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"old_getrlimit"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"old_fstat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"oldlstat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"oldolduname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"oldstat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"olduname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"or1k_atomic"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pciconfig_iobase"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"poll"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ppoll"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pread64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"preadv"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"preadv2"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"process_vm_readv"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"pselect6"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"query_module"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"request_key"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigsuspend"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"rt_sigtimedwait"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"s390_runtime_instr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"s390_sthyi"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_get_affinity"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_get_priority_max"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_get_priority_min"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_getaffinity"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_getattr"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_getparam"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_getscheduler"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_rr_get_interval"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sched_yield"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"select"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"semget"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sgetmask"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"shmget"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sigpending"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sigprocmask"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sigreturn"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sigsuspend"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ssetmask"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"stat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"stat64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"statfs"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"statfs64"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"statx"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"stime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sysfs"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sysinfo"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"syslog"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"sysmips"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"time"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timer_getoverrun"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timer_gettime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"timerfd_gettime"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"times"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ugetrlimit"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"uname"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ustat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"wait"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"wait4"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"waitid"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"waitpid"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"utrap_install"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["installation"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"ipc"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["protocol"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"execve"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"execveat"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"execv"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"io_uring_enter"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"perf_event_open"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"spu_run"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"swapon"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["start"]}}}}],"alt":[{"If":{"test":[[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"userfaultfd"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"usr26"},"negated":false,"lowered":false}}],[{"Compare":{"path":"falco.output_fields.evt.type","wanted":{"Text":"usr32"},"negated":false,"lowered":false}}]],"then":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["user"]}}}}],"alt":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}]}}],"alt":[{"Literal":{"Set":{"path":"event.type","value":{"Literal":["info"]}}}}]}}],"whole":false}}]}"#,
                    r#"def allowedValues = [\n    'access',\n    'admin',\n    'allowed',\n    'change',\n    'connection',\n    'creation',\n    'deletion',\n    'denied',\n    'end',\n    'error',\n    'group',\n    'indicator',\n    'info',\n    'installation',\n    'protocol',\n    'start',\n    'user'\n];\nif (ctx?.falco?.output_fields?.evt != null && ctx?.falco?.output_fields?.evt?.type != null) {\n    def inputType = ctx?.falco?.output_fields?.evt?.type;\n    def lowercaseType = inputType.toLowerCase();\n    if (allowedValues.contains(lowercaseType)) {\n        ctx.event.type = [inputType];\n    } else if (inputType == 'faccessat' || inputType == 'faccessat2' || inputType == 'fsopen' || inputType == 'name_to_handle_at' || inputType == 'nfsservctl' || inputType == 'open' || inputType == 'open_by_handle_at' || inputType == 'open_tree' || inputType == 'openat' || inputType == 'openat2' || inputType == 'pciconfig_read' || inputType == 'pidfd_open' || inputType == 'read' || inputType == 'readahead' || inputType == 'readdir' || inputType == 'readlink' || inputType == 'readlinkat' || inputType == 'readv' || inputType == 's390_pci_mmio_read' || inputType == 'uselib' || inputType == 'vm86old' || inputType == 'vm86') {\n        ctx.event.type = ['access'];\n    } else if (inputType == 'bdflush' || inputType == 'membarrier' || inputType == 'ptrace' || inputType == 'reboot' || inputType == 'restart_syscall') {\n        ctx.event.type = ['admin'];\n    } else if (inputType == 'fallocate' || inputType == 'finit_module') {\n        ctx.event.type = ['allowed'];\n    } else if (inputType == 'llseek' || inputType == 'sysctl' || inputType == 'acct' || inputType == 'adjtimex' || inputType == 'alarm' || inputType == 'arch_prctl' || inputType == 'bind' || inputType == 'bpf' || inputType == 'brk' || inputType == 'capset' || inputType == 'chdir' || inputType == 'chmod' || inputType == 'chown' || inputType == 'chown32' || inputType == 'chroot' || inputType == 'clock_adjtime' || inputType == 'clock_nanosleep' || inputType == 'clock_settime' || inputType == 'close' || inputType == 'close_range' || inputType == 'epoll_ctl' || inputType == 'fchdir' || inputType == 'fchmod' || inputType == 'fchmodat' || inputType == 'fchown' || inputType == 'fchown32' || inputType == 'fchownat' || inputType == 'fcntl' || inputType == 'fcntl64' || inputType == 'flock' || inputType == 'free_hugepages' || inputType == 'fsetxattr' || inputType == 'fsconfig' || inputType == 'ftruncate' || inputType == 'ftruncate64' || inputType == 'futimesat' || inputType == 'io_setup' || inputType == 'io_uring_setup' || inputType == 'ioctl' || inputType == 'ioperm' || inputType == 'iopl' || inputType == 'ioprio_set' || inputType == 'keyctl' || inputType == 'landlock_restrict_self' || inputType == 'lchown' || inputType == 'lchown32' || inputType == 'madvise' || inputType == 'mbind' || inputType == 'memory_ordering' || inputType == 'migrate_pages' || inputType == 'mlock' || inputType == 'mlock2' || inputType == 'mlockall' || inputType == 'mmap' || inputType == 'mmap2' || inputType == 'modify_ldt' || inputType == 'move_mount' || inputType == 'move_pages' || inputType == 'mprotect' || inputType == 'mq_getsetattr' || inputType == 'mremap' || inputType == 'msgctl' || inputType == 'munlock' || inputType == 'munlockall' || inputType == 'munmap' || inputType == 'nanosleep' || inputType == 'nice' || inputType == 'old_adjtimex' || inputType == 'pause' || inputType == 'pciconfig_write' || inputType == 'personality' || inputType == 'perfctr' || inputType == 'perfmonctl' || inputType == 'pivot_root' || inputType == 'pkey_alloc' || inputType == 'pkey_free' || inputType == 'pkey_mprotect' || inputType == 'prctl' || inputType == 'prlimit64' || inputType == 'process_madvise' || inputType == 'process_vm_writev' || inputType == 'pwrite64' || inputType == 'pwritev' || inputType == 'pwritev2' || inputType == 'quotactl' || inputType == 'quotactl_fd' || inputType == 'rename' || inputType == 'renameat' || inputType == 'renameat2' || inputType == 'rseq' || inputType == 'rt_sigaction' || inputType == 'rt_sigpending' || inputType == 'rt_sigprocmask' || inputType == 'rt_sigqueueinfo' || inputType == 'rt_sigreturn' || inputType == 'rtas' || inputType == 's390_pci_mmio_write' || inputType == 's390_guarded_storage' || inputType == 'sched_setaffinity' || inputType == 'sched_setattr' || inputType == 'sched_setparam' || inputType == 'sched_setscheduler' || inputType == 'seccomp' || inputType == 'semctl' || inputType == 'semop' || inputType == 'semtimedop' || inputType == 'set_mempolicy' || inputType == 'set_robust_list' || inputType == 'set_thread_area' || inputType == 'set_tid_address' || inputType == 'set_tls' || inputType == 'set_domainname' || inputType == 'set_fsgid' || inputType == 'setfsgid32' || inputType == 'setfsuid' || inputType == 'setfsuid32' || inputType == 'setgid' || inputType == 'setgid32' || inputType == 'sethae' || inputType == 'sethostimer' || inputType == 'setitimer' || inputType == 'setns' || inputType == 'setpgid' || inputType == 'setpgrp' || inputType == 'setpriority' || inputType == 'setregid' || inputType == 'setregid32' || inputType == 'setresgid' || inputType == 'setresgid32' || inputType == 'setresuid' || inputType == 'setresuid32' || inputType == 'setreuid' || inputType == 'setreuid32' || inputType == 'setrlimit' || inputType == 'setsid' || inputType == 'setsockopt' || inputType == 'settimeofday' || inputType == 'setuid' || inputType == 'setuid32' || inputType == 'setup' || inputType == 'setxattr' || inputType == 'shmat'  || inputType == 'shmctl' || inputType == 'shmdt' || inputType == 'sigaction' || inputType == 'sigaltstack' || inputType == 'subpage_prot' || inputType == 'swapcontext' || inputType == 'switch_endian' || inputType == 'sys_debug_setcontext' || inputType == 'timer_settime' || inputType == 'timerfd_settime' || inputType == 'truncate' || inputType == 'truncate64' || inputType == 'umask' || inputType == 'utime' || inputType == 'utimesat' || inputType == 'utimes' || inputType == 'write' || inputType == 'writev' || inputType == 'xtensa') {\n        ctx.event.type = ['change'];\n    } else if (inputType == 'accept' || inputType == 'accept4' || inputType == 'connect' || inputType == 'mq_timedreceive' || inputType == 'mq_timedsend' || inputType == 'msgrcv' || inputType == 'msgsnd' || inputType == 'pidfd_send_signal' || inputType == 'recv' || inputType == 'recvfrom' || inputType == 'recvmsg' || inputType == 'recvmmsg' || inputType == 'rt_sigqueueinfo' || inputType == 'rt_tgsigqueueinfo' || inputType == 'send' || inputType == 'sendfile' || inputType == 'sendfile64' || inputType == 'sendmmsg' || inputType == 'sendmsg' || inputType == 'sendto' || inputType == 'signal' || inputType == 'signalfd' || inputType == 'signalfd4' || inputType == 'socket' || inputType == 'socketcall' || inputType == 'socketpair' || inputType == 'syscall') {\n        ctx.event.type = ['connection'];\n    } else if (inputType == 'add_key' || inputType == 'clone' || inputType == 'clone2' || inputType == 'clone3' || inputType == 'copy_file_range' || inputType == 'creat' || inputType == 'create_module' || inputType == 'dup' || inputType == 'dup2' || inputType == 'dup3' || inputType == 'epoll_create' || inputType == 'epoll_create1' || inputType == 'eventfd' || inputType == 'eventfd2' || inputType == 'fdatasync' || inputType == 'fork' || inputType == 'fsmount' || inputType == 'fsync' || inputType == 'init_module' || inputType == 'inotify_add_watch' || inputType == 'inotify_init' || inputType == 'inotify_init1' || inputType == 'kexec_file_load' || inputType == 'kexec_load' || inputType == 'landlock_add_rule' || inputType == 'landlock_create_ruleset' || inputType == 'link' || inputType == 'linkat' || inputType == 'memfd_create' || inputType == 'memfd_secret' || inputType == 'mkdir' || inputType == 'mkdirat' || inputType == 'mknod' || inputType == 'mknodat' || inputType == 'mount' || inputType == 'mq_notify' || inputType == 'mq_open' || inputType == 'msync' || inputType == 'pidfd_getfd' || inputType == 'pipe' || inputType == 'pipe2' || inputType == 'remap_file_pages' || inputType == 'splice' || inputType == 'spu_create' || inputType == 'symlink' || inputType == 'symlinkat' || inputType == 'sync' || inputType == 'sync_file_range' || inputType == 'sync_file_range2' || inputType == 'syncfs' || inputType == 'tee' || inputType == 'timer_create' || inputType == 'timerfd_create' || inputType == 'vfork' || inputType == 'vmsplice') {\n        ctx.event.type = ['creation'];\n    } else if (inputType == 'cacheflush' || inputType == 'delete_module' || inputType == 'fremovexattr' || inputType == 'inotify_rm_watch' || inputType == 'io_destroy' || inputType == 'lremovexattr' || inputType == 'mq_unlink' || inputType == 'oldumount' || inputType == 'removexattr' || inputType == 'riscv_flush_icache' || inputType == 'rmdir' || inputType == 'spill' || inputType == 'timer_delete' || inputType == 'umount' || inputType == 'unlink' || inputType == 'unlinkat' || inputType == 'unshare') {\n        ctx.event.type = ['deletion'];\n    } else if (inputType == 'exit' || inputType == 'exit_group' || inputType == 'io_cancel' || inputType == 'kill' || inputType == 'shutdown'  || inputType == 'swapoff' || inputType == 'tgkill' || inputType == 'tkill' || inputType == 'vhangup') {\n        ctx.event.type = ['end'];\n    } else if (inputType == 'fanotify_init' || inputType == 'fanotify_mark' || inputType == 'setgroups' || inputType == 'setgroups32') {\n        ctx.event.type = ['group'];\n    } else if (inputType == 'alloc_hugepages' || inputType == 'capget' || inputType == 'clock_getres' || inputType == 'clock_gettime' || inputType == 'epoll_pwait' || inputType == 'epoll_pwait2' || inputType == 'epoll_wait' || inputType == 'fadvise64' || inputType == 'fadvise64_64' || inputType == 'fgetxattr' || inputType == 'flistxattr' || inputType == 'fspick' || inputType == 'fstat' || inputType == 'fstat64' || inputType == 'fstatat64' || inputType == 'fstatfs' || inputType == 'fstatfs64' || inputType == 'futex' || inputType == 'get_kernel_syms' || inputType == 'get_mempolicy' || inputType == 'get_robust_list' || inputType == 'get_thread_area' || inputType == 'get_tls' || inputType == 'getcpu' || inputType == 'getcwd' || inputType == 'getdents' || inputType == 'getdents64' || inputType == 'getdomainname' || inputType == 'getdtablesize' || inputType == 'getegid' || inputType == 'getegid32' || inputType == 'geteuid' || inputType == 'geteuid32' || inputType == 'getgid' || inputType == 'getgid32' || inputType == 'getgroups' || inputType == 'getgroups32' || inputType == 'gethostname' || inputType == 'getitimer' || inputType == 'getpeername' || inputType == 'getpagesize' || inputType == 'getpgid' || inputType == 'getpgrp' || inputType == 'getpid' || inputType == 'getppid' || inputType == 'getpriority' || inputType == 'getrandom' || inputType == 'getresgid' || inputType == 'getresgid32' || inputType == 'getresuid' || inputType == 'getresuid32' || inputType == 'getrlimit' || inputType == 'getrusage' || inputType == 'getsid' || inputType == 'getsockname' || inputType == 'getsockopt' || inputType == 'gettid' || inputType == 'gettimeofday' || inputType == 'getuid' || inputType == 'getuid32' || inputType == 'getunwind' || inputType == 'getxattr' || inputType == 'getxgid' || inputType == 'getxpid' || inputType == 'getxuid' || inputType == 'io_getevents' || inputType == 'io_pgetevents' || inputType == 'io_submit' || inputType == 'io_uring_register' || inputType == 'ioprio_get' || inputType == 'kcmp' || inputType == 'kern_features' || inputType == 'lgetxattr' || inputType == 'listen' || inputType == 'listxattr' || inputType == 'llistxattr' || inputType == 'lookup_dcookie' || inputType == 'lstat' || inputType == 'lstat64' || inputType == 'mincore' || inputType == 'msgget' || inputType == 'newfstatat' || inputType == 'old_getrlimit' || inputType == 'old_fstat' || inputType == 'oldlstat' || inputType == 'oldolduname' || inputType == 'oldstat' || inputType == 'olduname' || inputType == 'or1k_atomic' || inputType == 'pciconfig_iobase' || inputType == 'poll' || inputType == 'ppoll' || inputType == 'pread64' || inputType == 'preadv' || inputType == 'preadv2' || inputType == 'process_vm_readv' || inputType == 'pselect6' || inputType == 'query_module' || inputType == 'request_key' || inputType == 'rt_sigsuspend' || inputType == 'rt_sigtimedwait' || inputType == 's390_runtime_instr' || inputType == 's390_sthyi' || inputType == 'sched_get_affinity' || inputType == 'sched_get_priority_max' || inputType == 'sched_get_priority_min' || inputType == 'sched_getaffinity' || inputType == 'sched_getattr' || inputType == 'sched_getparam' || inputType == 'sched_getscheduler' || inputType == 'sched_rr_get_interval' || inputType == 'sched_yield' || inputType == 'select' || inputType == 'semget' || inputType == 'sgetmask' || inputType == 'shmget' || inputType == 'sigpending' || inputType == 'sigprocmask' || inputType == 'sigreturn' || inputType == 'sigsuspend' || inputType == 'ssetmask' || inputType == 'stat' || inputType == 'stat64' || inputType == 'statfs' || inputType == 'statfs64' || inputType == 'statx' || inputType == 'stime' || inputType == 'sysfs' || inputType == 'sysinfo' || inputType == 'syslog' || inputType == 'sysmips' || inputType == 'time' || inputType == 'timer_getoverrun' || inputType == 'timer_gettime' || inputType == 'timerfd_gettime' || inputType == 'times' || inputType == 'ugetrlimit' || inputType == 'uname' || inputType == 'ustat' || inputType == 'wait' || inputType == 'wait4' || inputType == 'waitid' || inputType == 'waitpid') {\n        ctx.event.type = ['info'];\n    } else if (inputType == 'utrap_install') {\n        ctx.event.type = ['installation'];\n    } else if (inputType == 'ipc') {\n        ctx.event.type = ['protocol'];\n    } else if (inputType == 'execve' || inputType == 'execveat' || inputType == 'execv' || inputType == 'io_uring_enter' || inputType == 'perf_event_open' || inputType == 'spu_run' || inputType == 'swapon') {\n        ctx.event.type = ['start'];\n    } else if (inputType == 'userfaultfd' || inputType == 'usr26' || inputType == 'usr32') {\n        ctx.event.type = ['user'];\n    } else {\n        ctx.event.type = ['info'];\n    }\n} else {\n    ctx.event.type = ['info'];\n}\n"#
                ),
            )?;

            let _cond = {
                event.has_value("falco.output_fields.evt.res")
                    && event.get_str("falco.output_fields.evt.res") == Some("SUCCESS")
            };
            if _cond {
                event.set("event.outcome", json!("success"))?;
            }

            let _cond = {
                event.has_value("falco.output_fields.evt.res")
                    && event.get_str("falco.output_fields.evt.res") != Some("SUCCESS")
                    && event.has_value("falco.output_fields.evt.failed")
                    && event.get_bool("falco.output_fields.evt.failed") == Some(true)
            };
            if _cond {
                event.set("event.outcome", json!("failure"))?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                let v = json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                );
                if !painless_is_empty_value(&v) {
                    event.set("event.ingested", v)?;
                }
                Ok(())
            })();

            let _cond = { event.has_value("falco.tags") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "ae2bd1a0082c13172463b78c9272efe7ba9450994168c86b313915263b31e698",
                        r#"{"params":null,"known":[{"FirstMatchInList":{"list":"falco.tags","target":"threat.technique.id","pattern":"T\\d{4}","matcher":"T\\d{4}","wrap_in_list":true}}]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("falco.tags") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan(
                    event,
                    compiled_painless!(
                        "9fde1249f7fa13c345a33595974c3f738d799a291b7a14cc0bf6290f208cfdcf",
                        r#"{"params":null,"known":[{"FirstMatchInList":{"list":"falco.tags","target":"threat.technique.subtechnique.id","pattern":"T\\d{4}.\\d{3}","matcher":"T\\d{4}.\\d{3}","wrap_in_list":true}}]}"#
                    ),
                )?;
            }

            let _cond = { event.has_value("falco.priority") };
            if _cond {
                // The pipeline's script, run by the runtime's equivalent
                painless_exec_plan_params(
                    event,
                    compiled_painless!(
                        "66807186b3c9a6a5076b08bcc683bc49d5e5ada9706796f3d73cb8784fab3154",
                        r#"{"params":{"NormalisedLookup":{"source":"falco.priority","target":"event.severity","fold":"Lower"}},"known":[]}"#
                    ),
                    cached_params!(
                        "{\"emergency\":99,\"alert\":99,\"critical\":99,\"error\":73,\"warning\":47,\"notice\":47,\"informational\":21,\"debug\":21}"
                    ),
                )?;
            }

            if let Some(v) = event
                .get("falco.rule")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("rule.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.evt.num")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.sequence", v)?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "e8aad7030a496c6d5eb2419fa1081bcb8256478bfe1d76e13862ce169123c1ed",
                    r#"{"params":null,"known":[{"LongDivide":{"source":"falco.output_fields.evt.time.iso8601","target":"falco.output_fields.evt.time.iso8601","divisor":1000000}}]}"#
                ),
            )?;

            if let Some(v) = event
                .get("falco.source")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("event.provider", v)?;
            }

            if let Some(v) = event
                .get("falco.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("user.name", v)?;
            }

            if let Some(v) = event
                .get("falco.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("observer.hostname", v)?;
            }

            if let Some(v) = event
                .get("falco.hostname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("host.name", v)?;
            }

            let _cond = { event.has_value("falco.hostname") };
            if _cond {
                event.append_unique(
                    "related.hosts",
                    json!(
                        event
                            .get("falco.hostname")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.exepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.executable", v)?;
            }

            let _cond = {
                event.has_value("falco.output_fields.fd.type")
                    && (event.get_str("falco.output_fields.fd.type") == Some("file")
                        || event.get_str("falco.output_fields.fd.type") == Some("directory"))
            };
            if _cond {
                event.set(
                    "file.path",
                    json!(
                        event
                            .get("falco.output_fields.fd.name")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            let _cond = {
                event.has_value("falco.output_fields.fd.type")
                    && (event.get_str("falco.output_fields.fd.type") == Some("file")
                        || event.get_str("falco.output_fields.fd.type") == Some("directory"))
            };
            if _cond {
                event.set(
                    "file.type",
                    json!(
                        event
                            .get("falco.output_fields.fd.type")
                            .map_or_else(String::new, template_to_string)
                    ),
                )?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.pexepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.executable", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.pname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.name", v)?;
            }

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "b60fb1748b0cb253b015de33150b9ec607d200dd0c4abfcf2620467b64408a10",
                    r#"{"params":null,"known":[{"PrependSplit":{"head":"falco.output_fields.proc.exepath","list":"falco.output_fields.proc.args","separator":" ","target":"process.args"}}]}"#
                ),
            )?;

            if let Some(v) = event
                .get("falco.output_fields.proc.cmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.command_line", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.pcmdline")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.command_line", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.cmdnargs")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.args_count", v)?;
            }

            if event.has_value("falco.output_fields.proc.env") {
                if let Some(s) = event.get_string("falco.output_fields.proc.env") {
                    let mut parts: Vec<Value> = cached_regex!("\\s+")
                        .split(&s)
                        .into_iter()
                        .map(|p| json!(p))
                        .collect();
                    if parts.len() > 1 {
                        while parts.last().and_then(Value::as_str) == Some("") {
                            parts.pop();
                        }
                    }
                    event.set("process.env_vars", Value::Array(parts))?;
                }
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.cwd")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.working_directory", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.process.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.pid", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.process.parent.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.pid", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.vpid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.vpid", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.pvpid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.vpid", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.process.session_leader.pid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.session_leader.pid", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.sname")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.session_leader.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.sid.exepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.session_leader.executable", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.process.group_leader.vpid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.group_leader.vpid", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.vpgid.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.group_leader.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.vpgid.exepath")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.group_leader.executable", v)?;
            }

            if event.has_value("falco.output_fields.proc.duration") {
                if let Some(val) = event.get("falco.output_fields.proc.duration") {
                    let converted = convert_value(val, "long").map_err(|message| {
                        TransformError::ParseError {
                            path: "falco.output_fields.proc.duration".into(),
                            message,
                        }
                    })?;
                    event.set("process.uptime", converted)?;
                }
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.ppid.duration")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.uptime", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.pid.ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.start", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.ppid.ts")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.parent.start", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.is_sid_leader")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.session_leader.same_as_process", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.proc.is_vpgid_leader")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.group_leader.same_as_process", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.thread.cap_permitted")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.thread.capabilities.permitted", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.thread.cap_effective")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.thread.capabilities.effective", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.thread.tid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.thread.id", v)?;
            }

            if event.has_value("falco.output_fields.user.uid") {
                if let Some(val) = event.get("falco.output_fields.user.uid") {
                    let converted = convert_value(val, "string").map_err(|message| {
                        TransformError::ParseError {
                            path: "falco.output_fields.user.uid".into(),
                            message,
                        }
                    })?;
                    event.set("falco.output_fields.user.uid", converted)?;
                }
            }

            if let Some(v) = event
                .get("falco.output_fields.user.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.id", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.user.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.user.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.group.gid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.group.id", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.group.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("process.group.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.container.id")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.id", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.container.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.container.image.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.image.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.container.type")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.runtime", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.container.privileged")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.security_context.privileged", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.container.image.digest")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("container.image.hash.all", v)?;
            }

            let _cond = { event.has_value("falco.output_fields.container.ip") };
            if _cond {
                let v = Value::Array(vec![json!(
                    event
                        .get("falco.output_fields.container.ip")
                        .map_or_else(String::new, template_to_string)
                )]);
                if !painless_is_empty_value(&v) {
                    event.set("host.ip", v)?;
                }
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.directory")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.directory", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.filename")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.name", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("falco.output_fields.client.ip") {
                    if let Some(val) = event.get("falco.output_fields.client.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "falco.output_fields.client.ip".into(),
                                message,
                            }
                        })?;
                        event.set("client.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("client.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.address", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("falco.output_fields.server.ip") {
                    if let Some(val) = event.get("falco.output_fields.server.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "falco.output_fields.server.ip".into(),
                                message,
                            }
                        })?;
                        event.set("server.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("server.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.address", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("falco.output_fields.source.ip") {
                    if let Some(val) = event.get("falco.output_fields.source.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "falco.output_fields.source.ip".into(),
                                message,
                            }
                        })?;
                        event.set("source.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("source.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.address", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("falco.output_fields.destination.ip") {
                    if let Some(val) = event.get("falco.output_fields.destination.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "falco.output_fields.destination.ip".into(),
                                message,
                            }
                        })?;
                        event.set("destination.ip", converted)?;
                    }
                }
                Ok(())
            })();

            if let Some(v) = event
                .get("destination.ip")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.address", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.cport")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.port", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.sport")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.port", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.lport")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.port", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.rport")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.port", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.cip.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("client.domain", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.sip.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("server.domain", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.lip.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("source.domain", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.rip.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("destination.domain", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.fd.ino")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("file.inode", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.k8s.ns.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.namespace", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.k8s.pod.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.name", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.k8s.pod.uid")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.id", v)?;
            }

            if let Some(v) = event
                .get("falco.output_fields.k8s.pod.labels")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("orchestrator.resource.label", v)?;
            }

            // ignore_failure: true
            let _ = (|| -> Result<()> {
                if event.has_value("falco.output_fields.k8s.pod.ip") {
                    if let Some(val) = event.get("falco.output_fields.k8s.pod.ip") {
                        let converted = convert_value(val, "ip").map_err(|message| {
                            TransformError::ParseError {
                                path: "falco.output_fields.k8s.pod.ip".into(),
                                message,
                            }
                        })?;
                        event.set("orchestrator.resource.ip", converted)?;
                    }
                }
                Ok(())
            })();

            let _cond = { event.has_value("falco.output_fields.k8s.pod.name") };
            if _cond {
                event.set("orchestrator.resource.type", json!("pod"))?;
            }

            event.set("observer.type", json!("sensor"))?;

            event.set("observer.vendor", json!("sysdig"))?;

            event.set("observer.product", json!("falco"))?;

            // The pipeline's script, run by the runtime's equivalent
            painless_exec_plan(
                event,
                compiled_painless!(
                    "c7512668fe97917cf3ac14c71ec29baaa1c0194e14e87976bcee4b0675384efa",
                    r#"{"params":null,"known":[{"SplitIntoRecords":{"source":"falco.output_fields.container.mounts","target":"falco.container.mounts","outer":" ","inner":":","members":[["source",0],["dest",1],["mode",2],["rdrw",3],["propagation",4]],"null_when_absent":true}}]}"#
                ),
            )?;

            event.remove("message");
            event.remove("falco.output_fields.evt.arg.flags");
            event.remove("falco.output_fields.proc.aname[2]");
            event.remove("falco.output_fields.proc.aname[3]");
            event.remove("falco.output_fields.proc.aname[4]");
            event.remove("falco.output_fields.proc.aname[5]");
            event.remove("falco.output_fields.proc.aname[6]");
            event.remove("falco.output_fields.proc.aname[7]");

            let _cond = {
                !event.has_value("tags")
                    || !(event.get("tags").is_some_and(|v| match v {
                        serde_json::Value::Array(a) => a
                            .iter()
                            .any(|x| x.as_str() == Some("preserve_falco_fields")),
                        serde_json::Value::String(s) => s.contains("preserve_falco_fields"),
                        _ => false,
                    }))
            };
            if _cond {
                event.remove("falco.rule");
                event.remove("falco.output_fields.evt.num");
                event.remove("falco.output_fields.evt.time");
                event.remove("falco.output_fields.proc.exepath");
                event.remove("falco.output_fields.proc.pexepath");
                event.remove("falco.output_fields.proc.name");
                event.remove("falco.output_fields.proc.pname");
                event.remove("falco.output_fields.proc.cmdline");
                event.remove("falco.output_fields.proc.pcmdline");
                event.remove("falco.output_fields.proc.cmdnargs");
                event.remove("falco.output_fields.proc.env");
                event.remove("falco.output_fields.proc.cwd");
                event.remove("falco.output_fields.proc.vpid");
                event.remove("falco.output_fields.proc.pvpid");
                event.remove("falco.output_fields.proc.sname");
                event.remove("falco.output_fields.proc.sid.exepath");
                event.remove("falco.output_fields.proc.vpgid");
                event.remove("falco.output_fields.proc.vpgid.name");
                event.remove("falco.output_fields.proc.vpgid.exepath");
                event.remove("falco.output_fields.proc.duration");
                event.remove("falco.output_fields.proc.ppid.duration");
                event.remove("falco.output_fields.proc.pid.ts");
                event.remove("falco.output_fields.proc.ppid.ts");
                event.remove("falco.output_fields.proc.is_sid_leader");
                event.remove("falco.output_fields.proc.is_vpgid_leader");
                event.remove("falco.output_fields.thread.cap_permitted");
                event.remove("falco.output_fields.thread.cap_effective");
                event.remove("falco.output_fields.thread.tid");
                event.remove("falco.output_fields.user.uid");
                event.remove("falco.output_fields.user.name");
                event.remove("falco.output_fields.group.gid");
                event.remove("falco.output_fields.group.name");
                event.remove("falco.output_fields.container.id");
                event.remove("falco.output_fields.container.name");
                event.remove("falco.output_fields.container.type");
                event.remove("falco.output_fields.container.privileged");
                event.remove("falco.output_fields.container.image.digest");
                event.remove("falco.output_fields.container.ip");
                event.remove("falco.output_fields.fd.directory");
                event.remove("falco.output_fields.fd.filename");
                event.remove("falco.output_fields.fd.cport");
                event.remove("falco.output_fields.fd.sport");
                event.remove("falco.output_fields.fd.lport");
                event.remove("falco.output_fields.fd.rport");
                event.remove("falco.output_fields.fd.cip.name");
                event.remove("falco.output_fields.fd.sip.name");
                event.remove("falco.output_fields.fd.lip");
                event.remove("falco.output_fields.fd.lip.name");
                event.remove("falco.output_fields.fd.rip.name");
                event.remove("falco.output_fields.fd.ino");
                event.remove("falco.output_fields.k8s.ns.name");
                event.remove("falco.output_fields.k8s.pod.name");
                event.remove("falco.output_fields.k8s.pod.uid");
                event.remove("falco.output_fields.k8s.pod.labels");
                event.remove("falco.output_fields.k8s.pod.ip");
            }

            if let Some(v) = event
                .get("rule.name")
                .filter(|v| !painless_is_empty_value(v))
                .cloned()
            {
                event.set("message", v)?;
            }

            Ok(TransformResult::Continue)
        })(event);

        match outcome {
            Ok(TransformResult::Drop) => return Ok(TransformResult::Drop),
            Ok(_) => {}
            Err(err) => {
                event.set("_ingest.on_failure_message", err.to_string())?;
                event.append("error.message", json!(format!("Processor \"{}\" with tag \"{}\" in pipeline \"{}\" failed with message \"{}\"", event.get("_ingest.on_failure_processor_type").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_processor_tag").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_pipeline").map_or_else(String::new, template_to_string), event.get("_ingest.on_failure_message").map_or_else(String::new, template_to_string))))?;
                event.set("event.kind", json!("pipeline_error"))?;
                event.append_unique("tags", json!("preserve_original_event"))?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
