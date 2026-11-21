#!/usr/bin/env python3
"""Run repeatable native and original-JAR checks with isolated progress files."""
from pathlib import Path
import argparse
import json
import os
import shutil
import subprocess
import tempfile

root=Path(__file__).resolve().parent.parent
parser=argparse.ArgumentParser()
parser.add_argument('--skip-build',action='store_true')
args=parser.parse_args()
def run(argv,**kw):
    print('+',' '.join(map(str,argv)),flush=True)
    return subprocess.run(list(map(str,argv)),check=True,cwd=kw.pop('cwd',root),**kw)
if not args.skip_build:run(['cargo','build','--release','--bins','--examples','--locked','--offline'])
binary=root/'target/release/zelda-mobile'
run(['cargo','test','--locked','--offline'])
run([binary,'--audit'])
with tempfile.TemporaryDirectory(prefix='zelda-validation-',dir=root/'target') as temp:
    temp=Path(temp)
    run([binary,'--audit-rooms','--save-dir',temp/'rooms'])
    run([binary,'--audit-saves','--save-dir',temp/'saves'])
    run([binary,'--replay',root/'reference/opening-replay.json','--save-dir',temp/'replay'])
    run([binary,'--replay',root/'reference/menu-replay.json','--save-dir',temp/'menus','--screenshot',root/'target/menu-check.png'])
run([root/'target/release/examples/behavior_check'])
if shutil.which('javac') and shutil.which('java'):
    work=root/'target/reference-run'
    work.mkdir(exist_ok=True)
    emulator=root/'emulator/freej2me_plus.jar'
    run(['javac','-cp',emulator,'-d',work,root/'reference/ReferenceHarness.java',root/'reference/BehaviorOracle.java'])
    cp=str(work)+os.pathsep+str(emulator)
    for harness in ['BehaviorOracle','ReferenceHarness']:
        with (root/'target'/f'{harness}.log').open('w') as log:
            run(['java','-Djava.awt.headless=true','-Dfile.encoding=ISO_8859_1','-cp',cp,harness,root/'TheLegendOfZeldaMobile-beta4.jar',root/'target'],cwd=work,stdout=log,stderr=subprocess.STDOUT)
    native=json.loads((root/'target/native-behavior.json').read_text())
    original=json.loads((root/'target/reference-behavior.json').read_text())
    if native!=original:raise SystemExit('FAIL: native behavior differs from the original JAR; inspect target/*-behavior.json')
    print('PASS: seven behavior snapshots and all original save bytes match.')
    print('\n'.join(line for line in (root/'target/ReferenceHarness.log').read_text(errors='replace').splitlines() if line.startswith(('REFERENCE state','OPENING PIXEL'))))
else:print('Java comparison skipped: Java/Javac are unavailable. All native checks ran.')
print('Validation complete.')
