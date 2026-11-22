#!/usr/bin/env python3
"""Build the React/Vite browser game; --setup installs its pinned Wasm tools."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import tomllib
from urllib.parse import quote

ROOT = Path(__file__).resolve().parent.parent
TARGET = 'wasm32-unknown-unknown'
TOOL_ROOT = ROOT / 'target/web-tools'
manifest = tomllib.loads((ROOT / 'Cargo.toml').read_text())
VERSION = manifest['target']['cfg(target_arch = "wasm32")']['dependencies']['wasm-bindgen'].removeprefix('=')


def run(arguments):
    print('+', ' '.join(map(str, arguments)), flush=True)
    subprocess.run(list(map(str, arguments)), cwd=ROOT, check=True)


def dependency_licenses(output):
    licenses = output / 'licenses'
    licenses.mkdir()
    notices = []

    def collect(source, name, license_name):
        links = []
        for file in sorted(source.iterdir()):
            if file.is_file() and file.name.upper().startswith(('LICENSE', 'COPYING', 'NOTICE')):
                destination = licenses / name
                destination.mkdir(parents=True, exist_ok=True)
                shutil.copy2(file, destination / file.name)
                links.append({'name': file.name, 'url': f'{quote(name)}/{quote(file.name)}'})
        notices.append({'name': name, 'license': license_name or 'see source', 'files': links})

    metadata = json.loads(subprocess.check_output([
        'cargo', 'metadata', '--format-version', '1', '--locked', '--offline', '--filter-platform', TARGET,
    ], cwd=ROOT))
    enabled = {node['id'] for node in metadata['resolve']['nodes']}
    for package in metadata['packages']:
        if package['source'] and package['id'] in enabled:
            collect(Path(package['manifest_path']).parent, f'{package["name"]}-{package["version"]}', package['license'])

    packages = json.loads((ROOT / 'web/package-lock.json').read_text())['packages']
    for location, package in packages.items():
        if not location or package.get('dev'):
            continue
        source = ROOT / 'web' / location
        manifest = json.loads((source / 'package.json').read_text())
        name = f'{manifest["name"]}-{manifest["version"]}'.replace('/', '-')
        collect(source, name, manifest.get('license'))
    collect(ROOT / 'web/licenses/shadcn', 'shadcn-ui', 'MIT')
    notices.sort(key=lambda entry: entry['name'])
    (licenses / 'index.json').write_text(json.dumps(notices, indent=2) + '\n')
    # Retain existing credits URLs; the content is now rendered by React.
    for destination, url in [(output / 'credits.html', './#credits'), (licenses / 'index.html', '../#licenses')]:
        destination.write_text(f'<!doctype html><html lang="en"><meta charset="utf-8">'
                              f'<meta http-equiv="refresh" content="0;url={url}">'
                              f'<title>Zelda Mobile · Credits</title><a href="{url}">View credits</a></html>')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--setup', action='store_true', help='Install target and matching wasm-bindgen CLI first')
    parser.add_argument('--offline', action='store_true', help='Use previously fetched Cargo and npm dependencies')
    parser.add_argument('--prepare-only', action='store_true', help='Prepare Wasm and public assets for the Vite dev server')
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
    if not shutil.which('npm'):
        parser.error('Missing npm. Install Node.js 22.12+ to build the React interface.')
    install = ['npm', 'ci', '--prefix', ROOT / 'web']
    if args.offline:
        install.append('--offline')
    run(install)
    output = ROOT / 'web/public'
    if output.exists():
        shutil.rmtree(output)
    output.mkdir(parents=True)
    run([tool, ROOT / f'target/{TARGET}/release/zelda_mobile.wasm', '--target', 'web', '--out-dir', output / 'pkg'])
    audio = output / 'assets/audio'
    audio.mkdir(parents=True)
    for name in ['GeneralUser-GS.sf2', 'LICENSE-GeneralUser-GS.txt']:
        shutil.copy2(ROOT / 'assets/audio' / name, audio / name)
    shutil.copy2(ROOT / 'assets/Images/Icon.png', output / 'icon.png')
    for name in ['audio-worker.js', 'audio-worklet.js']:
        shutil.copy2(ROOT / 'web' / name, output / name)
    (output / '.nojekyll').touch()
    dependency_licenses(output)
    if args.prepare_only:
        print('Browser assets ready. Start React: npm run dev --prefix web')
        return
    run(['npm', 'run', 'build', '--prefix', ROOT / 'web'])
    output = ROOT / 'dist/web'
    size = sum(p.stat().st_size for p in output.rglob('*') if p.is_file())
    print(f'Web build ready: {output} ({size / 1024 / 1024:.1f} MiB, including optional audio)')
    print('Serve locally: python3 -m http.server 8080 --directory dist/web')


if __name__ == '__main__':
    main()
