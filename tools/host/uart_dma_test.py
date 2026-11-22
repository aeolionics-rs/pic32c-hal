# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Drive a board's uart_dma: uart_dma_test.py PORT [BLOCKS] [BAUD].

uart_dma echoes 256-byte blocks by DMA, reading nothing while it writes one,
so this sends a block at a time, each after the last one's echo, BLOCKS of
them (100 by default) at BAUD (115,200 by default, as uart_dma is built
unless UART_BAUD says otherwise), and checks each comes back the same.
"""
import random
import sys
import time

import serial

BLOCK = 256

port = sys.argv[1]
blocks = int(sys.argv[2]) if len(sys.argv) > 2 else 100
baud = int(sys.argv[3]) if len(sys.argv) > 3 else 115200
s = serial.Serial(port, baud, timeout=2)
time.sleep(0.2)
s.reset_input_buffer()
rng = random.Random(3)
start = time.monotonic()
for n in range(blocks):
    data = bytes(rng.randrange(256) for _ in range(BLOCK))
    s.write(data)
    got = s.read(BLOCK)
    if got != data:
        first = next((i for i, (a, b) in enumerate(zip(got, data)) if a != b), None)
        print(f"block {n}: FAILED, got {len(got)} of {BLOCK} bytes, first difference at {first}")
        sys.exit(1)
took = time.monotonic() - start
print(f"{blocks} blocks of {BLOCK} bytes echoed identical in {took:.2f} s ({blocks * BLOCK * 20 / took:.0f} bit/s both ways)")
