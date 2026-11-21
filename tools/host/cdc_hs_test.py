# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""High-speed CDC-ACM echo test: cdc_hs_test.py PORT [MEGABYTES].

Echoes every length from 1 to 1100 bytes (across the 512-byte packet and
1024-byte FIFO boundaries), then streams MEGABYTES (default 16) of random
data in 64 KiB writes from one thread while another reads the echo back,
checks it, and reports the rate each way.
"""
import os, sys, threading, time, serial

port = sys.argv[1]
megabytes = float(sys.argv[2]) if len(sys.argv) > 2 else 16
s = serial.Serial(port, 115200, timeout=2)
s.reset_input_buffer()

failures = 0
def check(data):
    global failures
    s.write(data)
    got = b""
    end = time.monotonic() + 3
    while len(got) < len(data) and time.monotonic() < end:
        got += s.read(len(data) - len(got))
    if got != data:
        failures += 1
        diff = next((i for i, (a, b) in enumerate(zip(got, data)) if a != b), min(len(got), len(data)))
        print(f"FAIL len {len(data)}: got {len(got)} bytes, first diff at {diff}")

for n in range(1, 1101):
    check(os.urandom(n))
print(f"lengths 1-1100: {failures} failures")

total = int(megabytes * 1024 * 1024)
data = os.urandom(total)
received = bytearray()
def reader():
    end = time.monotonic() + 60
    while len(received) < total and time.monotonic() < end:
        chunk = s.read(min(65536, total - len(received)) or 1)
        received.extend(chunk)
t = threading.Thread(target=reader)
start = time.monotonic()
t.start()
for i in range(0, total, 65536):
    s.write(data[i:i + 65536])
written = time.monotonic() - start
t.join()
elapsed = time.monotonic() - start
ok = bytes(received) == data
print(f"stream {total} bytes: echoed {len(received)}, {'identical' if ok else 'MISMATCH'}; "
      f"{total / elapsed / 1e6:.2f} MB/s each way ({2 * total / elapsed / 1e6:.2f} MB/s both ways) in {elapsed:.2f} s")
