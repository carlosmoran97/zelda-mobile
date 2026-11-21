#!/usr/bin/env python3
"""Ahead-of-time translation of this game's class files into ordinary Rust.

No class files, bytecode dispatcher, JVM, or Java installation are used by the
resulting app. Each original method becomes a Rust function and each control-flow
block becomes a Rust match arm. The small host implements only the services used
by this game. CFR's readable output is retained separately for review.
"""
from pathlib import Path
import json
import struct
import zipfile
from classfile import ClassFile, instructions, signature

ROOT = Path(__file__).resolve().parent.parent
with zipfile.ZipFile(ROOT / 'TheLegendOfZeldaMobile-beta4.jar') as z:
    classes = {c.name: c for n in z.namelist() if n.endswith('.class') for c in [ClassFile(z.read(n))]}

methods, fields = [], []

def quoted(s): return json.dumps(s, ensure_ascii=False)

def canonical(key, is_field=False):
    c, name, desc = key
    while c in classes:
        members = classes[c].fields if is_field else classes[c].methods
        if any(m['name'] == name and m['desc'] == desc for m in members):
            return c, name, desc
        c = classes[c].parent
    return c, name, desc

def register(table, key):
    if key not in table: table.append(key)
    return table.index(key)

def mid(key): return register(methods, canonical(key))
def fid(key): return register(fields, canonical(key, True))

for c in classes.values():
    for m in c.methods: mid((c.name, m['name'], m['desc']))
    for f in c.fields: fid((c.name, f['name'], f['desc']))
    for i, item in enumerate(c.cp):
        if item and item[0] in (10, 11): mid(c.ref(i))
        if item and item[0] == 9: fid(c.ref(i))

out = ROOT / 'src/generated'
out.mkdir(parents=True, exist_ok=True)
stats = []

def translate(c, m):
    idx = mid((c.name, m['name'], m['desc']))
    ins = list(instructions(m['code']))
    starts = {0}
    for p, op, a, end in ins:
        if 153 <= op <= 168 or op in (198, 199, 200):
            off = a[0] if op == 200 else struct.unpack('>h', struct.pack('>H', a[0]))[0]
            starts.update([p+off, end])
        if op in (170, 171): starts.update([p+a[0], end, *(p+v for _,v in a[1])])
        if op in (172,173,174,175,176,177,191): starts.add(end)
    for start,end,handler,_ in m['exceptions']: starts.update([start,end,handler])
    starts.discard(len(m['code']))
    blocks = {}
    for inst in ins:
        if inst[0] in starts: block = inst[0]; blocks[block] = []
        blocks[block].append(inst)
    args,_ = signature(m['desc'])
    lines = [f'pub fn m{idx}(r: &mut Runtime, a: &[V]) -> JResult {{',
             f'let mut l = [V::I(0); {max(1,m["locals"])}];',
             f'let mut s: Vec<V> = Vec::with_capacity({m["stack"]});']
    slot = 0
    if not m['access'] & 8: lines.append('l[0] = a[0];'); slot = 1
    for i, d in enumerate(args):
        lines.append(f'l[{slot}] = a[{i + (0 if m["access"] & 8 else 1)}];')
        slot += 2 if d in ('J','D') else 1
    lines += ['let mut pc: u32 = 0;', 'loop {', 'let mut fault = pc;',
              'let step: Result<Option<V>, u32> = (|| { match pc {']
    for bp, seq in blocks.items():
        lines.append(f'{bp} => {{')
        terminal = False
        for p,op,a,end in seq:
            code = []
            if op == 0: pass
            elif op == 1: code = ['s.push(V::R(0));']
            elif 2 <= op <= 8: code = [f's.push(V::I({op-3}));']
            elif op in (9,10): code=[f's.push(V::J({op-9}));']
            elif 11 <= op <= 13: code=[f's.push(V::F({op-11}.0));']
            elif op in (16,17):
                n= a[0]
                if op == 16 and n>=128:n-=256
                if op == 17 and n>=32768:n-=65536
                code=[f's.push(V::I({n}));']
            elif op in (18,19,20):
                t,v=c.cp[a[0]]
                if t==8:code=[f'let v=r.intern({quoted(c.utf(v))});s.push(V::R(v));']
                elif t==3:code=[f's.push(V::I({v}));']
                elif t==4:code=[f's.push(V::F(f32::from_bits({struct.unpack(">I",struct.pack(">f",v))[0]})));']
                elif t==5:code=[f's.push(V::J({v}));']
                else:raise ValueError((t,v))
            elif 21<=op<=25:code=[f's.push(l[{a[0]}]);']
            elif 26<=op<=45:code=[f's.push(l[{(op-26)%4}]);']
            elif 46<=op<=53:code=['let i=pop(&mut s).i();let o=pop(&mut s).r();s.push(r.array_get(o,i)?);']
            elif 54<=op<=58:code=[f'l[{a[0]}]=pop(&mut s);']
            elif 59<=op<=78:code=[f'l[{(op-59)%4}]=pop(&mut s);']
            elif 79<=op<=86:code=[f'let v=pop(&mut s);let i=pop(&mut s).i();let o=pop(&mut s).r();r.array_set(o,i,v)?;']
            elif op==87:code=['pop(&mut s);']
            elif op==88:code=['let v=pop(&mut s);if !v.wide(){pop(&mut s);}']
            elif op==89:code=['let v=*s.last().expect("dup");s.push(v);']
            elif op==92:code=['let v=pop(&mut s);if v.wide(){s.push(v);s.push(v);}else{let u=pop(&mut s);s.extend_from_slice(&[u,v,u,v]);}']
            elif 96<=op<=115:
                ty=['I','J','F','D'][(op-96)%4];fn={'I':'i','J':'j','F':'f','D':'d'}[ty]
                group=(op-96)//4
                if group in [0,1,2]:
                    name=['add','sub','mul'][group]
                    expr=f'u.wrapping_{name}(v)' if ty in ['I','J'] else f'u{["+","-","*"][group]}v'
                elif group==3:expr='u.wrapping_div(v)' if ty in ['I','J'] else 'u/v'
                else:expr='u.wrapping_rem(v)' if ty in ['I','J'] else 'u%v'
                code=[f'let v=pop(&mut s).{fn}();let u=pop(&mut s).{fn}();']
                if group>=3 and ty in ['I','J']:code+=['if v==0{return Err(r.exception("java/lang/ArithmeticException","division by zero"));}']
                code+=[f's.push(V::{ty}({expr}));']
            elif 116<=op<=119:
                ty=['I','J','F','D'][op-116];fn=ty.lower()
                code=[f'let v=pop(&mut s).{fn}();s.push(V::{ty}('+('v.wrapping_neg()' if op<118 else '-v')+'));']
            elif 120<=op<=131:
                ty='I' if op%2==0 else 'J';fn=ty.lower();group=(op-120)//2
                expr=[f'u.wrapping_shl(v as u32)',f'u.wrapping_shr(v as u32)',f'((u as u{32 if ty=="I" else 64}).wrapping_shr(v as u32)) as i{32 if ty=="I" else 64}','u&v','u|v','u^v'][group]
                vf='i' if group<3 else fn
                code=[f'let v=pop(&mut s).{vf}();let u=pop(&mut s).{fn}();s.push(V::{ty}({expr}));']
            elif op==132:code=[f'l[{a[0]}]=V::I(l[{a[0]}].i().wrapping_add({a[1]}));']
            elif op in (133,134,135,136,137,138,139,140,141,142,143,144):
                src,dst={133:('i','J'),134:('i','F'),135:('i','D'),136:('j','I'),137:('j','F'),138:('j','D'),139:('f','I'),140:('f','J'),141:('f','D'),142:('d','I'),143:('d','J'),144:('d','F')}[op]
                cast={'I':'i32','J':'i64','F':'f32','D':'f64'}[dst]
                code=[f'let v=pop(&mut s).{src}();s.push(V::{dst}(v as {cast}));']
            elif op in (145,146,147):code=[f'let v=pop(&mut s).i();s.push(V::I(v as { {145:"i8",146:"u16",147:"i16"}[op]} as i32));']
            elif op in (148,149,150,151,152):
                fn={148:'j',149:'f',150:'f',151:'d',152:'d'}[op]
                nan=1 if op in (150,152) else -1
                code=[f'let v=pop(&mut s).{fn}();let u=pop(&mut s).{fn}();s.push(V::I(if u>v{{1}}else if u==v{{0}}else if u<v{{-1}}else{{{nan}}}));']
            elif 153<=op<=166 or op in (198,199):
                off=struct.unpack('>h',struct.pack('>H',a[0]))[0]
                if op<=158:
                    expr=['v==0','v!=0','v<0','v>=0','v>0','v<=0'][op-153];code=['let v=pop(&mut s).i();']
                elif op<=164:
                    expr=['u==v','u!=v','u<v','u>=v','u>v','u<=v'][op-159];code=['let v=pop(&mut s).i();let u=pop(&mut s).i();']
                elif op in (165,166):expr='u==v' if op==165 else 'u!=v';code=['let v=pop(&mut s).r();let u=pop(&mut s).r();']
                else:expr='v==0' if op==198 else 'v!=0';code=['let v=pop(&mut s).r();']
                code += [f'pc=if {expr} {{{p+off}}}else{{{end}}};']
                terminal=True
            elif op in (167,200):
                off=a[0] if op==200 else struct.unpack('>h',struct.pack('>H',a[0]))[0]
                code=[f'pc={p+off};'];terminal=True
            elif op in (170,171):
                arms=''.join(f'{n}=>{p+v},' for n,v in a[1])
                code=[f'pc=match pop(&mut s).i(){{{arms}_=>{p+a[0]}}};'];terminal=True
            elif 172<=op<=176:code=['return Ok(Some(pop(&mut s)));'];terminal=True
            elif op==177:code=['return Ok(Some(V::I(0)));'];terminal=True
            elif op==178:code=[f'let v=r.static_get({fid(c.ref(a[0]))});s.push(v);']
            elif op==179:code=[f'let v=pop(&mut s);r.statics.insert({fid(c.ref(a[0]))},v);']
            elif op==180:code=[f'let o=pop(&mut s).r();s.push(r.get(o,{fid(c.ref(a[0]))})?);']
            elif op==181:code=[f'let v=pop(&mut s);let o=pop(&mut s).r();r.set(o,{fid(c.ref(a[0]))},v)?;']
            elif 182<=op<=185:
                key=c.ref(a[0]);midx=mid(key);params,ret=signature(key[2])
                count=len(params)+(op!=184)
                code=[f'let a=take_args(&mut s,{count});let v=r.invoke({midx},&a,{str(op in (182,185)).lower()})?;']
                if ret!='V':code+=['s.push(v);']
            elif op==187:code=[f'let o=r.alloc({quoted(c.cls(a[0]))},Native::Empty);s.push(V::R(o));']
            elif op==188:code=[f'let n=pop(&mut s).i();let o=r.new_array({a[0]},n)?;s.push(V::R(o));']
            elif op==189:code=['let n=pop(&mut s).i();let o=r.new_array(0,n)?;s.push(V::R(o));']
            elif op==190:code=['let o=pop(&mut s).r();s.push(V::I(r.array_len(o)?));']
            elif op==191:code=['return Err(pop(&mut s).r());'];terminal=True
            elif op==192:
                code=[f'let o=s.last().unwrap().r();if o!=0 && !r.is_instance(o,{quoted(c.cls(a[0]))}){{return Err(r.exception("java/lang/ClassCastException","invalid cast"));}}']
            else:raise ValueError(f'Unsupported opcode {op} in {c.name}.{m["name"]} at {p}')
            if any('?' in t or 'return Err' in t for t in code):lines.append(f'fault={p};')
            lines.extend(code)
        if not terminal:lines.append(f'pc={seq[-1][3]};')
        lines.append('Ok(None) },')
    lines += ['_=>panic!("invalid native control-flow block: {}",pc),', '}})();',
              'match step { Ok(Some(v))=>return Ok(v),Ok(None)=>{},Err(e)=>{']
    if m['exceptions']:
        for i,(start,end,h,catch) in enumerate(m['exceptions']):
            cond='true' if not catch else f'r.is_instance(e,{quoted(c.cls(catch))})'
            lines.append(('if' if not i else 'else if')+f' fault>={start} && fault<{end} && {cond} {{s.clear();s.push(V::R(e));pc={h};}}')
        lines.append('else {r.trace_error('+str(idx)+',fault,e);return Err(e);}')
    else:lines.append(f'r.trace_error({idx},fault,e);return Err(e);')
    lines += ['}}','}','}']
    stats.append(dict(class_name=c.name,method=m['name'],descriptor=m['desc'],id=idx,instructions=len(ins),blocks=len(blocks)))
    return '\n'.join(lines)

for c in classes.values():
    text=['// Generated by scripts/translate.py. Do not hand-edit.\n#![allow(unused,unused_assignments,unused_comparisons)]\nuse crate::runtime::*;']
    text += [translate(c,m) for m in c.methods if 'code' in m]
    (out / (c.name+'.rs')).write_text('\n\n'.join(text)+'\n')

top=['// Generated by scripts/translate.py.\nuse crate::runtime::*;']
top += [f'#[allow(non_snake_case)]\n#[path="generated/{c}.rs"] mod {c};' for c in classes]
top += ['pub static METHODS: &[(&str,&str,&str)] = &[']
top += ['('+','.join(map(quoted,key))+'),' for key in methods]
top += ['];','pub static FIELDS: &[(&str,&str,&str)] = &[']
top += ['('+','.join(map(quoted,key))+'),' for key in fields]
top += ['];', 'pub fn parent(class:&str)->&str {match class {']
top += [f'{quoted(c.name)}=>{quoted(c.parent)},' for c in classes.values()]
top += ['_=>"",}}', 'pub fn dispatch(r:&mut Runtime,id:usize,a:&[V])->JResult {match id {']
for c in classes.values():
    for m in c.methods:
        if 'code' in m:
            idx=mid((c.name,m['name'],m['desc']))
            top.append(f'{idx}=>{c.name}::m{idx}(r,a),')
top+=['_=>r.native(id,a),}}']
(ROOT/'src/generated.rs').write_text('\n'.join(top)+'\n')
(ROOT/'reference/translation.json').write_text(json.dumps(dict(jar_sha256=__import__('hashlib').sha256((ROOT/'TheLegendOfZeldaMobile-beta4.jar').read_bytes()).hexdigest(),methods=stats),indent=2)+'\n')
print(f'Translated {len(stats)} methods, {sum(s["instructions"] for s in stats)} instructions into {sum(s["blocks"] for s in stats)} Rust control-flow blocks.')
