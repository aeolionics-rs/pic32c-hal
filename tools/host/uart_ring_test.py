# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Drive a board's uart_ring: uart_ring_test.py PORT [BAUD] [BYTES] [SHARE].

First the overrun: sends G until the board says `pause`, sends 4 KiB into
the pause (more than its ring), and reads its `overrun:` verdict. Then the
echo: sends BYTES pseudo-random bytes (100,000 by default) and checks what
comes back is the same, in order. Without SHARE they go as fast as the port
takes them; an echo at the line rate falls behind, so long streams need a
ring that holds the backlog. With SHARE (0 to 1) they go in 1 KiB pieces at
that share of the line rate, so the echo can't fall behind and any loss is
the receiver's; the most bytes not yet echoed at any point is reported.

BAUD is 115,200 by default, as uart_ring is built unless UART_BAUD says
otherwise. Reset the board before each run: the overrun phase runs once.
"""
import random
import sys
import time

import serial

port = sys.argv[1]
baud = int(sys.argv[2]) if len(sys.argv) > 2 else 115200
total = int(sys.argv[3]) if len(sys.argv) > 3 else 100000
share = float(sys.argv[4]) if len(sys.argv) > 4 else None
s = serial.Serial(port, baud, timeout=0.05)
time.sleep(0.2)
s.reset_input_buffer()

seen = b""
deadline = time.monotonic() + 10
while b"pause" not in seen and time.monotonic() < deadline:
    s.write(b"G")
    seen += s.read(256)
if b"pause" not in seen:
    print(f"overrun: FAILED, no pause: {seen[-80:]!r}")
    sys.exit(1)
s.write(bytes(random.Random(2).randrange(256) for _ in range(4096)))
seen = b""
deadline = time.monotonic() + 5
while b"overrun: ok" not in seen and b"overrun: FAILED" not in seen and time.monotonic() < deadline:
    seen += s.read(256)
overran = b"overrun: ok" in seen
print("overrun: ok" if overran else f"overrun: FAILED {seen[-80:]!r}")
time.sleep(0.1)
s.reset_input_buffer()

data = bytes(random.Random(1).randrange(256) for _ in range(total))
got = bytearray()
behind = 0
piece = 1024 if share else 4096
period = piece * 10 / baud / share if share else 0
start = time.monotonic()
for n, i in enumerate(range(0, total, piece)):
    while time.monotonic() < start + n * period:
        got += s.read(s.in_waiting)
    s.write(data[i:i + piece])
    got += s.read(s.in_waiting)
    behind = max(behind, min(i + piece, total) - len(got))
deadline = time.monotonic() + 5 + total * 10 / baud
while len(got) < total and time.monotonic() < deadline:
    got += s.read(max(1, s.in_waiting))
took = time.monotonic() - start
same = got == data
first = next((i for i, (a, b) in enumerate(zip(got, data)) if a != b), None)
verdict = "identical" if same else f"FAILED, first difference at {first}"
print(f"echo: sent {total}, got {len(got)}, {verdict}, at most {behind} not yet echoed, {total * 10 / took:.0f} bit/s")
sys.exit(0 if same and overran else 1)
