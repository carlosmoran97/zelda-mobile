#!/usr/bin/env python3
"""Build and locally sign a self-contained Apple Silicon application."""
from pathlib import Path
import argparse
import json
import plistlib
import shutil
import struct
import subprocess

root = Path(__file__).resolve().parent.parent
parser = argparse.ArgumentParser()
parser.add_argument('--no-build', action='store_true')
args = parser.parse_args()
if not args.no_build:
    subprocess.run(['cargo', 'build', '--release', '--locked', '--offline'], cwd=root, check=True)
app = root / 'dist/Zelda Mobile.app'
macos, resources = app/'Contents/MacOS', app/'Contents/Resources'
macos.mkdir(parents=True, exist_ok=True)
resources.mkdir(parents=True, exist_ok=True)
shutil.copy2(root/'target/release/zelda-mobile', macos/'zelda-mobile')
# No original JAR, emulator, JVM, build tools, or dynamically linked SDL go here.
shutil.copytree(root/'assets', resources/'assets', dirs_exist_ok=True)
iconset = root/'target/Zelda.iconset'
iconset.mkdir(exist_ok=True)
for size in (16,32,128,256,512):
    for retina in (False,True):
        suffix='@2x' if retina else ''
        pixels=size*(2 if retina else 1)
        subprocess.run(['/usr/bin/sips','-z',str(pixels),str(pixels),str(root/'assets/Images/Icon.png'),'--out',str(iconset/f'icon_{size}x{size}{suffix}.png')],check=True,stdout=subprocess.DEVNULL)
# Modern ICNS stores PNG representations directly. Writing the container avoids
# iconutil's dependence on a WindowServer image-conversion session in CI/sandboxes.
chunks=[]
for kind,size,suffix in [(b'icp5',32,''),(b'icp6',32,'@2x'),(b'ic07',128,''),(b'ic08',256,''),(b'ic09',512,''),(b'ic10',512,'@2x')]:
    png=(iconset/f'icon_{size}x{size}{suffix}.png').read_bytes()
    chunks.append(kind+struct.pack('>I',len(png)+8)+png)
body=b''.join(chunks)
(resources/'Zelda.icns').write_bytes(b'icns'+struct.pack('>I',len(body)+8)+body)
info={
    'CFBundleExecutable':'zelda-mobile','CFBundleIdentifier':'local.zeldamobile.rust',
    'CFBundleName':'Zelda Mobile','CFBundleDisplayName':'Zelda Mobile',
    'CFBundlePackageType':'APPL','CFBundleShortVersionString':'0.1.0','CFBundleVersion':'1',
    'CFBundleIconFile':'Zelda.icns','LSMinimumSystemVersion':'13.0',
    'NSHighResolutionCapable':True,'NSSupportsAutomaticGraphicsSwitching':True,
    'NSHumanReadableCopyright':'Original Zelda Mobile beta 4 by Gorthwogh. Original game credits retained.',
}
with (app/'Contents/Info.plist').open('wb') as f:plistlib.dump(info,f)
licenses=resources/'Licenses'
licenses.mkdir(exist_ok=True)
metadata=json.loads(subprocess.check_output(['cargo','metadata','--format-version','1','--locked','--offline'],cwd=root))
for package in metadata['packages']:
    if not package['source']:continue
    source=Path(package['manifest_path']).parent
    files=[p for p in source.iterdir() if p.is_file() and p.name.upper().startswith(('LICENSE','COPYING','NOTICE'))]
    if (source/'SDL/LICENSE.txt').exists():files.append(source/'SDL/LICENSE.txt')
    dest=licenses/f'{package["name"]}-{package["version"]}'
    for f in files:
        dest.mkdir(exist_ok=True)
        shutil.copy2(f,dest/f.name)
shutil.copy2(root/'THIRD_PARTY.md',licenses/'THIRD_PARTY.md')
subprocess.run(['/usr/bin/codesign','--force','--deep','--sign','-',str(app)],check=True)
subprocess.run(['/usr/bin/codesign','--verify','--deep','--strict',str(app)],check=True)
print(app)
