# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""clock_check.py CERT HOST...: each board's /status Unix time against this
host's clock (the host's own NTP sync aside), from the midpoint of the
request."""
import http.client
import ssl
import sys
import time

context = ssl.create_default_context(cafile=sys.argv[1])
for host in sys.argv[2:]:
    c = http.client.HTTPSConnection(host, 443, timeout=10, context=context)
    c.connect()
    before = time.time()
    c.request("GET", "/status")
    body = c.getresponse().read().decode()
    after = time.time()
    line = next((l for l in body.splitlines() if l.startswith("Unix time")), "")
    board = line.split("Some(")[-1].rstrip(")") if "Some(" in line else None
    if board is None:
        print(f"{host}: clockless ({line})")
    else:
        print(f"{host}: board {board} s, host {(before + after) / 2:.3f} s, difference {int(board) - (before + after) / 2:+.3f} s (the board reports whole seconds)")
