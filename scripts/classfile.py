"""Small, strict class-file reader used by the reproducible Rust AOT translator."""
import struct


class Reader:
    def __init__(self, data):
        self.data, self.pos = data, 0

    def read(self, n):
        v = self.data[self.pos:self.pos+n]
        if len(v) != n:
            raise ValueError('Truncated class file')
        self.pos += n
        return v

    def u1(self): return self.read(1)[0]
    def u2(self): return struct.unpack('>H', self.read(2))[0]
    def u4(self): return struct.unpack('>I', self.read(4))[0]


class ClassFile:
    def __init__(self, data):
        r = Reader(data)
        assert r.u4() == 0xcafebabe
        self.minor, self.major = r.u2(), r.u2()
        self.cp = [None]
        count = r.u2()
        while len(self.cp) < count:
            t = r.u1()
            if t == 1: v = r.read(r.u2()).decode('utf8', errors='replace')
            elif t == 3: v = struct.unpack('>i', r.read(4))[0]
            elif t == 4: v = struct.unpack('>f', r.read(4))[0]
            elif t == 5: v = struct.unpack('>q', r.read(8))[0]
            elif t == 6: v = struct.unpack('>d', r.read(8))[0]
            elif t in (7, 8): v = r.u2()
            elif t in (9, 10, 11, 12): v = (r.u2(), r.u2())
            else: raise ValueError(f'Unsupported constant pool tag {t}')
            self.cp.append((t, v))
            if t in (5, 6): self.cp.append(None)
        self.access = r.u2()
        self.name, self.parent = self.cls(r.u2()), self.cls(r.u2())
        self.interfaces = [self.cls(r.u2()) for _ in range(r.u2())]
        self.fields = self.members(r)
        self.methods = self.members(r)
        self.attributes(r)
        assert r.pos == len(data)

    def utf(self, i): return self.cp[i][1]
    def cls(self, i): return self.utf(self.cp[i][1]) if i else ''

    def ref(self, i):
        c, n = self.cp[i][1]
        n, d = self.cp[n][1]
        return self.cls(c), self.utf(n), self.utf(d)

    def attributes(self, r):
        return {self.utf(r.u2()): r.read(r.u4()) for _ in range(r.u2())}

    def members(self, r):
        result = []
        for _ in range(r.u2()):
            access, name, desc = r.u2(), self.utf(r.u2()), self.utf(r.u2())
            attrs = self.attributes(r)
            item = dict(access=access, name=name, desc=desc)
            if 'Code' in attrs:
                cr = Reader(attrs['Code'])
                item.update(stack=cr.u2(), locals=cr.u2(), code=cr.read(cr.u4()))
                item['exceptions'] = [tuple(cr.u2() for _ in range(4)) for _ in range(cr.u2())]
            result.append(item)
        return result


def signature(desc):
    """Return JVM parameter descriptors and return descriptor."""
    p, args = 1, []
    while desc[p] != ')':
        start = p
        while desc[p] == '[': p += 1
        if desc[p] == 'L': p = desc.index(';', p) + 1
        else: p += 1
        args.append(desc[start:p])
    return args, desc[p+1:]


def instructions(code):
    p = 0
    one = set([16, 18, *range(21, 26), *range(54, 59), 169, 188])
    two = set([17, 19, 20, 132, *range(153, 169), *range(178, 185), 187, 189, 192, 193, 198, 199])
    while p < len(code):
        start, op = p, code[p]
        p += 1
        args = []
        if op in one:
            args = [code[p]]; p += 1
        elif op in two:
            if op == 132: args = [code[p], struct.unpack('b', code[p+1:p+2])[0]]
            else: args = [struct.unpack_from('>H', code, p)[0]]
            p += 2
        elif op == 185:
            args = [struct.unpack_from('>H', code, p)[0]]; p += 4
        elif op == 196:
            sub = code[p]; p += 1
            args = [sub, struct.unpack_from('>H', code, p)[0]]; p += 2
            if sub == 132:
                args.append(struct.unpack_from('>h', code, p)[0]); p += 2
        elif op == 197:
            args = [struct.unpack_from('>H', code, p)[0], code[p+2]]; p += 3
        elif op in (200, 201):
            args = [struct.unpack_from('>i', code, p)[0]]; p += 4
        elif op in (170, 171):
            p = (p + 3) & ~3
            default = struct.unpack_from('>i', code, p)[0]; p += 4
            if op == 170:
                lo, hi = struct.unpack_from('>ii', code, p); p += 8
                pairs = [(n, struct.unpack_from('>i', code, p+4*(n-lo))[0]) for n in range(lo, hi+1)]
                p += 4 * (hi-lo+1)
            else:
                n = struct.unpack_from('>i', code, p)[0]; p += 4
                pairs = [struct.unpack_from('>ii', code, p+8*i) for i in range(n)]
                p += 8*n
            args = [default, pairs]
        yield start, op, args, p
