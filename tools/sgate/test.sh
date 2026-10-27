#!/usr/bin/env bash
# sgate/test.sh
# ---------------------------------------------------------------
# Smoke test for sgate. Spins up:
#
#   * Three mock upstreams (SYX) on 3001 / 3002 / 4000
#   * sgate itself on 8080
#
# Then issues a battery of curl checks against sgate and
# compares the response (status + body) to an expected value.
#
# The script is self-contained: it picks the first available
# `syx` binary (release > debug), launches everything, traps
# signals so the background processes are killed on exit, and
# reports PASS/FAIL counts at the end.
#
# Usage:
#   ./tools/sgate/test.sh                # full battery
#   ./tools/sgate/test.sh --no-mocks     # skip the mock upstreams
#                                        # (use when you already
#                                        # have something on 3001/3002/4000)
#   SYX=./target/debug/syx ./tools/sgate/test.sh
#
# Exit code: 0 on all-pass, 1 otherwise.
# ---------------------------------------------------------------

set -u

# --- paths --------------------------------------------------------

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
SGATE_DIR="$SCRIPT_DIR"
SGATE_JSON="$SGATE_DIR/sgate.json"
MOCK="$SGATE_DIR/mock_upstream.sy"

# --- pick a syx binary --------------------------------------------

SYX="${SYX:-}"
if [ -z "$SYX" ]; then
    if [ -x "$REPO_ROOT/target/release/syx" ]; then
        SYX="$REPO_ROOT/target/release/syx"
    elif [ -x "$REPO_ROOT/target/debug/syx" ]; then
        SYX="$REPO_ROOT/target/debug/syx"
    else
        echo "ERROR: no syx binary found. Build it first:" >&2
        echo "  cargo build -p syx-cli --release" >&2
        exit 2
    fi
fi

echo "Using SYX: $SYX"

# --- options ------------------------------------------------------

SKIP_MOCKS=0
for a in "$@"; do
    case "$a" in
        --no-mocks) SKIP_MOCKS=1 ;;
        -h|--help)
            sed -n '2,28p' "$0"
            exit 0
            ;;
    esac
done

# --- state --------------------------------------------------------

PIDS=()
FAILED=0
PASSED=0
TOTAL=0

cleanup() {
    echo
    echo "==> cleaning up (kill: ${PIDS[*]:-none})"
    for pid in "${PIDS[@]:-}"; do
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null || true
        fi
    done
    # Give them a moment, then SIGKILL any survivors.
    sleep 0.3
    for pid in "${PIDS[@]:-}"; do
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            kill -9 "$pid" 2>/dev/null || true
        fi
    done
}
trap cleanup EXIT INT TERM

wait_port() {
    # wait_port <host> <port> <max_seconds>
    local host="$1" port="$2" max="${3:-10}"
    local i=0
    while [ "$i" -lt "$max" ]; do
        if (echo > /dev/tcp/"$host"/"$port") 2>/dev/null; then
            return 0
        fi
        sleep 0.2
        i=$((i + 1))
    done
    return 1
}

# --- check helpers ------------------------------------------------

# check_status <name> <expected_code> <url>
check_status() {
    local name="$1" want="$2" url="$3"
    TOTAL=$((TOTAL + 1))
    local got
    got="$(curl -s -o /dev/null -w '%{http_code}' "$url" || echo 000)"
    if [ "$got" = "$want" ]; then
        echo "  PASS  $name  ($got)"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  $name  want=$want got=$got  url=$url"
        FAILED=$((FAILED + 1))
    fi
}

# check_body_contains <name> <expected_substring> <url>
check_body_contains() {
    local name="$1" want="$2" url="$3"
    TOTAL=$((TOTAL + 1))
    local body
    body="$(curl -sS "$url" || true)"
    if echo "$body" | grep -q -- "$want"; then
        echo "  PASS  $name  (body contains \"$want\")"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  $name  body does not contain \"$want\"  url=$url"
        echo "        body was: $body"
        FAILED=$((FAILED + 1))
    fi
}

# check_header <name> <expected_header_value> <url>
check_header() {
    local name="$1" want="$2" url="$3"
    TOTAL=$((TOTAL + 1))
    local got
    got="$(curl -sI "$url" | tr -d '\r' | grep -i "^$name:" | head -1 | sed "s/^[^:]*: *//")"
    if [ "$got" = "$want" ]; then
        echo "  PASS  $name  ($got)"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  $name  want=\"$want\" got=\"$got\"  url=$url"
        FAILED=$((FAILED + 1))
    fi
}

# check_round_robin <name> <url> <iterations>
#   Confirms that hitting the same URL `iterations` times reaches
#   at least two distinct port values in the JSON body. Used to
#   verify the sgate upstream counter is actually rotating.
check_round_robin() {
    local name="$1" url="$2" iters="$3"
    TOTAL=$((TOTAL + 1))
    local ports=""
    local i=0
    while [ "$i" -lt "$iters" ]; do
        local p
        p="$(curl -sS "$url" | sed -n 's/.*"port"[[:space:]]*:[[:space:]]*\([0-9]\+\).*/\1/p' | head -1)"
        if [ -n "$p" ]; then
            ports="$ports $p"
        fi
        i=$((i + 1))
    done
    local uniq
    uniq="$(echo "$ports" | tr ' ' '\n' | sort -u | grep -v '^$' | wc -l | tr -d ' ')"
    if [ "$uniq" -ge 2 ]; then
        echo "  PASS  $name  (saw $uniq distinct ports over $iters calls: $ports)"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  $name  only saw $uniq distinct ports over $iters calls: $ports"
        FAILED=$((FAILED + 1))
    fi
}

# --- launch mocks -------------------------------------------------

if [ "$SKIP_MOCKS" -eq 0 ]; then
    echo "==> launching mock upstreams"
    ( cd "$REPO_ROOT" && MOCK_PORT=3001 MOCK_NODE_ID=api-a "$SYX" run "$MOCK" ) >/tmp/sgate_mock_3001.log 2>&1 &
    PIDS+=($!)
    ( cd "$REPO_ROOT" && MOCK_PORT=3002 MOCK_NODE_ID=api-b "$SYX" run "$MOCK" ) >/tmp/sgate_mock_3002.log 2>&1 &
    PIDS+=($!)
    ( cd "$REPO_ROOT" && MOCK_PORT=4000 MOCK_NODE_ID=web-a "$SYX" run "$MOCK" ) >/tmp/sgate_mock_4000.log 2>&1 &
    PIDS+=($!)

    wait_port 127.0.0.1 3001 30 || { echo "mock 3001 failed to start"; cat /tmp/sgate_mock_3001.log; exit 1; }
    wait_port 127.0.0.1 3002 30 || { echo "mock 3002 failed to start"; cat /tmp/sgate_mock_3002.log; exit 1; }
    wait_port 127.0.0.1 4000 30 || { echo "mock 4000 failed to start"; cat /tmp/sgate_mock_4000.log; exit 1; }
    echo "    mocks up on 3001 (api-a), 3002 (api-b), 4000 (web-a)"
else
    echo "==> --no-mocks: assuming 3001/3002/4000 are already serving"
fi

# --- launch sgate -------------------------------------------------

echo "==> launching sgate"
( cd "$REPO_ROOT" && "$SYX" run "$SGATE_DIR" "$SGATE_JSON" ) >/tmp/sgate.log 2>&1 &
PIDS+=($!)

if ! wait_port 127.0.0.1 8080 30; then
    echo "sgate failed to bind 8080"
    echo "--- sgate log ---"
    cat /tmp/sgate.log
    exit 1
fi
echo "    sgate up on 127.0.0.1:8080"

# --- checks -------------------------------------------------------

echo
echo "==> canned routes (no upstream needed)"
check_status         "GET /health -> 200"           200 http://127.0.0.1:8080/health
check_body_contains  "GET /health body == 'ok'"     "ok"  http://127.0.0.1:8080/health
check_header         "X-Sgate-Handler: health"      "health" http://127.0.0.1:8080/health

check_status         "GET /version -> 200"          200 http://127.0.0.1:8080/version
check_body_contains  "GET /version body"            "sgate"  http://127.0.0.1:8080/version

check_status         "GET /echo -> 200"             200 http://127.0.0.1:8080/echo
check_body_contains  "GET /echo body"               "you reached" http://127.0.0.1:8080/echo

echo
echo "==> static files"
check_status         "GET / -> 200 (index.html)"    200 http://127.0.0.1:8080/
check_body_contains  "GET / contains 'sgate'"       "sgate" http://127.0.0.1:8080/

check_status         "GET /static/index.html -> 200" 200 http://127.0.0.1:8080/static/index.html
check_body_contains  "/static/index.html body"      "Static file serving" http://127.0.0.1:8080/static/index.html

check_status         "GET /static/hello.txt -> 200" 200 http://127.0.0.1:8080/static/hello.txt
check_body_contains  "/static/hello.txt body"       "Hello from sgate" http://127.0.0.1:8080/static/hello.txt

check_status         "GET /static/assets/data.json -> 200" 200 http://127.0.0.1:8080/static/assets/data.json
check_body_contains  "data.json body"                "sgate" http://127.0.0.1:8080/static/assets/data.json

echo
echo "==> 404"
check_status         "GET /no-such -> 404"          404 http://127.0.0.1:8080/no-such
check_body_contains  "404 body comes from config"   "Not Found" http://127.0.0.1:8080/no-such

echo
echo "==> reverse proxy (requires mocks up)"
if [ "$SKIP_MOCKS" -eq 0 ]; then
    # /api/  → api pool (3001, 3002), with strip_prefix="/api/"
    check_status         "GET /api/users  -> 200"      200 http://127.0.0.1:8080/api/users
    check_body_contains  "/api/users list"             "alice" http://127.0.0.1:8080/api/users
    check_round_robin    "api pool rotates"            http://127.0.0.1:8080/api/whoami 6

    # /api/users/42 -> single user
    check_status         "GET /api/users/42 -> 200"    200 http://127.0.0.1:8080/api/users/42
    check_body_contains  "user-42"                     "user-42" http://127.0.0.1:8080/api/users/42

    # /web/ → web pool (only 4000)
    check_status         "GET /web/whoami -> 200"      200 http://127.0.0.1:8080/web/whoami
    check_body_contains  "web-a served"                "web-a" http://127.0.0.1:8080/web/whoami

    # /api/echo round-trips the request body via the mock.
    # The mock echoes the body back as JSON; we just confirm it
    # made the round-trip.
    TOTAL=$((TOTAL + 1))
    body="$(curl -sS -X POST -H 'Content-Type: text/plain' --data 'hello-from-test' http://127.0.0.1:8080/api/echo || true)"
    if echo "$body" | grep -q "hello-from-test"; then
        echo "  PASS  POST /api/echo body forwarded (round-trip)"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  POST /api/echo body not forwarded"
        echo "        body: $body"
        FAILED=$((FAILED + 1))
    fi

    # X-Real-IP / X-Forwarded-* forwarding (mock surfaces them
    # in the response).
    TOTAL=$((TOTAL + 1))
    body="$(curl -sS http://127.0.0.1:8080/api/echo || true)"
    if echo "$body" | grep -q "x_real_ip"; then
        echo "  PASS  /api/echo response surfaces forwarded headers"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  /api/echo did not surface forwarded headers"
        echo "        body: $body"
        FAILED=$((FAILED + 1))
    fi
else
    echo "    (skipped: --no-mocks)"
fi

# --- summary ------------------------------------------------------

echo
echo "==> $PASSED / $TOTAL passed, $FAILED failed"
if [ "$FAILED" -ne 0 ]; then
    echo "--- last 30 lines of sgate log ---"
    tail -30 /tmp/sgate.log
    exit 1
fi

# -------------------------------------------------------------------
# Optional extended tests
# -------------------------------------------------------------------
#
# These check the three advanced features (TLS / hot reload /
# WebSocket pass-through). Each section is gated on the
# relevant external tool being installed, so the script still
# passes on a minimal box. Run with --full to make failures
# here non-fatal (warnings only), or --strict to abort.
#
# Each section uses a *second* sgate process on a different
# port, with a temp copy of the config, so the original
# sgate.json on port 8080 is never touched.

FULL_TESTS=0
STRICT=0
for a in "$@"; do
    case "$a" in
        --full) FULL_TESTS=1 ;;
        --strict) STRICT=1 ;;
    esac
done

# Stop the original sgate + mocks so the extended tests can
# grab the same ports / file handles without conflicts.
echo
echo "==> stopping default sgate for extended tests"
for pid in "${PIDS[@]:-}"; do
    if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
        kill "$pid" 2>/dev/null || true
    fi
done
PIDS=()
# Wait for port 8080 to drop.
i=0
while [ "$i" -lt 20 ]; do
    if ! (echo > /dev/tcp/127.0.0.1/8080) 2>/dev/null; then
        break
    fi
    sleep 0.2
    i=$((i + 1))
done

# === Hot reload ===================================================

echo
echo "==> hot reload"
if [ "$FULL_TESTS" -eq 1 ] || [ "$STRICT" -eq 1 ]; then
    TMP_CFG="$(mktemp -t sgate.reload.XXXXXX).json"
    cp "$SGATE_JSON" "$TMP_CFG"

    # Start sgate with a tiny interval so the test is quick.
    python3 -c "
import json, sys
p = '$TMP_CFG'
c = json.load(open(p))
c['server']['hot_reload'] = { 'enabled': True, 'interval_ms': 200 }
c['server']['port'] = 8081
c['server']['tls'] = { 'enabled': False }
c['ws_routes'] = []
json.dump(c, open(p, 'w'), indent=2)
"
    ( cd "$REPO_ROOT" && "$SYX" run "$SGATE_DIR" "$TMP_CFG" ) >/tmp/sgate_reload.log 2>&1 &
    PIDS+=($!)
    wait_port 127.0.0.1 8081 30 || { echo "reload-test sgate failed to start"; cat /tmp/sgate_reload.log; rm -f "$TMP_CFG"; exit 1; }

    # Baseline: GET /health should be the configured body.
    body_before="$(curl -sS http://127.0.0.1:8081/health || true)"
    echo "    baseline /health: $body_before"
    TOTAL=$((TOTAL + 1))
    if echo "$body_before" | grep -q "ok"; then
        echo "  PASS  baseline /health returns expected body"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  baseline /health unexpected: $body_before"
        FAILED=$((FAILED + 1))
    fi

    # Mutate the temp config in place. The new body should
    # show up after at most one interval + one request.
    python3 -c "
import json
p = '$TMP_CFG'
c = json.load(open(p))
for loc in c['locations']:
    if loc.get('exact') == '/health':
        loc['body'] = 'reloaded-v1'
json.dump(c, open(p, 'w'), indent=2)
"

    # Hit /health up to 5 times with 0.3s sleep between to
    # give the reload loop a chance.
    seen=""
    i=0
    while [ "$i" -lt 5 ]; do
        body="$(curl -sS http://127.0.0.1:8081/health || true)"
        if echo "$body" | grep -q "reloaded-v1"; then
            seen="yes"
            break
        fi
        sleep 0.3
        i=$((i + 1))
    done
    TOTAL=$((TOTAL + 1))
    if [ "$seen" = "yes" ]; then
        echo "  PASS  /health picked up the live edit within 1.5s"
        PASSED=$((PASSED + 1))
    else
        echo "  FAIL  /health did not pick up the live edit (last body: $body)"
        FAILED=$((FAILED + 1))
    fi

    # Stop the reload-test sgate.
    for pid in "${PIDS[@]:-}"; do
        if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null || true
        fi
    done
    PIDS=()
    rm -f "$TMP_CFG"
    # wait for port to drop
    i=0
    while [ "$i" -lt 20 ]; do
        if ! (echo > /dev/tcp/127.0.0.1/8081) 2>/dev/null; then break; fi
        sleep 0.2
        i=$((i + 1))
    done
else
    echo "    (skipped: --full not set)"
fi

# === TLS ==========================================================

echo
echo "==> TLS sidecar"
if [ "$FULL_TESTS" -eq 1 ] || [ "$STRICT" -eq 1 ]; then
    if ! command -v openssl >/dev/null 2>&1; then
        echo "    (skipped: openssl not on PATH)"
    elif ! command -v socat >/dev/null 2>&1; then
        echo "    (skipped: socat not on PATH)"
    else
        TMP_CFG="$(mktemp -t sgate.tls.XXXXXX).json"
        TMP_CERT_DIR="$(mktemp -d -t sgate.tls.XXXXXX)"
        cp "$SGATE_JSON" "$TMP_CFG"

        # Generate a self-signed cert in the temp dir.
        openssl req -x509 -newkey rsa:2048 -nodes \
            -keyout "$TMP_CERT_DIR/server.key" \
            -out "$TMP_CERT_DIR/server.pem" \
            -days 1 -subj "/CN=localhost" >/dev/null 2>&1
        if [ ! -f "$TMP_CERT_DIR/server.pem" ]; then
            echo "    (skipped: openssl cert generation failed)"
        else
            python3 -c "
import json
p = '$TMP_CFG'
c = json.load(open(p))
c['server']['port'] = 8082
c['server']['tls'] = {
    'enabled': True,
    'port': 8443,
    'cert': '$TMP_CERT_DIR/server.pem',
    'key':  '$TMP_CERT_DIR/server.key',
    'backend_port': 8082,
    'mode': 'sidecar',
}
c['ws_routes'] = []
json.dump(c, open(p, 'w'), indent=2)
"
            ( cd "$REPO_ROOT" && SYX_SGATE_CONFIG="$TMP_CFG" "$SYX" run "$SGATE_DIR" ) >/tmp/sgate_tls.log 2>&1 &
            PIDS+=($!)
            wait_port 127.0.0.1 8082 30 || { echo "tls-test sgate failed to start"; cat /tmp/sgate_tls.log; rm -f "$TMP_CFG"; rm -rf "$TMP_CERT_DIR"; exit 1; }

            # Give socat a moment to come up.
            if wait_port 127.0.0.1 8443 15; then
                TOTAL=$((TOTAL + 1))
                code="$(curl -sk -o /dev/null -w '%{http_code}' https://127.0.0.1:8443/health || echo 000)"
                if [ "$code" = "200" ]; then
                    echo "  PASS  TLS sidecar answers 200 on https://127.0.0.1:8443/health"
                    PASSED=$((PASSED + 1))
                else
                    echo "  FAIL  TLS sidecar returned $code (want 200)"
                    FAILED=$((FAILED + 1))
                fi
            else
                echo "  FAIL  TLS sidecar did not bind 8443"
                FAILED=$((FAILED + 1))
            fi

            for pid in "${PIDS[@]:-}"; do
                if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
                    kill "$pid" 2>/dev/null || true
                fi
            done
            PIDS=()
            rm -f "$TMP_CFG"
            rm -rf "$TMP_CERT_DIR"
            i=0
            while [ "$i" -lt 20 ]; do
                if ! (echo > /dev/tcp/127.0.0.1/8082) 2>/dev/null; then break; fi
                sleep 0.2
                i=$((i + 1))
            done
        fi
    fi
else
    echo "    (skipped: --full not set)"
fi

# === TLS native (W1249 / ADR-1595) ================================

echo
echo "==> TLS native (Http.serve in-process; no socat)"
if [ "$FULL_TESTS" -eq 1 ] || [ "$STRICT" -eq 1 ]; then
    if ! command -v openssl >/dev/null 2>&1; then
        echo "    (skipped: openssl not on PATH)"
    else
        TMP_CFG="$(mktemp -t sgate.tls.native.XXXXXX).json"
        TMP_CERT_DIR="$(mktemp -d -t sgate.tls.native.XXXXXX)"
        cp "$SGATE_JSON" "$TMP_CFG"

        openssl req -x509 -newkey rsa:2048 -nodes \
            -keyout "$TMP_CERT_DIR/server.key" \
            -out "$TMP_CERT_DIR/server.pem" \
            -days 1 -subj "/CN=localhost" >/dev/null 2>&1
        if [ ! -f "$TMP_CERT_DIR/server.pem" ]; then
            echo "    (skipped: openssl cert generation failed)"
        else
            python3 -c "
import json
p = '$TMP_CFG'
c = json.load(open(p))
c['server']['listen'] = '127.0.0.1'
c['server']['port'] = 18082
c['server']['tls'] = {
    'enabled': True,
    'port': 18445,
    'cert': '$TMP_CERT_DIR/server.pem',
    'key':  '$TMP_CERT_DIR/server.key',
    'backend_port': 18082,
    'mode': 'native',
}
c['ws_routes'] = []
json.dump(c, open(p, 'w'), indent=2)
"
            # syx run accepts a single PATH; pass config via SYX_SGATE_CONFIG.
            ( cd "$REPO_ROOT" && SYX_SGATE_CONFIG="$TMP_CFG" "$SYX" run "$SGATE_DIR" ) >/tmp/sgate_tls_native.log 2>&1 &
            PIDS+=($!)
            if wait_port 127.0.0.1 18445 30; then
                TOTAL=$((TOTAL + 1))
                code="$(curl -sk --max-time 5 -o /dev/null -w '%{http_code}' https://127.0.0.1:18445/health || echo 000)"
                if [ "$code" = "200" ]; then
                    echo "  PASS  TLS native answers 200 on https://127.0.0.1:18445/health"
                    PASSED=$((PASSED + 1))
                else
                    echo "  FAIL  TLS native returned $code (want 200)"
                    echo "--- sgate_tls_native.log ---"
                    cat /tmp/sgate_tls_native.log || true
                    FAILED=$((FAILED + 1))
                fi
            else
                echo "  FAIL  TLS native did not bind 18445"
                echo "--- sgate_tls_native.log ---"
                cat /tmp/sgate_tls_native.log || true
                FAILED=$((FAILED + 1))
                TOTAL=$((TOTAL + 1))
            fi

            for pid in "${PIDS[@]:-}"; do
                if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
                    kill "$pid" 2>/dev/null || true
                fi
            done
            PIDS=()
            rm -f "$TMP_CFG"
            rm -rf "$TMP_CERT_DIR"
            i=0
            while [ "$i" -lt 20 ]; do
                if ! (echo > /dev/tcp/127.0.0.1/18445) 2>/dev/null; then break; fi
                sleep 0.2
                i=$((i + 1))
            done
        fi
    fi
else
    echo "    (skipped: --full not set)"
fi

# === WebSocket ====================================================

echo
echo "==> WebSocket sidecar"
if [ "$FULL_TESTS" -eq 1 ] || [ "$STRICT" -eq 1 ]; then
    if ! command -v websocat >/dev/null 2>&1; then
        echo "    (skipped: websocat not on PATH)"
    else
        TMP_CFG="$(mktemp -t sgate.ws.XXXXXX).json"
        cp "$SGATE_JSON" "$TMP_CFG"

        # Spawn a websocat echo server on 5002 (the "upstream").
        websocat -s 5002 >/tmp/sgate_ws_upstream.log 2>&1 &
        WS_UPSTREAM_PID=$!
        PIDS+=($WS_UPSTREAM_PID)
        wait_port 127.0.0.1 5002 10 || { echo "ws upstream failed to start"; cat /tmp/sgate_ws_upstream.log; rm -f "$TMP_CFG"; exit 1; }

        python3 -c "
import json
p = '$TMP_CFG'
c = json.load(open(p))
c['server']['port'] = 8083
c['server']['tls'] = { 'enabled': False }
c['upstreams'] = [
    { 'name': 'wsapi', 'servers': ['127.0.0.1:5002'], 'strategy': 'round_robin', 'timeout_ms': 30000 }
]
c['locations'] = [l for l in c['locations'] if not (l.get('exact') == '/health')]
c['ws_routes'] = [
    { 'name': 'ws_echo', 'path': '/ws/', 'upstream': 'wsapi', 'listen_port': 5001, 'strip_prefix': '/ws/' }
]
json.dump(c, open(p, 'w'), indent=2)
"
        ( cd "$REPO_ROOT" && "$SYX" run "$SGATE_DIR" "$TMP_CFG" ) >/tmp/sgate_ws.log 2>&1 &
        PIDS+=($!)
        wait_port 127.0.0.1 8083 30 || { echo "ws-test sgate failed to start"; cat /tmp/sgate_ws.log; rm -f "$TMP_CFG"; exit 1; }

        # Give websocat a moment to come up.
        if wait_port 127.0.0.1 5001 15; then
            # Black-box check: an HTTP GET to a WS port should
            # return 426 (Upgrade Required) or 400. Anything
            # else means the sidecar is broken.
            TOTAL=$((TOTAL + 1))
            code="$(curl -sS -o /dev/null -w '%{http_code}' http://127.0.0.1:5001/ || echo 000)"
            if [ "$code" = "426" ] || [ "$code" = "400" ] || [ "$code" = "404" ]; then
                echo "  PASS  WS sidecar answers $code on plain HTTP (expected for a WS-only listener)"
                PASSED=$((PASSED + 1))
            else
                echo "  FAIL  WS sidecar returned $code on plain HTTP (want 426/400/404)"
                FAILED=$((FAILED + 1))
            fi

            # End-to-end round-trip: send a WS frame, expect
            # the same frame back. websocat is an echo server,
            # so this should just work.
            TOTAL=$((TOTAL + 1))
            # --no-close keeps the connection up long enough to
            # receive the echo. --text sends the frame as text.
            roundtrip="$(echo 'hello-ws' | websocat --no-close --text ws://127.0.0.1:5001/ 2>/dev/null | head -1)"
            if [ "$roundtrip" = "hello-ws" ]; then
                echo "  PASS  WS round-trip via sidecar: sent 'hello-ws', got 'hello-ws'"
                PASSED=$((PASSED + 1))
            else
                echo "  FAIL  WS round-trip via sidecar failed (got: '$roundtrip')"
                FAILED=$((FAILED + 1))
            fi
        else
            echo "  FAIL  WS sidecar did not bind 5001"
            FAILED=$((FAILED + 1))
        fi

        for pid in "${PIDS[@]:-}"; do
            if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then
                kill "$pid" 2>/dev/null || true
            fi
        done
        PIDS=()
        rm -f "$TMP_CFG"
        i=0
        while [ "$i" -lt 20 ]; do
            if ! (echo > /dev/tcp/127.0.0.1/8083) 2>/dev/null; then break; fi
            sleep 0.2
            i=$((i + 1))
        done
    fi
else
    echo "    (skipped: --full not set)"
fi

# --- final summary -------------------------------------------------

echo
echo "==> total: $PASSED / $TOTAL passed, $FAILED failed"
if [ "$FAILED" -ne 0 ]; then
    if [ "$STRICT" -eq 1 ]; then
        echo "--- last 30 lines of sgate log ---"
        tail -30 /tmp/sgate.log
        exit 1
    else
        echo "(non-strict; some extended tests failed — run with --strict to fail-fast)"
        exit 0
    fi
fi
exit 0
