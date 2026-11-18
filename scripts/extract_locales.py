#!/usr/bin/env python3
"""Extracts all unique tr(es, en) keys from src/ into locales/en.json
and checks translation coverage for all other locales/*.json files.
"""
import glob
import json
import os
import re

PATTERN = re.compile(
    r'\btr\(\s*"((?:[^"\\]|\\.)*)"\s*,\s*"((?:[^"\\]|\\.)*)"\s*,?\s*\)',
    re.S,
)


def unescape_rust_str(s: str) -> str:
    return (
        s.replace(r"\"", '"')
        .replace(r"\n", "\n")
        .replace(r"\t", "\t")
        .replace(r"\\", "\\")
    )


def main() -> None:
    os.makedirs("locales", exist_ok=True)
    keys: dict[str, str] = {}
    for path in sorted(glob.glob("src/**/*.rs", recursive=True)):
        with open(path, "r", encoding="utf-8") as f:
            src = f.read()
        for m in PATTERN.finditer(src):
            en = unescape_rust_str(m.group(2))
            keys[en] = en

    sorted_keys = dict(sorted(keys.items(), key=lambda kv: kv[0].lower()))
    with open("locales/en.json", "w", encoding="utf-8") as f:
        json.dump(sorted_keys, f, ensure_ascii=False, indent=2)
        f.write("\n")
    print(f"Wrote {len(sorted_keys)} keys to locales/en.json")

    for loc_path in sorted(glob.glob("locales/*.json")):
        if os.path.basename(loc_path) == "en.json":
            continue
        with open(loc_path, "r", encoding="utf-8") as f:
            data = json.load(f)
        missing = [k for k in sorted_keys if k not in data]
        covered = len(sorted_keys) - len(missing)
        pct = (covered / len(sorted_keys) * 100.0) if sorted_keys else 100.0
        print(f"  {loc_path}: {covered}/{len(sorted_keys)} ({pct:.1f}%)")


if __name__ == "__main__":
    main()
