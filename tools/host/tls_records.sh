#!/bin/sh
# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
# tls_records.sh HOST CERT PATH: how many application-data TLS records the
# server sends for one GET of PATH (openssl s_client -msg lists them).
printf 'GET %s HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n' "$3" |
  openssl s_client -connect "$1:443" -CAfile "$2" -verify_return_error -msg -ign_eof 2>/dev/null |
  grep -c '^<<< TLS 1.3, InnerContent' | sed "s|^|encrypted records received for GET $3 (one is the handshake, one the close_notify): |"
