#!/usr/bin/env python3
import os
import re
import shutil
import subprocess
import tempfile
from pathlib import Path
from dataclasses import dataclass
from typing import List, Optional, Tuple

# ====== CONFIG ======
ROOT = Path("/home/ikqnm/PycharmProjects/rudra_sandbox/rudra_big/rudra_big/src/dataset/o3_mini/data_generation/Real CVEs")
LOG_DIR = Path("/home/ikqnm/PycharmProjects/rudra_sandbox/rudra_big/rudra_big/ASan_Report/asan_logs")

# Toolchain you already use for ASan
TOOLCHAIN = "nightly"   # or "nightly-2021-10-21" if you prefer
EDITION = "2021"
TIMEOUT_SECS = 120

# ASan settings similar to what you run in terminal
ENV_EXTRA = {
    "RUST_BACKTRACE": "0",
    "ASAN_OPTIONS": "verbosity=0:halt_on_error=1",
}

# Use -Awarnings so you only see errors
RUSTFLAGS = "-Awarnings -Z sanitizer=address"
# ====================


@dataclass
class FileResult:
    file: Path
    compiled: bool
    ran: bool
    asan_hit: bool
    compile_log: str
    run_log: str
    exit_code: Optional[int]


ASAN_RE = re.compile(r"ERROR: AddressSanitizer:", re.IGNORECASE)


def require(cmd: str) -> None:
    if shutil.which(cmd) is None:
        raise SystemExit(f"ERROR: '{cmd}' not found in PATH")


def find_targets(root: Path) -> List[Path]:
    targets = []
    for p in root.rglob("fixed.rs"):
        targets.append(p)
    for p in root.rglob("vulnerable.rs"):
        targets.append(p)
    # keep stable order
    targets = sorted(set(targets))
    return targets


def compile_file_as_test(rs_file: Path, out_bin: Path) -> Tuple[bool, str]:
    cmd = [
        "rustup", "run", TOOLCHAIN, "rustc",
        "--edition", EDITION,
        "--test", str(rs_file),
        "-O",
        "-o", str(out_bin),
    ]
    env = os.environ.copy()
    env.update(ENV_EXTRA)
    env["RUSTFLAGS"] = RUSTFLAGS

    try:
        p = subprocess.run(
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            timeout=TIMEOUT_SECS,
            env=env,
        )
    except subprocess.TimeoutExpired:
        return False, "compile timeout"

    return (p.returncode == 0), (p.stdout or "")


def run_test_bin(bin_path: Path) -> Tuple[bool, bool, str, Optional[int]]:
    env = os.environ.copy()
    env.update(ENV_EXTRA)

    try:
        p = subprocess.run(
            [str(bin_path), "--color", "never"],
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            timeout=TIMEOUT_SECS,
            env=env,
        )
    except subprocess.TimeoutExpired:
        return False, False, "run timeout", None

    out = p.stdout or ""
    asan_hit = bool(ASAN_RE.search(out))
    return True, asan_hit, out, p.returncode


def safe_log_name(p: Path) -> str:
    rel = p.relative_to(ROOT)
    s = str(rel).replace("/", "__")
    s = re.sub(r"[^a-zA-Z0-9_.\-]+", "_", s)
    return s


def main():
    require("rustup")

    ROOT.mkdir(parents=True, exist_ok=True)
    LOG_DIR.mkdir(parents=True, exist_ok=True)

    targets = find_targets(ROOT)

    # write listing for debugging
    listing_path = LOG_DIR.parent / "file_list.log"
    listing_path.write_text("\n".join(str(p) for p in targets) + ("\n" if targets else ""), encoding="utf-8")

    if not targets:
        print("[summary]")
        print("  files discovered: 0")
        print(f"  NOTE: wrote listing output to {listing_path}")
        return

    results: List[FileResult] = []

    for i, rs_file in enumerate(targets, 1):
        print(f"[{i:>4}/{len(targets)}] {rs_file}")

        name = safe_log_name(rs_file)
        compile_log_file = LOG_DIR / f"{name}.compile.log"
        run_log_file = LOG_DIR / f"{name}.run.log"

        with tempfile.TemporaryDirectory(prefix="asan_rs_") as td:
            out_bin = Path(td) / "tbin"

            ok, comp_log = compile_file_as_test(rs_file, out_bin)
            compile_log_file.write_text(comp_log, encoding="utf-8")

            if not ok:
                results.append(FileResult(
                    file=rs_file,
                    compiled=False,
                    ran=False,
                    asan_hit=False,
                    compile_log=comp_log,
                    run_log="",
                    exit_code=None,
                ))
                continue

            ran, asan_hit, run_log, exit_code = run_test_bin(out_bin)
            run_log_file.write_text(run_log, encoding="utf-8")

            results.append(FileResult(
                file=rs_file,
                compiled=True,
                ran=ran,
                asan_hit=asan_hit,
                compile_log=comp_log,
                run_log=run_log,
                exit_code=exit_code,
            ))

    # Summarize
    compiled_ok = sum(1 for r in results if r.compiled)
    compile_fail = sum(1 for r in results if not r.compiled)
    ran_ok = sum(1 for r in results if r.ran)
    ran_fail = sum(1 for r in results if r.compiled and not r.ran)
    asan_hits = sum(1 for r in results if r.asan_hit)

    # breakdown fixed vs vulnerable
    fixed_hits = sum(1 for r in results if r.asan_hit and r.file.name == "fixed.rs")
    vuln_hits = sum(1 for r in results if r.asan_hit and r.file.name == "vulnerable.rs")

    summary_path = LOG_DIR.parent / "summary.txt"
    lines = []
    lines.append("[summary]")
    lines.append(f"  root                      : {ROOT}")
    lines.append(f"  files discovered          : {len(results)}")
    lines.append(f"  compiled ok               : {compiled_ok}")
    lines.append(f"  compile errors            : {compile_fail}")
    lines.append(f"  ran ok                    : {ran_ok}")
    lines.append(f"  run errors/timeouts       : {ran_fail}")
    lines.append(f"  ASan hits (total)         : {asan_hits}")
    lines.append(f"    ASan hits in vulnerable : {vuln_hits}")
    lines.append(f"    ASan hits in fixed      : {fixed_hits}")
    summary_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

    print("\n".join(lines))
    print(f"\nLogs written under: {LOG_DIR}")
    print(f"Summary written to: {summary_path}")
    print(f"File list written to: {listing_path}")


if __name__ == "__main__":
    main()

