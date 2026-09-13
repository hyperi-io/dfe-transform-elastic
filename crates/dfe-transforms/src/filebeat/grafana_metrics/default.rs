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
            event.set(
                "event.ingested",
                json!(
                    event
                        .get("_ingest.timestamp")
                        .map_or_else(String::new, template_to_string)
                ),
            )?;

            event.set("event.kind", json!("metric"))?;

            event.set("event.module", json!("grafana"))?;

            event.set("event.dataset", json!("grafana.metrics"))?;

            let _cond = { event.has_value("prometheus.labels.instance") };
            if _cond {
                if let Some(v) = event.get("prometheus.labels.instance").cloned() {
                    event.set("service.address", v)?;
                }
            }

            event.remove("prometheus.labels.instance");

            let _cond = { event.has_value("prometheus.labels.job") };
            if _cond {
                if let Some(v) = event.get("prometheus.labels.job").cloned() {
                    event.set("service.name", v)?;
                }
            }

            event.remove("prometheus.labels.job");

            if event.has_value("prometheus.metrics.grafana_stat_totals_dashboard") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_dashboard",
                    "grafana.stat.dashboards.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_total_users") {
                event.rename(
                    "prometheus.metrics.grafana_stat_total_users",
                    "grafana.stat.users.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_active_users") {
                event.rename(
                    "prometheus.metrics.grafana_stat_active_users",
                    "grafana.stat.users.active",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_datasource") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_datasource",
                    "grafana.stat.datasources.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_alert_rules") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_alert_rules",
                    "grafana.stat.alert_rules.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_total_orgs") {
                event.rename(
                    "prometheus.metrics.grafana_stat_total_orgs",
                    "grafana.stat.orgs.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_total_teams") {
                event.rename(
                    "prometheus.metrics.grafana_stat_total_teams",
                    "grafana.stat.teams.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_total_playlists") {
                event.rename(
                    "prometheus.metrics.grafana_stat_total_playlists",
                    "grafana.stat.playlists.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_total_service_accounts") {
                event.rename(
                    "prometheus.metrics.grafana_stat_total_service_accounts",
                    "grafana.stat.service_accounts.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_folder") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_folder",
                    "grafana.stat.folders.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_annotations") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_annotations",
                    "grafana.stat.annotations.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_admins") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_admins",
                    "grafana.stat.admins.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_editors") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_editors",
                    "grafana.stat.editors.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_viewers") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_viewers",
                    "grafana.stat.viewers.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_active_admins") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_active_admins",
                    "grafana.stat.active_admins.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_active_editors") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_active_editors",
                    "grafana.stat.active_editors.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_active_viewers") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_active_viewers",
                    "grafana.stat.active_viewers.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_dashboard_versions") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_dashboard_versions",
                    "grafana.stat.dashboard_versions.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_library_panels") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_library_panels",
                    "grafana.stat.library_panels.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_public_dashboard") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_public_dashboard",
                    "grafana.stat.public_dashboards.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_rule_groups") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_rule_groups",
                    "grafana.stat.rule_groups.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_correlations") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_correlations",
                    "grafana.stat.correlations.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_data_keys") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_data_keys",
                    "grafana.stat.data_keys.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_stat_totals_repositories") {
                event.rename(
                    "prometheus.metrics.grafana_stat_totals_repositories",
                    "grafana.stat.repositories.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_active_alerts") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_active_alerts",
                    "grafana.alerting.active_alerts",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_result_total") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_result_total",
                    "grafana.alerting.result.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_notification_sent_total") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_notification_sent_total",
                    "grafana.alerting.notification.sent.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_notification_failed_total") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_notification_failed_total",
                    "grafana.alerting.notification.failed.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_active_configurations") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_active_configurations",
                    "grafana.alerting.active_configurations",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_alerts") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_alerts",
                    "grafana.alerting.alerts",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_alerts_received_total") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_alerts_received_total",
                    "grafana.alerting.alerts_received.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_alerts_invalid_total") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_alerts_invalid_total",
                    "grafana.alerting.alerts_invalid.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_silences") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_silences",
                    "grafana.alerting.silences",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_schedule_alert_rules") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_schedule_alert_rules",
                    "grafana.alerting.schedule.alert_rules",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_scheduler_behind_seconds") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_scheduler_behind_seconds",
                    "grafana.alerting.scheduler.behind_seconds",
                )?;
            }

            if event
                .has_value("prometheus.metrics.grafana_alerting_execution_time_milliseconds_count")
            {
                event.rename(
                    "prometheus.metrics.grafana_alerting_execution_time_milliseconds_count",
                    "grafana.alerting.execution_time.count",
                )?;
            }

            if event
                .has_value("prometheus.metrics.grafana_alerting_execution_time_milliseconds_sum")
            {
                event.rename(
                    "prometheus.metrics.grafana_alerting_execution_time_milliseconds_sum",
                    "grafana.alerting.execution_time.milliseconds",
                )?;
            }

            if event
                .has_value("prometheus.metrics.grafana_alerting_notification_latency_seconds_count")
            {
                event.rename(
                    "prometheus.metrics.grafana_alerting_notification_latency_seconds_count",
                    "grafana.alerting.notification_latency.count",
                )?;
            }

            if event
                .has_value("prometheus.metrics.grafana_alerting_notification_latency_seconds_sum")
            {
                event.rename(
                    "prometheus.metrics.grafana_alerting_notification_latency_seconds_sum",
                    "grafana.alerting.notification_latency.seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_alerting_ticker_interval_seconds") {
                event.rename(
                    "prometheus.metrics.grafana_alerting_ticker_interval_seconds",
                    "grafana.alerting.ticker.interval_seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_open") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_open",
                    "grafana.database.connections.open",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_in_use") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_in_use",
                    "grafana.database.connections.in_use",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_idle") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_idle",
                    "grafana.database.connections.idle",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_max_open") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_max_open",
                    "grafana.database.connections.max_open",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_wait_count_total") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_wait_count_total",
                    "grafana.database.connections.wait_count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_wait_duration_seconds") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_wait_duration_seconds",
                    "grafana.database.connections.wait_duration_seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_max_idle_closed_total") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_max_idle_closed_total",
                    "grafana.database.connections.max_idle_closed",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_max_idle_closed_seconds") {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_max_idle_closed_seconds",
                    "grafana.database.connections.max_idle_closed_seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_database_conn_max_lifetime_closed_total")
            {
                event.rename(
                    "prometheus.metrics.grafana_database_conn_max_lifetime_closed_total",
                    "grafana.database.connections.max_lifetime_closed",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_datasource_request_total") {
                event.rename(
                    "prometheus.metrics.grafana_datasource_request_total",
                    "grafana.datasource.request.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_db_datasource_query_by_id_total") {
                event.rename(
                    "prometheus.metrics.grafana_db_datasource_query_by_id_total",
                    "grafana.datasource.query_by_id.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_response_status_total") {
                event.rename(
                    "prometheus.metrics.grafana_api_response_status_total",
                    "grafana.api.response.status.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_get_milliseconds") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_get_milliseconds",
                    "grafana.api.dashboard.get.milliseconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_get_milliseconds_count") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_get_milliseconds_count",
                    "grafana.api.dashboard.get.count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_get_milliseconds_sum") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_get_milliseconds_sum",
                    "grafana.api.dashboard.get.sum",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_save_milliseconds") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_save_milliseconds",
                    "grafana.api.dashboard.save.milliseconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_save_milliseconds_count") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_save_milliseconds_count",
                    "grafana.api.dashboard.save.count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_save_milliseconds_sum") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_save_milliseconds_sum",
                    "grafana.api.dashboard.save.sum",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_search_milliseconds") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_search_milliseconds",
                    "grafana.api.dashboard.search.milliseconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_search_milliseconds_count")
            {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_search_milliseconds_count",
                    "grafana.api.dashboard.search.count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_search_milliseconds_sum") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_search_milliseconds_sum",
                    "grafana.api.dashboard.search.sum",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dataproxy_request_all_milliseconds")
            {
                event.rename(
                    "prometheus.metrics.grafana_api_dataproxy_request_all_milliseconds",
                    "grafana.api.dataproxy.request.milliseconds",
                )?;
            }

            if event.has_value(
                "prometheus.metrics.grafana_api_dataproxy_request_all_milliseconds_count",
            ) {
                event.rename(
                    "prometheus.metrics.grafana_api_dataproxy_request_all_milliseconds_count",
                    "grafana.api.dataproxy.request.count",
                )?;
            }

            if event
                .has_value("prometheus.metrics.grafana_api_dataproxy_request_all_milliseconds_sum")
            {
                event.rename(
                    "prometheus.metrics.grafana_api_dataproxy_request_all_milliseconds_sum",
                    "grafana.api.dataproxy.request.sum",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_login_post_total") {
                event.rename(
                    "prometheus.metrics.grafana_api_login_post_total",
                    "grafana.api.login.post.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_login_oauth_total") {
                event.rename(
                    "prometheus.metrics.grafana_api_login_oauth_total",
                    "grafana.api.login.oauth.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_models_dashboard_insert_total") {
                event.rename(
                    "prometheus.metrics.grafana_api_models_dashboard_insert_total",
                    "grafana.api.dashboard.insert.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_snapshot_create_total") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_snapshot_create_total",
                    "grafana.api.dashboard.snapshot.create.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_api_dashboard_snapshot_get_total") {
                event.rename(
                    "prometheus.metrics.grafana_api_dashboard_snapshot_get_total",
                    "grafana.api.dashboard.snapshot.get.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_http_request_duration_seconds_count") {
                event.rename(
                    "prometheus.metrics.grafana_http_request_duration_seconds_count",
                    "grafana.http.request.count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_http_request_duration_seconds_sum") {
                event.rename(
                    "prometheus.metrics.grafana_http_request_duration_seconds_sum",
                    "grafana.http.request.duration.seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_http_request_in_flight") {
                event.rename(
                    "prometheus.metrics.grafana_http_request_in_flight",
                    "grafana.http.request.in_flight",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_http_response_size_bytes_count") {
                event.rename(
                    "prometheus.metrics.grafana_http_response_size_bytes_count",
                    "grafana.http.response.size.count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_http_response_size_bytes_sum") {
                event.rename(
                    "prometheus.metrics.grafana_http_response_size_bytes_sum",
                    "grafana.http.response.size.bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_page_response_status_total") {
                event.rename(
                    "prometheus.metrics.grafana_page_response_status_total",
                    "grafana.page.response.status.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_proxy_response_status_total") {
                event.rename(
                    "prometheus.metrics.grafana_proxy_response_status_total",
                    "grafana.proxy.response.status.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_authentication_attempts") {
                event.rename(
                    "prometheus.metrics.grafana_authentication_attempts",
                    "grafana.authentication.attempts",
                )?;
            }

            if event
                .has_value("prometheus.metrics.grafana_authn_authn_successful_authentication_total")
            {
                event.rename(
                    "prometheus.metrics.grafana_authn_authn_successful_authentication_total",
                    "grafana.authentication.successful.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_authn_authn_failed_authentication_total")
            {
                event.rename(
                    "prometheus.metrics.grafana_authn_authn_failed_authentication_total",
                    "grafana.authentication.failed.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_authenticated_user_requests") {
                event.rename(
                    "prometheus.metrics.grafana_authenticated_user_requests",
                    "grafana.authentication.user_requests",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_emails_sent_total") {
                event.rename(
                    "prometheus.metrics.grafana_emails_sent_total",
                    "grafana.emails.sent.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_emails_sent_failed") {
                event.rename(
                    "prometheus.metrics.grafana_emails_sent_failed",
                    "grafana.emails.sent.failed",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_live_node_num_channels") {
                event.rename(
                    "prometheus.metrics.grafana_live_node_num_channels",
                    "grafana.live.channels",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_live_node_num_clients") {
                event.rename(
                    "prometheus.metrics.grafana_live_node_num_clients",
                    "grafana.live.clients",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_live_node_num_subscriptions") {
                event.rename(
                    "prometheus.metrics.grafana_live_node_num_subscriptions",
                    "grafana.live.subscriptions",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_live_node_num_users") {
                event.rename(
                    "prometheus.metrics.grafana_live_node_num_users",
                    "grafana.live.users",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_frontend_boot_load_time_seconds_count") {
                event.rename(
                    "prometheus.metrics.grafana_frontend_boot_load_time_seconds_count",
                    "grafana.frontend.boot.load_time.count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_frontend_boot_load_time_seconds_sum") {
                event.rename(
                    "prometheus.metrics.grafana_frontend_boot_load_time_seconds_sum",
                    "grafana.frontend.boot.load_time.seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_frontend_boot_first_contentful_paint_time_seconds_count") {
                    event.rename("prometheus.metrics.grafana_frontend_boot_first_contentful_paint_time_seconds_count", "grafana.frontend.boot.fcp.count")?;
                }

            if event.has_value(
                "prometheus.metrics.grafana_frontend_boot_first_contentful_paint_time_seconds_sum",
            ) {
                event.rename("prometheus.metrics.grafana_frontend_boot_first_contentful_paint_time_seconds_sum", "grafana.frontend.boot.fcp.seconds")?;
            }

            if event.has_value("prometheus.metrics.grafana_public_dashboard_request_count") {
                event.rename(
                    "prometheus.metrics.grafana_public_dashboard_request_count",
                    "grafana.public_dashboard.request_count",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_rendering_queue_size") {
                event.rename(
                    "prometheus.metrics.grafana_rendering_queue_size",
                    "grafana.rendering.queue_size",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_cpu_seconds_total") {
                event.rename(
                    "prometheus.metrics.grafana_process_cpu_seconds_total",
                    "grafana.process.cpu.seconds.total",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_resident_memory_bytes") {
                event.rename(
                    "prometheus.metrics.grafana_process_resident_memory_bytes",
                    "grafana.process.memory.resident_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_virtual_memory_bytes") {
                event.rename(
                    "prometheus.metrics.grafana_process_virtual_memory_bytes",
                    "grafana.process.memory.virtual_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_open_fds") {
                event.rename(
                    "prometheus.metrics.grafana_process_open_fds",
                    "grafana.process.open_fds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_max_fds") {
                event.rename(
                    "prometheus.metrics.grafana_process_max_fds",
                    "grafana.process.max_fds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_start_time_seconds") {
                event.rename(
                    "prometheus.metrics.grafana_process_start_time_seconds",
                    "grafana.process.start_time_seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_network_receive_bytes_total") {
                event.rename(
                    "prometheus.metrics.grafana_process_network_receive_bytes_total",
                    "grafana.process.network.receive_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_process_network_transmit_bytes_total") {
                event.rename(
                    "prometheus.metrics.grafana_process_network_transmit_bytes_total",
                    "grafana.process.network.transmit_bytes",
                )?;
            }

            let _cond = { !event.has_value("grafana.process.cpu.seconds.total") };
            if _cond {
                if event.has_value("prometheus.metrics.process_cpu_seconds_total") {
                    event.rename(
                        "prometheus.metrics.process_cpu_seconds_total",
                        "grafana.process.cpu.seconds.total",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.memory.resident_bytes") };
            if _cond {
                if event.has_value("prometheus.metrics.process_resident_memory_bytes") {
                    event.rename(
                        "prometheus.metrics.process_resident_memory_bytes",
                        "grafana.process.memory.resident_bytes",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.memory.virtual_bytes") };
            if _cond {
                if event.has_value("prometheus.metrics.process_virtual_memory_bytes") {
                    event.rename(
                        "prometheus.metrics.process_virtual_memory_bytes",
                        "grafana.process.memory.virtual_bytes",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.open_fds") };
            if _cond {
                if event.has_value("prometheus.metrics.process_open_fds") {
                    event.rename(
                        "prometheus.metrics.process_open_fds",
                        "grafana.process.open_fds",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.max_fds") };
            if _cond {
                if event.has_value("prometheus.metrics.process_max_fds") {
                    event.rename(
                        "prometheus.metrics.process_max_fds",
                        "grafana.process.max_fds",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.start_time_seconds") };
            if _cond {
                if event.has_value("prometheus.metrics.process_start_time_seconds") {
                    event.rename(
                        "prometheus.metrics.process_start_time_seconds",
                        "grafana.process.start_time_seconds",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.network.receive_bytes") };
            if _cond {
                if event.has_value("prometheus.metrics.process_network_receive_bytes_total") {
                    event.rename(
                        "prometheus.metrics.process_network_receive_bytes_total",
                        "grafana.process.network.receive_bytes",
                    )?;
                }
            }

            let _cond = { !event.has_value("grafana.process.network.transmit_bytes") };
            if _cond {
                if event.has_value("prometheus.metrics.process_network_transmit_bytes_total") {
                    event.rename(
                        "prometheus.metrics.process_network_transmit_bytes_total",
                        "grafana.process.network.transmit_bytes",
                    )?;
                }
            }

            if event.has_value("prometheus.metrics.go_goroutines") {
                event.rename("prometheus.metrics.go_goroutines", "grafana.go.goroutines")?;
            }

            if event.has_value("prometheus.metrics.go_threads") {
                event.rename("prometheus.metrics.go_threads", "grafana.go.threads")?;
            }

            if event.has_value("prometheus.metrics.go_memstats_heap_alloc_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_heap_alloc_bytes",
                    "grafana.go.memstats.heap_alloc_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_alloc_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_alloc_bytes",
                    "grafana.go.memstats.alloc_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_alloc_bytes_total") {
                event.rename(
                    "prometheus.metrics.go_memstats_alloc_bytes_total",
                    "grafana.go.memstats.alloc_bytes_total",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_sys_bytes",
                    "grafana.go.memstats.sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_heap_idle_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_heap_idle_bytes",
                    "grafana.go.memstats.heap_idle_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_heap_inuse_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_heap_inuse_bytes",
                    "grafana.go.memstats.heap_inuse_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_heap_objects") {
                event.rename(
                    "prometheus.metrics.go_memstats_heap_objects",
                    "grafana.go.memstats.heap_objects",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_heap_released_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_heap_released_bytes",
                    "grafana.go.memstats.heap_released_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_heap_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_heap_sys_bytes",
                    "grafana.go.memstats.heap_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_stack_inuse_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_stack_inuse_bytes",
                    "grafana.go.memstats.stack_inuse_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_next_gc_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_next_gc_bytes",
                    "grafana.go.memstats.next_gc_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_mallocs_total") {
                event.rename(
                    "prometheus.metrics.go_memstats_mallocs_total",
                    "grafana.go.memstats.mallocs_total",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_frees_total") {
                event.rename(
                    "prometheus.metrics.go_memstats_frees_total",
                    "grafana.go.memstats.frees_total",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_buck_hash_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_buck_hash_sys_bytes",
                    "grafana.go.memstats.buck_hash_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_gc_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_gc_sys_bytes",
                    "grafana.go.memstats.gc_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_mcache_inuse_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_mcache_inuse_bytes",
                    "grafana.go.memstats.mcache_inuse_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_mcache_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_mcache_sys_bytes",
                    "grafana.go.memstats.mcache_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_mspan_inuse_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_mspan_inuse_bytes",
                    "grafana.go.memstats.mspan_inuse_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_mspan_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_mspan_sys_bytes",
                    "grafana.go.memstats.mspan_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_other_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_other_sys_bytes",
                    "grafana.go.memstats.other_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_stack_sys_bytes") {
                event.rename(
                    "prometheus.metrics.go_memstats_stack_sys_bytes",
                    "grafana.go.memstats.stack_sys_bytes",
                )?;
            }

            if event.has_value("prometheus.metrics.go_memstats_last_gc_time_seconds") {
                event.rename(
                    "prometheus.metrics.go_memstats_last_gc_time_seconds",
                    "grafana.go.memstats.last_gc_time_seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.go_gc_duration_seconds_count") {
                event.rename(
                    "prometheus.metrics.go_gc_duration_seconds_count",
                    "grafana.go.gc.duration.count",
                )?;
            }

            if event.has_value("prometheus.metrics.go_gc_duration_seconds_sum") {
                event.rename(
                    "prometheus.metrics.go_gc_duration_seconds_sum",
                    "grafana.go.gc.duration.seconds",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_build_info") {
                event.rename(
                    "prometheus.metrics.grafana_build_info",
                    "grafana.build_info._value",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_build_timestamp") {
                event.rename(
                    "prometheus.metrics.grafana_build_timestamp",
                    "grafana.build_info.timestamp",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_environment_info") {
                event.rename(
                    "prometheus.metrics.grafana_environment_info",
                    "grafana.environment_info._value",
                )?;
            }

            if event.has_value("prometheus.metrics.grafana_instance_start_total") {
                event.rename(
                    "prometheus.metrics.grafana_instance_start_total",
                    "grafana.instance.start_total",
                )?;
            }

            event.remove("prometheus.metrics");

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
                        "Processor {} with tag {} failed with message {}",
                        event
                            .get("_ingest.on_failure_processor_type")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_processor_tag")
                            .map_or_else(String::new, template_to_string),
                        event
                            .get("_ingest.on_failure_message")
                            .map_or_else(String::new, template_to_string)
                    )),
                )?;
                event.remove("_ingest.on_failure_message");
            }
        }

        Ok(TransformResult::Continue)
    }
}
