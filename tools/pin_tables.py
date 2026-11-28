# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""pin_tables.py ATDF [INSTANCE:PIN=IOSET ...]: prints the chip file's pin
tables for the SERCOMs, the CAN controllers and the Ethernet MAC's RMII
signals, from the ATDF's pin multiplexing (signal pad, function and IOSET),
as the invocations of `sercom::impl_pads!`, `can::impl_can_pins!` and
`eth::impl_rmii_pins!`.

pin_tables.py --pins-file ATDF [ATDF ...] [INSTANCE:PIN=IOSET ...]: prints a
whole pins file for one package, as `pic32c-hal/src/chip/pic32ck_pins_*.rs`
are: the package's GPIO pins (for `peripherals!` and `gpio::impl_pin!`),
those tables cut to the pins the package has, and the EIC's lines and NMI
pin. Every ATDF given must come out the same (the same package of other
devices on the die), or it stops. An ATDF from a pack with a
`clean-pack.json` (one corrected against the datasheet) has the vendor's
pack and the corrections that reach the device named in the file's heading.

A SERCOM or CAN signal the ATDF gives no IOSET needs one on the command line
(`SERCOM1:PD00=3`); the chip file says where it came from. The Ethernet
signals have no I/O sets.
"""
import json
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

args = sys.argv[1:]
pins_file = '--pins-file' in args
paths = [arg for arg in args if arg.endswith('.atdf')]
overrides = {}
for arg in args:
    if '=' in arg:
        key, ioset = arg.split('=')
        overrides[tuple(key.split(':'))] = int(ioset)
if not paths or (len(paths) > 1 and not pins_file):
    sys.exit(__doc__)


def iosets(name, s):
    """The signal's I/O sets: the ATDF lists more than one for a pad in both."""
    if s.get('ioset') is not None:
        return [int(ioset) for ioset in s.get('ioset').split()]
    if (name, s.get('pad')) in overrides:
        return [overrides[(name, s.get('pad'))]]
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
PORT_PAD = re.compile(r'P([A-Z])(\d\d)')


def tables(root, pads):
    """The pin tables as lines, cut to `pads` (every pad if None), and the
    EIC's lines and NMI pins."""
    sercoms, cans, eth, lines, nmi, analog = {}, {}, None, {}, set(), {}
    for instance in root.iter('instance'):
        name = instance.get('name')
        signals = instance.find('signals')
        if signals is None:
            continue
        for s in signals.iter('signal'):
            if pads is not None and s.get('pad') not in pads:
                continue
            if name.startswith('SERCOM') and s.get('group') == 'PAD':
                sercoms.setdefault(name, []).extend((int(s.get('index')), s.get('pad'), s.get('function'), ioset) for ioset in iosets(name, s))
            elif name in ('CAN0', 'CAN1') and s.get('group') in ('TX', 'RX'):
                cans.setdefault(name, []).extend((s.get('group').lower(), s.get('pad'), s.get('function'), ioset) for ioset in iosets(name, s))
            elif name == 'ETH':
                eth = eth or {}
                eth.setdefault((s.get('group'), s.get('index')), []).append((s.get('pad'), s.get('function')))
            elif name == 'EIC' and s.get('group') == 'EXTINT':
                lines[s.get('pad')] = int(s.get('index'))
            elif name == 'EIC' and s.get('group') == 'NMI':
                nmi.add(s.get('pad'))
            elif name == 'ADC' and s.get('group') == 'VINP':
                analog[int(s.get('index'))] = s.get('pad')

    out = ['crate::sercom::impl_pads! {']
    for name in sorted(sercoms, key=lambda n: int(n[6:])):
        rows = sorted(sercoms[name], key=lambda r: (r[3], r[0]))
        out.append(f'    {name}: ' + ', '.join(f'{pin} pad {pad} {function} ioset {ioset}' for pad, pin, function, ioset in rows) + ';')
    out += ['}', '', '#[cfg(feature = "can-hal")]', 'crate::can::instance::impl_can_pins! {']
    for name in sorted(cans):
        rows = sorted(cans[name], key=lambda r: (r[3], r[0]))
        out.append(f'    {name}: ' + ', '.join(f'{kind} {pin} {function} ioset {ioset}' for kind, pin, function, ioset in rows) + ';')
    out.append('}')
    # Chips without the MAC get no table.
    if eth is not None:
        out += ['', '#[cfg(feature = "eth")]', 'crate::eth::impl_rmii_pins! {']
        for signal, pin_trait in RMII:
            if signal not in eth:
                sys.exit(f'ETH: no pad for {signal[0]}{signal[1] or ""} in the ATDF')
            out.append(f'    {pin_trait}: ' + ', '.join(f'{pin} {function}' for pin, function in sorted(eth[signal])) + ';')
        out.append('}')
    return out, lines, nmi, analog


def package(path):
    """The pins file's lines after its heading, and the package's size."""
    root = ET.parse(path).getroot()
    # A device's pinouts (a TQFP and a BGA, say) must carry the same pads.
    # How many pins a pad is on may differ: a supply's, or an exposed pad.
    pinouts = [{pin.get('pad') for pin in pinout.iter('pin')} for pinout in root.iter('pinout')]
    if not pinouts or any(pinout != pinouts[0] for pinout in pinouts):
        sys.exit(f'{path}: its pinouts do not carry the same pads; make a file from one ATDF per package')
    pads = pinouts[0]
    # The package's size is in its name (TQFP64): a pinout may list an
    # exposed pad as one pin more.
    sizes = [re.search(r'(\d+)$', variant.get('package', '')) for variant in root.iter('variant')]
    leads = int(sizes[0].group(1)) if sizes and sizes[0] else max(len(list(pinout.iter('pin'))) for pinout in root.iter('pinout'))
    gpio = sorted(pad for pad in pads if PORT_PAD.fullmatch(pad))
    pin_tables, lines, nmi, analog = tables(root, pads)
    out = [
        '/// Calls `$callback!` with `$head`, the peripheral singletons, and then the',
        "/// package's GPIO pins: `peripherals!` takes them all at once.",
        'macro_rules! with_pins {',
        '    ($callback:path; $($head:tt)*) => {',
        '        $callback!(',
        '            $($head)*',
        *(f'            {pin},' for pin in gpio),
        '        );',
        '    };',
        '}',
        '',
        'crate::gpio::impl_pin! {',
        *(f'    {pin} = {ord(pin[1]) - ord("A")}, {int(pin[2:])};' for pin in gpio),
        '}',
        '',
        *pin_tables,
        '',
        "// The ADC's inputs (VINP, function B) that have a pin.",
        'crate::adc::impl_adc_pin!(' + ', '.join(f'{analog[n]} = {n}' for n in sorted(analog)) + ',);',
        '',
        "// The EIC's lines (function A), and the pins with the NMI on them.",
        '#[cfg(feature = "exti")]',
        'crate::exti::impl_exti_pins!(',
        *(f'    {pin} = EXTINT{lines[pin]},' for pin in gpio if pin in lines),
        ');',
        '',
        '#[cfg(feature = "exti")]',
        f'crate::exti::impl_nmi_pins!({", ".join(sorted(nmi))});',
    ]
    return out, leads, len(gpio)


def wrap(text, width=76):
    """`text` as lines no longer than `width`."""
    lines, line = [], ''
    for word in text.split():
        if line and len(line) + 1 + len(word) > width:
            lines.append(line)
            line = word
        else:
            line = f'{line} {word}'.strip()
    return lines + [line]


def source(path):
    """Where an ATDF's pin data comes from, for the heading: its pack, and
    if the pack was corrected, the corrections that reach this device."""
    device = Path(path).stem
    for folder in Path(path).resolve().parents:
        manifest = folder / 'clean-pack.json'
        if manifest.is_file():
            clean = json.loads(manifest.read_text())
            patches = sorted(patch for patch, devices in clean.get('patches', {}).items() if device in devices)
            return f'{clean["source"]}, corrected against the datasheet', patches
    return 'the vendor\'s pack', []


if pins_file:
    made = [package(path) for path in paths]
    if any(lines != made[0][0] for lines, _, _ in made):
        sys.exit('the ATDFs do not come out the same: ' + ', '.join(Path(path).stem for path in paths))
    lines, pins, gpio = made[0]
    pack, patches = source(paths[0])
    names = ', '.join(Path(path).stem for path in paths)
    print('// SPDX-FileCopyrightText: 2026 Aeolionics, LLC')
    print('//')
    print('// SPDX-License-Identifier: MIT OR Apache-2.0')
    print()
    print('// Generated by `tools/pin_tables.py --pins-file`: change the tool or its')
    print('// input, not this file.')
    print()
    print(f'//! The {pins}-pin package: its {gpio} GPIO pins, the pads of the SERCOMs, the')
    print("//! CAN controllers and the Ethernet MAC that it has pins for, each with its")
    print("//! function and I/O set, the ADC's inputs and the EIC's lines.")
    print('//!')
    for line in wrap(f'From the pin multiplexing of {names} ({pack}), which give the same.'):
        print(f'//! {line}')
    if patches:
        print('//!')
        for line in wrap('The corrections that reach these devices: ' + ', '.join(patches) + '.'):
            print(f'//! {line}')
    print()
    print('\n'.join(lines))
    sys.exit()

lines, *_ = tables(ET.parse(paths[0]).getroot(), None)
print('\n'.join(lines))
