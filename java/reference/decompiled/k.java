/*
 * Decompiled with CFR 0.152.
 */
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Random;
import java.util.Vector;
import javax.microedition.lcdui.Canvas;
import javax.microedition.lcdui.Display;
import javax.microedition.lcdui.Displayable;
import javax.microedition.lcdui.Font;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;
import javax.microedition.rms.RecordStore;
import javax.microedition.rms.RecordStoreException;

public final class k
extends Canvas {
    private Zelda J;
    boolean a = true;
    Display b;
    private Graphics K;
    private int L;
    private int M;
    private int N;
    private int O;
    private int P;
    private int Q;
    boolean c;
    private Image R;
    private Image S;
    private Image T;
    private Image U;
    private Image V;
    private Image W;
    private Image X;
    private Image Y;
    private Image Z;
    private Image aa;
    private Image ab;
    private Image ac;
    private Image ad;
    j d;
    h e;
    private h[] ae;
    private long af;
    private long ag;
    private int ah;
    private int ai;
    private int aj;
    boolean f;
    boolean g;
    private boolean ak;
    private int al;
    private int am;
    private int an;
    boolean h;
    private boolean ao;
    private boolean ap;
    boolean i;
    boolean j;
    private boolean aq;
    int k;
    private boolean ar;
    private int as;
    private int at;
    private long au;
    int l;
    private int av;
    private Vector aw;
    boolean m;
    int n;
    int o;
    private int ax = 0;
    int p;
    int q = 0;
    private int ay;
    private int az;
    private int aA;
    private int aB;
    private int aC;
    private int aD;
    private int aE;
    private RecordStore aF;
    private boolean aG;
    private int aH;
    boolean r = true;
    boolean s = false;
    private int aI = 50;
    private int aJ = 50;
    private int aK = 0;
    private int aL = 176;
    private int aM = 0;
    private boolean aN = false;
    boolean t = true;
    private boolean aO = true;
    boolean u = false;
    boolean v = true;
    private boolean aP = false;
    boolean w = true;
    boolean x = true;
    private int aQ = 0;
    private int aR;
    private int aS = 1;
    int y;
    int z;
    int A;
    int B;
    int C;
    int D;
    int E;
    private int aT;
    private int aU;
    int F;
    private int aV;
    private Vector aW;
    boolean G = false;
    private Random aX;
    private boolean aY = false;
    b H;
    private int[][] aZ = new int[][]{{0, 1, 1, 0, 0, 0}, {0, 0, 1, 0, 1, 1}, {1, 1, 1, 1, 1, 0}, {0, 1, 1, 1, 0, 0}, {0, 0, 1, 0, 0, 0}, {0, 1, 1, 1, 0, 0}};
    private int[][] ba = new int[][]{{0, 1, 1, 0}, {0, 0, 1, 1}, {0, 0, 1, 1}, {0, 0, 1, 1}, {0, 0, 1, 1}, {0, 0, 1, 1}, {1, 1, 1, 1}, {0, 1, 1, 0}};
    private int[][] bb = new int[][]{{0, 1, 1, 0, 0}, {0, 0, 1, 0, 1}, {1, 1, 1, 1, 1}, {1, 1, 1, 1, 1}, {1, 0, 1, 0, 0}, {0, 0, 1, 1, 0}};
    private int[][] bc = new int[][]{{1, 1, 1, 1}, {1, 0, 1, 1}, {1, 1, 0, 0}, {1, 1, 1, 0}, {1, 0, 0, 0}, {1, 1, 0, 0}, {0, 1, 1, 0}, {1, 1, 0, 0}};
    private int[][] bd = new int[][]{{0, 1, 1, 0}, {1, 1, 1, 1}, {1, 1, 1, 1}, {1, 0, 0, 1}, {0, 0, 1, 1}, {0, 1, 1, 1}, {1, 1, 1, 1}, {0, 0, 1, 1}};
    private int[][] be = new int[][]{{0, 1, 1, 1, 1, 0}, {1, 1, 1, 1, 1, 1}, {1, 1, 0, 0, 1, 1}, {1, 1, 1, 0, 1, 0}, {1, 0, 0, 0, 0, 0}, {1, 0, 0, 0, 0, 0}, {1, 0, 1, 0, 0, 0}, {1, 1, 1, 0, 0, 0}};
    private int[][] bf = new int[][]{{1, 1, 1, 1, 1, 1}, {1, 1, 1, 1, 1, 0}, {1, 1, 1, 1, 0, 0}, {1, 1, 1, 0, 0, 0}, {1, 1, 0, 0, 0, 0}, {1, 1, 1, 1, 0, 0}, {1, 1, 1, 1, 1, 1}, {1, 1, 1, 0, 0, 0}};
    private int[][] bg = new int[][]{{0, 0, 0, 1, 0}, {0, 0, 1, 1, 1}, {0, 1, 1, 1, 0}, {1, 1, 1, 1, 1}, {1, 1, 1, 1, 0}, {0, 1, 1, 1, 1}, {0, 0, 0, 0, 1, 0}, {0, 1, 1, 1, 1}};
    final String[] I = new String[]{"You found a piece of heart.. your maximum life increases by 1 heart..", "You found a piece of heart.. you need 3 more for a full heart..", "You got a piece of heart.. you need 2 more for a full heart..", "You got a piece of heart.. you need 1 more for a full heart..", "You got the wooden sword..", "You got the bow and arrows..", "You can now carry 500 rupies..", "You got the boomerang..", "You got the showel, lets start digging for treasure..", "You need the boss key to enter..", "You found the boss key!", "You found the bomb bag, you can now carry 10 bombs..", "You found the raft, you can use it to travel on water..", "You found the hookshot..", "You found the candle, use it to light up dark places..", "You found the mirror shield, use it to push objects and reflect magic..", "You found a piece of the triforce.. only 7 more to collect..", "You found a piece of the triforce.. only 6 more to collect..", "You found a piece of the triforce.. only 5 more to collect..", "You found a piece of the triforce.. only 4 more to collect..", "You found a piece of the triforce.. only 3 more to collect..", "You found a piece of the triforce.. only 2 more to collect..", "You found a piece of the triforce.. only 1 more to collect..", "The triforce is complete, you can now fight ganon!", "You found a heart container, maximum life increases by 1 heart..", "I will heal your wounds..", "You found a strange green suit...?!?", "Traded the green suit for red armor", "Have you seen my green suit??", "Thanks for finding it, good luck with the quest...", "Come back when you can travel on water....", "Here, have a fishing rod, now go get some fish!", "What are you waiting for go get some fish !!"};

    k() {
    }

    public k(Display object, Zelda object2) {
        this.J = object2;
        this.ae = new h[3];
        this.ar = false;
        this.aq = false;
        this.o = 0;
        this.aB = 0;
        this.aC = 0;
        this.m = false;
        this.ak = true;
        this.n = 0;
        this.az = 0;
        this.ay = 120;
        this.aL = this.getHeight();
        this.av = 0;
        this.ao = false;
        this.ap = false;
        this.i = false;
        this.j = false;
        this.k = 0;
        this.f = true;
        this.l = 0;
        this.b = object;
        this.L = this.getWidth();
        this.M = this.getHeight();
        this.aT = 0;
        this.aU = 0;
        this.F = 0;
        this.aV = 100;
        this.N = this.L;
        this.O = this.M;
        this.an = 32;
        this.al = 256 - this.L;
        this.am = 176 - (this.M - this.an);
        this.p = 0;
        this.y = 50;
        this.z = 56;
        this.A = 52;
        this.B = 54;
        this.C = 53;
        this.D = 55;
        this.E = 57;
        this.aX = new Random();
        this.aW = new Vector();
        this.c = true;
        try {
            this.R = Image.createImage(256, 176);
            this.S = Image.createImage(this.L, 32);
            try {
                this.V = Image.createImage("/Images/Zelda_mobile_logo.png");
                this.X = Image.createImage("/Images/Numbers2.png");
            }
            catch (IOException iOException) {}
            this.e = null;
        }
        catch (OutOfMemoryError outOfMemoryError) {
            this.p = -1;
            this.r = true;
        }
        object = this;
        try {
            if (RecordStore.listRecordStores() == null) {
                ((k)object).aG = true;
                ((k)object).y = 50;
                ((k)object).z = 56;
                ((k)object).A = 52;
                ((k)object).B = 54;
                ((k)object).C = 53;
                ((k)object).D = 55;
                ((k)object).E = 57;
                ((k)object).aI = 50;
                ((k)object).aJ = 50;
                ((k)object).aP = false;
                ((k)object).t = true;
                ((k)object).ak = true;
                ((k)object).aO = true;
                ((k)object).x = true;
                ((k)object).w = true;
                ((k)object).ax = 0;
            } else {
                ((k)object).aF = RecordStore.openRecordStore("ZeldaSettings", false);
                object2 = null;
                byte[] byArray = ((k)object).aF.getRecord(1);
                object2 = byArray;
                if (byArray != null) {
                    object2 = new ByteArrayInputStream((byte[])object2);
                    DataInputStream dataInputStream = new DataInputStream((InputStream)object2);
                    ((ByteArrayInputStream)object2).reset();
                    ((k)object).aI = dataInputStream.readInt();
                    ((k)object).aJ = dataInputStream.readInt();
                    ((k)object).D = dataInputStream.readInt();
                    ((k)object).y = dataInputStream.readInt();
                    ((k)object).z = dataInputStream.readInt();
                    ((k)object).A = dataInputStream.readInt();
                    ((k)object).B = dataInputStream.readInt();
                    ((k)object).C = dataInputStream.readInt();
                    ((k)object).E = dataInputStream.readInt();
                    ((k)object).aP = dataInputStream.readBoolean();
                    ((k)object).t = dataInputStream.readBoolean();
                    ((k)object).ak = dataInputStream.readBoolean();
                    ((k)object).aO = dataInputStream.readBoolean();
                    ((k)object).a = dataInputStream.readBoolean();
                    ((k)object).w = dataInputStream.readBoolean();
                    ((k)object).ax = dataInputStream.readInt();
                    ((k)object).x = dataInputStream.readBoolean();
                    if (((k)object).y == 0) {
                        ((k)object).y = 50;
                    }
                    if (((k)object).z == 0) {
                        ((k)object).z = 56;
                    }
                    if (((k)object).A == 0) {
                        ((k)object).A = 52;
                    }
                    if (((k)object).B == 0) {
                        ((k)object).B = 54;
                    }
                    if (((k)object).C == 0) {
                        ((k)object).C = 53;
                    }
                    if (((k)object).E == 0) {
                        ((k)object).E = 57;
                    }
                    if (((k)object).D == 0) {
                        ((k)object).D = 55;
                    }
                } else {
                    ((k)object).y = 50;
                    ((k)object).z = 56;
                    ((k)object).A = 52;
                    ((k)object).B = 54;
                    ((k)object).C = 53;
                    ((k)object).D = 55;
                    ((k)object).E = 57;
                    ((k)object).w = true;
                    ((k)object).ax = 0;
                }
                ((k)object).aF.closeRecordStore();
            }
        }
        catch (Exception exception) {
            ((k)object).y = 50;
            ((k)object).z = 56;
            ((k)object).A = 52;
            ((k)object).B = 54;
            ((k)object).C = 53;
            ((k)object).D = 55;
            ((k)object).E = 57;
            ((k)object).aI = 50;
            ((k)object).aJ = 50;
            ((k)object).aP = false;
            ((k)object).t = true;
            ((k)object).ak = true;
            ((k)object).aO = true;
            ((k)object).a = true;
        }
        if (this.aP) {
            this.setFullScreenMode(true);
        }
        this.aH = 0;
    }

    final void a() {
        try {
            this.aF = RecordStore.openRecordStore("ZeldaSettings", true);
            if (this.aF != null) {
                ByteArrayOutputStream byteArrayOutputStream = new ByteArrayOutputStream();
                DataOutputStream dataOutputStream = new DataOutputStream(byteArrayOutputStream);
                byteArrayOutputStream.reset();
                dataOutputStream.writeInt(this.aI);
                dataOutputStream.writeInt(this.aJ);
                dataOutputStream.writeInt(this.D);
                dataOutputStream.writeInt(this.y);
                dataOutputStream.writeInt(this.z);
                dataOutputStream.writeInt(this.A);
                dataOutputStream.writeInt(this.B);
                dataOutputStream.writeInt(this.C);
                dataOutputStream.writeInt(this.E);
                dataOutputStream.writeBoolean(this.aP);
                dataOutputStream.writeBoolean(this.t);
                dataOutputStream.writeBoolean(this.ak);
                dataOutputStream.writeBoolean(this.aO);
                dataOutputStream.writeBoolean(this.a);
                dataOutputStream.writeBoolean(this.w);
                dataOutputStream.writeInt(this.ax);
                dataOutputStream.writeBoolean(this.x);
                dataOutputStream.flush();
                byte[] byArray = byteArrayOutputStream.toByteArray();
                if (this.aF.getNumRecords() == 0) {
                    this.aF.addRecord(byArray, 0, byArray.length);
                } else {
                    this.aF.setRecord(1, byArray, 0, byArray.length);
                }
                byteArrayOutputStream.reset();
                byteArrayOutputStream.close();
                dataOutputStream.close();
            }
            this.aF.closeRecordStore();
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    final void a(boolean bl) {
        this.m = bl;
        if (bl) {
            if (this.U == null) {
                try {
                    this.U = Image.createImage("/Images/Menuitems.png");
                    this.Z = Image.createImage("/Images/Menu01.png");
                    this.aa = Image.createImage("/Images/MenuSelection01.png");
                    this.ab = Image.createImage("/Images/MenuSelectionArrows.png");
                    this.f = true;
                    return;
                }
                catch (IOException iOException) {
                    return;
                }
            }
        } else {
            this.U = null;
            this.Z = null;
            this.aa = null;
            this.ab = null;
            this.ac = null;
            System.gc();
        }
    }

    final void b() {
        if (this.m) {
            try {
                if (this.n == 1) {
                    this.Z = null;
                    System.gc();
                    this.Z = Image.createImage("/Images/Menu01.png");
                }
                if (this.n == 2) {
                    this.Z = null;
                    System.gc();
                    this.Z = Image.createImage("/Images/Menu02.png");
                }
                if (this.n == 3) {
                    this.Z = null;
                    this.ac = null;
                    System.gc();
                    if (this.az == 0 || this.az == 1) {
                        this.Z = Image.createImage("/Images/OverworldMap.png");
                    }
                    if (this.az == 3) {
                        this.Z = Image.createImage("/Images/Dungeon01.png");
                    }
                    if (this.az == 4) {
                        this.Z = Image.createImage("/Images/Dungeon02.png");
                    }
                    if (this.az == 5) {
                        this.Z = Image.createImage("/Images/Dungeon03.png");
                    }
                    if (this.az == 6) {
                        this.Z = Image.createImage("/Images/Dungeon04.png");
                    }
                    if (this.az == 7) {
                        this.Z = Image.createImage("/Images/Dungeon05.png");
                    }
                    if (this.az == 8) {
                        this.Z = Image.createImage("/Images/Dungeon06.png");
                    }
                    if (this.az == 9) {
                        this.Z = Image.createImage("/Images/Dungeon07.png");
                    }
                    if (this.az == 10) {
                        this.Z = Image.createImage("/Images/Dungeon08.png");
                    }
                    if (this.az == 11) {
                        this.Z = Image.createImage("/Images/Dungeon09.png");
                    }
                    this.ac = Image.createImage("/Images/MapItems.png");
                }
                if (this.n == 4) {
                    this.aa = null;
                    this.ab = null;
                    this.ad = null;
                    System.gc();
                    this.aa = Image.createImage("/Images/Menu03.png");
                    this.ab = Image.createImage("/Images/MenuLink.png");
                    this.ad = Image.createImage("/Images/Life2.png");
                }
                this.f = true;
                return;
            }
            catch (IOException iOException) {}
        }
    }

    private void b(boolean bl) {
        this.V = null;
        this.d = null;
        System.gc();
        this.e = new h();
        this.aD = 0;
        this.aE = 0;
        this.f = true;
        this.g = true;
        this.P = 0;
        this.Q = 0;
        this.p = 0;
        this.aH = 0;
        this.aG = false;
        int n2 = this.aA;
        boolean bl2 = bl;
        Object object = this;
        try {
            if (bl2) {
                if (n2 == 0) {
                    RecordStore.deleteRecordStore("ZeldaSave");
                }
                if (n2 == 1) {
                    RecordStore.deleteRecordStore("ZeldaSave2");
                }
                if (n2 == 2) {
                    RecordStore.deleteRecordStore("ZeldaSave3");
                }
            }
            if (RecordStore.listRecordStores() == null) {
                ((k)object).aG = true;
            }
            byte[] byArray = new byte[200];
            if (n2 == 0) {
                ((k)object).aF = RecordStore.openRecordStore("ZeldaSave", false);
            }
            if (n2 == 1) {
                ((k)object).aF = RecordStore.openRecordStore("ZeldaSave2", false);
            }
            if (n2 == 2) {
                ((k)object).aF = RecordStore.openRecordStore("ZeldaSave3", false);
            }
            if (!((k)object).aG) {
                byArray = ((k)object).aF.getRecord(1);
            }
            if (byArray != null) {
                ((k)object).e.a(byArray, ((k)object).aG, (k)object);
                ((k)object).az = -1;
                ((k)object).H.a(((k)object).e.O, 0);
                ((k)object).az = ((k)object).e.O;
                ((k)object).ay = ((k)object).e.P;
            }
            ((k)object).aF.closeRecordStore();
        }
        catch (RecordStoreException recordStoreException) {
            System.out.println("error Loading char ");
        }
        ((k)object).p = 1;
        ((k)object).r = true;
        ((k)object).H.a(((k)object).e.O, 0);
        ((k)object).e.p = ((k)object).ax == 6 ? ((k)object).e.C : false;
        this.a(this.e.i, this.e.j);
        this.d = new j(this.R.getGraphics(), this.K, this.e, this);
        try {
            this.T = Image.createImage("/Images/Life2.png");
            this.aw = new Vector();
            this.aw.removeAllElements();
            if (!(this.d.a(this.e.i, this.e.j, 0, this.e, false) || this.d.a(this.e.i, this.e.j, 1, this.e, false) || this.d.a(this.e.i, this.e.j, 2, this.e, false) || this.d.a(this.e.i, this.e.j, 3, this.e, false))) {
                this.e.B = true;
                if (!(this.d.a(this.e.i, this.e.j, 0, this.e, false) || this.d.a(this.e.i, this.e.j, 1, this.e, false) || this.d.a(this.e.i, this.e.j, 2, this.e, false) || this.d.a(this.e.i, this.e.j, 3, this.e, false))) {
                    this.e.O = 0;
                    this.e.B = false;
                    this.e.P = 120;
                    this.e.i = 75;
                    this.e.j = 85;
                    this.d = new j(this.R.getGraphics(), this.K, this.e, this);
                }
            }
        }
        catch (IOException iOException) {}
        object = this.e;
        try {
            if (((h)object).V == 0) {
                ((h)object).a = Image.createImage("/Images/LinkWalk.png");
                ((h)object).M = 1;
            }
            if (((h)object).V == 1) {
                ((h)object).a = Image.createImage("/Images/LinkWalk_Blue.png");
                ((h)object).M = 2;
            }
            if (((h)object).V == 2) {
                ((h)object).a = Image.createImage("/Images/LinkWalk_Red.png");
                ((h)object).M = 4;
            }
            if (((h)object).U == 0) {
                ((h)object).b = Image.createImage("/Images/Sword01.png");
                ((h)object).W = 1;
            }
            if (((h)object).U == 1) {
                ((h)object).b = Image.createImage("/Images/Sword02.png");
                ((h)object).W = (byte)2;
            }
            if (((h)object).U == 2) {
                ((h)object).b = Image.createImage("/Images/Sword03.png");
                ((h)object).W = (byte)4;
            }
            return;
        }
        catch (IOException iOException) {
            System.out.println("error loadin Map img");
            return;
        }
    }

    final void c() {
        byte[] byArray;
        this.ae[0] = new h();
        this.ae[1] = new h();
        this.ae[2] = new h();
        System.gc();
        try {
            this.aF = RecordStore.openRecordStore("ZeldaSave", false);
            if (this.aF != null) {
                byArray = null;
                byArray = this.aF.getRecord(1);
                if (byArray != null) {
                    this.ae[0].a(byArray, false, this);
                } else {
                    this.ae[0] = null;
                }
            } else {
                this.ae[0] = null;
            }
            this.aF.closeRecordStore();
        }
        catch (RecordStoreException recordStoreException) {
            System.out.println("error loading char 0");
            this.ae[0] = null;
        }
        try {
            this.aF = RecordStore.openRecordStore("ZeldaSave2", false);
            if (this.aF != null) {
                byArray = null;
                byArray = this.aF.getRecord(1);
                if (byArray != null) {
                    this.ae[1].a(byArray, false, this);
                } else {
                    this.ae[1] = null;
                }
            } else {
                this.ae[1] = null;
            }
            this.aF.closeRecordStore();
        }
        catch (RecordStoreException recordStoreException) {
            System.out.println("error loading char 1");
            this.ae[1] = null;
        }
        try {
            this.aF = RecordStore.openRecordStore("ZeldaSave3", false);
            if (this.aF != null) {
                byArray = null;
                byArray = this.aF.getRecord(1);
                if (byArray != null) {
                    this.ae[2].a(byArray, false, this);
                } else {
                    this.ae[2] = null;
                }
            } else {
                this.ae[2] = null;
            }
            this.aF.closeRecordStore();
        }
        catch (RecordStoreException recordStoreException) {
            System.out.println("error loading char 2");
            this.ae[2] = null;
        }
        System.out.println("exit load all players");
    }

    private void b(int n2) {
        try {
            if (n2 == 0) {
                RecordStore.deleteRecordStore("ZeldaSave");
            }
            if (n2 == 1) {
                RecordStore.deleteRecordStore("ZeldaSave2");
            }
            if (n2 == 2) {
                RecordStore.deleteRecordStore("ZeldaSave3");
            }
            this.c();
            return;
        }
        catch (RecordStoreException recordStoreException) {
            System.out.println("error deleting char " + recordStoreException.toString());
            return;
        }
    }

    final void d() {
        try {
            if (this.e != null) {
                if (this.aA == 0) {
                    this.aF = RecordStore.openRecordStore("ZeldaSave", true);
                }
                if (this.aA == 1) {
                    this.aF = RecordStore.openRecordStore("ZeldaSave2", true);
                }
                if (this.aA == 2) {
                    this.aF = RecordStore.openRecordStore("ZeldaSave3", true);
                }
                if (this.aF != null) {
                    this.e.O = this.az;
                    this.e.P = this.ay;
                    byte[] byArray = this.e.b();
                    if (byArray != null) {
                        if (this.aF.getNumRecords() == 0) {
                            this.aF.addRecord(byArray, 0, byArray.length);
                        } else {
                            this.aF.setRecord(1, byArray, 0, byArray.length);
                        }
                    }
                }
                this.aF.closeRecordStore();
            }
            return;
        }
        catch (RecordStoreException recordStoreException) {
            System.out.println("error saving char");
            return;
        }
    }

    /*
     * Unable to fully structure code
     */
    protected final void paint(Graphics var1_1) {
        block357: {
            block355: {
                block356: {
                    if (!this.aY) {
                        this.H = new b(false, true, -2, 0, this.aI, this.aJ);
                        this.aY = true;
                    }
                    var2_2 = Font.getFont(0, 0, 8);
                    var1_1.setFont((Font)var2_2);
                    if (this.f) {
                        var1_1.setColor(0);
                        var1_1.fillRect(0, 0, this.getWidth(), this.getHeight());
                        this.f = false;
                    }
                    var2_2 = this;
                    if (var2_2.ar && var2_2.au <= System.currentTimeMillis() - 150L) {
                        var3_23 = var2_2.as - (var2_2.e.i - var2_2.P);
                        var4_41 = var2_2.at - (var2_2.e.j - var2_2.Q);
                        if (Math.abs(var3_23) > Math.abs(var4_41)) {
                            if (Math.abs(var3_23) > 4) {
                                if (var3_23 < 0) {
                                    var2_2.a(var2_2.A);
                                } else {
                                    var2_2.a(var2_2.B);
                                }
                            } else {
                                super.c(var2_2.A);
                                super.c(var2_2.B);
                            }
                            if (Math.abs(var4_41) > 4) {
                                if (var4_41 < 0) {
                                    var2_2.a(var2_2.y);
                                } else {
                                    var2_2.a(var2_2.z);
                                }
                            } else {
                                super.c(var2_2.y);
                                super.c(var2_2.z);
                            }
                        } else {
                            if (Math.abs(var4_41) > 4) {
                                if (var4_41 < 0) {
                                    var2_2.a(var2_2.y);
                                } else {
                                    var2_2.a(var2_2.z);
                                }
                            } else {
                                super.c(var2_2.y);
                                super.c(var2_2.z);
                            }
                            if (Math.abs(var3_23) > 4) {
                                if (var3_23 < 0) {
                                    var2_2.a(var2_2.A);
                                } else {
                                    var2_2.a(var2_2.B);
                                }
                            } else {
                                super.c(var2_2.A);
                                super.c(var2_2.B);
                            }
                        }
                    }
                    if (this.h) {
                        this.L = this.getWidth();
                        this.M = this.getHeight();
                        if (this.M != this.O || this.L != this.N) {
                            var1_1.setColor(0);
                            var1_1.fillRect(0, 0, this.getWidth(), this.getHeight());
                            this.O = this.M;
                            this.N = this.L;
                            this.an = 32;
                            this.al = 256 - this.L;
                            this.am = 176 - (this.M - this.an);
                        }
                        this.h = false;
                        this.f = false;
                    }
                    if (this.p == 0) {
                        this.K = var1_1;
                        if (this.v) {
                            if (this.d == null && !this.u) {
                                this.e = null;
                                this.u = true;
                                this.d = new j(this.R.getGraphics(), this.K, this.e, this);
                            } else {
                                var1_1.drawString("Loading..", 0, 0, 20);
                            }
                        }
                        if (this.aG) {
                            this.aH = 1;
                        }
                        if (!this.v) {
                            var1_1.setColor(0);
                            var1_1.fillRect(0, 0, this.getWidth(), this.getHeight());
                            var1_1.translate(this.getWidth() / 2 - 64, 0);
                        } else {
                            if (this.u) {
                                var1_1.drawImage(this.R, -this.aR, 0, 0);
                                var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                                for (var2_3 = 0; var2_3 < this.d.r.size(); ++var2_3) {
                                    ((n)this.d.r.elementAt(var2_3)).a(var1_1, this.aR, 32, this.L, this.M);
                                }
                                for (var2_3 = 0; var2_3 < this.d.p.size(); ++var2_3) {
                                    ((c)this.d.p.elementAt(var2_3)).a(var1_1, this.aR, 32, this.L, this.M);
                                }
                            }
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            var1_1.setClip(0, 0, this.L, this.M);
                            var1_1.clipRect(0, 0, this.L, this.M);
                            this.a(var1_1, true);
                            this.aR += this.aS;
                            if (this.aR + this.L > 256) {
                                this.aS = -1;
                            }
                            if (this.aR < 0) {
                                this.aS = 1;
                            }
                            if (this.L >= 250) {
                                this.aR = 0;
                            }
                        }
                        var2_4 = this.L / 2 - this.V.getWidth() / 2;
                        var1_1.drawImage(this.V, var2_4, 2, 0);
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.translate(var2_4 + 10, 60);
                        var1_1.setColor(0xFFFFFF);
                        if (this.aH == 0) {
                            var1_1.setColor(0xFF0000);
                        }
                        var1_1.translate(0, 10);
                        var1_1.drawString("Start..", 0, 0, 20);
                        var1_1.setColor(0xFFFFFF);
                        if (this.aH == 1) {
                            var1_1.setColor(0xFF0000);
                        }
                        var1_1.translate(0, 10);
                        var1_1.drawString("Config", 0, 0, 20);
                        var1_1.setColor(0xFFFFFF);
                        var1_1.setColor(0xFFFFFF);
                        if (this.aH == 2) {
                            var1_1.setColor(0xFF0000);
                        }
                        var1_1.translate(0, 10);
                        var1_1.drawString("Credits", 0, 0, 20);
                        var1_1.setColor(0xFFFFFF);
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.translate(10, this.getHeight() - 35);
                        var1_1.setColor(0xCCCCCC);
                        var1_1.drawString("This game is FREE!", 0, 0, 20);
                        var1_1.translate(0, 10);
                        var1_1.drawString("Email me: gorthwogh@gmail.com", 0, 0, 20);
                        var1_1.translate(0, 10);
                        var1_1.drawString("(c) - Gorthwogh - 20010 (Beta 4)", 0, 0, 20);
                        System.gc();
                        return;
                    }
                    if (this.p == 7) {
                        if (this.u) {
                            var1_1.drawImage(this.R, -this.aR, 0, 0);
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            for (var2_5 = 0; var2_5 < this.d.r.size(); ++var2_5) {
                                ((n)this.d.r.elementAt(var2_5)).a(var1_1, this.aR, 32, this.L, this.M);
                            }
                        }
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, this.M);
                        var1_1.clipRect(0, 0, this.L, this.M);
                        var3_24 = var1_1;
                        var2_6 = this;
                        var3_24.translate(-var3_24.getTranslateX(), -var3_24.getTranslateY());
                        var4_41 = var2_6.M;
                        var5_46 = var2_6.L;
                        var6_59 = Font.getFont(64, 0, 8);
                        var3_24.setFont(var6_59);
                        var7_68 = var6_59.getHeight();
                        var3_24.setColor(0xCCCCCC);
                        var3_24.fillRoundRect(10, 10, var5_46 - 20, var4_41 - 10, 10, 10);
                        var3_24.setColor(0x555555);
                        var3_24.fillRoundRect(12, 12, var5_46 - 22, var4_41 - 12, 10, 10);
                        var3_24.translate(20, 0);
                        var3_24.translate(0, 20);
                        var3_24.setColor(0xFF0000);
                        var3_24.drawString("Config", 0, 0, 20);
                        var3_24.translate(0, var7_68);
                        var3_24.setColor(0xFFFFFF);
                        if (var2_6.o == 0) {
                            var3_24.setColor(65280);
                        }
                        var3_24.drawString("Key Config", 0, 0, 20);
                        var3_24.translate(0, var7_68);
                        var3_24.setColor(0xFFFFFF);
                        if (var2_6.o == 1) {
                            var3_24.setColor(65280);
                        }
                        var3_24.drawString("Sound", 0, 0, 20);
                        var3_24.translate(0, var7_68);
                        var3_24.setColor(0xFFFFFF);
                        if (var2_6.o == 2) {
                            var3_24.setColor(65280);
                        }
                        var3_24.drawString("Graphics", 0, 0, 20);
                        var3_24.translate(0, var7_68);
                        var3_24.setColor(0xFFFFFF);
                        if (var2_6.o == 3) {
                            var3_24.setColor(65280);
                        }
                        var3_24.drawString("GameSettings", 0, 0, 20);
                        var3_24.translate(0, var7_68);
                        var3_24.setColor(0xFFFFFF);
                        if (var2_6.o == 4) {
                            var3_24.setColor(65280);
                        }
                        var3_24.drawString("Back..", 0, 0, 20);
                        if (var2_6.q == 1) {
                            var3_24.translate(0, var7_68);
                            var3_24.setColor(0xFFFFFF);
                            if (var2_6.o == 5) {
                                var3_24.setColor(65280);
                            }
                            var3_24.drawString("Reset position..", 0, 0, 20);
                        }
                        if (var2_6.q == 1) {
                            var3_24.translate(0, var7_68);
                            var3_24.setColor(0xFFFFFF);
                            if (var2_6.o == 6) {
                                var3_24.setColor(65280);
                            }
                            var3_24.drawString("Save & Exit", 0, 0, 20);
                        }
                        return;
                    }
                    if (this.p == 8) {
                        if (this.u) {
                            var1_1.drawImage(this.R, -this.aR, 0, 0);
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            for (var2_7 = 0; var2_7 < this.d.r.size(); ++var2_7) {
                                ((n)this.d.r.elementAt(var2_7)).a(var1_1, this.aR, 32, this.L, this.M);
                            }
                        }
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, this.M);
                        var1_1.clipRect(0, 0, this.L, this.M);
                        var3_25 = var1_1;
                        var2_8 = this;
                        var4_42 = Font.getFont(64, 0, 8);
                        var3_25.setFont(var4_42);
                        var5_47 = var4_42.getHeight();
                        var3_25.translate(-var3_25.getTranslateX(), -var3_25.getTranslateY());
                        var6_60 = var2_8.M;
                        var7_69 = var2_8.L;
                        var3_25.setColor(0xCCCCCC);
                        var3_25.fillRoundRect(0, 0, var7_69, var6_60, 10, 10);
                        var3_25.setColor(0x555555);
                        var3_25.fillRoundRect(2, 2, var7_69 - 2, var6_60 - 2, 10, 10);
                        var3_25.translate(10, 0);
                        var3_25.translate(0, 5);
                        var3_25.setColor(0xFF0000);
                        var3_25.drawString("Key Config", 0, 0, 20);
                        var3_25.translate(5, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 0) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button UP: " + var2_8.y, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 1) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button DOWN: " + var2_8.z, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 2) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button LEFT: " + var2_8.A, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 3) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button RIGHT: " + var2_8.B, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 4) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button A: " + var2_8.C, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 5) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button B: " + var2_8.D, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 6) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Button MENU: " + var2_8.E, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 7) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("UseJavaButtons: " + var2_8.w, 0, 0, 20);
                        var3_25.translate(0, var5_47);
                        var3_25.setColor(0xFFFFFF);
                        if (var2_8.o == 8) {
                            var3_25.setColor(65280);
                        }
                        var3_25.drawString("Back... ", 0, 0, 20);
                        return;
                    }
                    if (this.p == 9) {
                        if (this.u) {
                            var1_1.drawImage(this.R, -this.aR, 0, 0);
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            for (var2_9 = 0; var2_9 < this.d.r.size(); ++var2_9) {
                                ((n)this.d.r.elementAt(var2_9)).a(var1_1, this.aR, 32, this.L, this.M);
                            }
                        }
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, this.M);
                        var1_1.clipRect(0, 0, this.L, this.M);
                        var3_26 = var1_1;
                        var2_10 = this;
                        var4_43 = Font.getFont(64, 0, 8);
                        var3_26.setFont(var4_43);
                        var5_48 = var4_43.getHeight();
                        var3_26.translate(-var3_26.getTranslateX(), -var3_26.getTranslateY());
                        var6_61 = var2_10.M;
                        var7_70 = var2_10.L;
                        var3_26.setColor(0xCCCCCC);
                        var3_26.fillRoundRect(0, 0, var7_70, var6_61, 10, 10);
                        var3_26.setColor(0x555555);
                        var3_26.fillRoundRect(2, 2, var7_70 - 2, var6_61 - 2, 10, 10);
                        var3_26.translate(10, 0);
                        var3_26.translate(0, 5);
                        var3_26.setColor(0xFF0000);
                        var3_26.drawString("Graphics Config", 0, 0, 20);
                        var3_26.translate(5, var5_48);
                        var3_26.setColor(0xFFFFFF);
                        if (var2_10.o == 0) {
                            var3_26.setColor(65280);
                        }
                        var3_26.drawString("FullScreen: " + var2_10.aP, 0, 0, 20);
                        var3_26.translate(0, var5_48);
                        var3_26.setColor(0xFFFFFF);
                        if (var2_10.o == 1) {
                            var3_26.setColor(65280);
                        }
                        var3_26.drawString("RainEffects: " + var2_10.t, 0, 0, 20);
                        var3_26.translate(0, var5_48);
                        var3_26.setColor(0xFFFFFF);
                        if (var2_10.o == 2) {
                            var3_26.setColor(65280);
                        }
                        var3_26.drawString("FogEffects: " + var2_10.aO, 0, 0, 20);
                        var3_26.translate(0, var5_48);
                        var3_26.setColor(0xFFFFFF);
                        if (var2_10.o == 3) {
                            var3_26.setColor(65280);
                        }
                        var3_26.drawString("AlphaEffects: " + var2_10.a, 0, 0, 20);
                        var3_26.translate(0, var5_48);
                        var3_26.setColor(0xFFFFFF);
                        if (var2_10.o == 4) {
                            var3_26.setColor(65280);
                        }
                        var3_26.drawString("AnimatedWater: " + var2_10.x, 0, 0, 20);
                        var3_26.translate(0, var5_48);
                        var3_26.setColor(0xFFFFFF);
                        if (var2_10.o == 5) {
                            var3_26.setColor(65280);
                        }
                        var3_26.drawString("Back.. ", 0, 0, 20);
                        return;
                    }
                    if (this.p == 12) {
                        if (this.u) {
                            var1_1.drawImage(this.R, -this.aR, 0, 0);
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            for (var2_11 = 0; var2_11 < this.d.r.size(); ++var2_11) {
                                ((n)this.d.r.elementAt(var2_11)).a(var1_1, this.aR, 32, this.L, this.M);
                            }
                        }
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, this.M);
                        var1_1.clipRect(0, 0, this.L, this.M);
                        var3_27 = var1_1;
                        var2_12 = this;
                        var3_27.translate(-var3_27.getTranslateX(), -var3_27.getTranslateY());
                        var4_41 = var2_12.M;
                        var5_49 = var2_12.L;
                        var6_62 = Font.getFont(64, 0, 8);
                        var3_27.setFont(var6_62);
                        var7_71 = var6_62.getHeight();
                        var3_27.setColor(0xCCCCCC);
                        var3_27.fillRoundRect(10, 10, var5_49 - 20, var4_41 - 10, 10, 10);
                        var3_27.setColor(0x555555);
                        var3_27.fillRoundRect(12, 12, var5_49 - 22, var4_41 - 12, 10, 10);
                        var3_27.translate(20, 0);
                        var3_27.translate(0, 20);
                        var3_27.setColor(0xFF0000);
                        var3_27.drawString("Sound settings", 0, 0, 20);
                        var3_27.translate(0, var7_71);
                        var3_27.setColor(0xFFFFFF);
                        if (var2_12.o == 0) {
                            var3_27.setColor(65280);
                        }
                        var3_27.translate(0, var7_71);
                        var3_27.drawString("SoundFX volume", 0, 0, 20);
                        var8_76 = (var5_49 - var3_27.getTranslateX() - 20) / 10;
                        var3_27.setColor(0xAAAAAA);
                        var3_27.translate(0, var7_71 + 4);
                        var3_27.fillRect(0, 0, var5_49 - var3_27.getTranslateX() - 20, 10);
                        if (var2_12.o == 0) {
                            var3_27.setColor(65280);
                        }
                        var3_27.drawRect(0, 0, var5_49 - var3_27.getTranslateX() - 20, 10);
                        var3_27.setColor(0x880000);
                        var3_27.translate(var8_76 * (var2_12.aJ / 10), 0);
                        var3_27.fillRect(1, -1, 5, 12);
                        var3_27.translate(-var8_76 * (var2_12.aJ / 10), 0);
                        var3_27.translate(0, 10);
                        var3_27.setColor(0xFFFFFF);
                        if (var2_12.o == 1) {
                            var3_27.setColor(65280);
                        }
                        var3_27.translate(0, var7_71);
                        var3_27.drawString("Music volume", 0, 0, 20);
                        var3_27.setColor(0xAAAAAA);
                        var3_27.translate(0, var7_71 + 4);
                        var3_27.fillRect(0, 0, var5_49 - var3_27.getTranslateX() - 20, 10);
                        if (var2_12.o == 1) {
                            var3_27.setColor(65280);
                        }
                        var3_27.drawRect(0, 0, var5_49 - var3_27.getTranslateX() - 20, 10);
                        var3_27.setColor(0x880000);
                        var3_27.translate(var8_76 * (var2_12.aI / 10), 0);
                        var3_27.fillRect(1, -1, 5, 12);
                        var3_27.translate(-var8_76 * (var2_12.aI / 10), 0);
                        var3_27.translate(0, 10);
                        var3_27.setColor(0xFFFFFF);
                        if (var2_12.o == 2) {
                            var3_27.setColor(65280);
                        }
                        var3_27.translate(0, var7_71);
                        var3_27.drawString("Back..", 0, 0, 20);
                        return;
                    }
                    if (this.p == 13) {
                        if (this.u) {
                            var1_1.drawImage(this.R, -this.aR, 0, 0);
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            for (var2_13 = 0; var2_13 < this.d.r.size(); ++var2_13) {
                                ((n)this.d.r.elementAt(var2_13)).a(var1_1, this.aR, 32, this.L, this.M);
                            }
                        }
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, this.M);
                        var1_1.clipRect(0, 0, this.L, this.M);
                        var3_28 = var1_1;
                        var2_14 = this;
                        var4_44 = Font.getFont(64, 0, 8);
                        var3_28.setFont(var4_44);
                        var5_50 = var4_44.getHeight();
                        var3_28.translate(-var3_28.getTranslateX(), -var3_28.getTranslateY());
                        var6_63 = var2_14.M;
                        var7_72 = var2_14.L;
                        var3_28.setColor(0xCCCCCC);
                        var3_28.fillRoundRect(0, 0, var7_72, var6_63, 10, 10);
                        var3_28.setColor(0x555555);
                        var3_28.fillRoundRect(2, 2, var7_72 - 2, var6_63 - 2, 10, 10);
                        var3_28.translate(10, 0);
                        var3_28.translate(0, 5);
                        var3_28.setColor(0xFF0000);
                        var3_28.drawString("Game settings", 0, 0, 20);
                        var3_28.translate(5, var5_50);
                        var3_28.setColor(0xFFFFFF);
                        if (var2_14.o == 0) {
                            var3_28.setColor(65280);
                        }
                        var3_28.drawString("Dying: " + var2_14.ak, 0, 0, 20);
                        var3_28.translate(0, var5_50);
                        var3_28.setColor(0xFFFFFF);
                        if (var2_14.o == 1) {
                            var3_28.setColor(65280);
                        }
                        var3_28.drawString("Back.. ", 0, 0, 20);
                        return;
                    }
                    if (this.p == 100) {
                        var3_29 = var1_1;
                        var2_2 = this;
                        var3_29.translate(-var3_29.getTranslateX(), -var3_29.getTranslateY());
                        var4_41 = var2_2.M;
                        var5_51 = var2_2.L;
                        var3_29.setColor(0xCCCCCC);
                        var3_29.fillRoundRect(10, 0, var5_51 - 20, var4_41, 10, 10);
                        var3_29.setColor(0x555555);
                        var3_29.fillRoundRect(12, 2, var5_51 - 22, var4_41 - 2, 10, 10);
                        var3_29.translate(20, 0);
                        var3_29.translate(0, var2_2.aL);
                        var3_29.setColor(0xFF0000);
                        var3_29.drawString("Thank you for playing", 0, 0, 20);
                        var3_29.setColor(0xFFFFFF);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.setColor(0xFF0000);
                        var3_29.drawString("Credits", 0, 0, 20);
                        var3_29.setColor(0xFFFFFF);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Code:", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Gorthwogh", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Music:", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Floaf (X-FormZ)", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Graphics:", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Nintendo.. duh!", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Gorthwogh", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Hansson", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Edwin Bento", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Graphics: (rips)", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Gorthwogh", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("xfinium", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Polar koala", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Helios", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("TRM", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("more.. :P", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Testers: ", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("D-Flax", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Floaf", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Linda - love you.. :P", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Nex", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Uppy", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Fox", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Lance", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Justin", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Sovereign", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Taarna23", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("zfgc.com", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Getjar.com", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Thank you all", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("for your support!", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Gorthwogh - 2009", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("zeldamobile.gorthwogh.com", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.drawString("Gorthwogh@gmail.com", 0, 0, 20);
                        var3_29.translate(0, 16);
                        var3_29.translate(0, 16);
                        if (var3_29.getTranslateY() < 0) {
                            var2_2.aL = var2_2.M;
                        }
                        --var2_2.aL;
                        return;
                    }
                    if (this.p == 10) {
                        if (this.u) {
                            var1_1.drawImage(this.R, -this.aR, 0, 0);
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            for (var2_15 = 0; var2_15 < this.d.r.size(); ++var2_15) {
                                ((n)this.d.r.elementAt(var2_15)).a(var1_1, this.aR, 32, this.L, this.M);
                            }
                        }
                        this.V.getWidth();
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, this.M);
                        var1_1.clipRect(0, 0, this.L, this.M);
                        this.aR += this.aS;
                        if (this.aR + this.L > 256) {
                            this.aS = -1;
                        }
                        if (this.aR < 0) {
                            this.aS = 1;
                        }
                        if (this.L >= 250) {
                            this.aR = 0;
                        }
                        var3_30 = var1_1;
                        var2_16 = this;
                        var3_30.translate(-var3_30.getTranslateX(), -var3_30.getTranslateY());
                        var4_41 = var2_16.M;
                        var5_52 = var2_16.L;
                        var3_30.setColor(0xCCCCCC);
                        var3_30.fillRoundRect(10, 0, var5_52 - 20, var4_41, 10, 10);
                        var3_30.setColor(0x555555);
                        var3_30.fillRoundRect(12, 2, var5_52 - 22, var4_41 - 2, 10, 10);
                        var3_30.translate(20, 0);
                        var3_30.translate(0, var2_16.aL);
                        var3_30.setColor(0xFF0000);
                        var3_30.drawString("Credits", 0, 0, 20);
                        var3_30.setColor(0xFFFFFF);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Code:", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Gorthwogh", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Music:", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Floaf (X-FormZ)", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Graphics:", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Nintendo.. duh!", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Gorthwogh", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Hansson", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Edwin Bento", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Graphics: (rips)", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Gorthwogh", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("xfinium", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Polar koala", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Helios", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("TRM", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("more.. :P", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Testers: ", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("D-Flax", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Floaf", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Linda - love you.. :P", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Nex", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Uppy", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Fox", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Lance", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Justin", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Sovereign", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Taarna23", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("zfgc.com", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Getjar.com", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Thank you all", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("for your support!", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Gorthwogh - 2007", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("zeldamobile.gorthwogh.com", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("Gorthwogh@gmail.com", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.translate(0, 16);
                        var3_30.drawString("CU in the next release..", 0, 0, 20);
                        var3_30.translate(0, 16);
                        var3_30.drawString("DEC - 2008", 0, 0, 20);
                        if (var3_30.getTranslateY() < 0) {
                            var2_16.aL = var2_16.M;
                        }
                        --var2_16.aL;
                        return;
                    }
                    if (this.p == -1) {
                        var1_1.setColor(0);
                        var1_1.fillRect(0, 0, this.getWidth(), this.getHeight());
                        var1_1.setColor(0xFFFFFF);
                        var1_1.translate(this.getWidth() / 2 - 50, this.getHeight() / 3 - 10);
                        var1_1.drawString("Out of memory!", 0, 0, 20);
                        var1_1.setColor(0xFFFFFF);
                        var1_1.translate(0, 10);
                        var1_1.drawString("Try Restarting..", 0, 0, 20);
                        var1_1.translate(0, 10);
                        var1_1.drawString("3 times.. then report this..", 0, 0, 20);
                        var1_1.translate(0, 10);
                        var1_1.drawString("in compability forum", 0, 0, 20);
                        var1_1.translate(0, 20);
                        var1_1.drawString("sorry.. ", 0, 0, 20);
                        return;
                    }
                    if (this.p == 30) {
                        var3_31 = var1_1;
                        var2_2 = this;
                        if (var2_2.aa != null && var2_2.ab != null && var2_2.ad != null) {
                            var3_31.translate(-var3_31.getTranslateX(), -var3_31.getTranslateY());
                            var3_31.translate(var2_2.getWidth() / 2 - 64, 10);
                            var3_31.clipRect(0, 0, 128, 41);
                            var3_31.setClip(0, 0, 128, 41);
                            var3_31.drawImage(var2_2.aa, 0, -26, 0);
                            var3_31.translate(4, 6);
                            var3_31.clipRect(0, 0, 12, 25);
                            var3_31.setClip(0, 0, 12, 25);
                            var3_31.drawImage(var2_2.aa, 0, 0, 0);
                            var3_31.translate(-4, -6);
                            super.a(var3_31, 0);
                            if (var2_2.o > 1) {
                                var3_31.translate(0, 42);
                            } else if (var2_2.o == 0 || var2_2.o == 1) {
                                var3_31.translate(30, 35);
                                var3_31.clipRect(0, 0, 31, 13);
                                var3_31.setClip(0, 0, 31, 13);
                                if (var2_2.o == 0) {
                                    var3_31.drawImage(var2_2.aa, -67, 0, 0);
                                } else {
                                    var3_31.drawImage(var2_2.aa, -36, 0, 0);
                                }
                                var3_31.translate(40, 0);
                                var3_31.clipRect(0, 0, 31, 13);
                                var3_31.setClip(0, 0, 31, 13);
                                if (var2_2.o == 1) {
                                    var3_31.drawImage(var2_2.aa, -67, -13, 0);
                                } else {
                                    var3_31.drawImage(var2_2.aa, -36, -13, 0);
                                }
                                var3_31.translate(-70, 15);
                            } else {
                                var3_31.translate(0, 50);
                            }
                            var3_31.clipRect(0, 0, 128, 41);
                            var3_31.setClip(0, 0, 128, 41);
                            var3_31.drawImage(var2_2.aa, 0, -26, 0);
                            var3_31.translate(4, 6);
                            var3_31.clipRect(0, 0, 12, 25);
                            var3_31.setClip(0, 0, 12, 25);
                            var3_31.drawImage(var2_2.aa, -12, 0, 0);
                            var3_31.translate(-4, -6);
                            super.a(var3_31, 1);
                            if (var2_2.o > 3) {
                                var3_31.translate(0, 42);
                            } else if (var2_2.o == 2 || var2_2.o == 3) {
                                var3_31.translate(30, 35);
                                var3_31.clipRect(0, 0, 31, 13);
                                var3_31.setClip(0, 0, 31, 13);
                                if (var2_2.o == 2) {
                                    var3_31.drawImage(var2_2.aa, -67, 0, 0);
                                } else {
                                    var3_31.drawImage(var2_2.aa, -36, 0, 0);
                                }
                                var3_31.translate(40, 0);
                                var3_31.clipRect(0, 0, 31, 13);
                                var3_31.setClip(0, 0, 31, 13);
                                if (var2_2.o == 3) {
                                    var3_31.drawImage(var2_2.aa, -67, -13, 0);
                                } else {
                                    var3_31.drawImage(var2_2.aa, -36, -13, 0);
                                }
                                var3_31.translate(-70, 15);
                            } else {
                                var3_31.translate(0, 50);
                            }
                            var3_31.clipRect(0, 0, 128, 41);
                            var3_31.setClip(0, 0, 128, 41);
                            var3_31.drawImage(var2_2.aa, 0, -26, 0);
                            var3_31.translate(4, 6);
                            var3_31.clipRect(0, 0, 12, 25);
                            var3_31.setClip(0, 0, 12, 25);
                            var3_31.drawImage(var2_2.aa, -24, 0, 0);
                            var3_31.translate(-4, -6);
                            super.a(var3_31, 2);
                            if (var2_2.o == 4 || var2_2.o == 5) {
                                var3_31.translate(30, 35);
                                var3_31.clipRect(0, 0, 31, 13);
                                var3_31.setClip(0, 0, 31, 13);
                                if (var2_2.o == 4) {
                                    var3_31.drawImage(var2_2.aa, -67, 0, 0);
                                } else {
                                    var3_31.drawImage(var2_2.aa, -36, 0, 0);
                                }
                                var3_31.translate(40, 0);
                                var3_31.clipRect(0, 0, 31, 13);
                                var3_31.setClip(0, 0, 31, 13);
                                if (var2_2.o == 5) {
                                    var3_31.drawImage(var2_2.aa, -67, -13, 0);
                                } else {
                                    var3_31.drawImage(var2_2.aa, -36, -13, 0);
                                }
                                var3_31.translate(-70, 15);
                            } else {
                                var3_31.translate(0, 50);
                            }
                            var2_2.f = true;
                            return;
                        }
                        System.out.println("is null");
                        return;
                    }
                    if (this.p == 6) {
                        var3_32 = var1_1;
                        var2_2 = this;
                        if (var2_2.p == 6) {
                            var3_32.getFont();
                            var5_53 = Font.getFont(0, 0, 8);
                            var3_32.setFont(var5_53);
                            var3_32.translate(-var3_32.getTranslateX(), -var3_32.getTranslateY());
                            var3_32.translate(var2_2.getWidth() / 2 - 50, var2_2.getHeight() / 2 - 40);
                            var3_32.setColor(0);
                            var3_32.fillRoundRect(0, 0, 100, 100, 10, 10);
                            var3_32.setColor(0xFFFFFF);
                            var3_32.translate(5, 0);
                            var3_32.drawString("Press New B button", 0, 0, 20);
                            var3_32.translate(0, 15);
                            var3_32.drawString("Current button: " + var2_2.D, 0, 0, 20);
                            return;
                        }
                        return;
                    }
                    if (this.p >= 20 && this.p <= 26) {
                        var1_1.setColor(0xFFFFFF);
                        var1_1.translate(this.getWidth() / 2 - 50, this.getHeight() / 3 - 10);
                        var1_1.drawString("Press new button", 0, 0, 20);
                        return;
                    }
                    if (this.p == 11) {
                        var1_1.translate(0, 32);
                        var1_1.drawImage(this.R, -this.P, -this.Q, 0);
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, 32);
                        var1_1.clipRect(0, 0, this.L, 32);
                        var1_1.drawImage(this.S, 0, 0, 0);
                        this.b(var1_1);
                        var3_33 = var1_1;
                        var2_2 = this;
                        var3_33.translate(-var3_33.getTranslateX(), -var3_33.getTranslateY());
                        var4_45 = var3_33.getFont();
                        var5_54 = Font.getFont(64, 0, 8);
                        var3_33.setFont(var5_54);
                        try {
                            var6_64 = var2_2.M;
                            var7_73 = var2_2.L;
                            var3_33.clipRect(0, 0, var2_2.L, var2_2.M);
                            var3_33.setClip(0, 0, var2_2.L, var2_2.M);
                            var8_77 = 40;
                            if (var2_2.aQ != 5) {
                                var2_2.W = Image.createImage("/Images/Filter5.png");
                                var2_2.aQ = 5;
                            }
                            var2_2.aK += 8;
                            if (var2_2.aK > 0) {
                                var2_2.aK = 0;
                            }
                            var3_33.translate(0, var2_2.aK);
                            var3_33.drawImage(var2_2.W, 0, 0, 0);
                            if (var2_2.M > 208) {
                                var6_64 = 208;
                            }
                            if (var2_2.L > 256) {
                                var7_73 = 256;
                            }
                            if (var4_45.stringWidth("Game Over") > 60) {
                                var8_77 = var4_45.stringWidth("Game Over") + 10;
                            }
                            var3_33.translate(var7_73 / 2 - var8_77, var6_64 / 2 - 50);
                            var3_33.setColor(0xAAAAAA);
                            var3_33.fillRoundRect(4, 5, var8_77 << 1, 100, 10, 10);
                            var3_33.setColor(0x666666);
                            var3_33.fillRoundRect(6, 7, (var8_77 << 1) - 2, 98, 10, 10);
                            var3_33.setColor(0xFFFFFF);
                            var3_33.translate(20, 8);
                            var3_33.drawString("Game Over", 0, 0, 20);
                            var3_33.translate(0, 20);
                            if (var2_2.aH == 0) {
                                var3_33.setColor(65280);
                                var3_33.translate(-12, 2);
                                var3_33.fillRoundRect(0, 0, 10, 10, 10, 10);
                                var3_33.translate(12, -2);
                            } else {
                                var3_33.setColor(0xFFFFFF);
                            }
                            var3_33.drawString("Continue", 0, 0, 20);
                            var3_33.translate(0, 20);
                            if (var2_2.aH == 1) {
                                var3_33.setColor(65280);
                                var3_33.translate(-12, 2);
                                var3_33.fillRoundRect(0, 0, 10, 10, 10, 10);
                                var3_33.translate(12, -2);
                            } else {
                                var3_33.setColor(0xFFFFFF);
                            }
                            var3_33.drawString("Quit", 0, 0, 20);
                            return;
                        }
                        catch (IOException v0) {
                            var2_2.W = null;
                            var2_2.aQ = 0;
                            return;
                        }
                    }
                    if (this.m) {
                        if (this.n == 1) {
                            var3_34 = var1_1;
                            var2_2 = this;
                            ++var2_2.aj;
                            if (var2_2.aj > 4) {
                                var2_2.aj = 0;
                            }
                            var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                            var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_34.clipRect(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_34.setClip(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_34.drawImage(var2_2.Z, 0, 0, 0);
                            if (var2_2.e.s) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(30, 44);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, 0, 0, 0);
                            }
                            if (var2_2.ax == 0) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(30, 44);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.e.r) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(52, 44);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -16, 0, 0);
                            }
                            if (var2_2.ax == 1) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(52, 44);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.e.t) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(74, 44);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -32, 0, 0);
                            }
                            if (var2_2.ax == 2) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(74, 44);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.e.u) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(96, 44);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -48, 0, 0);
                            }
                            if (var2_2.ax == 3) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(96, 44);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.e.v) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(30, 66);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -48, -32, 0);
                                if (var2_2.ax == 4) {
                                    var3_34.translate(-5, -5);
                                    var3_34.clipRect(0, 0, 28, 26);
                                    var3_34.setClip(0, 0, 28, 26);
                                    var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                                }
                            } else {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(30, 66);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -48, -16, 0);
                                if (var2_2.ax == 4) {
                                    var3_34.translate(-5, -5);
                                    var3_34.clipRect(0, 0, 28, 26);
                                    var3_34.setClip(0, 0, 28, 26);
                                    var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                                }
                            }
                            if (var2_2.e.w) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(54, 66);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -64, 0, 0);
                            }
                            if (var2_2.ax == 5) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(54, 66);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.e.C) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(74, 66);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -64, -16, 0);
                            }
                            if (var2_2.ax == 6) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(74, 66);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.e.y) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(96, 66);
                                var3_34.clipRect(0, 0, 16, 16);
                                var3_34.setClip(0, 0, 16, 16);
                                var3_34.drawImage(var2_2.U, -64, -32, 0);
                            }
                            if (var2_2.ax == 7) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(96, 66);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            if (var2_2.ax == 8) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(23, 89);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 54, 28);
                                var3_34.setClip(0, 0, 54, 28);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, -30, 0);
                            }
                            if (var2_2.ax == 9) {
                                var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                                var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_34.translate(78, 89);
                                var3_34.translate(-5, -5);
                                var3_34.clipRect(0, 0, 54, 28);
                                var3_34.setClip(0, 0, 54, 28);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, -30, 0);
                            }
                            var3_34.getFont();
                            var5_55 = Font.getFont(64, 0, 8);
                            var3_34.setFont(var5_55);
                            var5_55.getHeight();
                            var3_34.setColor(0x333333);
                            var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                            var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_34.translate(32, 22);
                            var3_34.clipRect(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_34.setClip(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            if (var2_2.ax == 0 && var2_2.e.s) {
                                var3_34.drawString("Bombs", 0, 0, 20);
                            }
                            if (var2_2.ax == 1 && var2_2.e.r) {
                                var3_34.drawString("Boomerang", 0, 0, 20);
                            }
                            if (var2_2.ax == 2 && var2_2.e.t) {
                                var3_34.drawString("Bow", 0, 0, 20);
                            }
                            if (var2_2.ax == 3 && var2_2.e.u) {
                                var3_34.drawString("Candle", 0, 0, 20);
                            }
                            if (var2_2.ax == 4) {
                                if (var2_2.e.v) {
                                    var3_34.drawString("Red Potion", 0, 0, 20);
                                } else {
                                    var3_34.drawString("Empty Bottle", 0, 0, 20);
                                }
                            }
                            if (var2_2.ax == 5 && var2_2.e.w) {
                                var3_34.drawString("HookShot", 0, 0, 20);
                            }
                            if (var2_2.ax == 6 && var2_2.e.C) {
                                var3_34.drawString("Shield", 0, 0, 20);
                            }
                            if (var2_2.ax == 7 && var2_2.e.y) {
                                var3_34.drawString("Shovel", 0, 0, 20);
                            }
                            if (var2_2.ax == 8) {
                                var3_34.drawString("Save", 0, 0, 20);
                            }
                            if (var2_2.ax == 9) {
                                var3_34.drawString("Exit", 0, 0, 20);
                            }
                            if (var2_2.ax == 10) {
                                var3_34.drawString("Maps", 0, 0, 20);
                            }
                            if (var2_2.ax == 11) {
                                var3_34.drawString("Status", 0, 0, 20);
                            }
                            var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                            var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_34.translate(-8, 48);
                            var3_34.clipRect(0, 0, 24, 24);
                            var3_34.setClip(0, 0, 24, 24);
                            var3_34.drawImage(var2_2.ab, 0, 0, 0);
                            if (var2_2.ax == 10) {
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            var3_34.translate(-var3_34.getTranslateX(), -var3_34.getTranslateY());
                            var3_34.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_34.translate(132, 48);
                            var3_34.clipRect(0, 0, 24, 24);
                            var3_34.setClip(0, 0, 24, 24);
                            var3_34.drawImage(var2_2.ab, -24, 0, 0);
                            if (var2_2.ax == 11) {
                                var3_34.clipRect(0, 0, 28, 26);
                                var3_34.setClip(0, 0, 28, 26);
                                var3_34.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                        } else if (this.n == 2) {
                            var3_35 = var1_1;
                            var2_2 = this;
                            ++var2_2.aj;
                            if (var2_2.aj > 4) {
                                var2_2.aj = 0;
                            }
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.clipRect(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_35.setClip(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_35.drawImage(var2_2.Z, 0, 0, 0);
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.translate(21, 37);
                            var3_35.clipRect(0, 0, 24, 24);
                            var3_35.setClip(0, 0, 24, 24);
                            var3_35.drawImage(var2_2.e.a, -24, -48, 0);
                            if (var2_2.e.q) {
                                var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                                var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                                var3_35.translate(48, 43);
                                var3_35.clipRect(0, 0, 16, 16);
                                var3_35.setClip(0, 0, 16, 16);
                                var3_35.drawImage(var2_2.U, -16 * var2_2.e.U, -16, 0);
                            }
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.translate(48, 65);
                            var3_35.clipRect(0, 0, 16, 16);
                            var3_35.setClip(0, 0, 16, 16);
                            var3_35.drawImage(var2_2.U, -16 * var2_2.e.V, -32, 0);
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.translate(48, 87);
                            var3_35.clipRect(0, 0, 16, 16);
                            var3_35.setClip(0, 0, 16, 16);
                            var3_35.drawImage(var2_2.U, -16 * (var2_2.e.I % 4), -48, 0);
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.translate(-8, 48);
                            var3_35.clipRect(0, 0, 24, 24);
                            var3_35.setClip(0, 0, 24, 24);
                            var3_35.drawImage(var2_2.ab, 0, -24, 0);
                            if (var2_2.ax == 0) {
                                var3_35.clipRect(0, 0, 28, 26);
                                var3_35.setClip(0, 0, 28, 26);
                                var3_35.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.translate(132, 48);
                            var3_35.clipRect(0, 0, 24, 24);
                            var3_35.setClip(0, 0, 24, 24);
                            var3_35.drawImage(var2_2.ab, -24, -24, 0);
                            if (var2_2.ax == 1) {
                                var3_35.clipRect(0, 0, 28, 26);
                                var3_35.setClip(0, 0, 28, 26);
                                var3_35.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            var3_35.translate(-var3_35.getTranslateX(), -var3_35.getTranslateY());
                            var3_35.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_35.translate(65, 41);
                            var3_35.clipRect(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_35.setClip(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            if (var2_2.e.aA[0]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(32, 10, 25, 25, 39, 25);
                            }
                            if (var2_2.e.aA[1]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(25, 25, 18, 39, 32, 39);
                            }
                            if (var2_2.e.aA[2]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(39, 25, 32, 39, 46, 39);
                            }
                            if (var2_2.e.aA[3]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(18, 39, 11, 53, 25, 53);
                            }
                            if (var2_2.e.aA[4]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(32, 39, 25, 53, 39, 53);
                            }
                            if (var2_2.e.aA[5]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(46, 39, 39, 53, 53, 53);
                            }
                            if (var2_2.e.aA[6]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(25, 25, 39, 25, 32, 39);
                            }
                            if (var2_2.e.aA[7]) {
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(18, 39, 32, 39, 25, 53);
                                var3_35.setColor(0xF8F800);
                                var3_35.fillTriangle(32, 39, 46, 39, 39, 53);
                            }
                        } else if (this.n == 3) {
                            var3_36 = var1_1;
                            var2_2 = this;
                            ++var2_2.aj;
                            if (var2_2.aj > 4) {
                                var2_2.aj = 0;
                            }
                            var3_36.translate(-var3_36.getTranslateX(), -var3_36.getTranslateY());
                            var3_36.translate(0, 32);
                            var3_36.clipRect(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            var3_36.setClip(0, 0, var2_2.getWidth(), var2_2.getHeight());
                            if (var2_2.Z != null) {
                                var4_41 = 0;
                                var5_56 = 0;
                                var6_65 = 256 - var2_2.getWidth();
                                var7_74 = 176 - (var2_2.getHeight() - 32);
                                if (var2_2.az == 0 || var2_2.az == 1) {
                                    var4_41 = (var2_2.ay - 1) % 16 << 4;
                                    var5_56 = (var2_2.ay - 1) / 16 * 22;
                                    var4_41 += var2_2.e.i / 16;
                                    var5_56 += var2_2.e.j / 8;
                                }
                                if (var2_2.az == 3) {
                                    var4_41 = (var2_2.ay - 1) % 6 * 42;
                                    var5_56 = (var2_2.ay - 1) / 6 * 29;
                                    var4_41 += var2_2.e.i / 6;
                                    var5_56 += var2_2.e.j / 6;
                                }
                                if (var2_2.az == 4) {
                                    var4_41 = (var2_2.ay - 1) % 4 << 5;
                                    var5_56 = (var2_2.ay - 1) / 4 * 22;
                                    var4_41 += var2_2.e.i / 8;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 64;
                                }
                                if (var2_2.az == 5) {
                                    var4_41 = (var2_2.ay - 1) % 5 * 42;
                                    var5_56 = (var2_2.ay - 1) / 5 * 29;
                                    var4_41 += var2_2.e.i / 8;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 22;
                                }
                                if (var2_2.az == 6) {
                                    var4_41 = (var2_2.ay - 1) % 4 << 5;
                                    var5_56 = (var2_2.ay - 1) / 4 * 22;
                                    var4_41 += var2_2.e.i / 8;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 64;
                                }
                                if (var2_2.az == 7) {
                                    var4_41 = (var2_2.ay - 1) % 4 << 5;
                                    var5_56 = (var2_2.ay - 1) / 4 * 22;
                                    var4_41 += var2_2.e.i / 8;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 64;
                                }
                                if (var2_2.az == 8) {
                                    var4_41 = (var2_2.ay - 1) % 6 << 5;
                                    var5_56 = (var2_2.ay - 1) / 6 * 22;
                                    var4_41 += var2_2.e.i / 6;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 32;
                                }
                                if (var2_2.az == 9) {
                                    var4_41 = (var2_2.ay - 1) % 6 << 5;
                                    var5_56 = (var2_2.ay - 1) / 6 * 22;
                                    var4_41 += var2_2.e.i / 6;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 32;
                                }
                                if (var2_2.az == 10) {
                                    var4_41 = (var2_2.ay - 1) % 5 << 5;
                                    var5_56 = (var2_2.ay - 1) / 5 * 22;
                                    var4_41 += var2_2.e.i / 5;
                                    var5_56 += var2_2.e.j / 8;
                                    var4_41 += 48;
                                }
                                if (var2_2.az == 11) {
                                    var4_41 = (var2_2.ay - 1) % 8 << 5;
                                    var5_56 = (var2_2.ay - 1) / 8 * 22;
                                    var4_41 += var2_2.e.i / 8;
                                    var5_56 += var2_2.e.j / 8;
                                }
                                if (var4_41 > var2_2.getWidth() / 2) {
                                    if (var4_41 - var2_2.getWidth() / 2 > var6_65) {
                                        var3_36.translate(-var6_65, 0);
                                        var4_41 -= var6_65;
                                    } else {
                                        var3_36.translate(-(var4_41 - var2_2.getWidth() / 2), 0);
                                        var4_41 -= var4_41 - var2_2.getWidth() / 2;
                                    }
                                }
                                if (var5_56 + 32 > var2_2.getHeight() / 2) {
                                    if (var5_56 > var7_74) {
                                        var3_36.translate(0, -var7_74);
                                        var5_56 -= var7_74;
                                    } else {
                                        var3_36.translate(0, -(var5_56 - var2_2.getHeight() / 2));
                                        var5_56 -= var5_56 - var2_2.getHeight() / 2;
                                    }
                                }
                                var3_36.drawImage(var2_2.Z, 0, 0, 0);
                                var6_66 = var3_36;
                                var8_78 = var2_2;
                                if (var8_78.az == 0 || var8_78.az == 1) {
                                    if (!var8_78.e.aA[0]) {
                                        var6_66.translate(116, 70);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[1]) {
                                        var6_66.translate(196, 71);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[2]) {
                                        var6_66.translate(74, 154);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[3]) {
                                        var6_66.translate(84, 93);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[4]) {
                                        var6_66.translate(180, 2);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[5]) {
                                        var6_66.translate(36, 44);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[6]) {
                                        var6_66.translate(36, 93);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else if (!var8_78.e.aA[7]) {
                                        var6_66.translate(212, 133);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 3 <= 1) {
                                            var6_66.drawImage(var8_78.ac, -8, 0, 0);
                                        }
                                    } else {
                                        var6_66.translate(84, 2);
                                        var6_66.clipRect(0, 0, 8, 8);
                                        var6_66.setClip(0, 0, 8, 8);
                                        if (var8_78.aj % 4 < 1) {
                                            var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                        }
                                    }
                                } else if (var8_78.az == 3) {
                                    var6_66.translate(184, 37);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 4) {
                                    var6_66.translate(141, 4);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 5) {
                                    var6_66.translate(208, 68);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 6) {
                                    var6_66.translate(168, 28);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 7) {
                                    var6_66.translate(76, 46);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 8) {
                                    var6_66.translate(172, 28);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 9) {
                                    var6_66.translate(108, 50);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 10) {
                                    var6_66.translate(89, 70);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                } else if (var8_78.az == 11) {
                                    var6_66.translate(109, 73);
                                    var6_66.clipRect(0, 0, 8, 8);
                                    var6_66.setClip(0, 0, 8, 8);
                                    if (var8_78.aj % 4 < 1) {
                                        var6_66.drawImage(var8_78.ac, -16, 0, 0);
                                    }
                                }
                                var3_36.translate(-var3_36.getTranslateX(), -var3_36.getTranslateY());
                                var3_36.translate(0, 32);
                                var3_36.translate(var4_41 - 4, var5_56 - 4);
                                var3_36.clipRect(0, 0, 8, 8);
                                var3_36.setClip(0, 0, 8, 8);
                                if (var2_2.aj % 4 > 1) {
                                    var3_36.drawImage(var2_2.ac, 0, 0, 0);
                                }
                            } else {
                                var3_36.setColor(0xAAAAAA);
                                var3_36.translate(var2_2.getWidth() / 2 - 30, var2_2.getHeight() / 2 - 32);
                                var3_36.drawString("No Map..", 0, 0, 20);
                            }
                            var3_36.translate(-var3_36.getTranslateX(), -var3_36.getTranslateY());
                            var3_36.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_36.translate(-8, 48);
                            var3_36.clipRect(0, 0, 24, 24);
                            var3_36.setClip(0, 0, 24, 24);
                            var3_36.drawImage(var2_2.ab, 0, -48, 0);
                            if (var2_2.ax == 0) {
                                var3_36.clipRect(0, 0, 28, 26);
                                var3_36.setClip(0, 0, 28, 26);
                                var3_36.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                            var3_36.translate(-var3_36.getTranslateX(), -var3_36.getTranslateY());
                            var3_36.translate(var2_2.getWidth() / 2 - 72, 32);
                            var3_36.translate(132, 48);
                            var3_36.clipRect(0, 0, 24, 24);
                            var3_36.setClip(0, 0, 24, 24);
                            var3_36.drawImage(var2_2.ab, -24, -48, 0);
                            if (var2_2.ax == 1) {
                                var3_36.clipRect(0, 0, 28, 26);
                                var3_36.setClip(0, 0, 28, 26);
                                var3_36.drawImage(var2_2.aa, -var2_2.aj / 2 * 54, 0, 0);
                            }
                        }
                        if (this.g) {
                            this.a(this.S.getGraphics());
                            this.g = false;
                        }
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setClip(0, 0, this.L, 32);
                        var1_1.clipRect(0, 0, this.L, 32);
                        var1_1.drawImage(this.S, 0, 0, 0);
                        if (this.aN) {
                            var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                            var1_1.setClip(0, 0, this.L, this.M);
                            var1_1.clipRect(0, 0, this.L, this.M);
                            var5_57 = this.aM;
                            var4_41 = this.M;
                            var3_37 = this.L;
                            var2_2 = var1_1;
                            var2_2.translate(-var2_2.getTranslateX(), -var2_2.getTranslateY());
                            var2_2.setColor(0xAAAAAA);
                            var2_2.fillRect(var3_37 / 2 - 50, var4_41 / 2 - 18, 98, 66);
                            var2_2.setColor(0);
                            var2_2.fillRect(var3_37 / 2 - 48, var4_41 / 2 - 16, 96, 64);
                            var2_2.translate(var3_37 / 2 - 32, var4_41 / 2 + 8);
                            var2_2.setColor(0xFFFFFF);
                            if ((var5_57 >> 1) % 3 == 0) {
                                var2_2.drawString("Saving ", 0, 0, 20);
                            }
                            if ((var5_57 >> 1) % 3 == 1) {
                                var2_2.drawString("Saving.. ", 0, 0, 20);
                            }
                            if ((var5_57 >> 1) % 3 == 2) {
                                var2_2.drawString("Saving.... ", 0, 0, 20);
                            }
                            ++this.aM;
                            if (this.aM > 20) {
                                this.aN = false;
                                this.aM = 0;
                            }
                        }
                        return;
                    }
                    var1_1.getClipX();
                    var1_1.getClipY();
                    var1_1.getClipHeight();
                    var1_1.getClipWidth();
                    this.af = System.currentTimeMillis();
                    if (this.e.F) {
                        this.d.d(this.e);
                    }
                    if (this.e.H == 0) {
                        if (this.ak) {
                            this.p = 11;
                            this.aK = -176;
                            this.aH = 0;
                            this.e.ag = false;
                            return;
                        }
                        if (this.e.ag) {
                            this.e.ag = false;
                        }
                    }
                    if (this.g) {
                        this.a(this.S.getGraphics());
                        this.g = false;
                    }
                    if (!(this.i || this.j || this.d.m)) {
                        var2_17 = 0;
                        var3_23 = 0;
                        if (this.aD != 0 && this.d.a(this.e.i + this.aD, this.e.j, this.e.e, this.e, false)) {
                            var2_17 = this.aD;
                        }
                        if (this.aE != 0 && this.d.a(this.e.i, this.e.j + this.aE, this.e.e, this.e, false)) {
                            var3_23 = this.aE;
                        }
                        var7_75 = this.aE;
                        var6_67 = this.aD;
                        var5_58 = this.d.a(this.e.i + var2_17, this.e.j + var3_23, this.e.e, this.e, true);
                        var4_41 = var3_23;
                        var3_23 = var2_17;
                        var2_2 = this.e;
                        ++var2_2.Y;
                        var8_79 = 0;
                        if (!(var2_2.g || var2_2.n || var2_2.F)) {
                            if (var6_67 < 0) {
                                var2_2.e = 3;
                                ++var8_79;
                            }
                            if (var6_67 > 0) {
                                var2_2.e = 1;
                                ++var8_79;
                            }
                            if (var7_75 < 0) {
                                var2_2.e = 0;
                                ++var8_79;
                            }
                            if (var7_75 > 0) {
                                var2_2.e = 2;
                                ++var8_79;
                            }
                            var2_2.f = var6_67 != 0 || var7_75 != 0 ? ++var2_2.f : 2;
                            if (var2_2.f > 5) {
                                var2_2.f = 0;
                            }
                            if (var5_58 && var8_79 > 0) {
                                var2_2.i += var3_23;
                                var2_2.j += var4_41;
                            }
                        } else {
                            if (var2_2.g) {
                                ++var2_2.f;
                                var2_2.n = false;
                                if (var2_2.f > 5) {
                                    var2_2.g = false;
                                    var2_2.f = 0;
                                }
                            }
                            if (var2_2.n) {
                                ++var2_2.f;
                                ++var2_2.X;
                                var2_2.g = false;
                                if (var2_2.f > 0) {
                                    var2_2.f = 0;
                                    var2_2.X = 0;
                                }
                            }
                        }
                    }
                    if ((var2_18 = this.d.a(this.e.i, this.e.j, false)) > 0 || this.d.g != this.d.h) {
                        if (this.d.g != this.d.h) {
                            if (this.d.f == 9) {
                                if (this.d.g == 0) {
                                    this.d.a("/Maps/Map" + this.ay + "-" + this.az + ".xmap.xmob", this.R.getGraphics(), this.e, this.ay, this.az, this.getWidth(), this.getHeight(), true);
                                    return;
                                }
                                this.d.a("/Maps/Map" + this.ay + "-" + this.az + "-1.xmap.xmob", this.R.getGraphics(), this.e, this.ay, this.az, this.getWidth(), this.getHeight(), true);
                                return;
                            }
                            this.d.h = this.d.g;
                            return;
                        }
                        var3_38 = (e)this.d.o.elementAt(var2_18 - 1);
                        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                        var1_1.setColor(0);
                        var1_1.fillRect(this.getWidth() / 2 - 48, this.getHeight() / 2 - 16, 96, 64);
                        var1_1.translate(this.getWidth() / 2 - 32, this.getHeight() / 2 + 8);
                        var1_1.setColor(0xFFFFFF);
                        var1_1.drawString("Loading.. ", 0, 0, 20);
                        if (this.av == var2_18) {
                            if (var3_38.d == 20 && var3_38.c == 3) {
                                this.p = 100;
                                this.H.a(20, 3);
                                this.r = true;
                                return;
                            }
                            var1_1.translate(-12, 12);
                            var1_1.drawString("Error - go back!!", 0, 0, 20);
                            return;
                        }
                        if (!this.ap) {
                            this.ap = true;
                            return;
                        }
                        if (this.av != var2_18) {
                            var4_41 = 0;
                            if (this.d.f == 9 && var3_38.d == 9) {
                                if ((this.e.an & 1 << this.az) > 0) {
                                    this.d.g = 1;
                                }
                                var4_41 = this.d.g == 0 ? (int)this.d.a("/Maps/Map" + Integer.toString(var3_38.c) + "-" + Integer.toString(var3_38.d) + ".xmap.xmob", this.R.getGraphics(), this.e, var3_38.c, var3_38.d, this.getWidth(), this.getHeight(), true) : (int)this.d.a("/Maps/Map" + Integer.toString(var3_38.c) + "-" + Integer.toString(var3_38.d) + "-1.xmap.xmob", this.R.getGraphics(), this.e, var3_38.c, var3_38.d, this.getWidth(), this.getHeight(), true);
                            } else {
                                this.d.g = (this.e.an & 1 << this.az) > 0 ? 1 : 0;
                                var4_41 = (int)this.d.a("/Maps/Map" + Integer.toString(var3_38.c) + "-" + Integer.toString(var3_38.d) + ".xmap.xmob", this.R.getGraphics(), this.e, var3_38.c, var3_38.d, this.getWidth(), this.getHeight(), true);
                            }
                            if (var4_41 != 0) {
                                this.H.a(var3_38.d, var3_38.c);
                                if (var3_38.d == 0) {
                                    if (var3_38.d != this.az) {
                                        var2_19 = new Random();
                                        var2_19.setSeed(var3_38.c * var3_38.d + var2_19.nextInt());
                                        if (var2_19.nextInt() % 8 == 1) {
                                            this.G = true;
                                        }
                                    }
                                } else {
                                    this.G = false;
                                    if (var3_38.d == 20 && var3_38.c == 2 && this.t) {
                                        this.G = true;
                                    }
                                }
                                this.ay = var3_38.c;
                                this.az = var3_38.d;
                                this.e.P = this.ay;
                                this.e.O = this.az;
                                this.e.i = var3_38.e << 4;
                                this.e.j = (var3_38.f << 4) + 32 - 8;
                                this.P = this.e.i - this.L / 2;
                                this.Q = this.e.j - (this.M + 32) / 2;
                                if (this.P < 0) {
                                    this.P = 0;
                                }
                                if (this.Q < 0) {
                                    this.Q = 0;
                                }
                                if (this.P > this.al) {
                                    this.P = this.al;
                                }
                                if (this.Q > this.am) {
                                    this.Q = this.am;
                                }
                                if (this.M > 208) {
                                    this.Q = 0;
                                }
                                if (this.L > 256) {
                                    this.P = 0;
                                }
                                this.f = true;
                                this.g = true;
                                this.ap = false;
                                this.av = 0;
                                if (!(this.d.f != 9 || this.d.a(this.e.i, this.e.j, 0, this.e, false) || this.d.a(this.e.i, this.e.j, 1, this.e, false) || this.d.a(this.e.i, this.e.j, 2, this.e, false) || this.d.a(this.e.i, this.e.j, 3, this.e, false))) {
                                    this.e.B = true;
                                }
                            } else {
                                this.av = var2_18;
                            }
                        }
                    }
                    if (!(this.i || this.j || this.d.m)) {
                        this.a(this.e.i, this.e.j - 32);
                        if (this.aq) {
                            this.a(this.d.k, this.d.l);
                        }
                        this.d.b(this.e);
                    }
                    if (this.af - this.ag < 1000L) {
                        ++this.ai;
                    } else {
                        this.ag = System.currentTimeMillis();
                        this.ah = this.ai;
                        this.ai = 0;
                    }
                    var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
                    var1_1.translate(0, 32);
                    var1_1.setClip(0, 0, this.L, this.M);
                    var1_1.clipRect(0, 0, this.L, this.M);
                    this.d.a(this.e);
                    var2_20 = this;
                    if (var2_20.az != 12) break block355;
                    if (var2_20.F <= var2_20.aV) break block356;
                    var2_20.aV = Math.abs(var2_20.aX.nextInt() % 200) + 50;
                    var2_20.F = 0;
                    ** GOTO lbl-1000
                }
                if (var2_20.F + 30 > var2_20.aV) {
                    var2_20.aT = var2_20.aX.nextInt() % 5;
                    var2_20.aU = var2_20.aX.nextInt() % 5;
                } else {
                    var2_20.aT = 0;
                    var2_20.aU = 0;
                }
                ++var2_20.F;
                break block357;
            }
            if (var2_20.F > 0) {
                --var2_20.F;
                var2_20.aT = var2_20.aX.nextInt() % 5;
                var2_20.aU = var2_20.aX.nextInt() % 5;
            } else lbl-1000:
            // 2 sources

            {
                var2_20.aT = 0;
                var2_20.aU = 0;
            }
        }
        var1_1.drawImage(this.R, -this.P + this.aT, -this.Q + this.aU, 0);
        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
        var1_1.translate(0, 32);
        var1_1.setClip(0, 0, this.L, this.M);
        var1_1.clipRect(0, 0, this.L, this.M);
        this.aw.removeAllElements();
        for (var2_21 = 0; var2_21 < this.d.r.size(); ++var2_21) {
            if (((n)this.d.r.elementAt((int)var2_21)).a != 12 && ((n)this.d.r.elementAt((int)var2_21)).a != 92 && ((n)this.d.r.elementAt((int)var2_21)).a != 62) {
                if (((n)this.d.r.elementAt((int)var2_21)).z) continue;
                ((n)this.d.r.elementAt(var2_21)).a(var1_1, this.P + this.aT, this.Q + this.aU, this.L, this.M);
                continue;
            }
            this.a(this.d.r.elementAt(var2_21));
        }
        if (this.e.G == 0) {
            this.a(this.e);
        }
        for (var2_21 = 0; var2_21 < this.d.p.size(); ++var2_21) {
            this.a(this.d.p.elementAt(var2_21));
        }
        for (var2_21 = 0; var2_21 < this.d.q.size(); ++var2_21) {
            this.a(this.d.q.elementAt(var2_21));
        }
        for (var2_21 = 0; var2_21 < this.aw.size(); ++var2_21) {
            var3_39 = (f)this.aw.elementAt(var2_21);
            var3_39.a(var1_1, this.P + this.aT, this.Q + this.aU, this.L, this.M);
        }
        if (!(this.i || this.j || this.d.m)) {
            if (this.e.g) {
                var2_21 = this.d.a(this.e, this.H, this.e.q);
                if (var2_21 > 0) {
                    this.g = true;
                }
            } else if (this.e.n) {
                this.d.c(this.e);
            } else if (this.d.a(this.e, this.H) > 0) {
                this.g = true;
            }
            if (this.aq) {
                this.d.a(this.aB, this.aC);
            }
            this.g |= this.d.a(var1_1, this.P + this.aT, this.Q + this.aU, this.L, this.M, this.e, this.H);
        }
        for (var2_21 = 0; var2_21 < this.d.s.size(); ++var2_21) {
            ((n)this.d.s.elementAt(var2_21)).a(var1_1, this.P + this.aT, this.Q + this.aU, this.L, this.M);
        }
        if (this.e.G == 1) {
            this.e.a(var1_1, this.P + this.aT, this.Q + this.aU, this.L, this.M);
        }
        this.d.a(var1_1, this.P + this.aT, this.Q + this.aU, this.L, this.M, this.e);
        this.a(var1_1, true);
        this.b(var1_1, this.P, this.Q);
        var1_1.translate(-var1_1.getTranslateX(), -var1_1.getTranslateY());
        var1_1.setClip(0, 0, this.L, 32);
        var1_1.clipRect(0, 0, this.L, 32);
        var1_1.drawImage(this.S, 0, 0, 0);
        this.b(var1_1);
        if (this.i) {
            this.d.a(this.l, var1_1, this.L, this.M);
        }
        if (this.j) {
            this.d.a(var1_1, this.L, this.M, this.k);
        }
        if (this.d.m) {
            var3_40 = var1_1;
            var2_22 = this.d;
            if (var2_22.d == null) {
                try {
                    var2_22.d = Image.createImage("/Images/BossName" + var2_22.f + ".png");
                }
                catch (IOException v1) {
                    System.out.println("error loadin Map img");
                    return;
                }
            }
            switch (var2_22.f) {
                case 3: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 65, var2_22.j / 2 - 23);
                    var3_40.setClip(0, 0, 131, 47);
                    var3_40.clipRect(0, 0, 131, 47);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 4: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 54, var2_22.j / 2 - 23);
                    var3_40.setClip(0, 0, 109, 47);
                    var3_40.clipRect(0, 0, 109, 47);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 5: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 63, var2_22.j / 2 - 22);
                    var3_40.setClip(0, 0, 127, 45);
                    var3_40.clipRect(0, 0, 127, 45);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 6: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 53, var2_22.j / 2 - 24);
                    var3_40.setClip(0, 0, 106, 48);
                    var3_40.clipRect(0, 0, 106, 48);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 7: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 50, var2_22.j / 2 - 40);
                    var3_40.setClip(0, 0, 100, 80);
                    var3_40.clipRect(0, 0, 100, 80);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 8: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 64, var2_22.j / 2 - 22);
                    var3_40.setClip(0, 0, 128, 44);
                    var3_40.clipRect(0, 0, 128, 44);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 9: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 66, var2_22.j / 2 - 23);
                    var3_40.setClip(0, 0, 132, 46);
                    var3_40.clipRect(0, 0, 132, 46);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                    return;
                }
                case 10: {
                    var3_40.translate(-var3_40.getTranslateX(), -var3_40.getTranslateY());
                    var3_40.translate(var2_22.i / 2 - 66, var2_22.j / 2 - 23);
                    var3_40.setClip(0, 0, 132, 46);
                    var3_40.clipRect(0, 0, 132, 46);
                    var3_40.drawImage(var2_22.d, 0, 0, 0);
                }
            }
        }
    }

    private void a(Object object) {
        for (int i2 = 0; i2 < this.aw.size(); ++i2) {
            if (((f)this.aw.elementAt((int)i2)).j + ((f)this.aw.elementAt((int)i2)).l + ((f)this.aw.elementAt((int)i2)).m <= ((f)object).j + ((f)object).l + ((f)object).m) continue;
            this.aw.insertElementAt(object, i2);
            return;
        }
        this.aw.addElement(object);
    }

    private void a(Graphics graphics, boolean bl) {
        if (this.aO) {
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(0, 32);
            graphics.setClip(0, 0, this.L, this.M);
            graphics.clipRect(0, 0, this.L, this.M);
            if (this.az == 0) {
                try {
                    if (this.ay == 12 || this.ay == 81 || this.ay == 82 || this.ay == 83 || this.ay == 84 || this.ay == 97 || this.ay == 98 || this.ay == 99 || this.ay == 100 || this.ay == 113 || this.ay == 114 || this.ay == 115 || this.ay == 116 || this.ay == 117) {
                        if (this.W == null || this.aQ != 1) {
                            if (this.a) {
                                this.W = Image.createImage("/Images/filter1.png");
                            }
                            this.aQ = 1;
                        }
                        if (this.a) {
                            graphics.drawImage(this.W, -this.P, -this.Q, 0);
                        }
                    }
                    if (this.ay == 58 || this.ay == 68) {
                        if (this.W == null || this.aQ != 3) {
                            if (this.a) {
                                this.W = Image.createImage("/Images/Filter4.png");
                            }
                            this.aQ = 3;
                        }
                        if (this.a) {
                            graphics.drawImage(this.W, -this.P, -this.Q, 0);
                        }
                    }
                }
                catch (IOException iOException) {
                    this.W = null;
                    this.aQ = 0;
                    this.aO = false;
                }
            }
            if (this.az == 100) {
                try {
                    if (this.ay == 1 || this.ay == 2) {
                        if (this.W == null || this.aQ != 2) {
                            if (this.a) {
                                this.W = Image.createImage("/Images/filter1.png");
                            }
                            this.aQ = 2;
                        }
                        if (this.a) {
                            graphics.drawImage(this.W, -this.P, -this.Q, 0);
                        }
                    }
                    if (this.ay == 3) {
                        if (this.W == null || this.aQ != 3) {
                            if (this.a) {
                                this.W = Image.createImage("/Images/Filter4.png");
                            }
                            this.aQ = 3;
                        }
                        if (this.a) {
                            graphics.drawImage(this.W, -this.P, -this.Q, 0);
                        }
                    }
                }
                catch (IOException iOException) {
                    this.W = null;
                    this.aQ = 0;
                    this.aO = false;
                }
            }
            if (this.v && this.p == 0) {
                try {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.setClip(0, 0, this.L, this.M);
                    graphics.clipRect(0, 0, this.L, this.M);
                    if (this.W == null || this.aQ != 3) {
                        if (this.a) {
                            this.W = Image.createImage("/Images/Filter4.png");
                        }
                        this.aQ = 3;
                    }
                    if (this.a) {
                        graphics.drawImage(this.W, -this.aR, 0, 0);
                    }
                }
                catch (IOException iOException) {
                    this.W = null;
                    this.aQ = 0;
                    this.aO = false;
                }
            }
            if (this.az == 12) {
                if (this.W == null || this.aQ != 10) {
                    if (this.a) {
                        try {
                            this.W = Image.createImage("/Images/Dark.png");
                        }
                        catch (IOException iOException) {
                            this.W = null;
                            this.aQ = 0;
                            this.aO = false;
                        }
                    }
                    this.aQ = 10;
                }
                if (this.a) {
                    graphics.drawImage(this.W, -this.P + this.aX.nextInt() % 5, -this.Q + this.aX.nextInt() % 5, 0);
                }
            }
        }
    }

    private void a(Graphics graphics) {
        Font font = graphics.getFont();
        Font font2 = Font.getFont(0, 0, 8);
        graphics.setFont(font2);
        graphics.setColor(0);
        graphics.fillRect(0, 0, this.getWidth(), this.getHeight());
        graphics.setColor(0xFF0000);
        graphics.translate(13, 0);
        graphics.translate(-11, 15);
        int n2 = this.e.H;
        for (int i2 = 0; i2 < this.e.I / 4; ++i2) {
            graphics.clipRect(0, 0, 7, 7);
            graphics.setClip(0, 0, 7, 7);
            if (n2 >= 4) {
                graphics.drawImage(this.T, 0, 0, 0);
            }
            if (n2 == 3) {
                graphics.drawImage(this.T, -7, 0, 0);
            }
            if (n2 == 2) {
                graphics.drawImage(this.T, -14, 0, 0);
            }
            if (n2 == 1) {
                graphics.drawImage(this.T, -21, 0, 0);
            }
            if (n2 < 1) {
                graphics.drawImage(this.T, -28, 0, 0);
            }
            if (i2 == 7) {
                graphics.translate(-64, 8);
            }
            graphics.translate(8, 0);
            n2 -= 4;
        }
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(2, 1);
        graphics.clipRect(0, 0, 8, 8);
        graphics.setClip(0, 0, 8, 8);
        if (this.e.K == 99) {
            graphics.drawImage(this.T, -35, 0, 0);
        }
        if (this.e.K == 500) {
            graphics.drawImage(this.T, -43, 0, 0);
        }
        if (this.e.K == 999) {
            graphics.drawImage(this.T, -51, 0, 0);
        }
        graphics.translate(9, 0);
        graphics.setColor(0xFFFFFF);
        graphics.clipRect(0, 0, this.L, 32);
        graphics.setClip(0, 0, this.L, 32);
        graphics.translate(0, 2);
        this.a(graphics, this.e.J, 0);
        graphics.translate(0, -2);
        graphics.translate(28, 0);
        graphics.clipRect(0, 0, 8, 8);
        graphics.setClip(0, 0, 8, 8);
        graphics.drawImage(this.T, -59, 0, 0);
        graphics.translate(9, 0);
        graphics.clipRect(0, 0, this.L, 32);
        graphics.setClip(0, 0, this.L, 32);
        graphics.translate(0, 2);
        this.a(graphics, this.e.N, 0);
        graphics.translate(0, -2);
        graphics.translate(17, 0);
        graphics.clipRect(0, 0, 8, 8);
        graphics.setClip(0, 0, 8, 8);
        graphics.drawImage(this.T, -67, 0, 0);
        graphics.translate(8, 0);
        graphics.clipRect(0, 0, this.L, 32);
        graphics.setClip(0, 0, this.L, 32);
        graphics.translate(0, 2);
        this.a(graphics, this.e.Q, 0);
        graphics.translate(0, -2);
        graphics.translate(20, 0);
        graphics.clipRect(0, 0, 14, 8);
        graphics.setClip(0, 0, 14, 8);
        graphics.drawImage(this.T, -83, 0, 0);
        graphics.translate(14, 0);
        graphics.clipRect(0, 0, this.L, 32);
        graphics.setClip(0, 0, this.L, 32);
        graphics.translate(0, 2);
        this.a(graphics, this.e.S, 0);
        graphics.translate(0, -2);
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(80, 20);
        graphics.clipRect(0, 0, this.L, 32);
        graphics.setClip(0, 0, this.L, 32);
        graphics.clipRect(0, 0, 7, 7);
        graphics.setClip(0, 0, 7, 7);
        graphics.drawImage(this.X, -73, 0, 0);
        graphics.translate(8, -5);
        graphics.clipRect(0, 0, 7, 16);
        graphics.setClip(0, 0, 7, 16);
        if (this.e.q) {
            graphics.drawImage(this.e.b, 0, 0, 0);
        }
        graphics.translate(10, 5);
        graphics.clipRect(0, 0, this.L, 32);
        graphics.setClip(0, 0, this.L, 32);
        graphics.clipRect(0, 0, 7, 7);
        graphics.setClip(0, 0, 7, 7);
        graphics.drawImage(this.X, -80, 0, 0);
        graphics.translate(8, 0);
        graphics.clipRect(0, 0, 8, 8);
        graphics.setClip(0, 0, 8, 8);
        if (this.ax == 0 && this.e.s) {
            graphics.drawImage(this.T, -67, 0, 0);
        }
        if (this.ax == 1 && this.e.r) {
            graphics.drawImage(this.T, -75, 0, 0);
        }
        if (this.ax == 2 && this.e.t) {
            graphics.clipRect(0, 0, 14, 8);
            graphics.setClip(0, 0, 14, 8);
            graphics.drawImage(this.T, -83, 0, 0);
        }
        if (this.ax == 3 && this.e.u) {
            graphics.drawImage(this.T, -(67 + 8 * this.ax + 6), 0, 0);
        }
        if (this.ax == 4 && this.e.v) {
            graphics.drawImage(this.T, -(67 + 8 * this.ax + 6), 0, 0);
        }
        if (this.ax == 5 && this.e.w) {
            graphics.drawImage(this.T, -(67 + 8 * this.ax + 6), 0, 0);
        }
        if (this.ax == 6 && this.e.C) {
            graphics.drawImage(this.T, -(67 + 8 * this.ax + 6), 0, 0);
        }
        graphics.setFont(font);
    }

    private void a(Graphics graphics, int n2, int n3) {
        int n4;
        int n5 = graphics.getTranslateX();
        if (n3 == 0) {
            graphics.clipRect(0, 0, 3, 7);
            graphics.setClip(0, 0, 3, 7);
            graphics.drawImage(this.X, 0, 0, 0);
            graphics.translate(4, 0);
        }
        if (n3 == 1) {
            graphics.clipRect(0, 0, 9, 7);
            graphics.setClip(0, 0, 9, 7);
            graphics.drawImage(this.X, -88, 0, 0);
            graphics.translate(9, 0);
        }
        n3 = n2;
        if (n2 >= 100) {
            n4 = n2 / 100;
            graphics.clipRect(0, 0, 7, 7);
            graphics.setClip(0, 0, 7, 7);
            graphics.drawImage(this.X, -(n4 * 7 + 3), 0, 0);
            graphics.translate(7, 0);
            n2 -= n4 * 100;
        }
        if (n2 >= 10 || n3 >= 100) {
            n4 = n2 / 10;
            graphics.clipRect(0, 0, 7, 7);
            graphics.setClip(0, 0, 7, 7);
            graphics.drawImage(this.X, -(n4 * 7 + 3), 0, 0);
            graphics.translate(7, 0);
            n2 -= n4 * 10;
        }
        if (n2 >= 0) {
            n4 = n2 % 10;
            graphics.clipRect(0, 0, 7, 7);
            graphics.setClip(0, 0, 7, 7);
            graphics.drawImage(this.X, -(n4 * 7 + 3), 0, 0);
        }
        graphics.translate(n5 - graphics.getTranslateX(), 0);
    }

    public final void pointerPressed(int n2, int n3) {
        this.ar = true;
        this.as = n2 - 12;
        this.at = n3 - 32 + 16;
        this.au = System.currentTimeMillis();
    }

    public final void pointerDragged(int n2, int n3) {
        this.ar = true;
        this.as = n2 - 12;
        this.at = n3 - 32 + 16;
        this.c(this.A);
        this.c(this.B);
        this.c(this.y);
        this.c(this.z);
    }

    public final void pointerReleased(int n2, int n3) {
        this.ar = false;
        if (this.au > System.currentTimeMillis() - 150L) {
            n2 = this.as - (this.e.i - this.P);
            n3 = this.at - (this.e.j - this.Q);
            this.e.e = Math.abs(n2) > Math.abs(n3) ? (n2 < 0 ? 3 : 1) : (n3 < 0 ? 0 : 2);
            this.a(this.C);
        }
        this.c(this.A);
        this.c(this.B);
        this.c(this.y);
        this.c(this.z);
    }

    public final void keyPressed(int n2) {
        if (this.p >= 20 && this.p <= 26) {
            this.a(n2);
            return;
        }
        if (this.w) {
            int n3 = this.getGameAction(n2);
            switch (n3) {
                case 2: {
                    this.a(this.A);
                    return;
                }
                case 5: {
                    this.a(this.B);
                    return;
                }
                case 1: {
                    this.a(this.y);
                    return;
                }
                case 6: {
                    this.a(this.z);
                    return;
                }
                case 8: {
                    this.a(this.C);
                    return;
                }
            }
        }
        this.a(n2);
    }

    public final void keyReleased(int n2) {
        if (this.w) {
            int n3 = this.getGameAction(n2);
            switch (n3) {
                case 2: {
                    this.c(this.A);
                    return;
                }
                case 5: {
                    this.c(this.B);
                    return;
                }
                case 1: {
                    this.c(this.y);
                    return;
                }
                case 6: {
                    this.c(this.z);
                    return;
                }
            }
        }
        this.c(n2);
    }

    private void a(int n2, int n3) {
        this.P = n2 - this.L / 2;
        this.Q = n3 - this.M / 2 + 32;
        if (this.P < 0) {
            this.P = 0;
        }
        if (this.Q < 0) {
            this.Q = 0;
        }
        if (this.P > this.al) {
            this.P = this.al;
        }
        if (this.Q > this.am) {
            this.Q = this.am;
        }
        if (this.L > 256) {
            this.P = 0;
        }
        if (this.M > 208) {
            this.Q = 0;
        }
    }

    private void b(Graphics graphics) {
        int n2 = 0;
        int n3 = this.getHeight();
        if (n3 > 208) {
            n3 = 208;
        }
        switch (this.az) {
            case 0: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(65280);
                graphics.fillRect(2 + ((this.ay - 1) % 16 << 1), n3 - 18 + ((this.ay - 1) / 16 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                n2 = !this.e.aA[0] ? 56 : (!this.e.aA[1] ? 61 : (!this.e.aA[2] ? 117 : (!this.e.aA[3] ? 70 : (!this.e.aA[4] ? 12 : (!this.e.aA[5] ? 35 : (!this.e.aA[6] ? 67 : (!this.e.aA[7] ? 110 : 6)))))));
                if (n2 == 0) break;
                graphics.fillRect(2 + ((n2 - 1) % 16 << 1), n3 - 18 + ((n2 - 1) / 16 << 1), 2, 2);
                return;
            }
            case 1: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(65280);
                graphics.fillRect(2 + ((this.ay - 1) % 16 << 1), n3 - 18 + ((this.ay - 1) / 16 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                n2 = !this.e.aA[0] ? 56 : (!this.e.aA[1] ? 61 : (!this.e.aA[2] ? 117 : (!this.e.aA[3] ? 70 : (!this.e.aA[4] ? 12 : (!this.e.aA[5] ? 35 : (!this.e.aA[6] ? 67 : (!this.e.aA[7] ? 110 : 6)))))));
                if (n2 == 0) break;
                graphics.fillRect(2 + ((n2 - 1) % 16 << 1), n3 - 18 + ((n2 - 1) / 16 << 1), 2, 2);
                return;
            }
            case 3: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 6; ++n2) {
                    for (int i2 = 0; i2 < 6; ++i2) {
                        if (this.aZ[n2][i2] != 1) continue;
                        graphics.fillRect(10 + (i2 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 6 << 1), n3 - 16 + ((this.ay - 1) / 6 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[0]) break;
                graphics.fillRect(20, n3 - 16 + 2, 2, 2);
                return;
            }
            case 4: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 8; ++n2) {
                    for (int i3 = 0; i3 < 4; ++i3) {
                        if (this.ba[n2][i3] != 1) continue;
                        graphics.fillRect(10 + (i3 << 1), (n2 << 1) + n3 - 18, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 4 << 1), n3 - 18 + ((this.ay - 1) / 4 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[1]) break;
                graphics.fillRect(12, n3 - 18, 2, 2);
                return;
            }
            case 5: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 6; ++n2) {
                    for (int i4 = 0; i4 < 5; ++i4) {
                        if (this.bb[n2][i4] != 1) continue;
                        graphics.fillRect(10 + (i4 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 5 << 1), n3 - 16 + ((this.ay - 1) / 5 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[2]) break;
                graphics.fillRect(18, n3 - 16 + 2, 2, 2);
                return;
            }
            case 6: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 8; ++n2) {
                    for (int i5 = 0; i5 < 4; ++i5) {
                        if (this.bc[n2][i5] != 1) continue;
                        graphics.fillRect(10 + (i5 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 4 << 1), n3 - 16 + ((this.ay - 1) / 4 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[3]) break;
                graphics.fillRect(16, n3 - 16, 2, 2);
                return;
            }
            case 7: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 8; ++n2) {
                    for (int i6 = 0; i6 < 4; ++i6) {
                        if (this.bd[n2][i6] != 1) continue;
                        graphics.fillRect(10 + (i6 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 4 << 1), n3 - 16 + ((this.ay - 1) / 4 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[4]) break;
                graphics.fillRect(10, n3 - 16 + 2, 2, 2);
                return;
            }
            case 8: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 8; ++n2) {
                    for (int i7 = 0; i7 < 6; ++i7) {
                        if (this.be[n2][i7] != 1) continue;
                        graphics.fillRect(10 + (i7 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 6 << 1), n3 - 16 + ((this.ay - 1) / 6 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[5]) break;
                graphics.fillRect(18, n3 - 16, 2, 2);
                return;
            }
            case 9: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 8; ++n2) {
                    for (int i8 = 0; i8 < 6; ++i8) {
                        if (this.bf[n2][i8] != 1) continue;
                        graphics.fillRect(10 + (i8 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 6 << 1), n3 - 16 + ((this.ay - 1) / 6 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[6]) break;
                graphics.fillRect(16, n3 - 16 + 4, 2, 2);
                return;
            }
            case 10: {
                graphics.setClip(0, 0, this.getWidth(), this.getHeight());
                graphics.setColor(0);
                graphics.fillRect(2, n3 - 18, 32, 16);
                graphics.setColor(0xFFFFFF);
                graphics.drawRect(2, n3 - 18, 32, 16);
                graphics.setColor(153);
                for (n2 = 0; n2 < 8; ++n2) {
                    for (int i9 = 0; i9 < 5; ++i9) {
                        if (this.bg[n2][i9] != 1) continue;
                        graphics.fillRect(10 + (i9 << 1), (n2 << 1) + n3 - 16, 2, 2);
                    }
                }
                graphics.setColor(65280);
                graphics.fillRect(10 + ((this.ay - 1) % 5 << 1), n3 - 16 + ((this.ay - 1) / 5 << 1), 2, 2);
                graphics.setColor(0xFF0000);
                if (this.e.aA[7]) break;
                graphics.fillRect(12, n3 - 16 + 4, 2, 2);
            }
        }
    }

    private void b(Graphics graphics, int n2, int n3) {
        if (!this.G) {
            return;
        }
        if (!this.t) {
            return;
        }
        if (this.Y == null) {
            try {
                this.Y = this.a ? Image.createImage("/Images/RainEffect.png") : Image.createImage("/Images/RainEffect2.png");
                this.aW.removeAllElements();
                for (int i2 = 0; i2 < 500; ++i2) {
                    g g2 = new g();
                    new g().a = this.aX.nextInt() % 256 + 50;
                    g2.b = this.aX.nextInt() % 176;
                    g2.d = this.aX.nextInt() % 176;
                    this.aW.addElement(g2);
                }
                return;
            }
            catch (IOException iOException) {
                this.Y = null;
                return;
            }
        }
        for (int i3 = 0; i3 < 500; ++i3) {
            g g3 = (g)this.aW.elementAt(i3);
            if (g3.b >= g3.d && g3.c <= 0) {
                g3.a = this.aX.nextInt() % 256 + 50;
                g3.b = 0;
                g3.d = this.aX.nextInt() % 100 + 76;
                g3.c = 6;
            }
            if (g3.b < g3.d) {
                g3.a -= 3;
                g3.b += 8 - g3.d / 44;
            } else {
                --g3.c;
            }
            if (g3.a < 0) {
                g3.b = 176;
            }
            if (g3.b - n3 + 32 + 8 > 208 || g3.b - n3 + 32 + 8 < 0 || g3.a - n2 + 8 > 256 || g3.a - n2 + 8 < 0) continue;
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(g3.a - n2, g3.b - n3 + 32);
            graphics.setClip(0, 0, 8, 8);
            graphics.clipRect(0, 0, 8, 8);
            if (g3.c == 6) {
                graphics.drawImage(this.Y, 0, 0, 0);
                continue;
            }
            if (g3.c > 4) {
                graphics.drawImage(this.Y, -8, 0, 0);
                continue;
            }
            if (g3.c > 2) {
                graphics.drawImage(this.Y, -16, 0, 0);
                continue;
            }
            if (g3.c < 0) continue;
            graphics.drawImage(this.Y, -24, 0, 0);
        }
    }

    final void a(int n2) {
        Object object;
        if (((k)object).e != null && ((k)object).e.E && ((k)object).p == 1) {
            ((k)object).aE = 0;
            ((k)object).aD = 0;
            return;
        }
        if (((k)object).p == 20) {
            ((k)object).y = n2;
            ((k)object).p = 8;
            return;
        }
        if (((k)object).p == 21) {
            ((k)object).z = n2;
            ((k)object).p = 8;
            return;
        }
        if (((k)object).p == 22) {
            ((k)object).A = n2;
            ((k)object).p = 8;
            return;
        }
        if (((k)object).p == 23) {
            ((k)object).B = n2;
            ((k)object).p = 8;
            return;
        }
        if (((k)object).p == 24) {
            ((k)object).C = n2;
            ((k)object).p = 8;
            return;
        }
        if (((k)object).p == 25) {
            ((k)object).D = n2;
            ((k)object).p = 8;
            return;
        }
        if (((k)object).p == 26) {
            ((k)object).E = n2;
            ((k)object).p = 8;
            return;
        }
        if (n2 == ((k)object).A) {
            if (((k)object).p == 10) {
                return;
            }
            if (((k)object).p == 100) {
                return;
            }
            if (((k)object).p == 30) {
                if (((k)object).o != 2 && ((k)object).o != 4) {
                    --((k)object).o;
                }
                if (((k)object).o < 0) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).m) {
                ((k)object).f = true;
                if (((k)object).n == 2) {
                    --((k)object).ax;
                    if (((k)object).ax < 0) {
                        ((k)object).n = 1;
                        ((k)object).ax = 10;
                        ((k)object).b();
                    }
                    return;
                }
                if (((k)object).n == 3) {
                    --((k)object).ax;
                    if (((k)object).ax < 0) {
                        ((k)object).n = 2;
                        ((k)object).ax = 0;
                        ((k)object).b();
                    }
                    return;
                }
                if (((k)object).ax == 4 || ((k)object).ax == 11 || ((k)object).ax == 10 || ((k)object).ax == 8) {
                    if (((k)object).ax != 11) {
                        if (((k)object).ax == 10) {
                            ((k)object).n = 3;
                            ((k)object).ax = 0;
                            ((k)object).b();
                        } else {
                            ((k)object).ax = 10;
                        }
                    } else {
                        ((k)object).ax = 3;
                    }
                } else {
                    --((k)object).ax;
                }
                if (((k)object).ax < 0) {
                    ((k)object).ax = 10;
                }
                ((k)object).g = true;
                ((k)object).f = true;
                ((k)object).e.A = false;
                ((k)object).e.p = false;
                if (((k)object).ax == 6) {
                    if (((k)object).e.C) {
                        ((k)object).e.p = true;
                        return;
                    }
                    ((k)object).e.p = false;
                }
                return;
            }
            if (((k)object).p == 0) {
                return;
            }
            if (((k)object).p == 5) {
                if (((k)object).aH == 0) {
                    ((k)object).H.a(-10);
                }
                if (((k)object).aH == 1) {
                    ((k)object).H.b(-10);
                }
                return;
            }
            if (((k)object).p == 12) {
                if (((k)object).o == 0) {
                    ((k)object).aJ -= 10;
                    if (((k)object).aJ < 0) {
                        ((k)object).aJ = 0;
                    }
                    ((k)object).H.a(((k)object).aJ);
                }
                if (((k)object).o == 1) {
                    ((k)object).aI -= 10;
                    if (((k)object).aI < 0) {
                        ((k)object).aI = 0;
                    }
                    ((k)object).H.b(((k)object).aI);
                }
                return;
            }
            if (((k)object).e.D) {
                ++((k)object).e.ac;
                if (((k)object).e.ac > 6) {
                    ((k)object).e.ac = 1;
                }
                return;
            }
            if (((k)object).aq) {
                ((k)object).aB = -1;
                ((k)object).aC = 0;
                return;
            }
            if (((k)object).e.ag) {
                return;
            }
            ((k)object).aD = -3;
            return;
        }
        if (n2 == ((k)object).B) {
            if (((k)object).p == 10) {
                return;
            }
            if (((k)object).p == 100) {
                return;
            }
            if (((k)object).p == 30) {
                if (((k)object).o != 1 && ((k)object).o != 3) {
                    ++((k)object).o;
                }
                if (((k)object).o > 5) {
                    ((k)object).o = 5;
                }
                return;
            }
            if (((k)object).m) {
                ((k)object).f = true;
                if (((k)object).n == 2) {
                    ++((k)object).ax;
                    if (((k)object).ax > 1) {
                        ((k)object).n = 3;
                        ((k)object).ax = 1;
                        ((k)object).b();
                    }
                    return;
                }
                if (((k)object).n == 3) {
                    ++((k)object).ax;
                    if (((k)object).ax > 1) {
                        ((k)object).n = 1;
                        ((k)object).ax = 11;
                        ((k)object).b();
                    }
                    return;
                }
                ((k)object).ax = ((k)object).ax == 3 || ((k)object).ax == 7 || ((k)object).ax == 10 || ((k)object).ax == 9 ? (((k)object).ax != 10 ? 11 : 0) : ++((k)object).ax;
                if (((k)object).ax > 11) {
                    ((k)object).ax = 1;
                    ((k)object).n = 2;
                    ((k)object).b();
                }
                ((k)object).g = true;
                ((k)object).e.A = false;
                if (((k)object).ax == 6) {
                    if (((k)object).e.C) {
                        ((k)object).e.p = true;
                        return;
                    }
                    ((k)object).e.p = false;
                    return;
                }
                ((k)object).e.p = false;
                return;
            }
            if (((k)object).p == 0) {
                return;
            }
            if (((k)object).p == 5) {
                if (((k)object).aH == 0) {
                    ((k)object).H.a(10);
                }
                if (((k)object).aH == 1) {
                    ((k)object).H.b(10);
                }
                return;
            }
            if (((k)object).p == 12) {
                if (((k)object).o == 0) {
                    ((k)object).aJ += 10;
                    if (((k)object).aJ > 100) {
                        ((k)object).aJ = 100;
                    }
                    ((k)object).H.a(((k)object).aJ);
                }
                if (((k)object).o == 1) {
                    ((k)object).aI += 10;
                    if (((k)object).aI > 100) {
                        ((k)object).aI = 100;
                    }
                    ((k)object).H.b(((k)object).aI);
                }
                return;
            }
            if (((k)object).e.D) {
                --((k)object).e.ac;
                if (((k)object).e.ac < 1) {
                    ((k)object).e.ac = 6;
                }
                return;
            }
            if (((k)object).aq) {
                ((k)object).aB = 1;
                ((k)object).aC = 0;
                return;
            }
            if (((k)object).e.ag) {
                return;
            }
            ((k)object).aD = 3;
            return;
        }
        if (n2 == ((k)object).y) {
            if (((k)object).p == 10) {
                return;
            }
            if (((k)object).p == 100) {
                return;
            }
            if (((k)object).p == 30) {
                ((k)object).o -= 2;
                if (((k)object).o < 0) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).m && ((k)object).n == 1) {
                ((k)object).ax -= 4;
                if (((k)object).ax < 0) {
                    ((k)object).ax = 0;
                }
                ((k)object).g = true;
                ((k)object).f = true;
                ((k)object).e.A = false;
                if (((k)object).ax == 6) {
                    if (((k)object).e.C) {
                        ((k)object).e.p = true;
                        return;
                    }
                    ((k)object).e.p = false;
                    return;
                }
                ((k)object).e.p = false;
                return;
            }
            if (((k)object).p == 7) {
                --((k)object).o;
                if (((k)object).o < 0) {
                    if (((k)object).q != 1) {
                        ((k)object).o = 4;
                        return;
                    }
                    ((k)object).o = 6;
                }
                return;
            }
            if (((k)object).p == 8) {
                --((k)object).o;
                if (((k)object).o < 0) {
                    ((k)object).o = 8;
                }
                return;
            }
            if (((k)object).p == 9) {
                --((k)object).o;
                if (((k)object).o < 0) {
                    ((k)object).o = 5;
                }
                return;
            }
            if (((k)object).p == 13) {
                --((k)object).o;
                if (((k)object).o < 0) {
                    ((k)object).o = 1;
                }
                return;
            }
            if (((k)object).p == 0) {
                --((k)object).aH;
                if (((k)object).aH < 0) {
                    ((k)object).aH = 2;
                }
                return;
            }
            if (((k)object).p == 5) {
                --((k)object).aH;
                if (((k)object).aH < 0) {
                    ((k)object).aH = 1;
                }
                return;
            }
            if (((k)object).p == 12) {
                --((k)object).o;
                if (((k)object).o < 0) {
                    ((k)object).o = 2;
                }
                return;
            }
            if (((k)object).p == 11) {
                --((k)object).aH;
                if (((k)object).aH < 0) {
                    ((k)object).aH = 1;
                }
                return;
            }
            if (((k)object).e.D) {
                return;
            }
            if (((k)object).aq) {
                ((k)object).aB = 0;
                ((k)object).aC = -1;
                return;
            }
            if (((k)object).e.ag) {
                return;
            }
            ((k)object).aE = -3;
            return;
        }
        if (n2 == ((k)object).z) {
            if (((k)object).p == 10) {
                return;
            }
            if (((k)object).p == 100) {
                return;
            }
            if (((k)object).p == 30) {
                ((k)object).o += 2;
                if (((k)object).o > 5) {
                    ((k)object).o = 4;
                }
                return;
            }
            if (((k)object).m && ((k)object).n == 1) {
                ((k)object).ax += 4;
                if (((k)object).ax > 9) {
                    ((k)object).ax = 9;
                }
                ((k)object).g = true;
                ((k)object).f = true;
                ((k)object).e.A = false;
                ((k)object).e.F = false;
                if (((k)object).ax == 6) {
                    if (((k)object).e.C) {
                        ((k)object).e.p = true;
                        return;
                    }
                    ((k)object).e.p = false;
                    return;
                }
                ((k)object).e.p = false;
                return;
            }
            if (((k)object).p == 0) {
                ++((k)object).aH;
                if (((k)object).aH > 2) {
                    ((k)object).aH = 0;
                }
                return;
            }
            if (((k)object).p == 5) {
                ++((k)object).o;
                if (((k)object).o > 1) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).p == 12) {
                ++((k)object).o;
                if (((k)object).o > 2) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).p == 7) {
                ++((k)object).o;
                if (((k)object).q != 1) {
                    if (((k)object).o > 4) {
                        ((k)object).o = 0;
                        return;
                    }
                } else if (((k)object).o > 6) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).p == 8) {
                ++((k)object).o;
                if (((k)object).o > 8) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).p == 9) {
                ++((k)object).o;
                if (((k)object).o > 5) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).p == 13) {
                ++((k)object).o;
                if (((k)object).o > 1) {
                    ((k)object).o = 0;
                }
                return;
            }
            if (((k)object).p == 11) {
                ++((k)object).aH;
                if (((k)object).aH > 1) {
                    ((k)object).aH = 0;
                }
                return;
            }
            if (((k)object).e.D) {
                return;
            }
            if (((k)object).aq) {
                ((k)object).aB = 0;
                ((k)object).aC = 1;
                return;
            }
            if (((k)object).e.ag) {
                return;
            }
            ((k)object).aE = 3;
            return;
        }
        if (n2 == ((k)object).C) {
            if (((k)object).p == 10) {
                ((k)object).p = 0;
                ((k)object).f = true;
                return;
            }
            ((k)object).aq = false;
            if (((k)object).m) {
                if (((k)object).ax == 8) {
                    ((k)object).d();
                    ((k)object).aN = true;
                    ((k)object).aM = 0;
                    return;
                }
                if (((k)object).ax == 9) {
                    ((k)object).s = true;
                    return;
                }
                ((k)object).m = false;
                ((k)object).r = true;
                return;
            }
            if (((k)object).p == 30) {
                if (((k)object).o == 0) {
                    ((k)object).aA = 0;
                    if (((k)object).ae[0] != null) {
                        ((k)object).b(false);
                    } else {
                        ((k)object).b(true);
                    }
                }
                if (((k)object).o == 1) {
                    ((k)object).b(0);
                }
                if (((k)object).o == 2) {
                    ((k)object).aA = 1;
                    if (((k)object).ae[1] != null) {
                        ((k)object).b(false);
                    } else {
                        ((k)object).b(true);
                    }
                }
                if (((k)object).o == 3) {
                    ((k)object).b(1);
                }
                if (((k)object).o == 4) {
                    ((k)object).aA = 2;
                    if (((k)object).ae[2] != null) {
                        ((k)object).b(false);
                    } else {
                        ((k)object).b(true);
                    }
                }
                if (((k)object).o == 5) {
                    ((k)object).b(2);
                }
                return;
            }
            if (((k)object).p == 10) {
                ((k)object).p = 0;
                ((k)object).f = true;
                return;
            }
            if (((k)object).p == 13) {
                if (((k)object).o == 0) {
                    ((k)object).ak = !((k)object).ak;
                    ((k)object).h = true;
                    return;
                }
                if (((k)object).o == 1) {
                    ((k)object).p = 7;
                    ((k)object).h = true;
                    ((k)object).o = 3;
                    return;
                }
            }
            if (((k)object).p == 12) {
                if (((k)object).o == 2) {
                    ((k)object).p = 7;
                    ((k)object).f = true;
                    ((k)object).o = 1;
                    return;
                }
                return;
            }
            if (((k)object).p == 7) {
                if (((k)object).o == 4) {
                    ((k)object).p = ((k)object).q;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    ((k)object).o = 0;
                    return;
                }
                if (((k)object).o == 0) {
                    ((k)object).p = 8;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 1) {
                    ((k)object).p = 12;
                    ((k)object).o = 0;
                    ((k)object).aH = 0;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 2) {
                    ((k)object).p = 9;
                    ((k)object).o = 0;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 3) {
                    ((k)object).p = 13;
                    ((k)object).o = 0;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 5) {
                    ((k)object).ay = 120;
                    ((k)object).e.O = ((k)object).az = 0;
                    ((k)object).e.B = false;
                    ((k)object).e.P = ((k)object).ay;
                    ((k)object).e.i = 110;
                    ((k)object).e.j = 110;
                    ((k)object).d.a("/Maps/Map" + ((k)object).ay + "-" + ((k)object).az + ".xmap.xmob", ((k)object).R.getGraphics(), ((k)object).e, ((k)object).az, ((k)object).ay, ((Displayable)object).getWidth(), ((Canvas)object).getHeight(), true);
                    ((k)object).P = ((k)object).e.i - ((k)object).L / 2;
                    ((k)object).Q = ((k)object).e.j - (((k)object).M + 32) / 2;
                    if (((k)object).P < 0) {
                        ((k)object).P = 0;
                    }
                    if (((k)object).Q < 0) {
                        ((k)object).Q = 0;
                    }
                    if (((k)object).P > ((k)object).al) {
                        ((k)object).P = ((k)object).al;
                    }
                    if (((k)object).Q > ((k)object).am) {
                        ((k)object).Q = ((k)object).am;
                    }
                    if (((k)object).M > 208) {
                        ((k)object).Q = 0;
                    }
                    if (((k)object).L > 256) {
                        ((k)object).P = 0;
                    }
                    ((k)object).H.a(((k)object).az, ((k)object).ay);
                    ((k)object).f = true;
                    ((k)object).g = true;
                    ((k)object).ap = false;
                    ((k)object).av = 0;
                    ((k)object).p = ((k)object).q;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 6) {
                    ((k)object).d();
                    ((k)object).s = true;
                    return;
                }
                return;
            }
            if (((k)object).p == 8) {
                if (((k)object).o == 7) {
                    ((k)object).h = true;
                    ((k)object).f = true;
                    ((k)object).w = !((k)object).w;
                    return;
                }
                if (((k)object).o == 8) {
                    ((k)object).p = 7;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    ((k)object).o = 0;
                    return;
                }
                if (((k)object).o == 0) {
                    ((k)object).p = 20;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 1) {
                    ((k)object).p = 21;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 2) {
                    ((k)object).p = 22;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 3) {
                    ((k)object).p = 23;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 4) {
                    ((k)object).p = 24;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 5) {
                    ((k)object).p = 25;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                if (((k)object).o == 6) {
                    ((k)object).p = 26;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    return;
                }
                return;
            }
            if (((k)object).p == 9) {
                if (((k)object).o == 0) {
                    ((k)object).aP = !((k)object).aP;
                    ((Canvas)object).setFullScreenMode(((k)object).aP);
                    ((k)object).h = true;
                    return;
                }
                if (((k)object).o == 1) {
                    ((k)object).t = !((k)object).t;
                    ((k)object).h = true;
                    return;
                }
                if (((k)object).o == 2) {
                    ((k)object).aO = !((k)object).aO;
                    ((k)object).h = true;
                    return;
                }
                if (((k)object).o == 3) {
                    ((k)object).a = !((k)object).a;
                    ((k)object).d.c = null;
                    ((k)object).h = true;
                    return;
                }
                if (((k)object).o == 4) {
                    ((k)object).x = !((k)object).x;
                    return;
                }
                if (((k)object).o == 5) {
                    ((k)object).p = 7;
                    ((k)object).o = 2;
                    return;
                }
                return;
            }
            if (((k)object).p == 5) {
                if (((k)object).o == 2) {
                    ((k)object).p = 6;
                    ((k)object).f = true;
                    ((k)object).o = 3;
                }
                return;
            }
            if (((k)object).p == 11) {
                if (((k)object).aK < 0) {
                    return;
                }
                if (((k)object).aH == 0) {
                    ((k)object).e.H = 12;
                    ((k)object).e.O = 0;
                    ((k)object).e.B = false;
                    ((k)object).e.P = 120;
                    ((k)object).e.i = 75;
                    ((k)object).e.j = 85;
                    ((k)object).ay = 120;
                    ((k)object).az = 0;
                    ((k)object).d();
                    ((k)object).d.a("/Maps/Map120-0.xmap.xmob", ((k)object).R.getGraphics(), ((k)object).e, 0, 120, ((Displayable)object).getWidth(), ((Canvas)object).getHeight(), true);
                    ((k)object).P = ((k)object).e.i - ((k)object).L / 2;
                    ((k)object).Q = ((k)object).e.j - (((k)object).M + 32) / 2;
                    if (((k)object).P < 0) {
                        ((k)object).P = 0;
                    }
                    if (((k)object).Q < 0) {
                        ((k)object).Q = 0;
                    }
                    if (((k)object).P > ((k)object).al) {
                        ((k)object).P = ((k)object).al;
                    }
                    if (((k)object).Q > ((k)object).am) {
                        ((k)object).Q = ((k)object).am;
                    }
                    if (((k)object).M > 208) {
                        ((k)object).Q = 0;
                    }
                    if (((k)object).L > 256) {
                        ((k)object).P = 0;
                    }
                    ((k)object).H.a(0, 120);
                    ((k)object).f = true;
                    ((k)object).g = true;
                    ((k)object).ap = false;
                    ((k)object).av = 0;
                    ((k)object).r = true;
                    ((k)object).f = true;
                    ((k)object).p = 1;
                    return;
                }
                if (((k)object).aH == 1) {
                    ((k)object).s = true;
                    ((k)object).J.destroyApp(false);
                }
                return;
            }
            if (((k)object).p == 0) {
                if (((k)object).aH == 0) {
                    ((k)object).p = 30;
                    ((k)object).m = true;
                    ((k)object).n = 4;
                    ((k)object).b();
                    ((k)object).c();
                    ((k)object).m = false;
                    ((k)object).n = 0;
                    return;
                }
                if (((k)object).aH == 1) {
                    ((k)object).p = 7;
                }
                if (((k)object).aH == 2) {
                    ((k)object).p = 10;
                }
                return;
            }
            if (((k)object).i || ((k)object).j || ((k)object).d.m) {
                ((k)object).e.g = false;
            }
            if (!((k)object).e.g && (((k)object).i || ((k)object).j || ((k)object).d.m)) {
                if (((k)object).j) {
                    ((k)object).j = false;
                    ((k)object).k = 0;
                    return;
                }
                if (((k)object).d.m) {
                    ((k)object).d.m = false;
                    return;
                }
                n n3 = (n)((k)object).d.r.elementAt(((k)object).l);
                ((k)object).i = false;
                ((k)object).k = 0;
                if (n3.a == 16) {
                    return;
                }
                int n4 = ((k)object).ay / 32;
                int n5 = n3.c % 32;
                if (n3.c == 0) {
                    if (n3.u < 2) {
                        ((k)object).d.r.removeElementAt(((k)object).l);
                    }
                    return;
                }
                if (((k)object).az == 0) {
                    int n6 = n4;
                    ((k)object).e.ap[n6] = ((k)object).e.ap[n6] | 1 << n5;
                }
                if (((k)object).az == 1) {
                    int n7 = n4;
                    ((k)object).e.aq[n7] = ((k)object).e.aq[n7] | 1 << n5;
                }
                if (((k)object).az == 3) {
                    int n8 = n4;
                    ((k)object).e.ar[n8] = ((k)object).e.ar[n8] | 1 << n5;
                }
                if (((k)object).az == 4) {
                    int n9 = n4;
                    ((k)object).e.as[n9] = ((k)object).e.as[n9] | 1 << n5;
                }
                if (((k)object).az == 5) {
                    int n10 = n4;
                    ((k)object).e.at[n10] = ((k)object).e.at[n10] | 1 << n5;
                }
                if (((k)object).az == 6) {
                    int n11 = n4;
                    ((k)object).e.au[n11] = ((k)object).e.au[n11] | 1 << n5;
                }
                if (((k)object).az == 7) {
                    int n12 = n4;
                    ((k)object).e.av[n12] = ((k)object).e.av[n12] | 1 << n5;
                }
                if (((k)object).az == 8) {
                    int n13 = n4;
                    ((k)object).e.aw[n13] = ((k)object).e.aw[n13] | 1 << n5;
                }
                if (((k)object).az == 9) {
                    int n14 = n4;
                    ((k)object).e.ax[n14] = ((k)object).e.ax[n14] | 1 << n5;
                }
                if (((k)object).az == 10) {
                    int n15 = n4;
                    ((k)object).e.ay[n15] = ((k)object).e.ay[n15] | 1 << n5;
                }
                if (((k)object).az == 11) {
                    int n16 = n4;
                    ((k)object).e.az[n16] = ((k)object).e.az[n16] | 1 << n5;
                }
                ((k)object).d.r.removeElementAt(((k)object).l);
                return;
            }
            if (((k)object).e.q) {
                if (((k)object).e.g) {
                    return;
                }
                if (((k)object).e.ah) {
                    return;
                }
                ((k)object).i = false;
                ((k)object).j = false;
                ((k)object).d.m = false;
                ((k)object).k = 0;
                ((k)object).e.g = true;
                ((k)object).e.F = false;
                ((k)object).e.ae = 0;
                ((k)object).e.n = false;
                ((k)object).e.ai.removeAllElements();
                ((k)object).e.o = false;
                ((k)object).e.X = 0;
                ((k)object).e.ah = false;
                ((k)object).e.f = 1;
                n2 = ((k)object).d.a(((k)object).e, ((k)object).H, ((k)object).e.q);
                if (n2 > 0) {
                    ((k)object).g = true;
                }
                if (n2 == 2) {
                    ((k)object).e.g = false;
                }
                if (((k)object).e.g) {
                    n2 = 1;
                    object = ((k)object).H;
                    ((k)object).H.d = n2;
                    return;
                }
            } else {
                ((k)object).i = false;
                ((k)object).j = false;
                ((k)object).d.m = false;
                ((k)object).k = 0;
                n2 = ((k)object).d.a(((k)object).e, ((k)object).H, ((k)object).e.q);
                if (n2 > 0) {
                    ((k)object).g = true;
                }
            }
            return;
        }
        if (n2 == ((k)object).E) {
            if (((k)object).p == 10) {
                return;
            }
            if (((k)object).p == 100) {
                return;
            }
            ((k)object).aq = false;
            ++((k)object).n;
            if (((k)object).n > 2) {
                ((k)object).n = 0;
            }
            if (((k)object).n != 0) {
                ((k)object).m = true;
                ((k)object).a(((k)object).m);
                ((k)object).r = true;
                return;
            }
            ((k)object).m = false;
            ((k)object).a(((k)object).m);
            ((k)object).r = true;
            return;
        }
        if (n2 == ((k)object).D) {
            if (((k)object).p == 10) {
                return;
            }
            if (((k)object).m) {
                return;
            }
            if (((k)object).ax == 0 && ((k)object).e.s && !((k)object).e.B) {
                boolean bl;
                k k2 = object;
                h h2 = ((k)object).e;
                j j2 = ((k)object).d;
                if (h2.Q > 0) {
                    int n17 = 0;
                    int n18 = 0;
                    if (h2.e == 0) {
                        n18 = -8;
                    }
                    if (h2.e == 1) {
                        n17 = 16;
                    }
                    if (h2.e == 2) {
                        n18 = 16;
                    }
                    if (h2.e == 3) {
                        n17 = -8;
                    }
                    object = new c(j2.b.a(4), 4, h2.i + n17, h2.j + n18 - 32, 1, h2);
                    j2.p.addElement(object);
                    h2.Q = (byte)(h2.Q - 1);
                    bl = true;
                } else {
                    bl = false;
                }
                k2.g = bl;
                return;
            }
            if (((k)object).ax == 1 && ((k)object).e.r && !((k)object).e.B) {
                if (!((k)object).e.h) {
                    h h3 = ((k)object).e;
                    j j3 = ((k)object).d;
                    if (!h3.h) {
                        int n19 = 0;
                        int n20 = 0;
                        if (h3.e == 0) {
                            n20 = -1;
                        }
                        if (h3.e == 1) {
                            n19 = 1;
                        }
                        if (h3.e == 2) {
                            n20 = 1;
                        }
                        if (h3.e == 3) {
                            n19 = -1;
                        }
                        p p2 = new p(j3.b.b(8), 8, n19, n20);
                        new p(j3.b.b(8), 8, n19, n20).i = h3.i + 8 + n19 * 12;
                        p2.j = h3.j + 12 + n20 * 16;
                        h3.h = true;
                        p2.p = false;
                        j3.n.addElement(p2);
                        h3.f = 6;
                        j3.k = p2.i;
                        j3.l = p2.j;
                    }
                    ((k)object).aq = true;
                    ((k)object).aB = 0;
                    ((k)object).aC = 0;
                    if (((k)object).e.e == 0) {
                        ((k)object).aC = -1;
                    }
                    if (((k)object).e.e == 1) {
                        ((k)object).aB = 1;
                    }
                    if (((k)object).e.e == 2) {
                        ((k)object).aC = 1;
                    }
                    if (((k)object).e.e == 3) {
                        ((k)object).aB = -1;
                    }
                }
                return;
            }
            if (((k)object).ax == 2 && ((k)object).e.t && !((k)object).e.B) {
                boolean bl;
                h h4 = ((k)object).e;
                j j4 = ((k)object).d;
                if (h4.S > 0 && h4.ao <= 0) {
                    h4.S = (byte)(h4.S - 1);
                    int n21 = 0;
                    int n22 = 0;
                    if (h4.e == 0) {
                        n21 = -1;
                    }
                    if (h4.e == 1) {
                        n22 = 1;
                    }
                    if (h4.e == 2) {
                        n21 = 1;
                    }
                    if (h4.e == 3) {
                        n22 = -1;
                    }
                    h4.ao = 10;
                    p p3 = new p(j4.b.b(3), 3, n22, n21);
                    new p(j4.b.b(3), 3, n22, n21).i = h4.i + 8 + n22 * 12;
                    p3.j = h4.j + 12 + n21 * 16;
                    p3.p = false;
                    j4.n.addElement(p3);
                    bl = true;
                } else {
                    bl = false;
                }
                ((k)object).g = bl;
                return;
            }
            if (((k)object).ax == 3 && ((k)object).e.u && !((k)object).e.B) {
                ((k)object).e.A = !((k)object).e.A;
                return;
            }
            if (((k)object).ax == 4 && ((k)object).e.v && !((k)object).e.B) {
                ((k)object).e.H = ((k)object).e.I;
                ((k)object).e.v = false;
                ((k)object).g = true;
                return;
            }
            if (((k)object).ax == 5 && ((k)object).e.w && !((k)object).e.B) {
                if (!((k)object).e.n) {
                    if (((k)object).e.c == null) {
                        ((k)object).e.c = ((k)object).d.b.b(4);
                    }
                    ((k)object).e.ai.removeAllElements();
                    ((k)object).e.o = false;
                    ((k)object).e.X = 0;
                    ((k)object).e.ah = false;
                    ((k)object).e.n = true;
                    ((k)object).d.c(((k)object).e);
                    return;
                }
                return;
            }
            if (((k)object).e.B && ((k)object).e.z) {
                ((k)object).e.D = !((k)object).e.D;
                ((k)object).e.ac = 1;
                ((k)object).e.aa = ((k)object).e.i;
                ((k)object).e.ab = ((k)object).e.j + 16;
                if (((k)object).e.e == 1) {
                    ((k)object).e.aa += 20;
                }
                if (((k)object).e.e == 3) {
                    ((k)object).e.aa -= 20;
                }
                return;
            }
            if (((k)object).ax == 7 && ((k)object).e.y && !((k)object).e.B) {
                if (!((k)object).e.F) {
                    ((k)object).d.d(((k)object).e);
                }
                return;
            }
            return;
        }
    }

    private void c(int n2) {
        if (n2 == this.A) {
            if (this.aD == -3) {
                this.aD = 0;
            }
            if (this.aq) {
                return;
            }
            return;
        }
        if (n2 == this.B) {
            if (this.aD == 3) {
                this.aD = 0;
            }
            if (this.aq) {
                return;
            }
            return;
        }
        if (n2 == this.y) {
            if (this.aE == -3) {
                this.aE = 0;
            }
            if (this.aq) {
                return;
            }
            return;
        }
        if (n2 == this.z) {
            if (this.aE == 3) {
                this.aE = 0;
            }
            if (this.aq) {
                return;
            }
            return;
        }
        if (n2 == this.D) {
            this.aq = false;
            this.aB = 0;
            this.aC = 0;
            return;
        }
    }

    private void a(Graphics graphics, int n2) {
        if (this.ae[n2] != null) {
            int n3 = graphics.getTranslateX();
            int n4 = graphics.getTranslateY();
            graphics.translate(18, 7);
            graphics.clipRect(0, 0, 18, 25);
            graphics.setClip(0, 0, 18, 25);
            graphics.drawImage(this.ab, -18 * this.ae[n2].V, 0, 0);
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(n3, n4);
            graphics.translate(40, 6);
            int n5 = this.ae[n2].H;
            for (int i2 = 0; i2 < this.ae[n2].I / 4; ++i2) {
                graphics.clipRect(0, 0, 7, 7);
                graphics.setClip(0, 0, 7, 7);
                if (n5 >= 4) {
                    graphics.drawImage(this.ad, 0, 0, 0);
                }
                if (n5 == 3) {
                    graphics.drawImage(this.ad, -7, 0, 0);
                }
                if (n5 == 2) {
                    graphics.drawImage(this.ad, -14, 0, 0);
                }
                if (n5 == 1) {
                    graphics.drawImage(this.ad, -21, 0, 0);
                }
                if (n5 < 1) {
                    graphics.drawImage(this.ad, -28, 0, 0);
                }
                if (i2 == 7) {
                    graphics.translate(-56, 8);
                }
                graphics.translate(7, 0);
                n5 -= 4;
            }
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(n3, n4);
            graphics.translate(40, 23);
            graphics.clipRect(0, 0, 8, 8);
            graphics.setClip(0, 0, 8, 8);
            if (this.ae[n2].K == 99) {
                graphics.drawImage(this.ad, -35, 0, 0);
            }
            if (this.ae[n2].K == 500) {
                graphics.drawImage(this.ad, -43, 0, 0);
            }
            if (this.ae[n2].K == 999) {
                graphics.drawImage(this.ad, -51, 0, 0);
            }
            graphics.translate(9, 0);
            graphics.setColor(0xFFFFFF);
            graphics.clipRect(0, 0, this.getWidth(), this.getHeight());
            graphics.setClip(0, 0, this.getWidth(), this.getHeight());
            graphics.translate(0, 2);
            this.a(graphics, this.ae[n2].J, 0);
            graphics.translate(0, -2);
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(n3, n4);
            graphics.translate(100, 6);
            graphics.clipRect(0, 0, 18, 25);
            graphics.setClip(0, 0, 18, 25);
            graphics.drawImage(this.ab, -72, 0, 0);
            graphics.translate(-1, 18);
            graphics.setColor(0xFFFFFF);
            graphics.clipRect(0, 0, this.getWidth(), this.getHeight());
            graphics.setClip(0, 0, this.getWidth(), this.getHeight());
            this.a(graphics, this.ae[n2].af, 2);
            graphics.translate(7, 0);
            graphics.setColor(0xFFFFFF);
            graphics.clipRect(0, 0, this.getWidth(), this.getHeight());
            graphics.setClip(0, 0, this.getWidth(), this.getHeight());
            graphics.drawLine(4, 0, 0, 6);
            graphics.translate(4, 0);
            this.a(graphics, 8, 2);
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(n3, n4);
            return;
        }
        graphics.translate(40, 7);
        graphics.clipRect(0, 0, this.getWidth(), this.getHeight());
        graphics.setClip(0, 0, this.getWidth(), this.getHeight());
        graphics.drawString("New Game..", 0, 0, 0);
        graphics.translate(-40, -7);
    }
}

