#!/bin/sh
# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
# usbmon_capture.sh [SECONDS] [OUT] (as root): records USB bus 1's traffic for
# SECONDS (default 240) with tcpdump on usbmon1 (the character device, which
# kernel lockdown allows; debugfs it doesn't), keeping each URB's header and
# its first 64 data bytes, to OUT (default usbmon1.pcap here), owned by the
# user who ran sudo.
OUT=${2:-usbmon1.pcap}
modprobe usbmon
timeout "${1:-240}" tcpdump -i usbmon1 -s 128 -w "$OUT"
[ -n "$SUDO_USER" ] && chown "$SUDO_USER" "$OUT"
ls -l "$OUT"
