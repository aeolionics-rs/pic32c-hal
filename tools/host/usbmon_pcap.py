# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""usbmon_pcap.py PCAP [DEVNUM]: summarises a usbmon (DLT 220) capture:
devices seen, then per bulk/interrupt endpoint of DEVNUM the URB counts by
status and the last events."""
import struct, sys, collections
data = open(sys.argv[1], 'rb').read()
magic, = struct.unpack_from('<I', data, 0)
assert magic in (0xa1b2c3d4, 0xa1b23c4d), hex(magic)
linktype, = struct.unpack_from('<I', data, 20)
pos = 24
events = []
while pos + 16 <= len(data):
    ts, tus, incl, orig = struct.unpack_from('<IIII', data, pos); pos += 16
    rec = data[pos:pos + incl]; pos += incl
    (uid, typ, xfer, ep, dev, bus, fsetup, fdata, s, us, status, length, lencap) = struct.unpack_from('<QcBBBHccqiiII', rec, 0)
    payload = rec[64 if linktype == 220 else 48:]
    events.append(dict(id=uid, t=s + us / 1e6, typ=typ.decode(), xfer=xfer, ep=ep, dev=dev, status=status, len=length, cap=lencap, data=payload))
print('linktype', linktype, 'events', len(events))
devs = collections.Counter((e['dev'], e['xfer'], e['ep']) for e in events)
for (d, x, ep), c in sorted(devs.items()):
    print(f'dev {d} xfer {x} ep {ep:#04x}: {c}')
if len(sys.argv) > 2:
    dev = int(sys.argv[2])
    names = {0: 'iso', 1: 'int', 2: 'ctrl', 3: 'bulk'}
    for ep in sorted({e['ep'] for e in events if e['dev'] == dev}):
        evs = [e for e in events if e['dev'] == dev and e['ep'] == ep]
        st = collections.Counter((e['typ'], e['status']) for e in evs)
        print(f'\n== dev {dev} ep {ep:#04x} {names[evs[0]["xfer"]]}: {dict(st)}')
        t0 = events[0]['t']
        for e in evs[-12:]:
            print(f"  {e['t'] - t0:9.4f} {e['typ']} id {e['id'] & 0xffff:04x} status {e['status']:5d} len {e['len']:5d} cap {e['cap']}")
if len(sys.argv) > 3:
    # Around the first error: every event of DEVNUM from 30 before it to 10 after.
    dev = int(sys.argv[2])
    evs = [e for e in events if e['dev'] == dev]
    first = next(i for i, e in enumerate(evs) if e['typ'] == 'C' and e['status'] not in (0, -115))
    print('\nfirst error at', round(evs[first]['t'], 6), 'abs; around it:')
    for e in evs[max(0, first - 30):first + 10]:
        print(f"  {e['t']:.6f} {e['typ']} ep {e['ep']:#04x} id {e['id'] & 0xffff:04x} status {e['status']:5d} len {e['len']:5d}")
