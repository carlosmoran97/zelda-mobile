#!/usr/bin/env python3
"""Build the static browser game; --setup installs its pinned Wasm tools."""
import argparse
import html
import json
from pathlib import Path
import shutil
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parent.parent
TARGET = 'wasm32-unknown-unknown'
TOOL_ROOT = ROOT / 'target/web-tools'
manifest = tomllib.loads((ROOT / 'Cargo.toml').read_text())
VERSION = manifest['target']['cfg(target_arch = "wasm32")']['dependencies']['wasm-bindgen'].removeprefix('=')


def run(arguments):
    print('+', ' '.join(map(str, arguments)), flush=True)
    subprocess.run(list(map(str, arguments)), cwd=ROOT, check=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--setup', action='store_true', help='Install target and matching wasm-bindgen CLI first')
    parser.add_argument('--offline', action='store_true', help='Use previously fetched Cargo dependencies')
    args = parser.parse_args()
    local_tool = TOOL_ROOT / 'bin/wasm-bindgen'
    tool = str(local_tool) if local_tool.exists() else shutil.which('wasm-bindgen')
    if args.setup:
        run(['rustup', 'target', 'add', TARGET])
        if not tool or subprocess.check_output([tool, '--version'], text=True).strip() != f'wasm-bindgen {VERSION}':
            run(['cargo', 'install', 'wasm-bindgen-cli', '--version', VERSION, '--locked', '--root', TOOL_ROOT])
            tool = str(local_tool)
    if not tool:
        parser.error('Missing wasm-bindgen. Run this command with --setup first.')
    if subprocess.check_output([tool, '--version'], text=True).strip() != f'wasm-bindgen {VERSION}':
        parser.error(f'wasm-bindgen must be {VERSION}. Run with --setup to install the matching version.')
    build = ['cargo', 'build', '--release', '--locked', '--lib', '--target', TARGET]
    if args.offline:
        build.append('--offline')
    run(build)
    output = ROOT / 'dist/web'
    if output.exists():
        shutil.rmtree(output)
    shutil.copytree(ROOT / 'web', output)
    run([tool, ROOT / f'target/{TARGET}/release/zelda_mobile.wasm', '--target', 'web', '--out-dir', output / 'pkg'])
    audio = output / 'assets/audio'
    audio.mkdir(parents=True)
    for name in ['GeneralUser-GS.sf2', 'LICENSE-GeneralUser-GS.txt']:
        shutil.copy2(ROOT / 'assets/audio' / name, audio / name)
    shutil.copy2(ROOT / 'assets/Images/Icon.png', output / 'icon.png')
    (output / '.nojekyll').touch()
    licenses = output / 'licenses'
    licenses.mkdir()
    metadata = json.loads(subprocess.check_output([
        'cargo', 'metadata', '--format-version', '1', '--locked', '--offline', '--filter-platform', TARGET,
    ], cwd=ROOT))
    enabled = {node['id'] for node in metadata['resolve']['nodes']}
    notices = []
    for package in metadata['packages']:
        if not package['source'] or package['id'] not in enabled:
            continue
        source = Path(package['manifest_path']).parent
        name = f'{package["name"]}-{package["version"]}'
        links = []
        for file in sorted(source.iterdir()):
            if file.is_file() and file.name.upper().startswith(('LICENSE', 'COPYING', 'NOTICE')):
                destination = licenses / name
                destination.mkdir(exist_ok=True)
                shutil.copy2(file, destination / file.name)
                links.append(f'<a href="./{html.escape(name)}/{html.escape(file.name)}">{html.escape(file.name)}</a>')
        notices.append(f'<li>{html.escape(name)} ({html.escape(package["license"] or "see source")}): {", ".join(links)}</li>')
    (licenses / 'index.html').write_text('''<!doctype html><html lang="en"><meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1"><title>Dependency licenses</title>
<link rel="stylesheet" href="../style.css"><article class="credits"><a href="../credits.html">Credits</a>
<h1>Dependency licenses</h1><ul>''' + ''.join(notices) + '</ul></article></html>')
    credits = html.escape((ROOT / 'THIRD_PARTY.md').read_text())
    (output / 'credits.html').write_text(f'''<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>Zelda Mobile · Credits</title><link rel="stylesheet" href="./style.css">
<article class="credits"><a href="./">Back to the game</a><h1>Credits &amp; licenses</h1>
<pre>{credits}</pre><p><a href="./assets/audio/LICENSE-GeneralUser-GS.txt">GeneralUser GS license</a></p>
<p><a href="./licenses/">Bundled dependency licenses</a></p>
<p><a href="https://github.com/carlosmoran97/zelda-mobile">Source code and dependency notices</a></p></article></html>''')
    size = sum(p.stat().st_size for p in output.rglob('*') if p.is_file())
    print(f'Web build ready: {output} ({size / 1024 / 1024:.1f} MiB, including audio)')
    print('Serve locally: python3 -m http.server 8080 --directory dist/web')


if __name__ == '__main__':
    main()
