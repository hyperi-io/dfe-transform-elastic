#!/usr/bin/env bash
# Project:   dfe-transform-elastic
# File:      scripts/pgo-workload.sh
# Purpose:   PGO workload orchestrator -- broker, service, and the load driver
# Language:  Bash
#
# License:   BUSL-1.1
# Copyright: (c) 2026 HYPERI PTY LIMITED
#
# Usage:
#   scripts/pgo-workload.sh <path-to-dfe-transform-elastic-binary>
#
# Drives the service's hot path (Kafka consume -> parse batch -> detect
# envelope -> transform -> serialise -> Kafka produce) so a PGO-instrumented
# binary accumulates a representative profile. Exits non-zero when a source's
# run transformed nothing or errored on more events than it transformed.
#
# Environment variables (all optional):
#   PGO_WORKLOAD_DURATION_SECS   Total load duration (default 600, floor 60)
#   PGO_WORKLOAD_KAFKA_IMAGE     Override the Redpanda image
#   PGO_WORKLOAD_KAFKA_PORT      Loopback port the broker publishes on (default 19092)
#   PGO_WORKLOAD_METRICS_PORT    Port the service serves /readyz on (default 9090)
#   PGO_WORKLOAD_KEEP            Set to 1 to skip cleanup (debug)
#   PGO_DRIVER_PATH              Override the pgo-driver binary path
#   PGO_DRIVER_RPS               Events per second (default 5000)
#
# Preconditions:
#   - Docker daemon running, and the user has access to it
#   - $1 is a dfe-transform-elastic binary built with PGO instrumentation
#   - pgo-driver built with --features driver (built on demand if missing)
#
# Breadth across sources is THIS script's job, not the driver's. One service
# instance resolves ONE source and applies that transform to everything that
# arrives, so the only way to profile a second parser family is to restart the
# service against it -- hence the loop below, which splits the duration budget
# rather than spending it twice.

set -euo pipefail

# ----------------------------------------------------------------------------
# Args and environment
# ----------------------------------------------------------------------------

if [[ $# -lt 1 ]]; then
    echo "usage: $0 <path-to-dfe-transform-elastic-binary>" >&2
    exit 1
fi

SERVICE_BIN="$(cd "$(dirname "$1")" && pwd -P)/$(basename "$1")"
if [[ ! -x "$SERVICE_BIN" ]]; then
    echo "error: $SERVICE_BIN is not executable" >&2
    exit 1
fi

DURATION="${PGO_WORKLOAD_DURATION_SECS:-600}"
# Equal to dfe-infra versions.yaml services.redpanda-version, pinned by digest
# so a rebuilt tag cannot change the broker. The digest sits on its own line:
# the Renovate regex stops at a colon.
# renovate: datasource=docker depName=docker.redpanda.com/redpandadata/redpanda
KAFKA_TAG="v26.2.3"
KAFKA_DIGEST="sha256:9e83cfa99278f30d0133271c26bf670cd69c94ffa6ba0b42830dd0c3bd9dcfd9"
KAFKA_IMAGE="${PGO_WORKLOAD_KAFKA_IMAGE:-docker.redpanda.com/redpandadata/redpanda:${KAFKA_TAG}@${KAFKA_DIGEST}}"
KAFKA_PORT="${PGO_WORKLOAD_KAFKA_PORT:-19092}"
METRICS_PORT="${PGO_WORKLOAD_METRICS_PORT:-9090}"
KEEP="${PGO_WORKLOAD_KEEP:-0}"

# The workload broker is plaintext. A host running the dev stack exports SASL
# settings under every spelling scalo reads, and scalo also loads a `.env`
# from the working directory, so the service starts from the scratch
# directory with those names unset and the protocol pinned.
KAFKA_ENV_SCRUB=(
    -u KAFKA_SASL_USER -u KAFKA_SASL_USERNAME -u KAFKA_SASL_PASSWORD
    -u KAFKA_SASL_MECHANISM
    -u DFE_TRANSFORM_ELASTIC_SASL_USER -u DFE_TRANSFORM_ELASTIC_SASL_USERNAME
    -u DFE_TRANSFORM_ELASTIC_SASL_PASSWORD -u DFE_TRANSFORM_ELASTIC_SASL_MECHANISM
    -u DFE_TRANSFORM_ELASTIC_SECURITY_PROTOCOL
    KAFKA_SECURITY_PROTOCOL=plaintext
)

# Both `-lt` and `$(( ))` evaluate a non-numeric string rather than rejecting
# it, so the duration is checked as digits before any arithmetic reads it.
if [[ ! "$DURATION" =~ ^[0-9]+$ ]]; then
    echo "error: PGO_WORKLOAD_DURATION_SECS must be a whole number of seconds (got '$DURATION')" >&2
    exit 1
fi

if [[ "$DURATION" -lt 60 ]]; then
    echo "error: PGO_WORKLOAD_DURATION_SECS must be >= 60 (got $DURATION)" >&2
    echo "  a short workload biases the profile toward startup paths rather" >&2
    echo "  than the transform, which produces NEGATIVE PGO gains" >&2
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Two sources spanning the parser families the transforms divide into: cisco_ios
# is grok over syslog lines, okta is Painless over JSON. Each entry is
# "<source name>|<fixture path relative to the repo root>", and every path is a
# licence-clean sample because the release build runs from this repository alone.
SOURCES=(
    "filebeat.cisco_ios.default|tests/fixtures/unencumbered/cisco_ios/cisco-ios-syslog.log"
    "filebeat.okta.default|tests/fixtures/unencumbered/okta/panther-okta-systemlog.ndjson"
)

SLICE=$(( DURATION / ${#SOURCES[@]} ))

# Each source is a separate profile, so the floor applies per run rather than
# to the total the runs divide.
if [[ "$SLICE" -lt 60 ]]; then
    echo "error: each source needs >= 60s, but ${#SOURCES[@]} sources split ${DURATION}s into ${SLICE}s each" >&2
    echo "  raise PGO_WORKLOAD_DURATION_SECS to at least $(( 60 * ${#SOURCES[@]} ))" >&2
    exit 1
fi

# ----------------------------------------------------------------------------
# Locate the driver
# ----------------------------------------------------------------------------

# A cargo shim redirects CARGO_TARGET_DIR on some hosts, so the build output is
# not necessarily under $PROJECT_ROOT/target. Cargo's own JSON message names the
# artefact wherever it landed, which is the only reading that survives the
# redirect.
find_driver() {
    local build_log="$WORK_DIR/pgo-driver-build.json"
    echo "pgo-workload: building pgo-driver" >&2
    if ! (cd "$PROJECT_ROOT" && cargo build --release -p pgo-driver \
        --features driver --message-format=json) >"$build_log"; then
        echo "error: failed to build pgo-driver" >&2
        return 1
    fi
    sed -n 's/.*"executable":"\([^"]*pgo-driver\)".*/\1/p' "$build_log" | tail -1
}

# Sum every series of the counter named $1 in the Prometheus text on stdin.
counter_total() {
    local name="$1" line series value total=0
    while IFS= read -r line; do
        [[ -z "$line" || "$line" == \#* ]] && continue
        series="${line%% *}"
        [[ "${series%%\{*}" == "$name" ]] || continue
        value="${line##* }"
        if [[ ! "$value" =~ ^[0-9]+$ ]]; then
            echo "error: $name reads '$value', not a whole count" >&2
            return 1
        fi
        total=$(( total + value ))
    done
    echo "$total"
}

# A profile is only worth keeping when the run spent its time transforming:
# no transformed events, or more errors than transforms, profiles the wrong path.
check_transformed() {
    local source_name="$1" scrape transformed errored
    if ! scrape="$(curl -sf --max-time 5 "http://127.0.0.1:$METRICS_PORT/metrics")"; then
        echo "error: could not scrape /metrics for $source_name" >&2
        return 1
    fi
    transformed="$(counter_total events_transformed_total <<<"$scrape")"
    errored="$(counter_total events_errored_total <<<"$scrape")"
    echo "pgo-workload: $source_name events_transformed_total=$transformed events_errored_total=$errored"
    if [[ "$transformed" -eq 0 ]]; then
        echo "error: $source_name transformed nothing, so its profile is of an idle service" >&2
        return 1
    fi
    if [[ "$errored" -gt "$transformed" ]]; then
        echo "error: $source_name errored on more events than it transformed, so its profile is of the error path" >&2
        return 1
    fi
}

# ----------------------------------------------------------------------------
# Cleanup
# ----------------------------------------------------------------------------

SERVICE_PID=""
KAFKA_CID=""
WORK_DIR="$(mktemp -d -t pgo-workload-XXXXXX)"

# SIGTERM, never SIGKILL: the instrumented binary writes its .profraw on a
# normal exit, so killing it outright discards the profile this run just spent
# its whole budget accumulating.
stop_service() {
    if [[ -z "$SERVICE_PID" ]] || ! kill -0 "$SERVICE_PID" 2>/dev/null; then
        SERVICE_PID=""
        return 0
    fi
    kill -TERM "$SERVICE_PID" 2>/dev/null || true
    local waited=0
    while kill -0 "$SERVICE_PID" 2>/dev/null && [[ $waited -lt 30 ]]; do
        sleep 1
        waited=$(( waited + 1 ))
    done
    if kill -0 "$SERVICE_PID" 2>/dev/null; then
        echo "warn: service did not exit in 30s; killing it and LOSING its profile" >&2
        kill -KILL "$SERVICE_PID" 2>/dev/null || true
    fi
    SERVICE_PID=""
}

cleanup() {
    local rc=$?
    if [[ "$KEEP" == "1" ]]; then
        echo "PGO_WORKLOAD_KEEP=1 -- skipping cleanup" >&2
        echo "  service PID: $SERVICE_PID" >&2
        echo "  kafka CID:   $KAFKA_CID" >&2
        echo "  work dir:    $WORK_DIR" >&2
        exit $rc
    fi
    echo "pgo-workload: cleanup" >&2
    stop_service
    if [[ -n "$KAFKA_CID" ]]; then
        docker rm -f "$KAFKA_CID" >/dev/null 2>&1 || true
    fi
    if [[ -n "$WORK_DIR" && -d "$WORK_DIR" ]]; then
        rm -rf "$WORK_DIR"
    fi
    exit $rc
}
trap cleanup EXIT INT TERM

PGO_DRIVER_PATH="${PGO_DRIVER_PATH:-}"
if [[ -z "$PGO_DRIVER_PATH" || ! -x "$PGO_DRIVER_PATH" ]]; then
    PGO_DRIVER_PATH="$(find_driver)"
fi
if [[ -z "$PGO_DRIVER_PATH" || ! -x "$PGO_DRIVER_PATH" ]]; then
    echo "error: pgo-driver not found after building" >&2
    exit 1
fi
echo "pgo-workload: driver at $PGO_DRIVER_PATH"

# ----------------------------------------------------------------------------
# Redpanda
# ----------------------------------------------------------------------------

# Redpanda rather than the Kafka JVM: a JVM heap starves the instrumented binary
# on a small runner, and the wire protocol is the same either way.
echo "pgo-workload: starting Redpanda ($KAFKA_IMAGE)"
# Two listeners: a client follows the broker's advertised address after its
# first connection, so rpk inside the container needs one it can reach there
# and the service and driver on the host need the published port.
KAFKA_CID=$(docker run -d --rm \
    -p "127.0.0.1:${KAFKA_PORT}:${KAFKA_PORT}" \
    "$KAFKA_IMAGE" \
    redpanda start \
    --mode dev-container \
    --smp 1 \
    --memory 512M \
    --kafka-addr "internal://0.0.0.0:9092,external://0.0.0.0:${KAFKA_PORT}" \
    --advertise-kafka-addr "internal://localhost:9092,external://localhost:${KAFKA_PORT}")
echo "pgo-workload: Redpanda CID: $KAFKA_CID"

# The admin API's own health verdict, not a TCP probe: the port accepts a
# connection well before the broker serves.
for attempt in $(seq 1 60); do
    if docker exec "$KAFKA_CID" rpk cluster health 2>/dev/null | grep -q "Healthy:.*true"; then
        echo "pgo-workload: Redpanda ready (attempt $attempt)"
        break
    fi
    if [[ "$attempt" -eq 60 ]]; then
        echo "error: Redpanda did not become ready in 120s" >&2
        docker logs --tail 50 "$KAFKA_CID" >&2
        exit 1
    fi
    sleep 2
done

# ----------------------------------------------------------------------------
# One run per source
# ----------------------------------------------------------------------------

# Profiles accumulate across the runs because %p expands to each process's pid,
# so restarting the service adds a file rather than overwriting one. Honour an
# externally set LLVM_PROFILE_FILE, which is how cargo-pgo points at its own
# directory.
TARGET_DIR="${CARGO_TARGET_DIR:-$PROJECT_ROOT/target}"
export LLVM_PROFILE_FILE="${LLVM_PROFILE_FILE:-$TARGET_DIR/pgo-profiles/pgo-%p_%m.profraw}"
mkdir -p "$(dirname "$LLVM_PROFILE_FILE")"
echo "pgo-workload: profiles to $LLVM_PROFILE_FILE"

for entry in "${SOURCES[@]}"; do
    SOURCE_NAME="${entry%%|*}"
    FIXTURE_REL="${entry##*|}"
    FIXTURE="$PROJECT_ROOT/$FIXTURE_REL"

    if [[ ! -f "$FIXTURE" ]]; then
        echo "error: fixture $FIXTURE is missing" >&2
        exit 1
    fi

    SLUG="${SOURCE_NAME//./_}"
    IN_TOPIC="pgo_${SLUG}_in"
    OUT_TOPIC="pgo_${SLUG}_out"

    # Pre-create both topics. Redpanda auto-creates on PRODUCE but NOT on a
    # consumer SUBSCRIBE, so the service would never find its topic, never
    # reach ready, and the driver -- which only starts once it is ready --
    # would never produce. Apache Kafka's auto-create-on-subscribe used to mask
    # this. The client runs inside the broker container, against the internal
    # listener the readiness check above already uses.
    for topic in "$IN_TOPIC" "$OUT_TOPIC"; do
        # An "already exists" failure is expected on a re-run, and any other
        # failure ends in the deadlock above, so presence is checked.
        docker exec "$KAFKA_CID" rpk \
            topic create "$topic" -p 3 -X brokers=localhost:9092 >/dev/null || true
        if ! docker exec "$KAFKA_CID" rpk \
            topic describe "$topic" -X brokers=localhost:9092 >/dev/null 2>&1; then
            echo "error: topic $topic is absent and could not be created" >&2
            echo "  the consumer would subscribe to a missing topic, never reach" >&2
            echo "  ready, and the driver would never produce" >&2
            exit 1
        fi
    done
    echo "pgo-workload: topics $IN_TOPIC, $OUT_TOPIC confirmed"

    CONFIG_FILE="$WORK_DIR/$SLUG.yaml"
    # geoip is off because provisioning downloads MMDB databases at startup,
    # which a workload run has no network budget for and no parity claim on.
    cat >"$CONFIG_FILE" <<YAML
source:
  name: $SOURCE_NAME
  envelope: auto
  topics:
  - $IN_TOPIC
  batch_size: 20000
  max_batch_bytes: 16777216
  group_id: pgo-workload-$SLUG
  brokers:
  - localhost:${KAFKA_PORT}
sink:
  topic: $OUT_TOPIC
  brokers:
  - localhost:${KAFKA_PORT}
geoip:
  enabled: false
  auto_download:
    enabled: false
YAML

    echo "pgo-workload: $SOURCE_NAME for ${SLICE}s (fixture $FIXTURE_REL)"
    env --chdir="$WORK_DIR" "${KAFKA_ENV_SCRUB[@]}" \
        METRICS_ADDR="127.0.0.1:$METRICS_PORT" \
        "$SERVICE_BIN" --config "$CONFIG_FILE" run \
        >"$WORK_DIR/$SLUG.log" 2>&1 &
    SERVICE_PID=$!

    for attempt in $(seq 1 60); do
        if ! kill -0 "$SERVICE_PID" 2>/dev/null; then
            echo "error: service died during startup on $SOURCE_NAME" >&2
            tail -100 "$WORK_DIR/$SLUG.log" >&2
            exit 1
        fi
        if curl -sf -o /dev/null --max-time 1 "http://127.0.0.1:$METRICS_PORT/readyz"; then
            echo "pgo-workload: service ready (attempt $attempt)"
            break
        fi
        if [[ "$attempt" -eq 60 ]]; then
            echo "error: service did not become ready in 60s on $SOURCE_NAME" >&2
            tail -100 "$WORK_DIR/$SLUG.log" >&2
            exit 1
        fi
        sleep 1
    done

    # Ready means the probes serve; the consumer group still has to join, and
    # anything produced before it does is read from the committed offset later
    # rather than transformed now.
    sleep 2

    env "${KAFKA_ENV_SCRUB[@]}" \
        PGO_DRIVER_DURATION_SECS="$SLICE" \
        PGO_DRIVER_BROKERS="127.0.0.1:${KAFKA_PORT}" \
        PGO_DRIVER_TOPIC="$IN_TOPIC" \
        PGO_DRIVER_RPS="${PGO_DRIVER_RPS:-5000}" \
        PGO_DRIVER_FIXTURES="$FIXTURE" \
        "$PGO_DRIVER_PATH"

    # Let the service drain what is still queued before it is asked to stop.
    sleep 5
    check_transformed "$SOURCE_NAME"
    stop_service
    echo "pgo-workload: $SOURCE_NAME done (log: $WORK_DIR/$SLUG.log)"
done

echo "pgo-workload: complete"
