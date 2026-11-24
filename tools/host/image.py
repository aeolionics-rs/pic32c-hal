# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""An image for pic32c-boot's bootloader from a program's ELF:
image.py ELF VERSION OUT.bin [OBJCOPY].

Takes the program flash part of the ELF (everything but `.bfm`, the two words
the board crate's `boot!` puts in boot flash, where the bootloader is), pads
it to a 32-byte flash word, puts its length in the vector table's word 8 (the
first reserved entry on Armv8-M; word 7 is SecureFault), and appends the
64-byte header pic32c-boot's `image` module checks: "P32I", the version and the length (u32, little-endian), 20 zero
bytes, then the image's SHA-256. The image's ID is the hash's first 16 bytes,
printed. Load OUT.bin at the start of a panel: 0x0C000000 for panel 1,
0x0C100000 for panel 2 while the panels aren't swapped.

OBJCOPY defaults to arm-none-eabi-objcopy on the PATH.
"""
import hashlib
import os
import struct
import subprocess
import sys
import tempfile

ALIGN = 32
MAGIC = b"P32I"


def main():
    if len(sys.argv) not in (4, 5):
        sys.exit(__doc__)
    elf, version, out = sys.argv[1], int(sys.argv[2], 0), sys.argv[3]
    objcopy = sys.argv[4] if len(sys.argv) == 5 else "arm-none-eabi-objcopy"
    with tempfile.TemporaryDirectory() as tmp:
        raw = os.path.join(tmp, "raw.bin")
        subprocess.run([objcopy, "-O", "binary", "-R", ".bfm", elf, raw], check=True)
        with open(raw, "rb") as f:
            image = bytearray(f.read())
    if len(image) < 64 or image[32:36] != b"\0\0\0\0":
        sys.exit("no vector table at the start, or its word 8 isn't free")
    image += b"\0" * (-len(image) % ALIGN)
    image[32:36] = struct.pack("<I", len(image))
    digest = hashlib.sha256(image).digest()
    header = MAGIC + struct.pack("<II", version, len(image)) + b"\0" * 20 + digest
    with open(out, "wb") as f:
        f.write(image + header)
    print(f"{out}: {len(image)} bytes, version {version}, id {digest[:16].hex()}")


if __name__ == "__main__":
    main()
