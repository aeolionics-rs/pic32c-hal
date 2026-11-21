# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Drive pic32ck-curiosity-ultra's eth_http: eth_http_test.py HOST.

TCP echo on port 7: blocks of 1 to 100,000 bytes, compared, with the rate.
HTTP on port 80: /, /status, a 404, /blob (64 KiB of bytes 0-255 repeated,
checked, with the rate), 20 requests on one kept-alive connection, and two
connections at once. Prints one line per check and a summary.
"""
import http.client
import os
import socket
import sys
import threading
import time

host = sys.argv[1]
failures = 0


def check(name, good, detail=""):
    global failures
    print(f"{name}: {'ok' if good else 'FAILED'} {detail}".rstrip(), flush=True)
    failures += 0 if good else 1


# TCP echo.
with socket.create_connection((host, 7), timeout=10) as s:
    s.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)
    for size in (1, 100, 1460, 10_000, 100_000):
        data = os.urandom(size)
        start = time.monotonic()
        sent = 0
        got = b""
        s.settimeout(10)
        # Send and read in turn, so neither side's window fills.
        while len(got) < size:
            if sent < size:
                n = s.send(data[sent:sent + 4096])
                sent += n
            try:
                s.settimeout(0.05 if sent < size else 10)
                chunk = s.recv(65536)
                if not chunk:
                    break
                got += chunk
            except socket.timeout:
                pass
        elapsed = time.monotonic() - start
        rate = f" ({size * 2 / elapsed / 1000:.0f} kB/s both ways)" if size >= 10_000 else ""
        check(f"echo {size} bytes", got == data, f"in {elapsed * 1000:.0f} ms{rate}" if got == data else f"({len(got)} back)")

# HTTP.
c = http.client.HTTPConnection(host, 80, timeout=10)
c.request("GET", "/")
r = c.getresponse()
body = r.read()
check("GET /", r.status == 200 and b"Hello from the PIC32CK" in body, f"({r.status}, {len(body)} bytes, {r.getheader('Content-Type')})")
c.request("GET", "/status")
r = c.getresponse()
body = r.read().decode(errors="replace")
check("GET /status", r.status == 200 and "link up" in body, "(" + body.strip().replace("\n", "; ") + ")")
c.request("GET", "/nothing-here")
r = c.getresponse()
r.read()
check("GET /nothing-here", r.status == 404, f"({r.status})")
expected = bytes(range(256)) * 256
start = time.monotonic()
c.request("GET", "/blob")
r = c.getresponse()
body = r.read()
elapsed = time.monotonic() - start
check("GET /blob", r.status == 200 and body == expected, f"(64 KiB in {elapsed * 1000:.0f} ms, {len(body) / elapsed / 1000:.0f} kB/s)")
start = time.monotonic()
statuses = []
for _ in range(20):
    c.request("GET", "/status")
    r = c.getresponse()
    r.read()
    statuses.append(r.status)
elapsed = time.monotonic() - start
check("20 requests on one connection", statuses == [200] * 20, f"({elapsed * 1000 / 20:.1f} ms each)")
c.close()

results = []


def blob(index):
    try:
        cc = http.client.HTTPConnection(host, 80, timeout=10)
        cc.request("GET", "/blob")
        rr = cc.getresponse()
        results.append((index, rr.status, rr.read() == expected))
        cc.close()
    except Exception as e:
        results.append((index, str(e), False))


threads = [threading.Thread(target=blob, args=(i,)) for i in range(2)]
start = time.monotonic()
for t in threads:
    t.start()
for t in threads:
    t.join()
elapsed = time.monotonic() - start
check("two /blob at once", sorted(results) == [(0, 200, True), (1, 200, True)], f"(both in {elapsed * 1000:.0f} ms)" if all(ok for *_, ok in results) else f"({results})")

print(f"eth_http_test {host}: {'all passed' if failures == 0 else f'{failures} failures'}")
sys.exit(1 if failures else 0)
