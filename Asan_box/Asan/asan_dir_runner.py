#!/usr/bin/env python3
import os
import sys
import shutil
import subprocess
import tempfile
from pathlib import Path
from dataclasses import dataclass

DEFAULT_ROOT = "/home/ikqnm/PycharmProjects/Asan_box/src/dataset/Qwen_Coder/DataAugmented_pairs_new_A1/CWE-415"

RUST_TOOLCHAIN = "+nightly"  # ASan requires nightly
RUST_EDITION = "2021"

RUSTFLAGS = "-Awarnings -Z sanitizer=address"
ASAN_OPTIONS = "verbosity=0:halt_on_error=0"
RUST_BACKTRACE = "0"

ASAN_MARKER = "ERROR: AddressSanitizer:"


@dataclass
class Result:
    status: str
    returncode: int
    has_asan: bool
    log_path: Path


def die(msg: str) -> None:
    print(f"ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def check_cmd(cmd: str) -> None:
    if shutil.which(cmd) is None:
        die(f"'{cmd}' not found in PATH")


def run_cmd_tee(cmd, env, log_path: Path, cwd: Path, timeout_secs: int) -> tuple[int, bool, str]:
    """
    Run command, stream output to console AND file, return (returncode, has_asan, full_text).
    """
    log_path.parent.mkdir(parents=True, exist_ok=True)
    full = []

    with log_path.open("w", encoding="utf-8", errors="replace") as f:
        f.write(f"$ (cwd={cwd}) " + " ".join(cmd) + "\n\n")
        f.flush()

        p = subprocess.Popen(
            cmd,
            cwd=str(cwd),
            env=env,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            universal_newlines=True,
        )

        has_asan = False
        try:
            for line in p.stdout:
                full.append(line)
                if ASAN_MARKER in line:
                    has_asan = True
                sys.stdout.write(line)
                f.write(line)

            p.wait(timeout=timeout_secs)
        except subprocess.TimeoutExpired:
            p.kill()
            full.append("\n[TIMEOUT]\n")
            f.write("\n[TIMEOUT]\n")
            return 124, False, "".join(full)

    return p.returncode, has_asan, "".join(full)


def make_temp_crate_from_file(rs_path: Path, crate_name: str) -> Path:
    td = Path(tempfile.mkdtemp(prefix=f"asan_{crate_name}_"))
    (td / "src").mkdir(parents=True, exist_ok=True)

    (td / "src" / "lib.rs").write_text(
        rs_path.read_text(encoding="utf-8", errors="replace"),
        encoding="utf-8",
    )

    (td / "Cargo.toml").write_text(
        f"""[package]
name = "{crate_name}"
version = "0.1.0"
edition = "{RUST_EDITION}"
""",
        encoding="utf-8",
    )
    return td


def classify_output(returncode: int, has_asan: bool, out: str) -> str:
    if has_asan:
        return "ASAN_FOUND"
    if returncode == 0:
        return "NO_ASAN_FOUND"
    if "could not compile" in out or "error[" in out or "\nerror:" in out:
        return "COMPILE_ERROR"
    return "RUN_ERROR"


def main():
    import argparse

    ap = argparse.ArgumentParser(
        description="Run ASan per fixed.rs/vulnerable.rs file under a root directory (continues on failures)."
    )
    ap.add_argument("--root", default=DEFAULT_ROOT, help="Root directory to scan")
    ap.add_argument("--logs", default="./asan_logs_out", help="Directory to store logs")
    ap.add_argument("--timeout", type=int, default=600, help="Timeout seconds per file")
    ap.add_argument("--keep-tmp", action="store_true", help="Do not delete temp crates (debugging)")
    args = ap.parse_args()

    root = Path(args.root).expanduser().resolve()
    if not root.exists():
        die(f"root not found: {root}")

    logs_dir = Path(args.logs).expanduser().resolve()
    logs_dir.mkdir(parents=True, exist_ok=True)

    check_cmd("cargo")

    pair_dirs = []
    for dirpath, _dirnames, filenames in os.walk(root):
        s = set(filenames)
        if "A1_fixed.rs" in s and "A1_vulnerable.rs" in s:
            pair_dirs.append(Path(dirpath))
    pair_dirs.sort()

    print(f"[info] root: {root}")
    print(f"[info] pair directories found: {len(pair_dirs)}")
    print(f"[info] logs: {logs_dir}")
    print()

    counts = {
        "ASAN_FOUND": 0,
        "NO_ASAN_FOUND": 0,
        "COMPILE_ERROR": 0,
        "RUN_ERROR": 0,
    }

    env = os.environ.copy()
    env["RUSTFLAGS"] = RUSTFLAGS
    env["ASAN_OPTIONS"] = ASAN_OPTIONS
    env["RUST_BACKTRACE"] = RUST_BACKTRACE

    tmp_to_cleanup = []

    for pdir in pair_dirs:
        rel = pdir.relative_to(root)
        print("\n" + "=" * 80)
        print(f"[pair] {rel}")
        print("=" * 80)

        for kind in ("A1_fixed", "A1_vulnerable"):
            rs_path = pdir / f"{kind}.rs"
            if not rs_path.exists():
                continue

            crate_name = f"{rel.as_posix().replace('/', '_').replace('-', '_')}_{kind}"
            crate_name = "".join(ch if ch.isalnum() or ch == "_" else "_" for ch in crate_name).lower()
            if crate_name and crate_name[0].isdigit():
                crate_name = "m_" + crate_name

            tmp_crate = make_temp_crate_from_file(rs_path, crate_name)
            tmp_to_cleanup.append(tmp_crate)

            log_path = logs_dir / rel / f"{kind}.log"
            print(f"\n--- [{kind}] {rs_path} ---")
            print(f"[tmp] {tmp_crate}")

            cmd = ["cargo", RUST_TOOLCHAIN, "test", "--lib", "--", "--nocapture"]
            rc, has_asan, out = run_cmd_tee(cmd, env, log_path, cwd=tmp_crate, timeout_secs=args.timeout)

            status = classify_output(rc, has_asan, out)
            counts[status] += 1
            print(f"\n[result] {kind}: status={status} returncode={rc} log={log_path}")

    if not args.keep_tmp:
        for td in tmp_to_cleanup:
            shutil.rmtree(td, ignore_errors=True)

    print("\n" + "=" * 80)
    print("[summary]")
    print(f"  pair directories processed : {len(pair_dirs)}")
    print(f"  files attempted            : {len(pair_dirs) * 2}")
    for k, v in counts.items():
        print(f"  {k:<24}: {v}")
    print("=" * 80)


if __name__ == "__main__":
    main()

