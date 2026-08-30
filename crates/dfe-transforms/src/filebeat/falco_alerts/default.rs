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
                // Painless script
                // Source: def m = new HashMap(); for (def v : params.fields) {\n    if (ctx.containsKey(v)) {\n        m[v] = ctx[v];\n        ctx.remove(v);\n    }\n} ctx['falco'] = m; def original = Json.dump(ctx.falco); if (original != null) {\n    if (ctx.event == null) {\n        ctx.event = new HashMap();\n    }\n    ctx.event.original = original;\n}
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def m = new HashMap(); for (def v : params.fields) {\n    if (ctx.containsKey(v)) {\n        m[v] = ctx[v];\n        ctx.remove(v);\n    }\n} ctx['falco'] = m; def original = Json.dump(ctx.falco); if (original != null) {\n    if (ctx.event == null) {\n        ctx.event = new HashMap();\n    }\n    ctx.event.original = original;\n}"#
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

            // Painless script
            // Source: def allowedValues = [\n    'api',\n    'authentication',\n    'configuration',\n    'database',\n    'driver',\n    'email',\n    'file',\n    'host',\n    'iam',\n    'intrusion_detection',\n    'library',\n    'malware',\n    'network',\n    'package',\n    'process',\n    'registry',\n    'session',\n    'threat',\n    'vulnerability',\n    'web'\n];\n\nif (ctx?.falco?.output_fields?.evt != null && ctx?.falco?.output_fields?.evt?.category != null) {\n    def inputCategory = ctx?.falco?.output_fields?.evt?.category;\n    def lowercaseCategory = inputCategory.toLowerCase();\n    if (allowedValues.contains(lowercaseCategory)) {\n        ctx.event.category = [inputCategory];\n    } else if (inputCategory == 'time' || inputCategory == 'scheduler') {\n        ctx.event.category = ['configuration'];\n    } else if (inputCategory == 'system' || inputCategory == 'memory' || inputCategory == 'sleep' || inputCategory == 'wait' || inputCategory == 'internal') {\n        ctx.event.category = ['host'];\n    } else if (inputCategory == 'ipc' || inputCategory == 'net' || inputCategory == 'signal') {\n        ctx.event.category = ['network'];\n    } else if (inputCategory == 'processing' || inputCategory == 'process' || inputCategory == 'io_read' || inputCategory == 'io_write' || inputCategory == 'io_other') {\n        ctx.event.category = ['process'];\n    } else if (inputCategory == 'user') {\n        ctx.event.category = ['session'];\n    } else {\n        ctx.event.category = ['process'];\n    }\n} else {\n    ctx.event.category = ['process'];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"def allowedValues = [\n    'api',\n    'authentication',\n    'configuration',\n    'database',\n    'driver',\n    'email',\n    'file',\n    'host',\n    'iam',\n    'intrusion_detection',\n    'library',\n    'malware',\n    'network',\n    'package',\n    'process',\n    'registry',\n    'session',\n    'threat',\n    'vulnerability',\n    'web'\n];\n\nif (ctx?.falco?.output_fields?.evt != null && ctx?.falco?.output_fields?.evt?.category != null) {\n    def inputCategory = ctx?.falco?.output_fields?.evt?.category;\n    def lowercaseCategory = inputCategory.toLowerCase();\n    if (allowedValues.contains(lowercaseCategory)) {\n        ctx.event.category = [inputCategory];\n    } else if (inputCategory == 'time' || inputCategory == 'scheduler') {\n        ctx.event.category = ['configuration'];\n    } else if (inputCategory == 'system' || inputCategory == 'memory' || inputCategory == 'sleep' || inputCategory == 'wait' || inputCategory == 'internal') {\n        ctx.event.category = ['host'];\n    } else if (inputCategory == 'ipc' || inputCategory == 'net' || inputCategory == 'signal') {\n        ctx.event.category = ['network'];\n    } else if (inputCategory == 'processing' || inputCategory == 'process' || inputCategory == 'io_read' || inputCategory == 'io_write' || inputCategory == 'io_other') {\n        ctx.event.category = ['process'];\n    } else if (inputCategory == 'user') {\n        ctx.event.category = ['session'];\n    } else {\n        ctx.event.category = ['process'];\n    }\n} else {\n    ctx.event.category = ['process'];\n}\n"#
                ),
            )?;

            // Painless script
            // Source: def allowedValues = [\n    'access',\n    'admin',\n    'allowed',\n    'change',\n    'connection',\n    'creation',\n    'deletion',\n    'denied',\n    'end',\n    'error',\n    'group',\n    'indicator',\n    'info',\n    'installation',\n    'protocol',\n    'start',\n    'user'\n];\nif (ctx?.falco?.output_fields?.evt != null && ctx?.falco?.output_fields?.evt?.type != null) {\n    def inputType = ctx?.falco?.output_fields?.evt?.type;\n    def lowercaseType = inputType.toLowerCase();\n    if (allowedValues.contains(lowercaseType)) {\n        ctx.event.type = [inputType];\n    } else if (inputType == 'faccessat' || inputType == 'faccessat2' || inputType == 'fsopen' || inputType == 'name_to_handle_at' || inputType == 'nfsservctl' || inputType == 'open' || inputType == 'open_by_handle_at' || inputType == 'open_tree' || inputType == 'openat' || inputType == 'openat2' || inputType == 'pciconfig_read' || inputType == 'pidfd_open' || inputType == 'read' || inputType == 'readahead' || inputType == 'readdir' || inputType == 'readlink' || inputType == 'readlinkat' || inputType == 'readv' || inputType == 's390_pci_mmio_read' || inputType == 'uselib' || inputType == 'vm86old' || inputType == 'vm86') {\n        ctx.event.type = ['access'];\n    } else if (inputType == 'bdflush' || inputType == 'membarrier' || inputType == 'ptrace' || inputType == 'reboot' || inputType == 'restart_syscall') {\n        ctx.event.type = ['admin'];\n    } else if (inputType == 'fallocate' || inputType == 'finit_module') {\n        ctx.event.type = ['allowed'];\n    } else if (inputType == 'llseek' || inputType == 'sysctl' || inputType == 'acct' || inputType == 'adjtimex' || inputType == 'alarm' || inputType == 'arch_prctl' || inputType == 'bind' || inputType == 'bpf' || inputType == 'brk' || inputType == 'capset' || inputType == 'chdir' || inputType == 'chmod' || inputType == 'chown' || inputType == 'chown32' || inputType == 'chroot' || inputType == 'clock_adjtime' || inputType == 'clock_nanosleep' || inputType == 'clock_settime' || inputType == 'close' || inputType == 'close_range' || inputType == 'epoll_ctl' || inputType == 'fchdir' || inputType == 'fchmod' || inputType == 'fchmodat' || inputType == 'fchown' || inputType == 'fchown32' || inputType == 'fchownat' || inputType == 'fcntl' || inputType == 'fcntl64' || inputType == 'flock' || inputType == 'free_hugepages' || inputType == 'fsetxattr' || inputType == 'fsconfig' || inputType == 'ftruncate' || inputType == 'ftruncate64' || inputType == 'futimesat' || inputType == 'io_setup' || inputType == 'io_uring_setup' || inputType == 'ioctl' || inputType == 'ioperm' || inputType == 'iopl' || inputType == 'ioprio_set' || inputType == 'keyctl' || inputType == 'landlock_restrict_self' || inputType == 'lchown' || inputType == 'lchown32' || inputType == 'madvise' || inputType == 'mbind' || inputType == 'memory_ordering' || inputType == 'migrate_pages' || inputType == 'mlock' || inputType == 'mlock2' || inputType == 'mlockall' || inputType == 'mmap' || inputType == 'mmap2' || inputType == 'modify_ldt' || inputType == 'move_mount' || inputType == 'move_pages' || inputType == 'mprotect' || inputType == 'mq_getsetattr' || inputType == 'mremap' || inputType == 'msgctl' || inputType == 'munlock' || inputType == 'munlockall' || inputType == 'munmap' || inputType == 'nanosleep' || inputType == 'nice' || inputType == 'old_adjtimex' || inputType == 'pause' || inputType == 'pciconfig_write' || inputType == 'personality' || inputType == 'perfctr' || inputType == 'perfmonctl' || inputType == 'pivot_root' || inputType == 'pkey_alloc' || inputType == 'pkey_free' || inputType == 'pkey_mprotect' || inputType == 'prctl' || inputType == 'prlimit64' || inputType == 'process_madvise' || inputType == 'process_vm_writev' || inputType == 'pwrite64' || inputType == 'pwritev' || inputType == 'pwritev2' || inputType == 'quotactl' || inputType == 'quotactl_fd' || inputType == 'rename' || inputType == 'renameat' || inputType == 'renameat2' || inputType == 'rseq' || inputType == 'rt_sigaction' || inputType == 'rt_sigpending' || inputType == 'rt_sigprocmask' || inputType == 'rt_sigqueueinfo' || inputType == 'rt_sigreturn' || inputType == 'rtas' || inputType == 's390_pci_mmio_write' || inputType == 's390_guarded_storage' || inputType == 'sched_setaffinity' || inputType == 'sched_setattr' || inputType == 'sched_setparam' || inputType == 'sched_setscheduler' || inputType == 'seccomp' || inputType == 'semctl' || inputType == 'semop' || inputType == 'semtimedop' || inputType == 'set_mempolicy' || inputType == 'set_robust_list' || inputType == 'set_thread_area' || inputType == 'set_tid_address' || inputType == 'set_tls' || inputType == 'set_domainname' || inputType == 'set_fsgid' || inputType == 'setfsgid32' || inputType == 'setfsuid' || inputType == 'setfsuid32' || inputType == 'setgid' || inputType == 'setgid32' || inputType == 'sethae' || inputType == 'sethostimer' || inputType == 'setitimer' || inputType == 'setns' || inputType == 'setpgid' || inputType == 'setpgrp' || inputType == 'setpriority' || inputType == 'setregid' || inputType == 'setregid32' || inputType == 'setresgid' || inputType == 'setresgid32' || inputType == 'setresuid' || inputType == 'setresuid32' || inputType == 'setreuid' || inputType == 'setreuid32' || inputType == 'setrlimit' || inputType == 'setsid' || inputType == 'setsockopt' || inputType == 'settimeofday' || inputType == 'setuid' || inputType == 'setuid32' || inputType == 'setup' || inputType == 'setxattr' || inputType == 'shmat'  || inputType == 'shmctl' || inputType == 'shmdt' || inputType == 'sigaction' || inputType == 'sigaltstack' || inputType == 'subpage_prot' || inputType == 'swapcontext' || inputType == 'switch_endian' || inputType == 'sys_debug_setcontext' || inputType == 'timer_settime' || inputType == 'timerfd_settime' || inputType == 'truncate' || inputType == 'truncate64' || inputType == 'umask' || inputType == 'utime' || inputType == 'utimesat' || inputType == 'utimes' || inputType == 'write' || inputType == 'writev' || inputType == 'xtensa') {\n        ctx.event.type = ['change'];\n    } else if (inputType == 'accept' || inputType == 'accept4' || inputType == 'connect' || inputType == 'mq_timedreceive' || inputType == 'mq_timedsend' || inputType == 'msgrcv' || inputType == 'msgsnd' || inputType == 'pidfd_send_signal' || inputType == 'recv' || inputType == 'recvfrom' || inputType == 'recvmsg' || inputType == 'recvmmsg' || inputType == 'rt_sigqueueinfo' || inputType == 'rt_tgsigqueueinfo' || inputType == 'send' || inputType == 'sendfile' || inputType == 'sendfile64' || inputType == 'sendmmsg' || inputType == 'sendmsg' || inputType == 'sendto' || inputType == 'signal' || inputType == 'signalfd' || inputType == 'signalfd4' || inputType == 'socket' || inputType == 'socketcall' || inputType == 'socketpair' || inputType == 'syscall') {\n        ctx.event.type = ['connection'];\n    } else if (inputType == 'add_key' || inputType == 'clone' || inputType == 'clone2' || inputType == 'clone3' || inputType == 'copy_file_range' || inputType == 'creat' || inputType == 'create_module' || inputType == 'dup' || inputType == 'dup2' || inputType == 'dup3' || inputType == 'epoll_create' || inputType == 'epoll_create1' || inputType == 'eventfd' || inputType == 'eventfd2' || inputType == 'fdatasync' || inputType == 'fork' || inputType == 'fsmount' || inputType == 'fsync' || inputType == 'init_module' || inputType == 'inotify_add_watch' || inputType == 'inotify_init' || inputType == 'inotify_init1' || inputType == 'kexec_file_load' || inputType == 'kexec_load' || inputType == 'landlock_add_rule' || inputType == 'landlock_create_ruleset' || inputType == 'link' || inputType == 'linkat' || inputType == 'memfd_create' || inputType == 'memfd_secret' || inputType == 'mkdir' || inputType == 'mkdirat' || inputType == 'mknod' || inputType == 'mknodat' || inputType == 'mount' || inputType == 'mq_notify' || inputType == 'mq_open' || inputType == 'msync' || inputType == 'pidfd_getfd' || inputType == 'pipe' || inputType == 'pipe2' || inputType == 'remap_file_pages' || inputType == 'splice' || inputType == 'spu_create' || inputType == 'symlink' || inputType == 'symlinkat' || inputType == 'sync' || inputType == 'sync_file_range' || inputType == 'sync_file_range2' || inputType == 'syncfs' || inputType == 'tee' || inputType == 'timer_create' || inputType == 'timerfd_create' || inputType == 'vfork' || inputType == 'vmsplice') {\n        ctx.event.type = ['creation'];\n    } else if (inputType == 'cacheflush' || inputType == 'delete_module' || inputType == 'fremovexattr' || inputType == 'inotify_rm_watch' || inputType == 'io_destroy' || inputType == 'lremovexattr' || inputType == 'mq_unlink' || inputType == 'oldumount' || inputType == 'removexattr' || inputType == 'riscv_flush_icache' || inputType == 'rmdir' || inputType == 'spill' || inputType == 'timer_delete' || inputType == 'umount' || inputType == 'unlink' || inputType == 'unlinkat' || inputType == 'unshare') {\n        ctx.event.type = ['deletion'];\n    } else if (inputType == 'exit' || inputType == 'exit_group' || inputType == 'io_cancel' || inputType == 'kill' || inputType == 'shutdown'  || inputType == 'swapoff' || inputType == 'tgkill' || inputType == 'tkill' || inputType == 'vhangup') {\n        ctx.event.type = ['end'];\n    } else if (inputType == 'fanotify_init' || inputType == 'fanotify_mark' || inputType == 'setgroups' || inputType == 'setgroups32') {\n        ctx.event.type = ['group'];\n    } else if (inputType == 'alloc_hugepages' || inputType == 'capget' || inputType == 'clock_getres' || inputType == 'clock_gettime' || inputType == 'epoll_pwait' || inputType == 'epoll_pwait2' || inputType == 'epoll_wait' || inputType == 'fadvise64' || inputType == 'fadvise64_64' || inputType == 'fgetxattr' || inputType == 'flistxattr' || inputType == 'fspick' || inputType == 'fstat' || inputType == 'fstat64' || inputType == 'fstatat64' || inputType == 'fstatfs' || inputType == 'fstatfs64' || inputType == 'futex' || inputType == 'get_kernel_syms' || inputType == 'get_mempolicy' || inputType == 'get_robust_list' || inputType == 'get_thread_area' || inputType == 'get_tls' || inputType == 'getcpu' || inputType == 'getcwd' || inputType == 'getdents' || inputType == 'getdents64' || inputType == 'getdomainname' || inputType == 'getdtablesize' || inputType == 'getegid' || inputType == 'getegid32' || inputType == 'geteuid' || inputType == 'geteuid32' || inputType == 'getgid' || inputType == 'getgid32' || inputType == 'getgroups' || inputType == 'getgroups32' || inputType == 'gethostname' || inputType == 'getitimer' || inputType == 'getpeername' || inputType == 'getpagesize' || inputType == 'getpgid' || inputType == 'getpgrp' || inputType == 'getpid' || inputType == 'getppid' || inputType == 'getpriority' || inputType == 'getrandom' || inputType == 'getresgid' || inputType == 'getresgid32' || inputType == 'getresuid' || inputType == 'getresuid32' || inputType == 'getrlimit' || inputType == 'getrusage' || inputType == 'getsid' || inputType == 'getsockname' || inputType == 'getsockopt' || inputType == 'gettid' || inputType == 'gettimeofday' || inputType == 'getuid' || inputType == 'getuid32' || inputType == 'getunwind' || inputType == 'getxattr' || inputType == 'getxgid' || inputType == 'getxpid' || inputType == 'getxuid' || inputType == 'io_getevents' || inputType == 'io_pgetevents' || inputType == 'io_submit' || inputType == 'io_uring_register' || inputType == 'ioprio_get' || inputType == 'kcmp' || inputType == 'kern_features' || inputType == 'lgetxattr' || inputType == 'listen' || inputType == 'listxattr' || inputType == 'llistxattr' || inputType == 'lookup_dcookie' || inputType == 'lstat' || inputType == 'lstat64' || inputType == 'mincore' || inputType == 'msgget' || inputType == 'newfstatat' || inputType == 'old_getrlimit' || inputType == 'old_fstat' || inputType == 'oldlstat' || inputType == 'oldolduname' || inputType == 'oldstat' || inputType == 'olduname' || inputType == 'or1k_atomic' || inputType == 'pciconfig_iobase' || inputType == 'poll' || inputType == 'ppoll' || inputType == 'pread64' || inputType == 'preadv' || inputType == 'preadv2' || inputType == 'process_vm_readv' || inputType == 'pselect6' || inputType == 'query_module' || inputType == 'request_key' || inputType == 'rt_sigsuspend' || inputType == 'rt_sigtimedwait' || inputType == 's390_runtime_instr' || inputType == 's390_sthyi' || inputType == 'sched_get_affinity' || inputType == 'sched_get_priority_max' || inputType == 'sched_get_priority_min' || inputType == 'sched_getaffinity' || inputType == 'sched_getattr' || inputType == 'sched_getparam' || inputType == 'sched_getscheduler' || inputType == 'sched_rr_get_interval' || inputType == 'sched_yield' || inputType == 'select' || inputType == 'semget' || inputType == 'sgetmask' || inputType == 'shmget' || inputType == 'sigpending' || inputType == 'sigprocmask' || inputType == 'sigreturn' || inputType == 'sigsuspend' || inputType == 'ssetmask' || inputType == 'stat' || inputType == 'stat64' || inputType == 'statfs' || inputType == 'statfs64' || inputType == 'statx' || inputType == 'stime' || inputType == 'sysfs' || inputType == 'sysinfo' || inputType == 'syslog' || inputType == 'sysmips' || inputType == 'time' || inputType == 'timer_getoverrun' || inputType == 'timer_gettime' || inputType == 'timerfd_gettime' || inputType == 'times' || inputType == 'ugetrlimit' || inputType == 'uname' || inputType == 'ustat' || inputType == 'wait' || inputType == 'wait4' || inputType == 'waitid' || inputType == 'waitpid') {\n        ctx.event.type = ['info'];\n    } else if (inputType == 'utrap_install') {\n        ctx.event.type = ['installation'];\n    } else if (inputType == 'ipc') {\n        ctx.event.type = ['protocol'];\n    } else if (inputType == 'execve' || inputType == 'execveat' || inputType == 'execv' || inputType == 'io_uring_enter' || inputType == 'perf_event_open' || inputType == 'spu_run' || inputType == 'swapon') {\n        ctx.event.type = ['start'];\n    } else if (inputType == 'userfaultfd' || inputType == 'usr26' || inputType == 'usr32') {\n        ctx.event.type = ['user'];\n    } else {\n        ctx.event.type = ['info'];\n    }\n} else {\n    ctx.event.type = ['info'];\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
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
                // Painless script
                // Source: def mitreRegex = /T\\d{4}/;\nfor (int i = 0; i < ctx?.falco?.tags.length; i++) {\n    def tag = ctx?.falco?.tags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.id'] = [matcher.group()];\n        break;\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def mitreRegex = /T\\d{4}/;\nfor (int i = 0; i < ctx?.falco?.tags.length; i++) {\n    def tag = ctx?.falco?.tags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.id'] = [matcher.group()];\n        break;\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("falco.tags") };
            if _cond {
                // Painless script
                // Source: def mitreRegex = /T\\d{4}.\\d{3}/;\nfor (int i = 0; i < ctx?.falco?.tags.length; i++) {\n    def tag = ctx?.falco?.tags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.subtechnique.id'] = [matcher.group()];\n        break;\n    }\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan(
                    event,
                    cached_painless!(
                        r#"def mitreRegex = /T\\d{4}.\\d{3}/;\nfor (int i = 0; i < ctx?.falco?.tags.length; i++) {\n    def tag = ctx?.falco?.tags[i];\n    def matcher = mitreRegex.matcher(tag);\n    if (matcher.find()) {\n        ctx['threat.technique.subtechnique.id'] = [matcher.group()];\n        break;\n    }\n}\n"#
                    ),
                )?;
            }

            let _cond = { event.has_value("falco.priority") };
            if _cond {
                // Painless script
                // Source: def sev = params[ctx.falco.priority.toLowerCase()];\nif (sev != null) {\n    if (ctx.event == null) {\n        ctx.event = new HashMap();\n    }\n    ctx.event.severity = sev;\n}\n
                // TODO: Transpile Painless to Rust (2.2.3)
                painless_exec_plan_params(
                    event,
                    cached_painless!(
                        r#"def sev = params[ctx.falco.priority.toLowerCase()];\nif (sev != null) {\n    if (ctx.event == null) {\n        ctx.event = new HashMap();\n    }\n    ctx.event.severity = sev;\n}\n"#
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

            // Painless script
            // Source: if (ctx.falco?.output_fields?.evt?.time != null) {\n    def timeField = ctx.falco.output_fields.evt.time;\n    def inputFormat = new SimpleDateFormat(\"yyyy-MM-dd'T'HH:mm:ss.SSSZ\");\n        if (timeField.iso8601 != null) {\n            if (timeField.iso8601 instanceof String) {\n                def formatted = inputFormat.parse(timeField.iso8601);\n                ctx['@timestamp'] = formatted;\n                ctx.falco.output_fields.evt.time.iso8601 = formatted;\n            } else if (timeField.iso8601 instanceof Long) {\n                long milliseconds = timeField.iso8601 / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n                ctx.falco.output_fields.evt.time.iso8601 = milliseconds;\n            }\n        } else if (timeField.rawtime != null) {\n            if (timeField.rawtime instanceof String) {\n                def formatted = inputFormat.parse(timeField.rawtime);\n                ctx['@timestamp'] = formatted;\n            } else if (timeField.rawtime instanceof Long) {\n                long milliseconds = timeField.rawtime / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n            }\n        } else {\n            if (timeField instanceof String) {\n                def formatted = inputFormat.parse(timeField);\n                ctx['@timestamp'] = formatted;\n            } else if (timeField instanceof Long) {\n                long milliseconds = timeField / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n            }\n        }\n} else {\n    def timeField = ctx.falco.output_fields.event.time;\n    def inputFormat = new SimpleDateFormat(\"yyyy-MM-dd'T'HH:mm:ss.SSSZ\");\n    if (ctx.falco?.output_fields?.event?.time != null) {\n      if (timeField instanceof String) {\n          def formatted = inputFormat.parse(timeField);\n          ctx['@timestamp'] = formatted;\n      } else if (timeField instanceof Long) {\n          long milliseconds = timeField / 1000000;\n          ctx['@timestamp'] = new Date(milliseconds);\n      }\n    }\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.falco?.output_fields?.evt?.time != null) {\n    def timeField = ctx.falco.output_fields.evt.time;\n    def inputFormat = new SimpleDateFormat(\"yyyy-MM-dd'T'HH:mm:ss.SSSZ\");\n        if (timeField.iso8601 != null) {\n            if (timeField.iso8601 instanceof String) {\n                def formatted = inputFormat.parse(timeField.iso8601);\n                ctx['@timestamp'] = formatted;\n                ctx.falco.output_fields.evt.time.iso8601 = formatted;\n            } else if (timeField.iso8601 instanceof Long) {\n                long milliseconds = timeField.iso8601 / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n                ctx.falco.output_fields.evt.time.iso8601 = milliseconds;\n            }\n        } else if (timeField.rawtime != null) {\n            if (timeField.rawtime instanceof String) {\n                def formatted = inputFormat.parse(timeField.rawtime);\n                ctx['@timestamp'] = formatted;\n            } else if (timeField.rawtime instanceof Long) {\n                long milliseconds = timeField.rawtime / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n            }\n        } else {\n            if (timeField instanceof String) {\n                def formatted = inputFormat.parse(timeField);\n                ctx['@timestamp'] = formatted;\n            } else if (timeField instanceof Long) {\n                long milliseconds = timeField / 1000000;\n                ctx['@timestamp'] = new Date(milliseconds);\n            }\n        }\n} else {\n    def timeField = ctx.falco.output_fields.event.time;\n    def inputFormat = new SimpleDateFormat(\"yyyy-MM-dd'T'HH:mm:ss.SSSZ\");\n    if (ctx.falco?.output_fields?.event?.time != null) {\n      if (timeField instanceof String) {\n          def formatted = inputFormat.parse(timeField);\n          ctx['@timestamp'] = formatted;\n      } else if (timeField instanceof Long) {\n          long milliseconds = timeField / 1000000;\n          ctx['@timestamp'] = new Date(milliseconds);\n      }\n    }\n}\n"#
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

            // Painless script
            // Source: if (ctx.falco.output_fields?.proc?.exepath != null && ctx.falco.output_fields?.proc?.args != null) {\n    def path = ctx.falco.output_fields.proc.exepath;\n    def args = ctx.falco.output_fields.proc.args;\n    def argItems = args.splitOnToken(' ');\n    def finalList = [];\n    finalList.add(path);\n    for (int i = 0; i < argItems.length; i++) {\n        finalList.add(argItems[i]);\n    }\n    ctx['process']['args'] = finalList;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.falco.output_fields?.proc?.exepath != null && ctx.falco.output_fields?.proc?.args != null) {\n    def path = ctx.falco.output_fields.proc.exepath;\n    def args = ctx.falco.output_fields.proc.args;\n    def argItems = args.splitOnToken(' ');\n    def finalList = [];\n    finalList.add(path);\n    for (int i = 0; i < argItems.length; i++) {\n        finalList.add(argItems[i]);\n    }\n    ctx['process']['args'] = finalList;\n}\n"#
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
                    while parts.last().and_then(Value::as_str) == Some("") {
                        parts.pop();
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

            // Painless script
            // Source: if (ctx.falco.output_fields?.container?.mounts != null) {\n    def mountsString = ctx.falco.output_fields.container.mounts;\n    def mountItems = mountsString.splitOnToken(' ');            \n    def mountsList = [];\n    for (int i = 0; i < mountItems.length; i++) {\n        def mountItem = mountItems[i];\n        def parts = mountItem.splitOnToken(':');\n        def mountRecord = [:];\n        mountRecord.source = parts.length > 0 ? parts[0] : null;\n        mountRecord.dest = parts.length > 1 ? parts[1] : null;\n        mountRecord.mode = parts.length > 2 ? parts[2] : null;\n        mountRecord.rdrw = parts.length > 3 ? parts[3] : null;\n        mountRecord.propagation = parts.length > 4 ? parts[4] : null;\n        mountsList.add(mountRecord);\n    }\n    ctx['falco.container.mounts'] = mountsList;\n} else {\n    ctx['falco.container.mounts'] = null;\n}\n
            // TODO: Transpile Painless to Rust (2.2.3)
            painless_exec_plan(
                event,
                cached_painless!(
                    r#"if (ctx.falco.output_fields?.container?.mounts != null) {\n    def mountsString = ctx.falco.output_fields.container.mounts;\n    def mountItems = mountsString.splitOnToken(' ');            \n    def mountsList = [];\n    for (int i = 0; i < mountItems.length; i++) {\n        def mountItem = mountItems[i];\n        def parts = mountItem.splitOnToken(':');\n        def mountRecord = [:];\n        mountRecord.source = parts.length > 0 ? parts[0] : null;\n        mountRecord.dest = parts.length > 1 ? parts[1] : null;\n        mountRecord.mode = parts.length > 2 ? parts[2] : null;\n        mountRecord.rdrw = parts.length > 3 ? parts[3] : null;\n        mountRecord.propagation = parts.length > 4 ? parts[4] : null;\n        mountsList.add(mountRecord);\n    }\n    ctx['falco.container.mounts'] = mountsList;\n} else {\n    ctx['falco.container.mounts'] = null;\n}\n"#
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
