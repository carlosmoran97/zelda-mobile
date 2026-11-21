//! Stable object-field keys used by the native game.
//! Reference names are retained here only for inspection and legacy tooling.
pub(crate) mod drawable {
    /// Reference `f.i: I`.
    pub const X: usize = 0;
    /// Reference `f.j: I`.
    pub const Y: usize = 1;
    /// Reference `f.k: I`.
    pub const HALF_WIDTH: usize = 2;
    /// Reference `f.l: I`.
    pub const HALF_HEIGHT: usize = 3;
    /// Reference `f.m: I`.
    pub const DRAW_ORDER: usize = 4;
}
pub(crate) mod trail_point {
    /// Reference `g.a: I`.
    pub const X: usize = 5;
    /// Reference `g.b: I`.
    pub const Y: usize = 6;
    /// Reference `g.c: I`.
    pub const STATE_C: usize = 7;
    /// Reference `g.d: I`.
    pub const STATE_D: usize = 8;
}
pub(crate) mod motion {
    /// Reference `d.a: I`.
    pub const X: usize = 9;
    /// Reference `d.b: I`.
    pub const Y: usize = 10;
    /// Reference `d.c: I`.
    pub const STATE_C: usize = 11;
    /// Reference `d.d: F`.
    pub const VELOCITY_X: usize = 12;
    /// Reference `d.e: F`.
    pub const VELOCITY_Y: usize = 13;
    /// Reference `d.f: F`.
    pub const STATE_F: usize = 14;
}
pub(crate) mod map_object {
    /// Reference `n.a: I`.
    pub const KIND: usize = 15;
    /// Reference `n.b: I`.
    pub const TEXT_INDEX: usize = 16;
    /// Reference `n.c: I`.
    pub const PERSISTENCE_ID: usize = 17;
    /// Reference `n.d: I`.
    pub const STATE_D: usize = 18;
    /// Reference `n.e: Z`.
    pub const STATE_E: usize = 19;
    /// Reference `n.f: Z`.
    pub const STATE_F: usize = 20;
    /// Reference `n.g: Z`.
    pub const STATE_G: usize = 21;
    /// Reference `n.h: Z`.
    pub const STATE_H: usize = 22;
    /// Reference `n.n: Z`.
    pub const STATE_N: usize = 23;
    /// Reference `n.o: Z`.
    pub const STATE_O: usize = 24;
    /// Reference `n.p: Z`.
    pub const STATE_P: usize = 25;
    /// Reference `n.q: Z`.
    pub const STATE_Q: usize = 26;
    /// Reference `n.r: B`.
    pub const TRIGGER_KIND: usize = 27;
    /// Reference `n.s: Z`.
    pub const TRIGGER_ACTIVE: usize = 28;
    /// Reference `n.t: I`.
    pub const TRIGGER_VALUE: usize = 29;
    /// Reference `n.u: I`.
    pub const HAS_DIALOGUE: usize = 30;
    /// Reference `n.v: Z`.
    pub const STATE_V: usize = 31;
    /// Reference `n.w: Z`.
    pub const SOLID: usize = 32;
    /// Reference `n.x: Z`.
    pub const STATE_X: usize = 33;
    /// Reference `n.y: Z`.
    pub const STATE_Y: usize = 34;
    /// Reference `n.z: Z`.
    pub const STATE_Z: usize = 35;
    /// Reference `n.A: I`.
    pub const STATE_A: usize = 36;
    /// Reference `n.B: B`.
    pub const STATE_B: usize = 37;
    /// Reference `n.L: B`.
    pub const STATE_L: usize = 38;
    /// Reference `n.C: I`.
    pub const STATE_C: usize = 39;
    /// Reference `n.D: B`.
    pub const STATE_D_UPPER: usize = 40;
    /// Reference `n.E: B`.
    pub const STATE_E_UPPER: usize = 41;
    /// Reference `n.F: B`.
    pub const WIDTH: usize = 42;
    /// Reference `n.G: B`.
    pub const HEIGHT: usize = 43;
    /// Reference `n.H: I`.
    pub const STATE_H_UPPER: usize = 44;
    /// Reference `n.I: I`.
    pub const STATE_I: usize = 45;
    /// Reference `n.M: I`.
    pub const STATE_M: usize = 46;
    /// Reference `n.J: Ljavax/microedition/lcdui/Image;`.
    pub const IMAGE: usize = 47;
    /// Reference `n.K: Ljavax/microedition/lcdui/Image;`.
    pub const OVERLAY_IMAGE: usize = 48;
}
pub(crate) mod item {
    /// Reference `c.n: B`.
    pub const ANIMATION: usize = 49;
    /// Reference `c.a: I`.
    pub const KIND: usize = 50;
    /// Reference `c.b: I`.
    pub const LIFETIME: usize = 51;
    /// Reference `c.c: I`.
    pub const PERSISTENCE_ID: usize = 52;
    /// Reference `c.o: Ljavax/microedition/lcdui/Image;`.
    pub const IMAGE: usize = 53;
    /// Reference `c.d: Ljavax/microedition/lcdui/Image;`.
    pub const OVERLAY_IMAGE: usize = 54;
    /// Reference `c.e: Z`.
    pub const ARMED: usize = 55;
    /// Reference `c.f: Z`.
    pub const ACTIVE: usize = 56;
    /// Reference `c.g: I`.
    pub const EXPLOSION_TIMER: usize = 57;
    /// Reference `c.p: Ljava/util/Random;`.
    pub const RANDOM: usize = 58;
    /// Reference `c.h: Z`.
    pub const COLLECTED: usize = 59;
    /// Reference `c.q: Lh;`.
    pub const PLAYER: usize = 60;
}
pub(crate) mod world {
    /// Reference `j.t: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_T: usize = 96;
    /// Reference `j.a: Lk;`.
    pub const CANVAS: usize = 97;
    /// Reference `j.b: Lo;`.
    pub const RESOURCES: usize = 98;
    /// Reference `j.u: I`.
    pub const TEXT_SCROLL: usize = 99;
    /// Reference `j.v: I`.
    pub const TEXT_FRAME: usize = 100;
    /// Reference `j.w: Ljavax/microedition/lcdui/Image;`.
    pub const DEATH_IMAGE: usize = 101;
    /// Reference `j.x: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_X: usize = 102;
    /// Reference `j.c: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_C: usize = 103;
    /// Reference `j.y: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_Y: usize = 104;
    /// Reference `j.d: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_D: usize = 105;
    /// Reference `j.z: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_Z: usize = 106;
    /// Reference `j.A: [B`.
    pub const TILES: usize = 107;
    /// Reference `j.B: B`.
    pub const WIDTH: usize = 108;
    /// Reference `j.C: B`.
    pub const HEIGHT: usize = 109;
    /// Reference `j.D: B`.
    pub const TILESET: usize = 110;
    /// Reference `j.e: Ljava/util/Random;`.
    pub const RANDOM: usize = 111;
    /// Reference `j.E: I`.
    pub const MAP: usize = 112;
    /// Reference `j.f: I`.
    pub const ZONE: usize = 113;
    /// Reference `j.g: I`.
    pub const ALTERNATE_MAP: usize = 114;
    /// Reference `j.h: I`.
    pub const ALTERNATE_ROOM: usize = 115;
    /// Reference `j.i: I`.
    pub const STATE_I: usize = 116;
    /// Reference `j.j: I`.
    pub const STATE_J: usize = 117;
    /// Reference `j.F: I`.
    pub const SWITCH_COOLDOWN: usize = 118;
    /// Reference `j.G: I`.
    pub const STATE_G: usize = 119;
    /// Reference `j.k: I`.
    pub const HOOKSHOT_X: usize = 120;
    /// Reference `j.l: I`.
    pub const HOOKSHOT_Y: usize = 121;
    /// Reference `j.m: Z`.
    pub const STATE_M: usize = 122;
    /// Reference `j.n: Ljava/util/Vector;`.
    pub const PROJECTILES: usize = 123;
    /// Reference `j.o: Ljava/util/Vector;`.
    pub const PORTALS: usize = 124;
    /// Reference `j.p: Ljava/util/Vector;`.
    pub const ITEMS: usize = 125;
    /// Reference `j.q: Ljava/util/Vector;`.
    pub const ENEMIES: usize = 126;
    /// Reference `j.r: Ljava/util/Vector;`.
    pub const OBJECTS: usize = 127;
    /// Reference `j.s: Ljava/util/Vector;`.
    pub const OVERLAY_OBJECTS: usize = 128;
    /// Reference `j.H: Ljava/util/Vector;`.
    pub const DIALOGUE: usize = 129;
    /// Reference `j.I: Ljava/util/Vector;`.
    pub const ROOM_HISTORY: usize = 130;
}
pub(crate) mod sound {
    /// Reference `b.d: I`.
    pub const PENDING_EFFECT: usize = 61;
    /// Reference `b.f: Ljava/lang/Thread;`.
    pub const WORKER: usize = 244;
    /// Reference `b.a: Z`.
    pub const ENABLED: usize = 245;
    /// Reference `b.g: Ljavax/microedition/media/control/VolumeControl;`.
    pub const MUSIC_CONTROL: usize = 246;
    /// Reference `b.h: Ljavax/microedition/media/control/VolumeControl;`.
    pub const EFFECT_CONTROL: usize = 247;
    /// Reference `b.b: Ljavax/microedition/media/Player;`.
    pub const MUSIC_PLAYER: usize = 248;
    /// Reference `b.c: Ljavax/microedition/media/Player;`.
    pub const EFFECT_PLAYER: usize = 249;
    /// Reference `b.i: I`.
    pub const QUEUED_EFFECT: usize = 250;
    /// Reference `b.j: Z`.
    pub const USE_MPEG: usize = 251;
    /// Reference `b.k: Z`.
    pub const EFFECTS_READY: usize = 252;
    /// Reference `b.l: I`.
    pub const MUSIC_TRACK: usize = 253;
    /// Reference `b.m: I`.
    pub const MUSIC_VOLUME: usize = 254;
    /// Reference `b.n: I`.
    pub const EFFECT_VOLUME: usize = 255;
    /// Reference `b.o: I`.
    pub const EFFECT_PRIORITY: usize = 256;
    /// Reference `b.p: J`.
    pub const EFFECT_STARTED_AT: usize = 257;
    /// Reference `b.e: Z`.
    pub const STOP_REQUESTED: usize = 258;
    /// Reference `b.q: I`.
    pub const ZONE: usize = 259;
    /// Reference `b.r: I`.
    pub const MAP: usize = 260;
}
pub(crate) mod enemy {
    /// Reference `l.A: I`.
    pub const STATE_A: usize = 187;
    /// Reference `l.B: I`.
    pub const WIDTH: usize = 188;
    /// Reference `l.C: I`.
    pub const HEIGHT: usize = 189;
    /// Reference `l.D: I`.
    pub const STATE_D: usize = 190;
    /// Reference `l.E: I`.
    pub const STATE_E: usize = 191;
    /// Reference `l.F: I`.
    pub const STATE_TIMER: usize = 192;
    /// Reference `l.G: I`.
    pub const STATE_G: usize = 193;
    /// Reference `l.H: I`.
    pub const VELOCITY_X: usize = 194;
    /// Reference `l.I: I`.
    pub const VELOCITY_Y: usize = 195;
    /// Reference `l.J: I`.
    pub const HIT_COUNT: usize = 196;
    /// Reference `l.K: I`.
    pub const KNOCKBACK_TIMER: usize = 197;
    /// Reference `l.L: I`.
    pub const STATE_L: usize = 198;
    /// Reference `l.M: I`.
    pub const STATE_M: usize = 199;
    /// Reference `l.N: I`.
    pub const HEALTH: usize = 200;
    /// Reference `l.O: I`.
    pub const STATE_O: usize = 201;
    /// Reference `l.P: Z`.
    pub const STATE_P: usize = 202;
    /// Reference `l.Q: Z`.
    pub const STATE_Q: usize = 203;
    /// Reference `l.R: Ljava/util/Vector;`.
    pub const STATE_R: usize = 204;
    /// Reference `l.S: Ljava/util/Vector;`.
    pub const STATE_S: usize = 205;
    /// Reference `l.T: Ljava/util/Vector;`.
    pub const STATE_T: usize = 206;
    /// Reference `l.U: Ljava/util/Random;`.
    pub const RANDOM: usize = 207;
    /// Reference `l.V: [I`.
    pub const STATE_V: usize = 208;
    /// Reference `l.a: B`.
    pub const ANIMATION: usize = 209;
    /// Reference `l.b: I`.
    pub const KIND: usize = 210;
    /// Reference `l.c: B`.
    pub const FACING: usize = 211;
    /// Reference `l.d: B`.
    pub const STATE_D_LOWER: usize = 212;
    /// Reference `l.e: B`.
    pub const DEATH_FRAME: usize = 213;
    /// Reference `l.f: Z`.
    pub const DYING: usize = 214;
    /// Reference `l.g: Z`.
    pub const DEAD: usize = 215;
    /// Reference `l.h: Z`.
    pub const STATE_H: usize = 216;
    /// Reference `l.n: Z`.
    pub const ACTIVE: usize = 217;
    /// Reference `l.o: Z`.
    pub const STATE_O_LOWER: usize = 218;
    /// Reference `l.p: Z`.
    pub const HAS_PROJECTILE: usize = 219;
    /// Reference `l.q: Ljavax/microedition/lcdui/Image;`.
    pub const IMAGE: usize = 220;
    /// Reference `l.r: Ljavax/microedition/lcdui/Image;`.
    pub const OVERLAY_IMAGE: usize = 221;
    /// Reference `l.s: I`.
    pub const STATE_S_LOWER: usize = 222;
    /// Reference `l.t: I`.
    pub const HURT_TIMER: usize = 223;
    /// Reference `l.u: I`.
    pub const STATE_U: usize = 224;
    /// Reference `l.v: F`.
    pub const STATE_V_LOWER: usize = 225;
    /// Reference `l.w: F`.
    pub const STATE_W: usize = 226;
    /// Reference `l.x: F`.
    pub const STATE_X: usize = 227;
    /// Reference `l.y: F`.
    pub const STATE_Y: usize = 228;
    /// Reference `l.z: I`.
    pub const BEHAVIOR_STATE: usize = 229;
    /// Reference `l.W: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_W_UPPER: usize = 261;
    /// Reference `l.X: I`.
    pub const STATE_X_UPPER: usize = 262;
    /// Reference `l.Y: Ljava/util/Vector;`.
    pub const STATE_Y_UPPER: usize = 263;
    /// Reference `l.Z: Ljava/util/Vector;`.
    pub const STATE_Z_UPPER: usize = 264;
    /// Reference `l.aa: Ljava/util/Vector;`.
    pub const STATE_AA: usize = 265;
    /// Reference `l.ab: [I`.
    pub const STATE_AB: usize = 266;
}
pub(crate) mod player {
    /// Reference `h.C: Z`.
    pub const HAS_MIRROR_SHIELD: usize = 62;
    /// Reference `h.E: Z`.
    pub const HOLDING_ITEM: usize = 63;
    /// Reference `h.H: I`.
    pub const HEALTH: usize = 64;
    /// Reference `h.I: I`.
    pub const MAX_HEALTH: usize = 65;
    /// Reference `h.J: I`.
    pub const RUPEES: usize = 66;
    /// Reference `h.K: I`.
    pub const WALLET_CAPACITY: usize = 67;
    /// Reference `h.M: I`.
    pub const DEFENSE: usize = 68;
    /// Reference `h.N: I`.
    pub const KEYS: usize = 69;
    /// Reference `h.Q: B`.
    pub const BOMBS: usize = 70;
    /// Reference `h.R: B`.
    pub const BOMB_CAPACITY: usize = 71;
    /// Reference `h.S: B`.
    pub const ARROWS: usize = 72;
    /// Reference `h.T: B`.
    pub const ARROW_CAPACITY: usize = 73;
    /// Reference `h.U: B`.
    pub const SWORD_LEVEL: usize = 74;
    /// Reference `h.V: B`.
    pub const TUNIC_COLOR: usize = 75;
    /// Reference `h.W: B`.
    pub const SWORD_DAMAGE: usize = 76;
    /// Reference `h.a: Ljavax/microedition/lcdui/Image;`.
    pub const SPRITE: usize = 77;
    /// Reference `h.ad: I`.
    pub const ITEM_POSE_TIMER: usize = 78;
    /// Reference `h.aj: I`.
    pub const PROGRESSION_FLAGS: usize = 79;
    /// Reference `h.ak: I`.
    pub const HEART_PICKUPS: usize = 80;
    /// Reference `h.al: I`.
    pub const KEY_PICKUPS: usize = 81;
    /// Reference `h.am: I`.
    pub const RUPEE_PICKUPS: usize = 82;
    /// Reference `h.b: Ljavax/microedition/lcdui/Image;`.
    pub const EQUIPMENT_SPRITE: usize = 83;
    /// Reference `h.g: Z`.
    pub const ATTACKING: usize = 84;
    /// Reference `h.q: Z`.
    pub const HAS_SWORD: usize = 85;
    /// Reference `h.r: Z`.
    pub const HAS_BOOMERANG: usize = 86;
    /// Reference `h.s: Z`.
    pub const HAS_BOMBS: usize = 87;
    /// Reference `h.t: Z`.
    pub const HAS_BOW: usize = 88;
    /// Reference `h.u: Z`.
    pub const HAS_CANDLE: usize = 89;
    /// Reference `h.v: Z`.
    pub const HAS_POTION: usize = 90;
    /// Reference `h.w: Z`.
    pub const HAS_RAFT: usize = 91;
    /// Reference `h.x: Z`.
    pub const HAS_HOOKSHOT: usize = 92;
    /// Reference `h.y: Z`.
    pub const HAS_SHOVEL: usize = 93;
    /// Reference `h.A: Z`.
    pub const STATE_A: usize = 137;
    /// Reference `h.B: Z`.
    pub const ON_RAFT: usize = 138;
    /// Reference `h.F: Z`.
    pub const DIGGING: usize = 139;
    /// Reference `h.G: I`.
    pub const STATE_G: usize = 140;
    /// Reference `h.L: I`.
    pub const HURT_TIMER: usize = 141;
    /// Reference `h.O: I`.
    pub const ZONE: usize = 142;
    /// Reference `h.P: I`.
    pub const MAP: usize = 143;
    /// Reference `h.X: I`.
    pub const ACTION_TIMER: usize = 144;
    /// Reference `h.Z: I`.
    pub const COLLISION_HEIGHT: usize = 145;
    /// Reference `h.ae: I`.
    pub const DIG_TIMER: usize = 146;
    /// Reference `h.ag: Z`.
    pub const STATE_AG: usize = 147;
    /// Reference `h.ah: Z`.
    pub const STATE_AH: usize = 148;
    /// Reference `h.ai: Ljava/util/Vector;`.
    pub const HOOKSHOT_TRAIL: usize = 149;
    /// Reference `h.an: I`.
    pub const WORLD_FLAGS: usize = 150;
    /// Reference `h.ao: I`.
    pub const PROJECTILE_COOLDOWN: usize = 151;
    /// Reference `h.ap: [I`.
    pub const OVERWORLD_EVENTS: usize = 152;
    /// Reference `h.aq: [I`.
    pub const CAVE_EVENTS: usize = 153;
    /// Reference `h.ar: [I`.
    pub const DUNGEON_1_EVENTS: usize = 154;
    /// Reference `h.as: [I`.
    pub const DUNGEON_2_EVENTS: usize = 155;
    /// Reference `h.at: [I`.
    pub const DUNGEON_3_EVENTS: usize = 156;
    /// Reference `h.au: [I`.
    pub const DUNGEON_4_EVENTS: usize = 157;
    /// Reference `h.av: [I`.
    pub const DUNGEON_5_EVENTS: usize = 158;
    /// Reference `h.aw: [I`.
    pub const DUNGEON_6_EVENTS: usize = 159;
    /// Reference `h.ax: [I`.
    pub const DUNGEON_7_EVENTS: usize = 160;
    /// Reference `h.ay: [I`.
    pub const DUNGEON_8_EVENTS: usize = 161;
    /// Reference `h.az: [I`.
    pub const DUNGEON_9_EVENTS: usize = 162;
    /// Reference `h.d: Ljavax/microedition/lcdui/Image;`.
    pub const OVERLAY_SPRITE: usize = 163;
    /// Reference `h.e: I`.
    pub const FACING: usize = 164;
    /// Reference `h.f: I`.
    pub const ANIMATION: usize = 165;
    /// Reference `h.h: Z`.
    pub const BOOMERANG_ACTIVE: usize = 166;
    /// Reference `h.n: Z`.
    pub const HOOKSHOT_EXTENDING: usize = 167;
    /// Reference `h.o: Z`.
    pub const HOOKSHOT_RETURNING: usize = 168;
    /// Reference `h.p: Z`.
    pub const SHIELD_RAISED: usize = 169;
    /// Reference `h.z: Z`.
    pub const HAS_FISH: usize = 170;
    /// Reference `h.c: Ljavax/microedition/lcdui/Image;`.
    pub const ACTION_SPRITE: usize = 267;
    /// Reference `h.aB: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AB: usize = 268;
    /// Reference `h.aC: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AC: usize = 269;
    /// Reference `h.aD: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AD: usize = 270;
    /// Reference `h.aE: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AE: usize = 271;
    /// Reference `h.aF: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AF: usize = 272;
    /// Reference `h.aG: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AG_UPPER: usize = 273;
    /// Reference `h.aH: Z`.
    pub const STATE_AH_UPPER: usize = 274;
    /// Reference `h.D: Z`.
    pub const FISHING: usize = 275;
    /// Reference `h.Y: I`.
    pub const STATE_Y_NUMBER: usize = 276;
    /// Reference `h.aa: I`.
    pub const STATE_AA: usize = 277;
    /// Reference `h.ab: I`.
    pub const STATE_AB_LOWER: usize = 278;
    /// Reference `h.ac: I`.
    pub const STATE_AC_LOWER: usize = 279;
    /// Reference `h.af: I`.
    pub const TRIFORCE_COUNT: usize = 280;
    /// Reference `h.aA: [Z`.
    pub const TRIFORCE_PIECES: usize = 281;
    /// Reference `h.aI: Lk;`.
    pub const CANVAS: usize = 282;
}
pub(crate) mod projectile {
    /// Reference `p.a: B`.
    pub const ANIMATION: usize = 231;
    /// Reference `p.b: I`.
    pub const VELOCITY_X: usize = 232;
    /// Reference `p.c: I`.
    pub const VELOCITY_Y: usize = 233;
    /// Reference `p.d: Ljavax/microedition/lcdui/Image;`.
    pub const OVERLAY_IMAGE: usize = 234;
    /// Reference `p.e: I`.
    pub const FACING: usize = 235;
    /// Reference `p.f: I`.
    pub const WIDTH: usize = 236;
    /// Reference `p.g: I`.
    pub const HEIGHT: usize = 237;
    /// Reference `p.h: I`.
    pub const KIND: usize = 238;
    /// Reference `p.n: I`.
    pub const STATE_N: usize = 239;
    /// Reference `p.o: I`.
    pub const DAMAGE: usize = 240;
    /// Reference `p.p: Z`.
    pub const ENEMY_OWNED: usize = 241;
    /// Reference `p.q: I`.
    pub const OWNER_INDEX: usize = 242;
    /// Reference `p.r: Z`.
    pub const RETURNING: usize = 243;
    /// Reference `p.s: Ljavax/microedition/lcdui/Image;`.
    pub const IMAGE: usize = 284;
    /// Reference `p.t: I`.
    pub const STATE_T: usize = 285;
    /// Reference `p.u: I`.
    pub const STATE_U: usize = 286;
    /// Reference `p.v: I`.
    pub const STATE_V: usize = 287;
}
pub(crate) mod resources {
    /// Reference `o.a: [Ljavax/microedition/lcdui/Image;`.
    pub const ENEMY_IMAGES: usize = 230;
    /// Reference `o.b: Lj;`.
    pub const WORLD: usize = 288;
    /// Reference `o.c: [Ljavax/microedition/lcdui/Image;`.
    pub const OBJECT_IMAGES: usize = 289;
    /// Reference `o.d: [Ljavax/microedition/lcdui/Image;`.
    pub const ITEM_IMAGES: usize = 290;
    /// Reference `o.e: [Ljavax/microedition/lcdui/Image;`.
    pub const PROJECTILE_IMAGES: usize = 291;
    /// Reference `o.f: [I`.
    pub const OBJECT_AGES: usize = 292;
    /// Reference `o.g: [I`.
    pub const ITEM_AGES: usize = 293;
    /// Reference `o.h: [I`.
    pub const ENEMY_AGES: usize = 294;
    /// Reference `o.i: [I`.
    pub const PROJECTILE_AGES: usize = 295;
    /// Reference `o.j: [Z`.
    pub const OBJECT_USED: usize = 296;
    /// Reference `o.k: [Z`.
    pub const ITEM_USED: usize = 297;
    /// Reference `o.l: [Z`.
    pub const ENEMY_USED: usize = 298;
    /// Reference `o.m: [Z`.
    pub const PROJECTILE_USED: usize = 299;
}
pub(crate) mod room_history {
    /// Reference `i.a: I`.
    pub const ZONE: usize = 171;
    /// Reference `i.b: I`.
    pub const MAP: usize = 172;
    /// Reference `i.c: I`.
    pub const ENEMY_COUNT: usize = 173;
}
pub(crate) mod portal {
    /// Reference `e.a: I`.
    pub const TILE_X: usize = 131;
    /// Reference `e.b: I`.
    pub const TILE_Y: usize = 132;
    /// Reference `e.c: I`.
    pub const DESTINATION_MAP: usize = 133;
    /// Reference `e.d: I`.
    pub const DESTINATION_ZONE: usize = 134;
    /// Reference `e.e: I`.
    pub const DESTINATION_X: usize = 135;
    /// Reference `e.f: I`.
    pub const DESTINATION_Y: usize = 136;
}
pub(crate) mod frame_timer {
    /// Reference `m.a: LZelda;`.
    pub const APPLICATION: usize = 300;
}
pub(crate) mod second_timer {
    /// Reference `a.a: LZelda;`.
    pub const APPLICATION: usize = 304;
}
pub(crate) mod lifecycle {
    /// Reference `Zelda.a: Lk;`.
    pub const CANVAS: usize = 307;
    /// Reference `Zelda.b: Ljavax/microedition/lcdui/Command;`.
    pub const EXIT_COMMAND: usize = 308;
    /// Reference `Zelda.c: Ljavax/microedition/lcdui/Command;`.
    pub const MENU_COMMAND: usize = 309;
    /// Reference `Zelda.d: Ljavax/microedition/lcdui/Command;`.
    pub const SAVE_EXIT_COMMAND: usize = 310;
    /// Reference `Zelda.e: Ljavax/microedition/lcdui/Command;`.
    pub const RETURN_COMMAND: usize = 311;
    /// Reference `Zelda.f: Ljavax/microedition/lcdui/Command;`.
    pub const START_COMMAND: usize = 312;
    /// Reference `Zelda.g: Ljavax/microedition/lcdui/Command;`.
    pub const SETTINGS_COMMAND: usize = 313;
    /// Reference `Zelda.h: Ljavax/microedition/lcdui/Command;`.
    pub const DEFAULT_COMMAND: usize = 314;
    /// Reference `Zelda.i: Ljavax/microedition/lcdui/Command;`.
    pub const PLAY_COMMAND: usize = 315;
    /// Reference `Zelda.j: Ljavax/microedition/lcdui/Command;`.
    pub const MAIN_MENU_COMMAND: usize = 316;
    /// Reference `Zelda.k: Ljavax/microedition/lcdui/Display;`.
    pub const DISPLAY: usize = 317;
    /// Reference `Zelda.l: Lm;`.
    pub const FRAME_TIMER: usize = 318;
    /// Reference `Zelda.m: Ljava/util/Timer;`.
    pub const TIMER: usize = 319;
    /// Reference `Zelda.n: La;`.
    pub const SECOND_TIMER: usize = 320;
}
pub(crate) mod canvas {
    /// Reference `k.j: Z`.
    pub const SHOW_ITEM_MESSAGE: usize = 94;
    /// Reference `k.k: I`.
    pub const ITEM_MESSAGE: usize = 95;
    /// Reference `k.F: I`.
    pub const STATE_F: usize = 175;
    /// Reference `k.G: Z`.
    pub const STATE_G: usize = 176;
    /// Reference `k.H: Lb;`.
    pub const SOUND: usize = 177;
    /// Reference `k.I: [Ljava/lang/String;`.
    pub const ITEM_MESSAGES: usize = 178;
    /// Reference `k.a: Z`.
    pub const VISUAL_EFFECTS: usize = 179;
    /// Reference `k.e: Lh;`.
    pub const PLAYER: usize = 180;
    /// Reference `k.g: Z`.
    pub const REDRAW_WORLD: usize = 181;
    /// Reference `k.i: Z`.
    pub const STATE_I: usize = 182;
    /// Reference `k.l: I`.
    pub const STATE_L: usize = 183;
    /// Reference `k.p: I`.
    pub const SCREEN_STATE: usize = 184;
    /// Reference `k.t: Z`.
    pub const USE_PHONE_BUTTONS: usize = 185;
    /// Reference `k.x: Z`.
    pub const STATE_X: usize = 186;
    /// Reference `k.d: Lj;`.
    pub const WORLD: usize = 283;
    /// Reference `k.m: Z`.
    pub const INVENTORY_OPEN: usize = 301;
    /// Reference `k.r: Z`.
    pub const REFRESH_COMMANDS: usize = 302;
    /// Reference `k.s: Z`.
    pub const EXIT_REQUESTED: usize = 303;
    /// Reference `k.f: Z`.
    pub const REDRAW: usize = 305;
    /// Reference `k.h: Z`.
    pub const SECOND_ELAPSED: usize = 306;
    /// Reference `k.A: I`.
    pub const LEFT_KEY: usize = 321;
    /// Reference `k.B: I`.
    pub const RIGHT_KEY: usize = 322;
    /// Reference `k.C: I`.
    pub const PRIMARY_KEY: usize = 323;
    /// Reference `k.D: I`.
    pub const SECONDARY_KEY: usize = 324;
    /// Reference `k.E: I`.
    pub const MENU_KEY: usize = 325;
    /// Reference `k.b: Ljavax/microedition/lcdui/Display;`.
    pub const DISPLAY: usize = 326;
    /// Reference `k.c: Z`.
    pub const STATE_C: usize = 327;
    /// Reference `k.n: I`.
    pub const INVENTORY_PAGE: usize = 328;
    /// Reference `k.o: I`.
    pub const MENU_SELECTION: usize = 329;
    /// Reference `k.q: I`.
    pub const PREVIOUS_SCREEN: usize = 330;
    /// Reference `k.u: Z`.
    pub const STATE_U: usize = 331;
    /// Reference `k.v: Z`.
    pub const STATE_V: usize = 332;
    /// Reference `k.w: Z`.
    pub const STATE_W: usize = 333;
    /// Reference `k.y: I`.
    pub const UP_KEY: usize = 334;
    /// Reference `k.z: I`.
    pub const DOWN_KEY: usize = 335;
    /// Reference `k.J: LZelda;`.
    pub const APPLICATION: usize = 336;
    /// Reference `k.K: Ljavax/microedition/lcdui/Graphics;`.
    pub const GRAPHICS: usize = 337;
    /// Reference `k.L: I`.
    pub const STATE_L_UPPER: usize = 338;
    /// Reference `k.M: I`.
    pub const STATE_M: usize = 339;
    /// Reference `k.N: I`.
    pub const STATE_N: usize = 340;
    /// Reference `k.O: I`.
    pub const STATE_O: usize = 341;
    /// Reference `k.P: I`.
    pub const STATE_P: usize = 342;
    /// Reference `k.Q: I`.
    pub const STATE_Q: usize = 343;
    /// Reference `k.R: Ljavax/microedition/lcdui/Image;`.
    pub const SCENE_IMAGE: usize = 344;
    /// Reference `k.S: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_S: usize = 345;
    /// Reference `k.T: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_T: usize = 346;
    /// Reference `k.U: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_U_UPPER: usize = 347;
    /// Reference `k.V: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_V_UPPER: usize = 348;
    /// Reference `k.W: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_W_UPPER: usize = 349;
    /// Reference `k.X: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_X_UPPER: usize = 350;
    /// Reference `k.Y: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_Y: usize = 351;
    /// Reference `k.Z: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_Z: usize = 352;
    /// Reference `k.aa: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AA: usize = 353;
    /// Reference `k.ab: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AB: usize = 354;
    /// Reference `k.ac: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AC: usize = 355;
    /// Reference `k.ad: Ljavax/microedition/lcdui/Image;`.
    pub const STATE_AD: usize = 356;
    /// Reference `k.ae: [Lh;`.
    pub const SAVE_SLOTS: usize = 357;
    /// Reference `k.af: J`.
    pub const STATE_AF: usize = 358;
    /// Reference `k.ag: J`.
    pub const STATE_AG: usize = 359;
    /// Reference `k.ah: I`.
    pub const STATE_AH: usize = 360;
    /// Reference `k.ai: I`.
    pub const STATE_AI: usize = 361;
    /// Reference `k.aj: I`.
    pub const STATE_AJ: usize = 362;
    /// Reference `k.ak: Z`.
    pub const STATE_AK: usize = 363;
    /// Reference `k.al: I`.
    pub const STATE_AL: usize = 364;
    /// Reference `k.am: I`.
    pub const STATE_AM: usize = 365;
    /// Reference `k.an: I`.
    pub const STATE_AN: usize = 366;
    /// Reference `k.ao: Z`.
    pub const STATE_AO: usize = 367;
    /// Reference `k.ap: Z`.
    pub const STATE_AP: usize = 368;
    /// Reference `k.aq: Z`.
    pub const STATE_AQ: usize = 369;
    /// Reference `k.ar: Z`.
    pub const STATE_AR: usize = 370;
    /// Reference `k.as: I`.
    pub const STATE_AS: usize = 371;
    /// Reference `k.at: I`.
    pub const STATE_AT: usize = 372;
    /// Reference `k.au: J`.
    pub const STATE_AU: usize = 373;
    /// Reference `k.av: I`.
    pub const STATE_AV: usize = 374;
    /// Reference `k.aw: Ljava/util/Vector;`.
    pub const STATE_AW: usize = 375;
    /// Reference `k.ax: I`.
    pub const STATE_AX: usize = 376;
    /// Reference `k.ay: I`.
    pub const MAP: usize = 377;
    /// Reference `k.az: I`.
    pub const ZONE: usize = 378;
    /// Reference `k.aA: I`.
    pub const SAVE_SLOT: usize = 379;
    /// Reference `k.aB: I`.
    pub const STATE_AB_UPPER: usize = 380;
    /// Reference `k.aC: I`.
    pub const STATE_AC_UPPER: usize = 381;
    /// Reference `k.aD: I`.
    pub const STATE_AD_UPPER: usize = 382;
    /// Reference `k.aE: I`.
    pub const STATE_AE: usize = 383;
    /// Reference `k.aF: Ljavax/microedition/rms/RecordStore;`.
    pub const RECORD_STORE: usize = 384;
    /// Reference `k.aG: Z`.
    pub const CONFIGURE_PHONE_KEYS: usize = 385;
    /// Reference `k.aH: I`.
    pub const KEY_CONFIGURATION_STEP: usize = 386;
    /// Reference `k.aI: I`.
    pub const MUSIC_VOLUME: usize = 387;
    /// Reference `k.aJ: I`.
    pub const EFFECT_VOLUME: usize = 388;
    /// Reference `k.aK: I`.
    pub const STATE_AK_UPPER: usize = 389;
    /// Reference `k.aL: I`.
    pub const STATE_AL_UPPER: usize = 390;
    /// Reference `k.aM: I`.
    pub const STATE_AM_UPPER: usize = 391;
    /// Reference `k.aN: Z`.
    pub const STATE_AN_UPPER: usize = 392;
    /// Reference `k.aO: Z`.
    pub const STATE_AO_UPPER: usize = 393;
    /// Reference `k.aP: Z`.
    pub const STATE_AP_UPPER: usize = 394;
    /// Reference `k.aQ: I`.
    pub const STATE_AQ_UPPER: usize = 395;
    /// Reference `k.aR: I`.
    pub const STATE_AR_UPPER: usize = 396;
    /// Reference `k.aS: I`.
    pub const STATE_AS_UPPER: usize = 397;
    /// Reference `k.aT: I`.
    pub const STATE_AT_UPPER: usize = 398;
    /// Reference `k.aU: I`.
    pub const STATE_AU_UPPER: usize = 399;
    /// Reference `k.aV: I`.
    pub const STATE_AV_UPPER: usize = 400;
    /// Reference `k.aW: Ljava/util/Vector;`.
    pub const STATE_AW_UPPER: usize = 401;
    /// Reference `k.aX: Ljava/util/Random;`.
    pub const RANDOM: usize = 402;
    /// Reference `k.aY: Z`.
    pub const STATE_AY: usize = 403;
    /// Reference `k.aZ: [[I`.
    pub const STATE_AZ: usize = 404;
    /// Reference `k.ba: [[I`.
    pub const STATE_BA: usize = 405;
    /// Reference `k.bb: [[I`.
    pub const STATE_BB: usize = 406;
    /// Reference `k.bc: [[I`.
    pub const STATE_BC: usize = 407;
    /// Reference `k.bd: [[I`.
    pub const STATE_BD: usize = 408;
    /// Reference `k.be: [[I`.
    pub const STATE_BE: usize = 409;
    /// Reference `k.bf: [[I`.
    pub const STATE_BF: usize = 410;
    /// Reference `k.bg: [[I`.
    pub const STATE_BG: usize = 411;
}
pub(crate) mod system {
    /// Reference `java/lang/System.out: Ljava/io/PrintStream;`.
    pub const STATE_OUT: usize = 174;
}
