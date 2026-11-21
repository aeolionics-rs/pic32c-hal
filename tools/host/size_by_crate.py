# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""size_by_crate.py SYMS: sum `size name` lines (llvm-nm -C --print-size,
text and rodata) by the first crate named in each demangled symbol."""
import re
import sys
from collections import Counter

totals = Counter()
for line in open(sys.argv[1]):
    size, _, name = line.strip().partition(" ")
    size = int(size)
    # The crate: the first path segment in the name, skipping `<`, `&`, `dyn`.
    match = re.search(r"([a-z_][a-z0-9_]*)::", name)
    totals[match.group(1) if match else "(other)"] += size
total = sum(totals.values())
for crate, size in totals.most_common(40):
    print(f"{size:8d} {size * 100 / total:5.1f}%  {crate}")
print(f"{total:8d} total")
