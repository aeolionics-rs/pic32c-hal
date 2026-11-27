# SPDX-FileCopyrightText: 2026 Aeolionics, LLC
#
# SPDX-License-Identifier: MIT OR Apache-2.0
"""Point the workspace at embassy main: embassy-main.py [EMBASSY_REV].

Rewrites every embassy git `rev` in the workspace's manifests to EMBASSY_REV
(embassy main's head by default), and every xarxa git `rev` to the one that
revision's embassy-net names, then lets Cargo re-resolve. The daily CI run
(`.github/workflows/embassy-main.yml`) runs it before the usual checks, so a
change on embassy main that breaks us shows up the next day, not at the next
release.

It fails if a patch ends up unused: embassy main has moved a crate to a version
our requirement doesn't match, so the build would quietly take crates.io's
instead. Run it from the workspace root.
"""
import os
import pathlib
import re
import subprocess
import sys
import tomllib
import urllib.request

EMBASSY = "https://github.com/embassy-rs/embassy"
XARXA = "https://github.com/embassy-rs/xarxa"
REV = re.compile(r'(git = "(%s|%s)", rev = ")([0-9a-f]{40})(")' % (re.escape(EMBASSY), re.escape(XARXA)))


def embassy_main():
    out = subprocess.run(["git", "ls-remote", EMBASSY, "refs/heads/main"], capture_output=True, text=True, check=True).stdout
    return out.split()[0]


def xarxa_rev(embassy_rev):
    """The xarxa revision embassy-net names at `embassy_rev`."""
    url = f"https://raw.githubusercontent.com/embassy-rs/embassy/{embassy_rev}/embassy-net/Cargo.toml"
    with urllib.request.urlopen(url, timeout=30) as response:
        deps = tomllib.loads(response.read().decode())["dependencies"]
    revs = {deps[name]["rev"] for name in ("xarxa", "xarxa-driver") if deps[name]["git"] == XARXA}
    if len(revs) != 1:
        sys.exit(f"embassy-net at {embassy_rev} names xarxa as {deps['xarxa']} and {deps['xarxa-driver']}")
    return revs.pop()


def main():
    embassy = sys.argv[1] if len(sys.argv) > 1 else embassy_main()
    xarxa = xarxa_rev(embassy)
    new = {EMBASSY: embassy, XARXA: xarxa}
    old = {}
    for manifest in sorted(pathlib.Path(".").glob("**/Cargo.toml")):
        if "target" in manifest.parts:
            continue
        text = manifest.read_text()

        def bump(m):
            old.setdefault(m[2], set()).add(m[3])
            return m[1] + new[m[2]] + m[4]

        changed = REV.sub(bump, text)
        if changed != text:
            manifest.write_text(changed)
            print(f"{manifest}: updated")

    lines = []
    for url, rev in new.items():
        was = ", ".join(sorted(r[:9] for r in old.get(url, ()))) or "none"
        lines.append(f"- {url.rsplit('/', 1)[1]}: {was} -> {rev[:9]}")
    report = "\n".join(lines)
    print(report)

    # Re-resolve the lockfile, and catch patches left out of the graph.
    update = subprocess.run(["cargo", "update", "--workspace"], capture_output=True, text=True)
    sys.stderr.write(update.stderr)
    unused = [line for line in update.stderr.splitlines() if "was not used in the crate graph" in line]

    summary = os.environ.get("GITHUB_STEP_SUMMARY")
    if summary:
        with open(summary, "a") as f:
            f.write(f"### Against embassy main\n\n{report}\n")
            if unused:
                f.write("\nUnused patches:\n\n" + "\n".join(f"- `{line.strip()}`" for line in unused) + "\n")
    if update.returncode != 0:
        sys.exit("cargo update failed")
    if unused:
        sys.exit("some patches were not used: embassy main has a version our requirements don't match")


if __name__ == "__main__":
    main()
