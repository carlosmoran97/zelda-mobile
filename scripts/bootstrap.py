#!/usr/bin/env python3
"""Install project-local build tooling, then fetch the locked Rust dependencies."""
from pathlib import Path
import subprocess
import sys

root = Path(__file__).resolve().parent.parent
cmake = root / 'tools/python/cmake/data/bin/cmake'
if not cmake.exists():
    subprocess.run([sys.executable, '-m', 'pip', 'install', '--target', str(root/'tools/python'), 'cmake==4.4.5'], check=True)
subprocess.run(['cargo', 'fetch', '--locked'], cwd=root, check=True)
print('Build dependencies are ready. Run: python3 scripts/package.py')
