#!/usr/bin/env bash
# Project:   dfe-transform-elastic
# File:      scripts/download-geoip.sh
# Purpose:   Fetch the DB-IP Lite MMDB databases the GeoIP tests need
# Language:  Bash
#
# License:   BUSL-1.1
# Copyright: (c) 2026 HYPERI PTY LIMITED
#
# The databases themselves are DB-IP Lite, CC BY 4.0, and are gitignored.
# Same approach as dfe-loader's scripts/download-geoip.sh.

set -euo pipefail

DEST="${1:-testdata/geoip}"
mkdir -p "${DEST}"

YEAR=$(date +%Y)
MONTH=$(date +%m)

CITY_URL="https://download.db-ip.com/free/dbip-city-lite-${YEAR}-${MONTH}.mmdb.gz"
ASN_URL="https://download.db-ip.com/free/dbip-asn-lite-${YEAR}-${MONTH}.mmdb.gz"

echo "Downloading DB-IP Lite City..."
curl -fSL "${CITY_URL}" | gunzip > "${DEST}/dbip-city-lite.mmdb" || echo "WARN: City download failed"

echo "Downloading DB-IP Lite ASN..."
curl -fSL "${ASN_URL}" | gunzip > "${DEST}/dbip-asn-lite.mmdb" || echo "WARN: ASN download failed"

cat > "${DEST}/ATTRIBUTION.txt" << 'EOF'
This product includes GeoLite2 Data created by DB-IP.
Licensed under CC BY 4.0: https://creativecommons.org/licenses/by/4.0/
Source: https://db-ip.com/db/lite.php
EOF

echo "Done. Files in ${DEST}:"
ls -la "${DEST}"
