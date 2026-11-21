# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Awkward clients against eth_https: eth_https_abuse.py HOST CERT.

Each case runs, then a normal verified GET / checks the server still
answers. Stops at the first case after which it doesn't.
"""
import socket
import ssl
import sys
import threading
import time

host, cert = sys.argv[1], sys.argv[2]
good = ssl.create_default_context(cafile=cert)
unverified = ssl.create_default_context()  # rejects the self-signed cert, like a browser


def alive():
    try:
        with socket.create_connection((host, 443), timeout=10) as raw:
            with good.wrap_socket(raw, server_hostname=host) as tls:
                tls.sendall(b"GET / HTTP/1.1\r\nHost: x\r\nConnection: close\r\n\r\n")
                response = b""
                while chunk := tls.recv(4096):
                    response += chunk
                return response.startswith(b"HTTP/1.1 200")
    except Exception as e:
        print(f"    alive check: {e!r}")
        return False


def close_at_once():
    socket.create_connection((host, 443), timeout=5).close()


def garbage():
    with socket.create_connection((host, 443), timeout=5) as s:
        s.sendall(b"GET / HTTP/1.1\r\n\r\n" * 10)
        try:
            s.recv(100)
        except Exception:
            pass


def tls12_only():
    ctx = ssl.create_default_context(cafile=cert)
    ctx.maximum_version = ssl.TLSVersion.TLSv1_2
    try:
        with socket.create_connection((host, 443), timeout=5) as raw:
            ctx.wrap_socket(raw, server_hostname=host)
    except Exception:
        pass


def half_hello():
    # The start of a real ClientHello, then silence past the server's timeout.
    with socket.create_connection((host, 443), timeout=15) as s:
        s.sendall(bytes.fromhex("160301020001"))
        time.sleep(12)


def browser_rejects_cert():
    try:
        with socket.create_connection((host, 443), timeout=5) as raw:
            unverified.wrap_socket(raw, server_hostname=host)
    except ssl.SSLError:
        pass


def drop_mid_download():
    raw = socket.create_connection((host, 443), timeout=5)
    tls = good.wrap_socket(raw, server_hostname=host)
    tls.sendall(b"GET /blob HTTP/1.1\r\nHost: x\r\n\r\n")
    tls.recv(2000)
    raw.setsockopt(socket.SOL_SOCKET, socket.SO_LINGER, b"\x01\x00\x00\x00\x00\x00\x00\x00")
    raw.close()  # RST


def idle_after_handshake():
    with socket.create_connection((host, 443), timeout=15) as raw:
        with good.wrap_socket(raw, server_hostname=host):
            time.sleep(8)


def three_at_once():
    threads = [threading.Thread(target=idle_after_handshake) for _ in range(3)]
    for t in threads:
        t.start()
    for t in threads:
        t.join()


def many_quick(n=30):
    for _ in range(n):
        alive()


cases = [close_at_once, garbage, tls12_only, half_hello, browser_rejects_cert,
         drop_mid_download, idle_after_handshake, three_at_once, many_quick]
for case in cases:
    start = time.monotonic()
    try:
        case()
    except Exception as e:
        print(f"{case.__name__}: raised {e!r}")
    ok = alive()
    print(f"{case.__name__}: {'server ok' if ok else 'SERVER DOWN'} ({time.monotonic() - start:.1f} s)", flush=True)
    if not ok:
        time.sleep(3)
        print(f"  after 3 s: {'back' if alive() else 'still down'}")
        sys.exit(1)
print("eth_https_abuse: server survived every case")
