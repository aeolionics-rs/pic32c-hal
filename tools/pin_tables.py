# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""pin_tables.py ATDF [INSTANCE:PIN=IOSET ...]: prints the chip file's pin
tables for the SERCOMs, the CAN controllers and the Ethernet MAC's RMII
signals, from the ATDF's pin multiplexing (signal pad, function and IOSET),
as the invocations of `sercom::impl_pads!`, `can::impl_can_pins!` and
`eth::impl_rmii_pins!`.

A SERCOM or CAN signal the ATDF gives no IOSET needs one on the command line
(`SERCOM1:PD00=3`); the chip file says where it came from. The Ethernet
signals have no I/O sets.
"""
import sys
import xml.etree.ElementTree as ET

root = ET.parse(sys.argv[1]).getroot()
overrides = {}
for arg in sys.argv[2:]:
    key, ioset = arg.split('=')
    overrides[tuple(key.split(':'))] = int(ioset)


def ioset(name, s):
    if s.get('ioset') is not None:
        return int(s.get('ioset'))
    if (name, s.get('pad')) in overrides:
        return overrides[(name, s.get('pad'))]
    sys.exit(f'{name} {s.get("pad")}: no IOSET in the ATDF; give one as {name}:{s.get("pad")}=N')


# The RMII signals, as the ATDF names them (group and index: MII's names,
# so RMII's CRS_DV is RXDV, as in the datasheet's table of the two modes),
# and the driver's pin trait for each, in the order of `eth::Pins`' fields.
RMII = [
    (('REF_CLK', None), 'RefClkPin'),
    (('MDC', None), 'MdcPin'),
    (('MDIO', None), 'MdioPin'),
    (('RXDV', None), 'CrsDvPin'),
    (('RXER', None), 'RxErPin'),
    (('RXD', '0'), 'Rxd0Pin'),
    (('RXD', '1'), 'Rxd1Pin'),
    (('TXEN', None), 'TxEnPin'),
    (('TXD', '0'), 'Txd0Pin'),
    (('TXD', '1'), 'Txd1Pin'),
]

sercoms, cans, eth = {}, {}, None
for instance in root.iter('instance'):
    name = instance.get('name')
    signals = instance.find('signals')
    if signals is None:
        continue
    for s in signals.iter('signal'):
        if name.startswith('SERCOM') and s.get('group') == 'PAD':
            sercoms.setdefault(name, []).append((int(s.get('index')), s.get('pad'), s.get('function'), ioset(name, s)))
        elif name in ('CAN0', 'CAN1') and s.get('group') in ('TX', 'RX'):
            cans.setdefault(name, []).append((s.get('group').lower(), s.get('pad'), s.get('function'), ioset(name, s)))
        elif name == 'ETH':
            eth = eth or {}
            eth.setdefault((s.get('group'), s.get('index')), []).append((s.get('pad'), s.get('function')))

print('crate::sercom::impl_pads! {')
for name in sorted(sercoms, key=lambda n: int(n[6:])):
    rows = sorted(sercoms[name], key=lambda r: (r[3], r[0]))
    print(f'    {name}: ' + ', '.join(f'{pin} pad {pad} {function} ioset {ioset}' for pad, pin, function, ioset in rows) + ';')
print('}')
print()
print('#[cfg(feature = "can-hal")]')
print('crate::can::instance::impl_can_pins! {')
for name in sorted(cans):
    rows = sorted(cans[name], key=lambda r: (r[3], r[0]))
    print(f'    {name}: ' + ', '.join(f'{kind} {pin} {function} ioset {ioset}' for kind, pin, function, ioset in rows) + ';')
print('}')

# Chips without the MAC get no table.
if eth is not None:
    print()
    print('#[cfg(feature = "eth")]')
    print('crate::eth::impl_rmii_pins! {')
    for signal, pin_trait in RMII:
        if signal not in eth:
            sys.exit(f'ETH: no pad for {signal[0]}{signal[1] or ""} in the ATDF')
        print(f'    {pin_trait}: ' + ', '.join(f'{pin} {function}' for pin, function in sorted(eth[signal])) + ';')
    print('}')
