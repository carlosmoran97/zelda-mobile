#!/usr/bin/env python3
"""Recover reference Java, original assets, and font atlases. Never writes Rust."""
from pathlib import Path
import hashlib
import json
import subprocess
import urllib.request
import zipfile

root=Path(__file__).resolve().parent.parent
jar=root/'TheLegendOfZeldaMobile-beta4.jar'
expected='5a9183745571117f850a51414ededee4afe3b99e3dc014a7ece0cae584bc199a'
if hashlib.sha256(jar.read_bytes()).hexdigest()!=expected:
    raise SystemExit('The source JAR differs from the audited beta 4 input.')
cfr=root/'tools/cfr-0.152.jar'
if not cfr.exists():
    cfr.parent.mkdir(exist_ok=True)
    urllib.request.urlretrieve('https://www.benf.org/other/cfr/cfr-0.152.jar',cfr)
if hashlib.sha256(cfr.read_bytes()).hexdigest()!='f686e8f3ded377d7bc87d216a90e9e9512df4156e75b06c655a16648ae8765b2':
    raise SystemExit('Unexpected CFR checksum.')
subprocess.run(['java','-jar',str(cfr),str(jar),'--outputdir',str(root/'reference/java'),'--extraclasspath',str(root/'emulator/freej2me_plus.jar'),'--silent','true'],check=True)
manifest={}
with zipfile.ZipFile(jar) as z:
    for name in z.namelist():
        if name.startswith(('Images/','Maps/','Sound/')):
            data=z.read(name);dest=root/'assets'/name;dest.parent.mkdir(parents=True,exist_ok=True);dest.write_bytes(data)
            manifest[name]={'bytes':len(data),'sha256':hashlib.sha256(data).hexdigest()}
(root/'reference/asset-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
work=root/'target/reference-run'
work.mkdir(parents=True,exist_ok=True)
subprocess.run(['javac','-cp',str(root/'emulator/freej2me_plus.jar'),'-d',str(work),str(root/'reference/ExportFonts.java')],check=True)
subprocess.run(['java','-Djava.awt.headless=true','-Dfile.encoding=ISO_8859_1','-cp',str(work)+':'+str(root/'emulator/freej2me_plus.jar'),'ExportFonts',str(jar),str(root/'assets/fonts')],cwd=work,check=True)
print('Reference Java, original resources, glyphs and metrics are up to date. Native Rust sources were not modified.')
