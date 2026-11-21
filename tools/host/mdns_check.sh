#!/bin/sh
# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
# mdns_check.sh NAME...: what macOS's mDNS sees for each board NAME (no
# .local): its addresses, the _https._tcp services browsed, the service's
# host and port, and a ping by name over IPv4 and IPv6.
run() { # run SECONDS COMMAND...: dns-sd keeps going, so stop it
  t=$1; shift
  "$@" & pid=$!
  sleep "$t"; kill "$pid" 2>/dev/null; wait "$pid" 2>/dev/null
}
for name in "$@"; do
  echo "== $name.local"
  run 2 dns-sd -G v4v6 "$name.local" | grep -i " add " | awk '{print "  address:", $6}'
  run 2 dns-sd -L "$name" _https._tcp local | grep -i "can be reached" | sed 's/^.*can be reached at/  service at/'
  ping -c 2 -t 4 "$name.local" | tail -1 | sed 's/^/  ping v4: /'
  ping6 -c 2 "$name.local" 2>&1 | tail -1 | sed 's/^/  ping v6: /'
done
echo "== _https._tcp browse"
run 3 dns-sd -B _https._tcp local | grep -i " add " | awk '{print "  instance:", $7}'
