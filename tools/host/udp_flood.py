# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""udp_flood.py HOST SECONDS [SIZE]: UDP datagrams to HOST's port 9 as fast
as this host sends them, to load a board's receive path. Prints the rate."""
import socket
import sys
import time

host, seconds = sys.argv[1], float(sys.argv[2])
size = int(sys.argv[3]) if len(sys.argv) > 3 else 64
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
data = bytes(size)
sent = 0
end = time.monotonic() + seconds
while time.monotonic() < end:
    for _ in range(100):
        try:
            s.sendto(data, (host, 9))
            sent += 1
        except OSError:
            time.sleep(0.001)
print(f"udp_flood: {sent} datagrams of {size} bytes in {seconds:.0f} s ({sent / seconds:.0f}/s)")
