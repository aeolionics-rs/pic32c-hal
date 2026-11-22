# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Host side of the `soak` bench program: soak.py [--fs PORT] [--hs PORT]
[--host HOST] [--hours H] [--log FILE].

Keeps each of the board's interfaces busy until stopped (Ctrl-C) or for
HOURS, every byte checked:

- USB full and high speed (--fs, --hs, the CDC-ACM ports): over and over,
  open the port, echo a sample of lengths, stream random data both ways at
  once, close it. Full speed streams 1 MiB a session, high speed 32 MiB
  (with every interface busy, the board shares its CPU out, so each runs
  well below its rate alone).
- Ethernet (--host, the board's address): over and over, connect to the TCP
  echo on port 7 and stream 4 MiB both ways; and, on one kept-alive
  connection, GET /blob (64 KiB of bytes 0-255 repeated) and check it, with
  the board's /status line logged once a minute.

A failure is counted and logged and the stream starts again, so one fault
doesn't end the soak. A summary goes out every minute, to the terminal and
to the log file (default soak-YYYYMMDD-HHMMSS.log here). Serial needs
pyserial (`uvx --with pyserial python soak.py ...`).
"""
import argparse, http.client, os, random, socket, sys, threading, time

parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
parser.add_argument("--fs", help="the full-speed CDC-ACM port")
parser.add_argument("--hs", help="the high-speed CDC-ACM port")
parser.add_argument("--host", help="the board's network address")
parser.add_argument("--hours", type=float, default=0, help="stop after this long (0: until Ctrl-C)")
parser.add_argument("--log", default=time.strftime("soak-%Y%m%d-%H%M%S.log"))
args = parser.parse_args()

stop = threading.Event()
lock = threading.Lock()
log_file = open(args.log, "a", buffering=1)
start = time.monotonic()


def log(text):
    line = f"{time.strftime('%Y-%m-%d %H:%M:%S')} {text}"
    with lock:
        print(line, flush=True)
        log_file.write(line + "\n")


class Counts:
    """One stream's tally."""

    def __init__(self, name):
        self.name = name
        self.sessions = 0
        self.bytes = 0
        self.errors = 0
        self.rate = 0.0
        self.last_error = ""

    def ok(self, nbytes, seconds):
        with lock:
            self.sessions += 1
            self.bytes += nbytes
            self.rate = nbytes / seconds / 1e6 if seconds > 0 else 0.0

    def fail(self, why):
        with lock:
            self.errors += 1
            self.last_error = why
        log(f"{self.name}: ERROR {why}")

    def summary(self):
        return f"{self.name} {self.sessions} ok {self.bytes / 1e6:.0f} MB {self.rate:.2f} MB/s errors {self.errors}"


def first_diff(got, want):
    return next((i for i, (a, b) in enumerate(zip(got, want)) if a != b), min(len(got), len(want)))


def usb_soak(counts, port, session_bytes, max_length):
    import serial
    missing = 0
    while not stop.is_set():
        try:
            s = serial.Serial(port, 115200, timeout=2)
        except (serial.SerialException, OSError) as e:
            # A port that has gone stays gone: logged when it goes, then
            # every 5 minutes.
            if missing % 60 == 0:
                counts.fail(f"open ({missing} tries before): {e}")
            missing += 1
            stop.wait(5)
            continue
        if missing:
            log(f"{counts.name}: port back after {missing} failed opens")
            missing = 0
        try:
            s.reset_input_buffer()
            # A sample of lengths, across the packet and FIFO sizes.
            for n in random.sample(range(1, max_length + 1), 40):
                data = os.urandom(n)
                s.write(data)
                got = b""
                end = time.monotonic() + 3
                while len(got) < n and time.monotonic() < end:
                    got += s.read(n - len(got))
                if got != data:
                    raise RuntimeError(f"length {n}: got {len(got)} bytes, first diff at {first_diff(got, data)}")
            # Both ways at once.
            data = os.urandom(session_bytes)
            received = bytearray()

            def reader():
                end = time.monotonic() + 600
                try:
                    while len(received) < session_bytes and time.monotonic() < end:
                        received.extend(s.read(min(65536, session_bytes - len(received)) or 1))
                except (serial.SerialException, OSError):
                    pass  # the check below reports the shortfall

            t = threading.Thread(target=reader, daemon=True)
            began = time.monotonic()
            t.start()
            for i in range(0, session_bytes, 65536):
                s.write(data[i:i + 65536])
            t.join()
            elapsed = time.monotonic() - began
            if bytes(received) != data:
                raise RuntimeError(f"stream: echoed {len(received)} of {session_bytes}, first diff at {first_diff(received, data)}")
            counts.ok(session_bytes, elapsed)
        except Exception as e:  # noqa: BLE001 - every failure is counted and the soak goes on
            counts.fail(str(e))
            stop.wait(2)
        finally:
            s.close()


def tcp_soak(counts, host, session_bytes):
    while not stop.is_set():
        try:
            sock = socket.create_connection((host, 7), timeout=10)
        except OSError as e:
            counts.fail(f"connect: {e}")
            stop.wait(5)
            continue
        try:
            data = os.urandom(session_bytes)
            received = bytearray()

            def reader():
                try:
                    while len(received) < session_bytes:
                        chunk = sock.recv(65536)
                        if not chunk:
                            break
                        received.extend(chunk)
                except OSError:
                    pass  # the check below reports the shortfall

            t = threading.Thread(target=reader, daemon=True)
            began = time.monotonic()
            t.start()
            # A piece at a time: sendall's timeout covers the whole call, so
            # one call for it all would count a slow stream as a stall.
            for i in range(0, session_bytes, 65536):
                sock.sendall(data[i:i + 65536])
            t.join(60)
            elapsed = time.monotonic() - began
            if bytes(received) != data:
                raise RuntimeError(f"echoed {len(received)} of {session_bytes}, first diff at {first_diff(received, data)}")
            counts.ok(session_bytes, elapsed)
        except Exception as e:  # noqa: BLE001
            counts.fail(str(e))
            stop.wait(2)
        finally:
            sock.close()


BLOB = bytes(range(256)) * 256


def http_soak(counts, host):
    conn = None
    next_status = 0.0
    while not stop.is_set():
        try:
            if conn is None:
                conn = http.client.HTTPConnection(host, 80, timeout=10)
            if time.monotonic() >= next_status:
                next_status = time.monotonic() + 60
                conn.request("GET", "/status")
                response = conn.getresponse()
                log(f"board: {response.read().decode(errors='replace').strip()}")
            began = time.monotonic()
            conn.request("GET", "/blob")
            response = conn.getresponse()
            body = response.read()
            elapsed = time.monotonic() - began
            if response.status != 200 or body != BLOB:
                raise RuntimeError(f"/blob: status {response.status}, {len(body)} bytes, first diff at {first_diff(body, BLOB)}")
            counts.ok(len(body), elapsed)
        except Exception as e:  # noqa: BLE001
            counts.fail(str(e))
            if conn is not None:
                conn.close()
            conn = None
            stop.wait(2)


streams = []
if args.fs:
    streams.append((Counts("fs"), usb_soak, (args.fs, 1 << 20, 300)))
if args.hs:
    streams.append((Counts("hs"), usb_soak, (args.hs, 32 << 20, 1100)))
if args.host:
    streams.append((Counts("tcp"), tcp_soak, (args.host, 4 << 20)))
    streams.append((Counts("http"), http_soak, (args.host,)))
if not streams:
    sys.exit("nothing to soak: give --fs, --hs and/or --host")

log(f"soak: {', '.join(c.name for c, _, _ in streams)}; log {args.log}")
for counts, target, target_args in streams:
    threading.Thread(target=target, args=(counts, *target_args), daemon=True).start()
try:
    while not stop.wait(60):
        hours = (time.monotonic() - start) / 3600
        log(f"after {hours:.2f} h: " + " | ".join(c.summary() for c, _, _ in streams))
        if args.hours and hours >= args.hours:
            stop.set()
except KeyboardInterrupt:
    stop.set()
log("stopped: " + " | ".join(c.summary() for c, _, _ in streams))
