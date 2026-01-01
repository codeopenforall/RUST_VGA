#!/usr/bin/env python3
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path
from dataclasses import dataclass

ROOT_DEFAULT = "/home/ikqnm/PycharmProjects/rudra_sandbox/rudra_big/rudra_big/src/dataset/o3_mini/data_generation/CWE-119"
TOOLCHAIN_DEFAULT = "nightly"  # you can set nightly-2021-10-21 if needed
EDITION = "2021"
TIMEOUT_SECS = 600

ASAN_RE = re.compile(r"(ERROR:\s*AddressSanitizer|AddressSanitizer:)", re.IGNORECASE)

@dataclass
class RunResult:
    compiled: bool
    ran: bool
    exit_code: int
    asan: bool
    output: str
    phase: str  # compile/run

def _check(cmd: str):
    if shutil.which(cmd) is None:
        raise SystemExit(f"ERROR: '{cmd}' not found in PATH")

def compile_and_run_single_rs(rs_path: Path, toolchain: str) -> RunResult:
    """
    Compiles rs_path as a standalone test crate and runs it with ASan enabled.
    The file must be self-contained (tests inside the file).
    """
    with tempfile.TemporaryDirectory(prefix="asan_one_") as td:
        td = Path(td)
        bin_path = td / (rs_path.stem + ("_tests.exe" if os.name == "nt" else "_tests"))

        # Compile with ASan instrumentation (this is the critical part!)
        cmd = [
            "rustup", "run", toolchain, "rustc",
            "--edition", EDITION,
            "--test", str(rs_path),
            "-C", "debuginfo=1",
            "-O",
            "-Z", "sanitizer=address",
            "-o", str(bin_path),
        ]
        try:
            comp = subprocess.run(
                cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
            )
        except subprocess.TimeoutExpired:
            return RunResult(False, False, 124, False, "compile timeout", "compile")

        if comp.returncode != 0:
            out = comp.stdout or ""
            return RunResult(False, False, comp.returncode, bool(ASAN_RE.search(out)), out, "compile")

        # Run with ASan env (helps make output quieter and stop on first error)
        env = os.environ.copy()
        env["RUST_BACKTRACE"] = "0"
        env["ASAN_OPTIONS"] = env.get("ASAN_OPTIONS", "")
        # You can tweak these options if you want:
        env["ASAN_OPTIONS"] = "verbosity=0:halt_on_error=1:" + env["ASAN_OPTIONS"]

        try:
            run = subprocess.run(
                [str(bin_path), "--color", "never"],
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
                env=env,
            )
        except subprocess.TimeoutExpired:
            return RunResult(True, False, 124, False, "run timeout", "run")

        out = run.stdout or ""
        return RunResult(True, True, run.returncode, bool(ASAN_RE.search(out)), out, "run")

def classify_pair(fixed_res: RunResult, vuln_res: RunResult) -> str:
    # Priority: compile errors first
    if not fixed_res.compiled or not vuln_res.compiled:
        return "COMPILE_ERROR"

    # If ASan triggers only for vulnerable => perfect
    if vuln_res.asan and not fixed_res.asan:
        return "VULNERABLE_CONFIRMED"

    # If both trigger ASan => generator bug or both unsafe
    if vuln_res.asan and fixed_res.asan:
        return "BOTH_TRIGGER_ASAN"

    # If only fixed triggers ASan => wrong labeling
    if fixed_res.asan and not vuln_res.asan:
        return "FIXED_TRIGGERS_ASAN_ONLY"

    # No ASan in either
    if fixed_res.exit_code == 0 and vuln_res.exit_code == 0:
        return "NO_ASAN_FOUND"

    # Nonzero exits but not ASan
    return "RUN_ERROR"

def main():
    import argparse
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=ROOT_DEFAULT)
    ap.add_argument("--toolchain", default=TOOLCHAIN_DEFAULT)
    ap.add_argument("--show-output", action="store_true", help="print stdout/stderr for each file")
    args = ap.parse_args()

    root = Path(args.root).expanduser().resolve()
    if not root.exists():
        raise SystemExit(f"ERROR: root not found: {root}")

    _check("rustup")

    counts = {
        "VULNERABLE_CONFIRMED": 0,
        "NO_ASAN_FOUND": 0,
        "BOTH_TRIGGER_ASAN": 0,
        "FIXED_TRIGGERS_ASAN_ONLY": 0,
        "COMPILE_ERROR": 0,
        "RUN_ERROR": 0,
    }

    scanned = 0

    for dirpath, _, filenames in os.walk(root):
        files = set(filenames)
        if not {"fixed.rs", "vulnerable.rs"}.issubset(files):
            continue

        scanned += 1
        d = Path(dirpath)
        fixed_rs = d / "fixed.rs"
        vuln_rs = d / "vulnerable.rs"

        fixed_res = compile_and_run_single_rs(fixed_rs, args.toolchain)
        vuln_res  = compile_and_run_single_rs(vuln_rs,  args.toolchain)

        status = classify_pair(fixed_res, vuln_res)
        counts[status] += 1

        print(f"\n=== {d.relative_to(root)} ===")
        print(f"[fixed] compiled={fixed_res.compiled} exit={fixed_res.exit_code} asan={fixed_res.asan}")
        print(f"[vuln ] compiled={vuln_res.compiled}  exit={vuln_res.exit_code}  asan={vuln_res.asan}")
        print(f"[pair ] {status}")

        if args.show_output:
            print("\n---- fixed output ----")
            print(fixed_res.output[:15000])
            print("\n---- vuln output ----")
            print(vuln_res.output[:15000])

    print("\n[summary]")
    print(f"root scanned dirs with pairs : {scanned}")
    for k, v in counts.items():
        print(f"  {k:26s}: {v}")

if __name__ == "__main__":
    main()

