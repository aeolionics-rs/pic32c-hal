# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""TCP rates against usbhs_ncm (or any host with echo/discard/chargen):
tcp_bench.py HOST [MEGABYTES]. HOST may be a scoped IPv6 address
(fe80::...%en10). Echo on 7 (streamed and checked), discard on 9 (host to
device), chargen on 19 (device to host, its 0..255 pattern checked).
"""
import os, socket, sys, threading, time

host = sys.argv[1]
megabytes = float(sys.argv[2]) if len(sys.argv) > 2 else 16
total = int(megabytes * 1024 * 1024)

def connect(port):
    info = socket.getaddrinfo(host, port, type=socket.SOCK_STREAM)[0]
    s = socket.socket(info[0], socket.SOCK_STREAM)
    s.settimeout(10)
    s.connect(info[4])
    return s

# Echo: one thread writes, this one reads and compares.
data = os.urandom(total)
s = connect(7)
got = bytearray()
def writer():
    for i in range(0, total, 65536):
        s.sendall(data[i:i + 65536])
start = time.monotonic()
t = threading.Thread(target=writer)
t.start()
while len(got) < total:
    chunk = s.recv(65536)
    if not chunk:
        break
    got.extend(chunk)
elapsed = time.monotonic() - start
t.join()
s.close()
print(f"echo {total} bytes: {'identical' if bytes(got) == data else 'MISMATCH'}, {total / elapsed / 1e6:.2f} MB/s each way")

# Discard: host to device.
s = connect(9)
start = time.monotonic()
for i in range(0, total, 65536):
    s.sendall(data[i:i + 65536])
s.shutdown(socket.SHUT_WR)
s.recv(1)  # the device closes once it has read everything
elapsed = time.monotonic() - start
s.close()
print(f"discard {total} bytes: {total / elapsed / 1e6:.2f} MB/s host to device")

# Chargen: device to host.
s = connect(19)
pattern = bytes(range(256)) * 16
got = 0
ok = True
start = time.monotonic()
while got < total:
    chunk = s.recv(65536)
    if not chunk:
        break
    offset = got % 4096
    expect = (pattern * ((len(chunk) + offset) // 4096 + 2))[offset:offset + len(chunk)]
    ok = ok and chunk == expect
    got += len(chunk)
elapsed = time.monotonic() - start
s.close()
print(f"chargen {got} bytes: {'pattern ok' if ok else 'PATTERN WRONG'}, {got / elapsed / 1e6:.2f} MB/s device to host")
