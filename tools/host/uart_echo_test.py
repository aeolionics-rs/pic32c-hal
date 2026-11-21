# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Drive pic32ck-curiosity-ultra's uart_echo: uart_echo_test.py PORT BAUD.

Checks the echo of single bytes, every byte value, blocks up to 2,000 bytes,
and 20,000 bytes in gapped chunks; then opens the port at the wrong rate
to make framing errors, and asks for the counts (ENQ) back at the right one.
Also stalls the receiver for 500 us (BEL) with 8 bytes on the way, which
the FIFO must hold.
Prints one result line per check and a summary.
"""
import os
import sys
import time

import serial

port, baud = sys.argv[1], int(sys.argv[2])
ENQ = b"\x05"
failures = 0


def check(name, good, detail=""):
    global failures
    print(f"{name}: {'ok' if good else 'FAILED'} {detail}".rstrip())
    failures += 0 if good else 1


def echo(s, data, timeout=5.0):
    s.write(data)
    got = b""
    deadline = time.monotonic() + timeout
    while len(got) < len(data) and time.monotonic() < deadline:
        got += s.read(len(data) - len(got))
    return got


def counts(s):
    s.reset_input_buffer()
    s.write(ENQ)
    line = s.readline().decode(errors="replace").strip()
    return line


s = serial.Serial(port, baud, timeout=0.5)
time.sleep(0.2)
s.reset_input_buffer()
print(f"port {port} at {baud}: {counts(s)}")

check("single bytes", all(echo(s, bytes([b])) == bytes([b]) for b in (0x00, 0x41, 0x7F, 0x80, 0xFF)))
every = bytes(b for b in range(256) if b not in (0x05, 0x07))
check("every byte value", echo(s, every) == every)
for size in (1, 17, 64, 255, 1000, 2000):
    data = os.urandom(size).replace(ENQ, b"\x06").replace(b"\x07", b"\x08")
    got = echo(s, data)
    check(f"block of {size}", got == data, "" if got == data else f"({len(got)} back)")

# The bulk goes in 200-byte chunks with 20 % gaps: an echo can't send
# faster than its own baud rate, so a long unbroken burst from a host whose
# clock runs a little fast fills any buffer in the end (seen at 460,800 and
# 921,600 through the PKoB4's VCOM, while gapped chunks pass).
data = os.urandom(20_000).replace(ENQ, b"\x06").replace(b"\x07", b"\x08")
start = time.monotonic()
got = b""
for i in range(0, len(data), 200):
    s.write(data[i:i + 200])
    time.sleep(200 * 10 / baud * 1.2)
    got += s.read(s.in_waiting)
deadline = time.monotonic() + 5
while len(got) < len(data) and time.monotonic() < deadline:
    got += s.read(len(data) - len(got))
elapsed = time.monotonic() - start
check("bulk 20,000 in gapped chunks", got == data, f"in {elapsed:.2f} s ({len(data) / elapsed / 1000:.1f} kB/s)" if got == data else f"({len(got)} back)")
before = counts(s)
check("no errors so far", before.startswith("errors: framing 0 parity 0 overrun 0;"), f"({before})")

# A 500 µs stall (BEL), then 8 bytes at once: 6 bytes' time at 115,200, so
# without the FIFO (2 bytes and the shift register) some overrun; with it,
# none.
fifo = before.endswith("fifo on")
overruns_before = int(before.split()[6].rstrip(";")) if before.startswith("errors:") else 0
data = bytes(range(0x40, 0x48))
s.write(b"\x07" + data)
got = b""
deadline = time.monotonic() + 2
while len(got) < len(data) and time.monotonic() < deadline:
    got += s.read(len(data) - len(got))
after_stall = counts(s)
overruns = (int(after_stall.split()[6].rstrip(";")) if after_stall.startswith("errors:") else 0) - overruns_before
if fifo:
    check("8 bytes after a 500 us stall, FIFO on", got == data and overruns == 0, f"({len(got)} back, {overruns} overruns)")
else:
    print(f"8 bytes after a 500 us stall, FIFO off: {len(got)} back, {overruns} overruns (for comparison)")

wrong = baud // 3
s.baudrate = wrong
time.sleep(0.1)
s.write(bytes([0x00] * 50))
time.sleep(0.2)
s.baudrate = baud
time.sleep(0.1)
after = counts(s)
framing = int(after.split()[2]) if after.startswith("errors:") else 0
check(f"framing errors from {wrong} baud", framing > 0, f"({after})")
check("echo after the errors", echo(s, b"hello") == b"hello")

print(f"uart_echo_test at {baud}: {'all passed' if failures == 0 else f'{failures} failures'}")
sys.exit(1 if failures else 0)
