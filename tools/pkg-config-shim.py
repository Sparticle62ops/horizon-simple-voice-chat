#!/usr/bin/env python3
"""Minimal pkg-config shim over a sysroot of unpacked .deb trees.

Rewrites the .pc's declared prefix paths to the owning sysroot, so a build
without a real pkg-config/ALSA install still resolves include/lib paths.
Save to /tmp/bin/pkg-config with ROOTS pointing at the unpacked sysroot(s).
"""
import sys, os, re, glob
ROOTS = ["/tmp/alsa-root"]
def pc_dirs():
    out = []
    for r in ROOTS:
        out += glob.glob(r + "/usr/lib/*/pkgconfig") + glob.glob(r + "/usr/share/pkgconfig") + glob.glob(r + "/usr/lib/pkgconfig")
    return out
def find_pc(name):
    for d in pc_dirs():
        p = os.path.join(d, name + ".pc")
        if os.path.exists(p):
            root = next((r for r in ROOTS if p.startswith(r)), ROOTS[0])
            return p, root
    return None, None
def subst(line, variables):
    for _ in range(5):
        line = re.sub(r"\$\{([^}]+)\}", lambda m: variables.get(m.group(1), m.group(0)), line)
    return line
def parse_pc(path):
    variables, fields = {}, {}
    for raw in open(path):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        m = re.match(r"([A-Za-z_][A-Za-z0-9_.]*)\s*=\s*(.*)", line)
        if m and not line[:1].isupper() or (m and m.group(1) in ("prefix", "libdir", "includedir", "exec_prefix", "bindir", "datarootdir", "datadir", "sysconfdir")):
            variables[m.group(1)] = subst(m.group(2).strip(), variables)
            continue
        m = re.match(r"([A-Za-z_][A-Za-z0-9_.]*)\s*:\s*(.*)", line)
        if m:
            fields[m.group(1)] = m.group(2).strip()
    for k in list(variables):
        variables[k] = subst(variables[k], variables)
    fields = {k: subst(v, variables) for k, v in fields.items()}
    return fields
def remap_root(s, root=None):
    if root:
        s = s.replace("/usr/", root + "/usr/")
    return s
args = sys.argv[1:]
pkgs = []
mode = []
atleast = None
exists_only = False
i = 0
while i < len(args):
    a = args[i]
    if a in ("--atleast-version", "--max-version", "--exact-version"):
        atleast = args[i+1]; i += 2; continue
    if a in ("--libs", "--cflags", "--modversion"):
        mode.append((a, None)); i += 1; continue
    if a == "--variable":
        mode.append(("variable", args[i+1])); i += 2; continue
    if a.startswith("--variable="):
        mode.append(("variable", a.split("=",1)[1])); i += 1; continue
    if a == "--exists":
        exists_only = True; i += 1; continue
    if a.startswith("-"):
        i += 1; continue
    pkgs.append(a); i += 1
found = {}
owners = {}
for p in pkgs:
    path, pcroot = find_pc(p)
    if not path:
        sys.exit(1)
    found[p] = parse_pc(path)
    owners[p] = pcroot
if exists_only or atleast is not None:
    sys.exit(0)
out = []
for (m, arg) in mode:
    for p in pkgs:
        f = found[p]
        if m == "--modversion":
            out.append(f.get("Version", "0"))
        elif m == "--cflags":
            out.append(remap_root(f.get("Cflags", ""), owners[p]))
        elif m == "--libs":
            out.append(remap_root(f.get("Libs", ""), owners[p]))
        elif m == "variable":
            out.append(f.get(arg, ""))
print(" ".join(out).strip())
