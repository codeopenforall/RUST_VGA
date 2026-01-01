#!/usr/bin/env python3
import os
import sys
import csv
import shutil
import subprocess
import tempfile
from pathlib import Path
from dataclasses import dataclass
from typing import Optional, Tuple, List

# Change these defaults if you want
DEFAULT_ROOT = "/home/ikqnm/PycharmProjects/rudra_sandbox/rudra_big/rudra_big/src/dataset/o3_mini/data_generation/"
DEFAULT_TOOLCHAIN = "nightly"  # or "nightly-2021-10-21" if you want to pin
EDITION = "2021"
TIMEOUT_SECS = 120

# Prefer these oracle/test filenames if present
TEST_CANDIDATES = ("demo_test.rs", "oracle.rs", "orcale.rs", "test.rs", "tests.rs")

# Make sure cargo/rustup are on PATH
os.environ["PATH"] = str(Path.home() / ".cargo" / "bin") + os.pathsep + os.environ.get("PATH", "")

@dataclass
class RunResult:
    status: str          # OK / ASAN / COMPILE_ERROR / RUNTIME_ERROR / TIMEOUT / SKIPPED
    exit_code: Optional[int]
    output_head: str     # first N chars of output for debugging


def _check_cmd(cmd: str) -> None:
    if shutil.which(cmd) is None:
        raise SystemExit(f"ERROR: '{cmd}' not found in PATH")


def _read_text(p: Path) -> str:
    try:
        return p.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return p.read_text(encoding="latin-1")


def _find_test_file(dirpath: Path) -> Optional[Path]:
    files = {p.name for p in dirpath.iterdir() if p.is_file()}
    for name in TEST_CANDIDATES:
        if name in files:
            return dirpath / name
    return None


def _compile_and_run_asan(rs_file: Path, test_file: Optional[Path], toolchain: str) -> RunResult:
    """
    If test_file exists, build a temp `--test` binary = rs_file + test_file
    else try to compile as a normal binary and run it (only works if rs_file has main()).
    """
    with tempfile.TemporaryDirectory(prefix="asan_oracle_") as td:
        td = Path(td)
        combined = td / f"{rs_file.stem}_combined.rs"

        src = _read_text(rs_file)
        if test_file is not None:
            src += "\n\n\n" + _read_text(test_file) + "\n"
        combined.write_text(src, encoding="utf-8")

        out_bin = td / (rs_file.stem + ("_tests.exe" if os.name == "nt" else "_tests"))

        # Build command
        # NOTE: -Z sanitizer=address requires nightly.
        rustc_cmd = [
            "rustup", "run", toolchain, "rustc",
            "--edition", EDITION,
        ]

        is_test = (test_file is not None)
        if is_test:
            rustc_cmd += ["--test"]
        rustc_cmd += [
            str(combined),
            "-o", str(out_bin),
            "-C", "debuginfo=1",
            "-Z", "sanitizer=address",
        ]

        # Compile
        try:
            comp = subprocess.run(
                rustc_cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
                env=dict(os.environ),
            )
        except subprocess.TimeoutExpired:
            return RunResult("TIMEOUT", None, "compile timeout")

        if comp.returncode != 0:
            out = (comp.stdout or "")
            return RunResult("COMPILE_ERROR", comp.returncode, out[:4000])

        # Run
        run_env = dict(os.environ)
        run_env["RUST_BACKTRACE"] = "0"
        # Keep ASan output concise; halt on first error.
        run_env["ASAN_OPTIONS"] = run_env.get("ASAN_OPTIONS", "")
        if run_env["ASAN_OPTIONS"]:
            run_env["ASAN_OPTIONS"] += ":"
        run_env["ASAN_OPTIONS"] += "verbosity=0:halt_on_error=1"

        run_cmd = [str(out_bin)]
        if is_test:
            run_cmd += ["--color", "never"]

        try:
            run = subprocess.run(
                run_cmd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                timeout=TIMEOUT_SECS,
                env=run_env,
            )
        except subprocess.TimeoutExpired:
            return RunResult("TIMEOUT", None, "run timeout")

        out = (run.stdout or "")
        # Heuristic: ASan reports include this signature
        if "ERROR: AddressSanitizer:" in out:
            return RunResult("ASAN", run.returncode, out[:4000])

        if run.returncode == 0:
            return RunResult("OK", 0, out[:2000])

        return RunResult("RUNTIME_ERROR", run.returncode, out[:4000])


def main():
    import argparse
    ap = argparse.ArgumentParser(description="Summarize fixed.rs vs vulnerable.rs using AddressSanitizer (ASan).")
    ap.add_argument("--root", default=DEFAULT_ROOT, help="Root folder to scan recursively")
    ap.add_argument("--toolchain", default=DEFAULT_TOOLCHAIN, help="Rust toolchain for rustup run (e.g. nightly, nightly-2021-10-21)")
    ap.add_argument("--csv", default="asan_report.csv", help="Output CSV filename")
    args = ap.parse_args()

    _check_cmd("rustup")

    root = Path(args.root).expanduser().resolve()
    if not root.exists():
        raise SystemExit(f"ERROR: root not found: {root}")

    rows = []
    scanned_dirs = 0

    # Counters
    c_ok_both = 0
    c_asan_only_vuln = 0
    c_asan_only_fixed = 0
    c_asan_both = 0
    c_compile_err = 0
    c_skipped_no_test = 0
    c_other = 0

    # Walk directories
    for dirpath, _dirnames, filenames in os.walk(root):
        d = Path(dirpath)

        # Look for fixed.rs and vulnerable.rs (case-insensitive match)
        files_lower = {name.lower(): name for name in filenames}
        if "fixed.rs" not in files_lower or "vulnerable.rs" not in files_lower:
            continue

        fixed_path = d / files_lower["fixed.rs"]
        vuln_path  = d / files_lower["vulnerable.rs"]

        test_path = _find_test_file(d)

        scanned_dirs += 1

        if test_path is None:
            # Without a test/oracle we usually can't trigger anything reliably.
            # (We could try running main(), but most dataset files are modules.)
            rows.append([str(d), "SKIPPED", "", "no demo_test.rs/oracle.rs found"])
            c_skipped_no_test += 1
            continue

        fixed_res = _compile_and_run_asan(fixed_path, test_path, args.toolchain)
        vuln_res  = _compile_and_run_asan(vuln_path,  test_path, args.toolchain)

        # Classify directory-level outcome
        if fixed_res.status == "OK" and vuln_res.status == "ASAN":
            verdict = "VULNERABLE_CONFIRMED"  # good: fixed clean, vuln triggers ASan
            c_asan_only_vuln += 1
        elif fixed_res.status == "OK" and vuln_res.status == "OK":
            verdict = "NO_ASAN_FOUND"  # could still be vulnerable but not triggered by tests
            c_ok_both += 1
        elif fixed_res.status == "ASAN" and vuln_res.status == "ASAN":
            verdict = "BOTH_TRIGGER_ASAN"
            c_asan_both += 1
        elif fixed_res.status == "ASAN" and vuln_res.status == "OK":
            verdict = "FIXED_TRIGGERS_ASAN_BUT_VULN_DOESNT"
            c_asan_only_fixed += 1
        elif fixed_res.status == "COMPILE_ERROR" or vuln_res.status == "COMPILE_ERROR":
            verdict = "COMPILE_ERROR"
            c_compile_err += 1
        else:
            verdict = f"OTHER({fixed_res.status},{vuln_res.status})"
            c_other += 1

        # Save row (keep output short)
        rows.append([
            str(d),
            verdict,
            f"fixed={fixed_res.status} vuln={vuln_res.status}",
            # include a small hint if ASAN happened
            ("ASAN in vulnerable" if vuln_res.status == "ASAN" else ("ASAN in fixed" if fixed_res.status == "ASAN" else "")),
        ])

    # Write CSV
    out_csv = Path(args.csv).resolve()
    with out_csv.open("w", newline="", encoding="utf-8") as f:
        w = csv.writer(f)
        w.writerow(["dir", "verdict", "statuses", "note"])
        w.writerows(rows)

    print("\n[ASan Summary]")
    print(f"  root scanned dirs with pairs : {scanned_dirs}")
    print(f"  VULNERABLE_CONFIRMED         : {c_asan_only_vuln}")
    print(f"  NO_ASAN_FOUND                : {c_ok_both}")
    print(f"  BOTH_TRIGGER_ASAN            : {c_asan_both}")
    print(f"  FIXED_TRIGGERS_ASAN_ONLY     : {c_asan_only_fixed}")
    print(f"  COMPILE_ERROR                : {c_compile_err}")
    print(f"  SKIPPED (no oracle/test)     : {c_skipped_no_test}")
    print(f"  OTHER                        : {c_other}")
    print(f"\nWrote CSV report: {out_csv}")

if __name__ == "__main__":
    main()
