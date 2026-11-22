# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""The far end of a board's can_ping on a Linux SocketCAN host:
can_echo.py INTERFACE SECONDS.

Sends back every frame it receives with its identifier's top bit set (bit 10
of a standard ID, bit 28 of an extended one), unless that bit is already set:
those are echoes, its own or another's. Classic and FD frames, data and
remote, as the CK board's can_echo does. Stops after SECONDS and prints its
counts. Python 3 only, no packages: a raw CAN socket.
"""
import socket
import struct
import sys
import time

CAN_EFF_FLAG = 0x8000_0000
CAN_RTR_FLAG = 0x4000_0000
CAN_ERR_FLAG = 0x2000_0000
CAN_MTU = 16
CANFD_MTU = 72

interface, seconds = sys.argv[1], float(sys.argv[2])
s = socket.socket(socket.PF_CAN, socket.SOCK_RAW, socket.CAN_RAW)
s.setsockopt(socket.SOL_CAN_RAW, socket.CAN_RAW_FD_FRAMES, 1)
s.bind((interface,))
s.settimeout(0.2)
echoed = skipped = 0
deadline = time.monotonic() + seconds
print(f"can_echo: {interface} for {seconds:g} s", flush=True)
while time.monotonic() < deadline:
    try:
        frame = s.recv(CANFD_MTU)
    except socket.timeout:
        continue
    can_id = struct.unpack_from("=I", frame)[0]
    if can_id & CAN_ERR_FLAG:
        continue
    top = 1 << 28 if can_id & CAN_EFF_FLAG else 1 << 10
    if can_id & top:
        skipped += 1
        continue
    s.send(struct.pack("=I", can_id | top) + frame[4:])
    echoed += 1
print(f"can_echo: echoed {echoed}, left {skipped} echoes alone", flush=True)
