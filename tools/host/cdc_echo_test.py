# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Echo test of a CDC-ACM port: cdc_echo_test.py PORT."""
import os, sys, time, serial
s = serial.Serial(sys.argv[1], 115200, timeout=2)
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
        print(f"FAIL len {len(data)}: got {len(got)} bytes, first diff at {next((i for i,(a,b) in enumerate(zip(got,data)) if a!=b), min(len(got),len(data)))}")
for n in range(1, 65):
    check(os.urandom(n))
start = time.monotonic()
total = 0
for n in (100, 128, 500, 1000, 4096, 10000):
    check(os.urandom(n)); total += n
elapsed = time.monotonic() - start
check(b"hello from the host\n")
print(f"lengths 1-64 and 100..10000: {failures} failures; bulk {total} bytes each way in {elapsed:.2f} s ({2*total/elapsed/1000:.0f} kB/s both ways)")
