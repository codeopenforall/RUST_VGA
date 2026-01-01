#!/usr/bin/env python3
import os
from pathlib import Path

SOURCE_ROOT = Path(
    "/home/ikqnm/PycharmProjects/PythonProject/Data Augmentation/O3_mini/Data Generation/"
)

DEST_ROOT = Path(
    "/home/ikqnm/PycharmProjects/Asan_box/src/dataset/Qwen_Coder/DataAugmented_pairs_new_A3/"
)

def read_text_safe(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return path.read_text(encoding="latin-1")

def append_test(dest_file: Path, test_content: str):
    with dest_file.open("a", encoding="utf-8") as f:
        f.write("\n\n")      # one empty line
        f.write(test_content.rstrip())
        f.write("\n")

def main():
    if not SOURCE_ROOT.exists():
        raise SystemExit(f"Source root does not exist: {SOURCE_ROOT}")
    if not DEST_ROOT.exists():
        raise SystemExit(f"Destination root does not exist: {DEST_ROOT}")

    count = 0

    for src_dir, _, files in os.walk(SOURCE_ROOT):
        if "demo_test.rs" not in files:
            continue

        src_dir = Path(src_dir)
        demo_test = src_dir / "demo_test.rs"
        test_content = read_text_safe(demo_test)

        # Preserve relative structure
        rel_path = src_dir.relative_to(SOURCE_ROOT)
        dest_dir = DEST_ROOT / rel_path

        if not dest_dir.exists():
            continue

        for target_name in ("A3_fixed.rs", "A3_vulnerable.rs"):
            target_file = dest_dir / target_name
            if target_file.exists():
                append_test(target_file, test_content)
                print(f"Appended demo_test.rs → {target_file}")
                count += 1

    print(f"\nDone. Appended tests to {count} file(s).")

if __name__ == "__main__":
    main()
