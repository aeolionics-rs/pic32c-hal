# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Drive pic32ck-curiosity-ultra's eth_https: eth_https_test.py HOST CERT.

Verifies the server against CERT (the bench's self-signed certificate,
whose SAN holds the boards' mDNS names, so HOST is one of them), then: the negotiated version, cipher
and group; 10 fresh handshakes timed; /, /status, a 404; /blob (64 KiB of
bytes 0-255 repeated, checked, with the rate); 20 requests on one kept-alive
connection; two /blob at once. One line per check and a summary.
"""
import http.client
import socket
import ssl
import statistics
import sys
import threading
import time

host, cert = sys.argv[1], sys.argv[2]
context = ssl.create_default_context(cafile=cert)
failures = 0


def check(name, good, detail=""):
    global failures
    print(f"{name}: {'ok' if good else 'FAILED'} {detail}".rstrip(), flush=True)
    failures += 0 if good else 1


def connection():
    return http.client.HTTPSConnection(host, 443, timeout=20, context=context)


# The handshake: what was negotiated, and how long fresh ones take.
times = []
for i in range(10):
    start = time.monotonic()
    with socket.create_connection((host, 443), timeout=20) as raw:
        with context.wrap_socket(raw, server_hostname=host) as tls:
            times.append(time.monotonic() - start)
            if i == 0:
                group = tls.group() if hasattr(tls, "group") else "?"
                detail = f"({tls.version()}, {tls.cipher()[0]}, group {group})"
check("handshake", len(times) == 10, detail)
print(f"10 handshakes: median {statistics.median(times) * 1000:.0f} ms, min {min(times) * 1000:.0f}, max {max(times) * 1000:.0f}")

c = connection()
c.request("GET", "/")
r = c.getresponse()
body = r.read()
check("GET /", r.status == 200 and b"Hello over TLS" in body, f"({r.status}, {len(body)} bytes, {r.getheader('Content-Type')})")
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
        cc = connection()
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

print(f"eth_https_test {host}: {'all passed' if failures == 0 else f'{failures} failures'}")
sys.exit(1 if failures else 0)
