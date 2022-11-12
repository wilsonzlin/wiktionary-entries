#!/usr/bin/env bash

set -Eeuo pipefail

curl -fLSs https://dumps.wikimedia.org/enwiktionary/20221101/enwiktionary-20221101-all-titles-in-ns0.gz | gzip -c -d - > entries.txt
