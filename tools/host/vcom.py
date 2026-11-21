# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Timestamped capture of a serial port: vcom.py PORT SECONDS [BAUD]."""
import sys, time, serial
port, seconds = sys.argv[1], float(sys.argv[2])
baud = int(sys.argv[3]) if len(sys.argv) > 3 else 115200
s = serial.Serial(port, baud, timeout=0.2)
end = time.monotonic() + seconds
buf = b""
while time.monotonic() < end:
    buf += s.read(256)
    while b"\n" in buf:
        line, buf = buf.split(b"\n", 1)
        print(f"{time.time():.3f} {line.decode(errors='replace').rstrip()}", flush=True)
if buf:
    print(f"{time.time():.3f} {buf.decode(errors='replace')} (no newline)", flush=True)
