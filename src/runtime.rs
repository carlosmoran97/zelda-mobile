//! Native object storage and platform services for the Rust game.
//! Legacy signatures are isolated in `game::compat` for reference tooling.
use crate::{
    game,
    graphics::{self, Bitmap, Fonts, Graphics, Surface},
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    path::PathBuf,
    rc::Rc,
};

#[derive(Clone, Copy, Debug)]
pub enum V {
    I(i32),
    J(i64),
    F(f32),
    D(f64),
    R(u32),
}
impl V {
    pub fn i(self) -> i32 {
        match self {
            Self::I(v) => v,
            Self::R(0) => 0,
            _ => panic!("expected int, got {self:?}"),
        }
    }
    pub fn j(self) -> i64 {
        match self {
            Self::J(v) => v,
            _ => self.i() as i64,
        }
    }
    pub fn f(self) -> f32 {
        match self {
            Self::F(v) => v,
            _ => panic!("expected float {self:?}"),
        }
    }
    pub fn d(self) -> f64 {
        match self {
            Self::D(v) => v,
            _ => panic!("expected double {self:?}"),
        }
    }
    pub fn r(self) -> u32 {
        match self {
            Self::R(v) => v,
            Self::I(0) => 0,
            _ => panic!("expected reference, got {self:?}"),
        }
    }
}
pub type JResult = Result<V, u32>;

#[derive(Clone)]
pub struct Stream {
    pub bytes: Vec<u8>,
    pub pos: usize,
    pub name: String,
}
pub enum Native {
    Empty,
    Text(String),
    Array(u8, Vec<V>),
    Vector(Vec<V>),
    Random(u64),
    Image(Surface),
    Graphics(Graphics),
    Font {
        size: u8,
        bold: bool,
    },
    Stream(Rc<RefCell<Stream>>),
    Command {
        label: String,
        priority: i32,
    },
    Thread(u32),
    Record {
        name: String,
        data: Option<Vec<u8>>,
    },
    Player {
        name: String,
        state: i32,
        position: i64,
        loops: i32,
        volume: i32,
    },
    Volume(u32),
}
pub struct Object {
    pub class: &'static str,
    pub fields: HashMap<usize, V>,
    pub native: Native,
}
#[derive(Clone, Copy)]
pub struct Timer {
    pub task: u32,
    pub period: u64,
    pub next: u64,
}

pub struct Runtime {
    pub heap: Vec<Option<Object>>,
    free: Vec<u32>,
    pub statics: HashMap<usize, V>,
    interned: HashMap<String, u32>,
    image_cache: HashMap<String, Surface>,
    pub assets: PathBuf,
    pub saves: PathBuf,
    pub resources: Rc<dyn crate::assets::Assets>,
    pub records: Rc<dyn crate::storage::RecordStore>,
    pub fonts: Fonts,
    pub default_font: u32,
    pub timers: Vec<Timer>,
    pub threads: Vec<u32>,
    pub commands: Vec<u32>,
    pub display: u32,
    pub now: u64,
    pub quit: bool,
    pub roots: Vec<u32>,
    pub audio: Vec<crate::audio::AudioCommand>,
    pub errors: Vec<String>,
    pub verbose: bool,
    pub repaint: bool,
    seed: u64,
}
impl Runtime {
    pub fn new(assets: PathBuf, saves: PathBuf) -> Result<Self, String> {
        let mut runtime = Self::with_services(
            Rc::new(crate::assets::FileAssets(assets.clone())),
            Rc::new(crate::storage::FileRecords(saves.clone())),
        )?;
        runtime.assets = assets;
        runtime.saves = saves;
        Ok(runtime)
    }
    pub fn with_services(
        resources: Rc<dyn crate::assets::Assets>,
        records: Rc<dyn crate::storage::RecordStore>,
    ) -> Result<Self, String> {
        let fonts = Fonts::from_assets(resources.as_ref())?;
        let mut r = Self {
            heap: vec![None],
            free: Vec::new(),
            statics: HashMap::new(),
            interned: HashMap::new(),
            image_cache: HashMap::new(),
            assets: PathBuf::new(),
            saves: PathBuf::new(),
            resources,
            records,
            fonts,
            default_font: 0,
            timers: Vec::new(),
            threads: Vec::new(),
            commands: Vec::new(),
            display: 0,
            now: 0,
            quit: false,
            roots: Vec::new(),
            audio: Vec::new(),
            errors: Vec::new(),
            verbose: false,
            repaint: true,
            seed: 0x5a454c4441,
        };
        r.default_font = r.alloc(
            "javax/microedition/lcdui/Font",
            Native::Font {
                size: 15,
                bold: false,
            },
        );
        Ok(r)
    }
    pub fn alloc(&mut self, class: &'static str, native: Native) -> u32 {
        let o = Some(Object {
            class,
            fields: HashMap::new(),
            native,
        });
        if let Some(id) = self.free.pop() {
            self.heap[id as usize] = o;
            id
        } else {
            let id = self.heap.len() as u32;
            self.heap.push(o);
            id
        }
    }
    pub fn obj(&self, id: u32) -> &Object {
        self.heap
            .get(id as usize)
            .and_then(Option::as_ref)
            .unwrap_or_else(|| panic!("invalid object {id}"))
    }
    pub fn obj_mut(&mut self, id: u32) -> &mut Object {
        self.heap
            .get_mut(id as usize)
            .and_then(Option::as_mut)
            .unwrap_or_else(|| panic!("invalid object {id}"))
    }
    pub fn intern(&mut self, text: &str) -> u32 {
        if let Some(id) = self.interned.get(text) {
            *id
        } else {
            let id = self.text(text.to_owned());
            self.interned.insert(text.to_owned(), id);
            id
        }
    }
    pub fn text(&mut self, text: String) -> u32 {
        self.alloc("java/lang/String", Native::Text(text))
    }
    pub fn string(&self, id: u32) -> String {
        if id == 0 {
            return "null".into();
        }
        match &self.obj(id).native {
            Native::Text(s) => s.clone(),
            _ => format!("{}@{id}", self.obj(id).class),
        }
    }
    pub fn exception(&mut self, class: &'static str, message: &str) -> u32 {
        self.alloc(class, Native::Text(message.to_owned()))
    }
    pub fn null(&mut self) -> u32 {
        self.exception("java/lang/NullPointerException", "null reference")
    }
    pub fn trace_error(&mut self, method: usize, pc: u32, e: u32) {
        let m = game::METHODS[method];
        let msg = format!(
            "{}.{}{} at {pc}: {}: {}",
            m.0,
            m.1,
            m.2,
            self.obj(e).class,
            self.string(e)
        );
        if self.verbose {
            eprintln!("{msg}");
        }
        self.errors.push(msg);
        if self.errors.len() > 256 {
            self.errors.remove(0);
        }
    }
    pub fn get(&mut self, o: u32, f: usize) -> JResult {
        if o == 0 {
            return Err(self.null());
        }
        Ok(*self
            .obj(o)
            .fields
            .get(&f)
            .unwrap_or(&default_value(game::FIELDS[f].2)))
    }
    pub fn set(&mut self, o: u32, f: usize, v: V) -> JResult {
        if o == 0 {
            return Err(self.null());
        }
        self.obj_mut(o).fields.insert(f, v);
        Ok(V::I(0))
    }
    pub fn field_id(class: &str, name: &str, desc: &str) -> usize {
        let mut class = class;
        loop {
            if let Some(id) = game::FIELDS
                .iter()
                .position(|x| x.0 == class && x.1 == name && x.2 == desc)
            {
                return id;
            }
            class = game::parent(class);
            assert!(!class.is_empty(), "unknown field {name}{desc}");
        }
    }
    pub fn field(&mut self, o: u32, name: &str, desc: &str) -> V {
        let id = Self::field_id(self.obj(o).class, name, desc);
        self.get(o, id).unwrap()
    }
    pub fn put(&mut self, o: u32, name: &str, desc: &str, v: V) {
        let id = Self::field_id(self.obj(o).class, name, desc);
        self.set(o, id, v).unwrap();
    }
    pub fn static_get(&mut self, f: usize) -> V {
        if let Some(v) = self.statics.get(&f) {
            return *v;
        }
        let value = if game::FIELDS[f].0 == "java/lang/System" {
            V::R(self.alloc("java/io/PrintStream", Native::Empty))
        } else {
            default_value(game::FIELDS[f].2)
        };
        self.statics.insert(f, value);
        value
    }
    pub fn method_id(class: &str, name: &str, desc: &str) -> usize {
        let mut class = class;
        loop {
            if let Some(id) = game::METHODS
                .iter()
                .position(|x| x.0 == class && x.1 == name && x.2 == desc)
            {
                return id;
            }
            class = game::parent(class);
            assert!(!class.is_empty(), "unknown method {name}{desc}");
        }
    }
    pub fn call(&mut self, o: u32, name: &str, desc: &str, args: &[V]) -> JResult {
        let id = Self::method_id(self.obj(o).class, name, desc);
        let mut a = vec![V::R(o)];
        a.extend_from_slice(args);
        self.invoke(id, &a, false)
    }
    pub fn invoke(&mut self, mut id: usize, a: &[V], virtual_call: bool) -> JResult {
        if virtual_call {
            let o = a[0].r();
            if o == 0 {
                return Err(self.null());
            }
            let (_, name, desc) = game::METHODS[id];
            let mut class = self.obj(o).class;
            while !class.is_empty() {
                if let Some(i) = game::METHODS
                    .iter()
                    .position(|x| x.0 == class && x.1 == name && x.2 == desc)
                {
                    id = i;
                    break;
                }
                class = game::parent(class);
            }
        }
        game::dispatch(self, id, a)
    }
    pub fn is_instance(&self, o: u32, want: &str) -> bool {
        if o == 0 {
            return false;
        }
        if want == "java/lang/Object" {
            return true;
        }
        let mut class = self.obj(o).class;
        while !class.is_empty() {
            if class == want {
                return true;
            }
            class = game::parent(class);
        }
        let class = self.obj(o).class;
        if want.starts_with('[') {
            return class.starts_with('[');
        }
        if want == "java/lang/Exception" {
            return class.ends_with("Exception");
        }
        if want == "java/io/IOException" {
            return matches!(class, "java/io/IOException" | "java/io/EOFException");
        }
        if want == "java/lang/RuntimeException" {
            return matches!(
                class,
                "java/lang/NullPointerException"
                    | "java/lang/IndexOutOfBoundsException"
                    | "java/lang/ArrayIndexOutOfBoundsException"
                    | "java/lang/ArithmeticException"
                    | "java/lang/IllegalArgumentException"
                    | "java/lang/ClassCastException"
            );
        }
        if want == "javax/microedition/rms/RecordStoreException" {
            return class.contains("RecordStore") && class.ends_with("Exception");
        }
        if want == "java/io/InputStream" {
            return matches!(self.obj(o).native, Native::Stream(_));
        }
        if want == "javax/microedition/media/control/VolumeControl" {
            return matches!(self.obj(o).native, Native::Volume(_));
        }
        false
    }
    pub fn new_array(&mut self, kind: u8, n: i32) -> Result<u32, u32> {
        if !(0..=16_000_000).contains(&n) {
            return Err(
                self.exception("java/lang/NegativeArraySizeException", "invalid array size")
            );
        }
        let v = match kind {
            0 => V::R(0),
            6 => V::F(0.0),
            7 => V::D(0.0),
            11 => V::J(0),
            _ => V::I(0),
        };
        Ok(self.alloc(
            if kind == 0 {
                "[Ljava/lang/Object;"
            } else {
                "[I"
            },
            Native::Array(kind, vec![v; n as usize]),
        ))
    }
    pub fn array_len(&mut self, o: u32) -> Result<i32, u32> {
        if o == 0 {
            return Err(self.null());
        }
        if let Native::Array(_, v) = &self.obj(o).native {
            Ok(v.len() as i32)
        } else {
            panic!("not an array")
        }
    }
    pub fn array_get(&mut self, o: u32, i: i32) -> JResult {
        if o == 0 {
            return Err(self.null());
        }
        if let Native::Array(_, v) = &self.obj(o).native {
            if let Some(v) = v.get(i as usize) {
                return Ok(*v);
            }
        } else {
            panic!("not an array");
        }
        Err(self.exception("java/lang/ArrayIndexOutOfBoundsException", &i.to_string()))
    }
    pub fn array_set(&mut self, o: u32, i: i32, mut value: V) -> JResult {
        if o == 0 {
            return Err(self.null());
        }
        if let Native::Array(kind, v) = &mut self.obj_mut(o).native {
            if let Some(slot) = v.get_mut(i as usize) {
                value = match *kind {
                    4 => V::I(value.i() & 1),
                    8 => V::I(value.i() as i8 as i32),
                    5 => V::I(value.i() as u16 as i32),
                    9 => V::I(value.i() as i16 as i32),
                    _ => value,
                };
                *slot = value;
                return Ok(V::I(0));
            }
        } else {
            panic!("not an array");
        }
        Err(self.exception("java/lang/ArrayIndexOutOfBoundsException", &i.to_string()))
    }
    pub fn bytes(&self, o: u32) -> Vec<u8> {
        if let Native::Array(_, a) = &self.obj(o).native {
            a.iter().map(|v| v.i() as u8).collect()
        } else {
            panic!("not byte array")
        }
    }
    pub fn byte_array(&mut self, b: &[u8]) -> u32 {
        self.alloc(
            "[I",
            Native::Array(8, b.iter().map(|b| V::I(*b as i8 as i32)).collect()),
        )
    }
    fn stream(&self, o: u32) -> Rc<RefCell<Stream>> {
        if let Native::Stream(s) = &self.obj(o).native {
            s.clone()
        } else {
            panic!("not a stream: {}", self.obj(o).class)
        }
    }
    fn read_bytes(&mut self, o: u32, n: usize) -> Result<Vec<u8>, u32> {
        let s = self.stream(o);
        let mut s = s.borrow_mut();
        if s.pos + n > s.bytes.len() {
            return Err(self.exception("java/io/EOFException", "end of input"));
        }
        let out = s.bytes[s.pos..s.pos + n].to_vec();
        s.pos += n;
        Ok(out)
    }
    pub fn surface(&self, o: u32) -> Surface {
        if let Native::Image(s) = &self.obj(o).native {
            s.clone()
        } else {
            panic!("not an image")
        }
    }
    pub fn with_graphics<T>(&mut self, o: u32, f: impl FnOnce(&mut Graphics, &mut Self) -> T) -> T {
        let native = std::mem::replace(&mut self.obj_mut(o).native, Native::Empty);
        if let Native::Graphics(mut g) = native {
            let value = f(&mut g, self);
            self.obj_mut(o).native = Native::Graphics(g);
            value
        } else {
            panic!("not graphics")
        }
    }
    pub fn font_info(&self, o: u32) -> (u8, bool) {
        if o == 0 {
            return (15, false);
        }
        if let Native::Font { size, bold } = &self.obj(o).native {
            (*size, *bold)
        } else {
            (15, false)
        }
    }
    pub fn load_image(&mut self, name: &str) -> Result<u32, u32> {
        let name = name.trim_start_matches('/');
        let image = if let Some(s) = self.image_cache.get(name) {
            s.clone()
        } else {
            let b = self
                .resources
                .read(name)
                .and_then(|bytes| Bitmap::decode(&bytes))
                .map_err(|e| self.exception("java/io/IOException", &e))?;
            let s = Rc::new(RefCell::new(b));
            self.image_cache.insert(name.to_owned(), s.clone());
            s
        };
        Ok(self.alloc("javax/microedition/lcdui/Image", Native::Image(image)))
    }
    pub fn collect(&mut self) {
        let mut work = self.roots.clone();
        work.push(self.default_font);
        work.push(self.display);
        work.extend(self.interned.values().copied());
        work.extend(self.commands.iter().copied());
        work.extend(self.threads.iter().copied());
        work.extend(self.timers.iter().map(|t| t.task));
        work.extend(
            self.statics
                .values()
                .filter_map(|v| if let V::R(id) = v { Some(*id) } else { None }),
        );
        let mut seen = HashSet::new();
        while let Some(id) = work.pop() {
            if id == 0 || !seen.insert(id) {
                continue;
            }
            let o = self.obj(id);
            work.extend(
                o.fields
                    .values()
                    .filter_map(|v| if let V::R(id) = v { Some(*id) } else { None }),
            );
            match &o.native {
                Native::Array(_, a) | Native::Vector(a) => work.extend(
                    a.iter()
                        .filter_map(|v| if let V::R(id) = v { Some(*id) } else { None }),
                ),
                Native::Thread(id) | Native::Volume(id) => work.push(*id),
                Native::Graphics(g) => work.push(g.font),
                _ => {}
            }
        }
        for id in 1..self.heap.len() {
            if self.heap[id].is_some() && !seen.contains(&(id as u32)) {
                self.heap[id] = None;
                self.free.push(id as u32);
            }
        }
    }
    pub fn native(&mut self, id: usize, a: &[V]) -> JResult {
        let (class, name, desc) = game::METHODS[id];
        let recv = || a[0].r();
        let int = |i: usize| a[i].i();
        match (class, name, desc) {
            (
                "java/lang/Object"
                | "javax/microedition/midlet/MIDlet"
                | "javax/microedition/lcdui/Canvas"
                | "java/util/Timer"
                | "java/util/TimerTask",
                "<init>",
                _,
            ) => {}
            ("java/lang/Object", "getClass", _) => {
                let text = self.obj(recv()).class.to_owned();
                return Ok(V::R(self.alloc("java/lang/Class", Native::Text(text))));
            }
            ("java/lang/Class", "getResourceAsStream", _) => {
                let name = self.string(a[1].r()).trim_start_matches('/').to_owned();
                let Ok(bytes) = self.resources.read(&name).map(|b| b.into_owned()) else {
                    return Ok(V::R(0));
                };
                let stream = Stream {
                    bytes,
                    pos: 0,
                    name,
                };
                return Ok(V::R(self.alloc(
                    "java/io/InputStream",
                    Native::Stream(Rc::new(RefCell::new(stream))),
                )));
            }
            ("java/lang/System", "currentTimeMillis", _) => return Ok(V::J(self.now as i64)),
            ("java/lang/System", "gc", _) => {}
            ("java/lang/Math", "abs", _) => return Ok(V::I(int(0).wrapping_abs())),
            ("java/lang/Integer", "toString", _) => return Ok(V::R(self.text(int(0).to_string()))),
            ("java/lang/Runtime", "getRuntime", _) => {
                return Ok(V::R(self.alloc(class, Native::Empty)));
            }
            ("java/lang/Runtime", "totalMemory", _) => return Ok(V::J(128 * 1024 * 1024)),
            ("java/io/PrintStream", "println", _) => {
                if self.verbose {
                    println!("{}", self.string(a[1].r()));
                }
            }
            (
                "java/lang/Exception" | "javax/microedition/rms/RecordStoreException",
                "toString",
                _,
            ) => return Ok(V::R(self.text(self.string(recv())))),
            ("java/lang/String", "<init>", "([C)V") => {
                let chars = if let Native::Array(_, v) = &self.obj(a[1].r()).native {
                    v.iter().map(|v| v.i() as u16).collect::<Vec<_>>()
                } else {
                    panic!("char array")
                };
                self.obj_mut(recv()).native = Native::Text(String::from_utf16_lossy(&chars));
            }
            ("java/lang/String", "length", _) => {
                return Ok(V::I(self.string(recv()).encode_utf16().count() as i32));
            }
            ("java/lang/String", "substring", _) => {
                let text: Vec<u16> = self.string(recv()).encode_utf16().collect();
                let start = int(1) as usize;
                let end = if a.len() > 2 {
                    int(2) as usize
                } else {
                    text.len()
                };
                let Some(slice) = text.get(start..end) else {
                    return Err(self.exception("java/lang/IndexOutOfBoundsException", "substring"));
                };
                return Ok(V::R(self.text(String::from_utf16_lossy(slice))));
            }
            ("java/lang/StringBuffer", "<init>", _) => {
                self.obj_mut(recv()).native = Native::Text(String::new())
            }
            ("java/lang/StringBuffer", "append", _) => {
                let text = match desc {
                    "(I)Ljava/lang/StringBuffer;" => int(1).to_string(),
                    "(Z)Ljava/lang/StringBuffer;" => (int(1) != 0).to_string(),
                    _ => self.string(a[1].r()),
                };
                if let Native::Text(s) = &mut self.obj_mut(recv()).native {
                    s.push_str(&text);
                }
                return Ok(a[0]);
            }
            ("java/lang/StringBuffer", "toString", _) => {
                return Ok(V::R(self.text(self.string(recv()))));
            }
            ("java/util/Random", "<init>", _) => {
                self.seed = self.seed.wrapping_add(0x9e3779b97f4a7c15);
                self.obj_mut(recv()).native =
                    Native::Random((self.seed ^ 0x5deece66d) & ((1 << 48) - 1));
            }
            ("java/util/Random", "setSeed", _) => {
                self.obj_mut(recv()).native =
                    Native::Random((a[1].j() as u64 ^ 0x5deece66d) & ((1 << 48) - 1))
            }
            ("java/util/Random", "nextInt", _) => {
                if let Native::Random(seed) = &mut self.obj_mut(recv()).native {
                    let mut next = |bits: u32| {
                        *seed = (seed.wrapping_mul(0x5deece66d).wrapping_add(11)) & ((1 << 48) - 1);
                        (*seed >> (48 - bits)) as i32
                    };
                    if a.len() == 1 {
                        return Ok(V::I(next(32)));
                    }
                    let bound = int(1);
                    if bound <= 0 {
                        return Err(
                            self.exception("java/lang/IllegalArgumentException", "random bound")
                        );
                    }
                    if bound & (bound - 1) == 0 {
                        return Ok(V::I(((bound as i64 * next(31) as i64) >> 31) as i32));
                    }
                    loop {
                        let bits = next(31);
                        let value = bits % bound;
                        if bits.wrapping_sub(value).wrapping_add(bound - 1) >= 0 {
                            return Ok(V::I(value));
                        }
                    }
                }
                panic!("random state")
            }
            ("java/util/Vector", "<init>", _) => {
                self.obj_mut(recv()).native = Native::Vector(Vec::new())
            }
            ("java/util/Vector", _, _) => return self.vector(name, a),
            ("java/io/ByteArrayInputStream", "<init>", _) => {
                let bytes = self.bytes(a[1].r());
                self.obj_mut(recv()).native = Native::Stream(Rc::new(RefCell::new(Stream {
                    bytes,
                    pos: 0,
                    name: String::new(),
                })));
            }
            ("java/io/ByteArrayOutputStream", "<init>", _) => {
                self.obj_mut(recv()).native = Native::Stream(Rc::new(RefCell::new(Stream {
                    bytes: Vec::new(),
                    pos: 0,
                    name: String::new(),
                })))
            }
            ("java/io/DataInputStream" | "java/io/DataOutputStream", "<init>", _) => {
                let source = self.stream(a[1].r());
                self.obj_mut(recv()).native = Native::Stream(source);
            }
            ("java/io/ByteArrayOutputStream", "reset", _) => {
                let s = self.stream(recv());
                s.borrow_mut().bytes.clear();
                s.borrow_mut().pos = 0;
            }
            ("java/io/ByteArrayInputStream", "reset", _) => {
                self.stream(recv()).borrow_mut().pos = 0
            }
            ("java/io/ByteArrayOutputStream", "toByteArray", _) => {
                let bytes = self.stream(recv()).borrow().bytes.clone();
                return Ok(V::R(self.byte_array(&bytes)));
            }
            ("java/io/DataInputStream", "read", "()I") => {
                let s = self.stream(recv());
                let mut s = s.borrow_mut();
                if s.pos == s.bytes.len() {
                    return Ok(V::I(-1));
                }
                let v = s.bytes[s.pos];
                s.pos += 1;
                return Ok(V::I(v as i32));
            }
            ("java/io/DataInputStream", "read", "([BII)I") => {
                let stream = self.stream(recv());
                let mut stream = stream.borrow_mut();
                let n = (int(3).max(0) as usize).min(stream.bytes.len() - stream.pos);
                if n == 0 {
                    return Ok(V::I(if int(3) == 0 { 0 } else { -1 }));
                }
                for j in 0..n {
                    self.array_set(
                        a[1].r(),
                        int(2) + j as i32,
                        V::I(stream.bytes[stream.pos + j] as i8 as i32),
                    )?;
                }
                stream.pos += n;
                return Ok(V::I(n as i32));
            }
            ("java/io/DataInputStream", "readInt", _) => {
                let b = self.read_bytes(recv(), 4)?;
                return Ok(V::I(i32::from_be_bytes(b.try_into().unwrap())));
            }
            ("java/io/DataInputStream", "readByte" | "readBoolean" | "readUnsignedByte", _) => {
                let b = self.read_bytes(recv(), 1)?[0];
                return Ok(V::I(match name {
                    "readByte" => b as i8 as i32,
                    "readBoolean" => (b != 0) as i32,
                    _ => b as i32,
                }));
            }
            ("java/io/DataOutputStream", "writeInt", _) => self
                .stream(recv())
                .borrow_mut()
                .bytes
                .extend_from_slice(&int(1).to_be_bytes()),
            ("java/io/DataOutputStream", "write" | "writeBoolean", _) => self
                .stream(recv())
                .borrow_mut()
                .bytes
                .push(if name == "writeBoolean" {
                    (int(1) != 0) as u8
                } else {
                    int(1) as u8
                }),
            (
                "java/io/ByteArrayInputStream"
                | "java/io/ByteArrayOutputStream"
                | "java/io/DataInputStream"
                | "java/io/DataOutputStream",
                "close" | "flush",
                _,
            ) => {}
            ("java/lang/Thread", "<init>", _) => {
                self.obj_mut(recv()).native = Native::Thread(a[1].r())
            }
            ("java/lang/Thread", "start", _) => {
                if let Native::Thread(o) = self.obj(recv()).native {
                    self.threads.push(o);
                }
            }
            ("java/lang/Thread", "setPriority" | "sleep", _) => {}
            ("java/util/Timer", "scheduleAtFixedRate", _) => self.timers.push(Timer {
                task: a[1].r(),
                next: self.now + a[2].j() as u64,
                period: a[3].j() as u64,
            }),
            ("javax/microedition/lcdui/Display", "getDisplay", _) => {
                return Ok(V::R(self.alloc(class, Native::Empty)));
            }
            ("javax/microedition/lcdui/Display", "setCurrent", _) => self.display = a[1].r(),
            ("javax/microedition/lcdui/Canvas", _, _) => match name {
                "getWidth" => return Ok(V::I(graphics::WIDTH)),
                "getHeight" => return Ok(V::I(graphics::HEIGHT)),
                "repaint" => self.repaint = true,
                "serviceRepaints" | "setFullScreenMode" | "setCommandListener" => {}
                "hasPointerEvents" | "hasPointerMotionEvents" | "isDoubleBuffered" => {
                    return Ok(V::I(1));
                }
                "getGameAction" => {
                    return Ok(V::I(match int(1) {
                        -1 | 50 => 1,
                        -2 | 56 => 6,
                        -3 | 52 => 2,
                        -4 | 54 => 5,
                        -5 | 53 => 8,
                        _ => 0,
                    }));
                }
                "getKeyName" => {
                    return Ok(V::R(
                        self.text(char::from_u32(int(1) as u32).unwrap_or('?').to_string()),
                    ));
                }
                "addCommand" => {
                    let id = a[1].r();
                    if !self.commands.contains(&id) {
                        self.commands.push(id);
                    }
                }
                "removeCommand" => self.commands.retain(|id| *id != a[1].r()),
                _ => panic!("unimplemented Canvas method {name}{desc}"),
            },
            ("javax/microedition/lcdui/Command", "<init>", _) => {
                let label = self.string(a[1].r());
                self.obj_mut(recv()).native = Native::Command {
                    label,
                    priority: int(3),
                };
            }
            ("javax/microedition/midlet/MIDlet", "notifyDestroyed", _) => self.quit = true,
            (
                "javax/microedition/lcdui/Image",
                "createImage",
                "(II)Ljavax/microedition/lcdui/Image;",
            ) => {
                if int(0) <= 0 || int(1) <= 0 || int(0) > 4096 || int(1) > 4096 {
                    return Err(
                        self.exception("java/lang/IllegalArgumentException", "image dimensions")
                    );
                }
                return Ok(V::R(self.alloc(
                    class,
                    Native::Image(Rc::new(RefCell::new(Bitmap::new(int(0), int(1))))),
                )));
            }
            ("javax/microedition/lcdui/Image", "createImage", _) => {
                let name = self.string(a[0].r());
                return self.load_image(&name).map(V::R);
            }
            ("javax/microedition/lcdui/Image", "getWidth", _) => {
                return Ok(V::I(self.surface(recv()).borrow().w));
            }
            ("javax/microedition/lcdui/Image", "getGraphics", _) => {
                let g = Graphics::new(self.surface(recv()));
                return Ok(V::R(
                    self.alloc("javax/microedition/lcdui/Graphics", Native::Graphics(g)),
                ));
            }
            ("javax/microedition/lcdui/Font", "getFont", _) => {
                return Ok(V::R(self.alloc(
                    class,
                    Native::Font {
                        size: Fonts::size(int(2)),
                        bold: int(1) & 1 != 0,
                    },
                )));
            }
            ("javax/microedition/lcdui/Font", "getHeight", _) => {
                return Ok(V::I(self.fonts.height(self.font_info(recv()).0)));
            }
            ("javax/microedition/lcdui/Font", "stringWidth" | "substringWidth", _) => {
                let (size, bold) = self.font_info(recv());
                let text = self.string(a[1].r());
                let text = if name == "substringWidth" {
                    text.chars()
                        .skip(int(2) as usize)
                        .take(int(3) as usize)
                        .collect()
                } else {
                    text
                };
                return Ok(V::I(self.fonts.width(&text, size, bold)));
            }
            ("javax/microedition/lcdui/Graphics", _, _) => return self.draw(name, desc, a),
            ("javax/microedition/rms/RecordStore", _, _) => return self.record(name, a),
            (
                "javax/microedition/media/Manager"
                | "javax/microedition/media/Player"
                | "javax/microedition/media/control/VolumeControl",
                _,
                _,
            ) => return self.media(class, name, a),
            _ => panic!("unimplemented native service: {class}.{name}{desc}"),
        }
        Ok(V::I(0))
    }
    fn vector(&mut self, name: &str, a: &[V]) -> JResult {
        let o = a[0].r();
        let Native::Vector(v) = &mut self.obj_mut(o).native else {
            panic!("vector state")
        };
        match name {
            "size" => return Ok(V::I(v.len() as i32)),
            "addElement" => v.push(a[1]),
            "removeAllElements" => v.clear(),
            "setSize" => {
                if a[1].i() < 0 {
                    return Err(
                        self.exception("java/lang/ArrayIndexOutOfBoundsException", "vector size")
                    );
                }
                v.resize(a[1].i() as usize, V::R(0));
            }
            "firstElement" | "elementAt" => {
                let i = if name == "firstElement" {
                    0
                } else {
                    a[1].i() as usize
                };
                if let Some(x) = v.get(i) {
                    return Ok(*x);
                }
                return Err(
                    self.exception("java/lang/ArrayIndexOutOfBoundsException", "vector index")
                );
            }
            "setElementAt" => {
                if let Some(x) = v.get_mut(a[2].i() as usize) {
                    *x = a[1];
                } else {
                    return Err(
                        self.exception("java/lang/ArrayIndexOutOfBoundsException", "vector index")
                    );
                }
            }
            "insertElementAt" => {
                let i = a[2].i() as usize;
                if i > v.len() {
                    return Err(self.exception(
                        "java/lang/ArrayIndexOutOfBoundsException",
                        "vector insertion",
                    ));
                }
                v.insert(i, a[1]);
            }
            "removeElementAt" => {
                let i = a[1].i() as usize;
                if i >= v.len() {
                    return Err(self
                        .exception("java/lang/ArrayIndexOutOfBoundsException", "vector removal"));
                }
                v.remove(i);
            }
            _ => panic!("vector {name}"),
        }
        Ok(V::I(0))
    }
    fn draw(&mut self, name: &str, desc: &str, a: &[V]) -> JResult {
        let o = a[0].r();
        let v: Vec<i32> = a
            .iter()
            .skip(1)
            .map(|v| match v {
                V::I(n) => *n,
                _ => 0,
            })
            .collect();
        self.with_graphics(o, |g, r| {
            match name {
                "translate" => {
                    g.tx += v[0];
                    g.ty += v[1];
                }
                "getTranslateX" => return Ok(V::I(g.tx)),
                "getTranslateY" => return Ok(V::I(g.ty)),
                "setClip" => g.set_clip(v[0], v[1], v[2], v[3]),
                "clipRect" => g.clip_rect(v[0], v[1], v[2], v[3]),
                "getClipX" => return Ok(V::I(g.clip.x - g.tx)),
                "getClipY" => return Ok(V::I(g.clip.y - g.ty)),
                "getClipWidth" => return Ok(V::I(g.clip.w)),
                "getClipHeight" => return Ok(V::I(g.clip.h)),
                "setColor" => g.color(if desc == "(I)V" {
                    v[0]
                } else {
                    (v[0] << 16) | (v[1] << 8) | v[2]
                }),
                "fillRect" => g.fill_rect(v[0], v[1], v[2], v[3]),
                "drawRect" => g.draw_rect(v[0], v[1], v[2], v[3]),
                "drawLine" => g.draw_line(v[0], v[1], v[2], v[3]),
                "fillTriangle" => g.triangle(&v),
                "fillRoundRect" => g.round_rect(v[0], v[1], v[2], v[3], v[4], v[5]),
                "setFont" => g.font = a[1].r(),
                "getFont" => return Ok(V::R(if g.font == 0 { r.default_font } else { g.font })),
                "drawImage" => {
                    if a[1].r() == 0 {
                        return Err(r.null());
                    }
                    let src = r.surface(a[1].r());
                    let (w, h) = {
                        let b = src.borrow();
                        (b.w, b.h)
                    };
                    g.blit(
                        &src,
                        graphics::Rect { x: 0, y: 0, w, h },
                        0,
                        v[1],
                        v[2],
                        v[3],
                    )
                    .map_err(|e| r.exception("java/lang/IllegalArgumentException", &e))?;
                }
                "drawRegion" => {
                    if a[1].r() == 0 {
                        return Err(r.null());
                    }
                    let src = r.surface(a[1].r());
                    g.blit(
                        &src,
                        graphics::Rect {
                            x: v[1],
                            y: v[2],
                            w: v[3],
                            h: v[4],
                        },
                        v[5],
                        v[6],
                        v[7],
                        v[8],
                    )
                    .map_err(|e| r.exception("java/lang/IllegalArgumentException", &e))?;
                }
                "drawString" => {
                    let text = r.string(a[1].r());
                    let (size, bold) = r.font_info(g.font);
                    r.fonts.draw(g, &text, v[1], v[2], v[3], size, bold);
                }
                _ => panic!("graphics method {name}{desc}"),
            }
            Ok(V::I(0))
        })
    }
    fn record(&mut self, name: &str, a: &[V]) -> JResult {
        if name == "listRecordStores" {
            let names = self
                .records
                .list()
                .map_err(|e| self.exception("javax/microedition/rms/RecordStoreException", &e))?;
            if names.is_empty() {
                return Ok(V::R(0));
            }
            let mut vals = Vec::new();
            for n in names {
                vals.push(V::R(self.text(n)));
            }
            return Ok(V::R(
                self.alloc("[Ljava/lang/Object;", Native::Array(0, vals)),
            ));
        }
        if name == "openRecordStore" || name == "deleteRecordStore" {
            let key = self.string(a[0].r());
            if !key.chars().all(|c| c.is_ascii_alphanumeric()) {
                return Err(self.exception(
                    "javax/microedition/rms/RecordStoreException",
                    "invalid store name",
                ));
            }
            if name == "deleteRecordStore" {
                self.records.delete(&key).map_err(|e| {
                    self.exception(
                        "javax/microedition/rms/RecordStoreException",
                        &e.to_string(),
                    )
                })?;
                return Ok(V::I(0));
            }
            let data = match self.records.read(&key) {
                Ok(Some(bytes)) => Some(crate::storage::decode_record(&bytes).map_err(|e| {
                    self.exception("javax/microedition/rms/RecordStoreException", &e)
                })?),
                Ok(None) => None,
                Err(e) => {
                    return Err(self.exception(
                        "javax/microedition/rms/RecordStoreException",
                        &e.to_string(),
                    ));
                }
            };
            if data.is_none() && a[1].i() == 0 {
                return Err(
                    self.exception("javax/microedition/rms/RecordStoreNotFoundException", &key)
                );
            }
            return Ok(V::R(self.alloc(
                "javax/microedition/rms/RecordStore",
                Native::Record { name: key, data },
            )));
        }
        let o = a[0].r();
        if name == "getNumRecords" {
            if let Native::Record { data, .. } = &self.obj(o).native {
                return Ok(V::I(data.is_some() as i32));
            }
        }
        if name == "getRecord" {
            if let Native::Record {
                data: Some(bytes), ..
            } = &self.obj(o).native
            {
                let b = bytes.clone();
                return Ok(V::R(self.byte_array(&b)));
            }
            return Err(self.exception(
                "javax/microedition/rms/RecordStoreException",
                "record missing",
            ));
        }
        if name == "addRecord" || name == "setRecord" {
            let start = if name == "addRecord" { 1 } else { 2 };
            let bytes = self.bytes(a[start].r());
            let offset = a[start + 1].i() as usize;
            let len = a[start + 2].i() as usize;
            let Some(payload) = bytes.get(offset..offset + len) else {
                return Err(self.exception(
                    "javax/microedition/rms/RecordStoreException",
                    "record range",
                ));
            };
            let Native::Record { name: key, .. } = &self.obj(o).native else {
                panic!("record state")
            };
            let key = key.clone();
            self.records
                .write(&key, &crate::storage::encode_record(payload))
                .map_err(|e| {
                    self.exception(
                        "javax/microedition/rms/RecordStoreException",
                        &e.to_string(),
                    )
                })?;
            self.obj_mut(o).native = Native::Record {
                name: key,
                data: Some(payload.to_vec()),
            };
            return Ok(V::I(1));
        }
        if name != "closeRecordStore" {
            panic!("record method {name}");
        }
        Ok(V::I(0))
    }
    fn media(&mut self, class: &str, name: &str, a: &[V]) -> JResult {
        use crate::audio::AudioCommand;
        if class == "javax/microedition/media/Manager" {
            if a[0].r() == 0 {
                return Err(self.null());
            }
            let source = self.stream(a[0].r()).borrow().name.clone();
            return Ok(V::R(self.alloc(
                "javax/microedition/media/Player",
                Native::Player {
                    name: source,
                    state: 100,
                    position: 0,
                    loops: 1,
                    volume: 50,
                },
            )));
        }
        let mut o = a[0].r();
        if o == 0 {
            return Err(self.null());
        }
        if name == "getControl" {
            return Ok(V::R(self.alloc(
                "javax/microedition/media/control/VolumeControl",
                Native::Volume(o),
            )));
        }
        if let Native::Volume(player) = self.obj(o).native {
            o = player;
        }
        let Native::Player {
            name: source,
            state,
            position,
            loops,
            volume,
        } = &mut self.obj_mut(o).native
        else {
            panic!("player state")
        };
        let mut command = None;
        match name {
            "getState" => return Ok(V::I(*state)),
            "realize" => *state = 200,
            "prefetch" => *state = 300,
            "start" => {
                *state = 400;
                command = Some(AudioCommand::Play {
                    id: o,
                    name: source.clone(),
                    position: *position,
                    loops: *loops,
                    volume: *volume,
                });
            }
            "stop" | "close" => {
                *state = if name == "close" { 0 } else { 300 };
                command = Some(AudioCommand::Stop(o));
            }
            "setLoopCount" => *loops = a[1].i(),
            "setMediaTime" => {
                *position = a[1].j();
                return Ok(a[1]);
            }
            "setLevel" => {
                *volume = a[1].i().clamp(0, 100);
                command = Some(AudioCommand::Volume(o, *volume));
            }
            "addPlayerListener" | "removePlayerListener" => {}
            _ => panic!("media {name}"),
        }
        if let Some(c) = command {
            self.audio.push(c);
        }
        Ok(if name == "setLevel" { a[1] } else { V::I(0) })
    }
}
fn default_value(desc: &str) -> V {
    match desc.as_bytes()[0] {
        b'L' | b'[' => V::R(0),
        b'J' => V::J(0),
        b'F' => V::F(0.0),
        b'D' => V::D(0.0),
        _ => V::I(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn java_random_matches_jdk() {
        let mut r = Runtime::new(PathBuf::from("assets"), std::env::temp_dir()).unwrap();
        let o = r.alloc("java/util/Random", Native::Empty);
        r.call(o, "<init>", "()V", &[]).unwrap();
        r.call(o, "setSeed", "(J)V", &[V::J(0)]).unwrap();
        assert_eq!(r.call(o, "nextInt", "()I", &[]).unwrap().i(), -1155484576);
        assert_eq!(r.call(o, "nextInt", "()I", &[]).unwrap().i(), -723955400);
    }
    #[test]
    fn signed_arrays_and_gc() {
        let mut r = Runtime::new(PathBuf::from("assets"), std::env::temp_dir()).unwrap();
        let a = r.byte_array(&[128, 255, 127]);
        r.roots.push(a);
        assert_eq!(r.array_get(a, 0).unwrap().i(), -128);
        assert_eq!(r.array_get(a, 1).unwrap().i(), -1);
        r.alloc("java/lang/Object", Native::Empty);
        r.collect();
        assert_eq!(r.bytes(a), vec![128, 255, 127]);
    }
}
