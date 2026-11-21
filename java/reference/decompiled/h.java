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
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public final class h
extends f {
    Image a;
    Image b;
    Image c;
    Image d;
    private Image aB;
    private Image aC;
    private Image aD;
    private Image aE;
    private Image aF;
    private Image aG;
    int e;
    int f;
    boolean g;
    boolean h;
    boolean n;
    boolean o;
    private boolean aH;
    boolean p;
    boolean q;
    boolean r;
    boolean s;
    boolean t;
    boolean u;
    boolean v;
    boolean w;
    boolean x;
    boolean y;
    boolean z;
    boolean A;
    boolean B;
    boolean C;
    boolean D;
    boolean E;
    boolean F;
    int G = 0;
    int H;
    int I;
    int J;
    int K;
    int L;
    int M;
    int N = 0;
    int O = 0;
    int P = 120;
    byte Q;
    byte R;
    byte S;
    byte T;
    byte U = 0;
    byte V = 0;
    byte W;
    int X;
    int Y = 0;
    int Z = 24;
    int aa = 100;
    int ab = 75;
    int ac = 2;
    int ad;
    int ae = 0;
    int af = 0;
    boolean ag;
    boolean ah;
    Vector ai;
    int aj = 0;
    int ak;
    int al;
    int am;
    int an = 0;
    int ao = 0;
    int[] ap = new int[4];
    int[] aq = new int[4];
    int[] ar = new int[2];
    int[] as = new int[2];
    int[] at = new int[2];
    int[] au = new int[2];
    int[] av = new int[2];
    int[] aw = new int[2];
    int[] ax = new int[2];
    int[] ay = new int[2];
    int[] az = new int[2];
    boolean[] aA = new boolean[8];
    private k aI;

    h() {
        this.i = 75;
        this.W = 1;
        this.j = 85;
        this.e = 0;
        this.f = 0;
        this.H = 12;
        this.I = 12;
        this.K = 99;
        this.R = 0;
        this.Q = 0;
        this.S = 0;
        this.T = 0;
        this.J = 0;
        this.k = 12;
        this.l = 16;
        this.am = 0;
        this.af = 0;
        this.D = false;
        this.B = false;
        this.g = false;
        this.aH = false;
        this.q = true;
        this.s = true;
        this.r = true;
        this.t = true;
        this.u = true;
        this.w = false;
        this.C = false;
        this.p = false;
        this.y = false;
        this.q = false;
        this.s = false;
        this.r = false;
        this.t = false;
        this.u = false;
        this.v = false;
        this.x = false;
        this.z = false;
        this.A = false;
        this.ag = false;
        this.L = 0;
        this.M = 1;
        this.ai = new Vector();
        this.ah = false;
    }

    public final int a() {
        if ((this.aj & 0x200) > 0) {
            this.aA[0] = true;
        }
        if ((this.aj & 0x400) > 0) {
            this.aA[1] = true;
        }
        if ((this.aj & 0x800) > 0) {
            this.aA[2] = true;
        }
        if ((this.aj & 0x4000) > 0) {
            this.aA[3] = true;
        }
        if ((this.aj & 0x8000) > 0) {
            this.aA[4] = true;
        }
        if ((this.aj & 0x40000) > 0) {
            this.aA[5] = true;
        }
        if ((this.aj & 0x400000) > 0) {
            this.aA[6] = true;
        }
        if ((this.aj & 0x800000) > 0) {
            this.aA[7] = true;
        }
        int n2 = 0;
        for (int i2 = 0; i2 < 8; ++i2) {
            if (!this.aA[i2]) continue;
            ++n2;
        }
        if (n2 == 8) {
            this.ap[0] = this.ap[0] | 0x40;
        }
        return n2;
    }

    public final void a(byte[] object, boolean bl, k k2) {
        this.aI = k2;
        System.out.println("enter load player");
        if (bl) {
            return;
        }
        object = new ByteArrayInputStream((byte[])object);
        DataInputStream dataInputStream = new DataInputStream((InputStream)object);
        ((ByteArrayInputStream)object).reset();
        try {
            int n2;
            this.i = dataInputStream.readInt();
            this.j = dataInputStream.readInt();
            this.q = dataInputStream.readBoolean();
            this.r = dataInputStream.readBoolean();
            this.s = dataInputStream.readBoolean();
            this.t = dataInputStream.readBoolean();
            this.u = dataInputStream.readBoolean();
            this.v = dataInputStream.readBoolean();
            this.H = dataInputStream.readInt();
            this.I = dataInputStream.readInt();
            this.J = dataInputStream.readInt();
            this.K = dataInputStream.readInt();
            this.N = dataInputStream.readInt();
            this.O = dataInputStream.readInt();
            this.P = dataInputStream.readInt();
            this.Q = (byte)dataInputStream.read();
            this.R = (byte)dataInputStream.read();
            this.S = (byte)dataInputStream.read();
            this.T = (byte)dataInputStream.read();
            this.U = (byte)dataInputStream.read();
            this.V = (byte)dataInputStream.read();
            this.U = 0;
            this.V = 0;
            this.aj = dataInputStream.readInt();
            if ((this.aj & 0x100) > 0) {
                this.w = true;
            }
            if ((this.aj & 0x1000) > 0) {
                this.x = true;
            }
            if ((this.aj & 0x20000) > 0) {
                this.C = true;
            }
            if ((this.aj & 0x2000) > 0) {
                this.U = 1;
                this.W = (byte)2;
            }
            if ((this.aj & 0x10000) > 0) {
                this.U = (byte)2;
                this.W = (byte)4;
            }
            if ((this.aj & 0x40) > 0) {
                this.V = 1;
            }
            if ((this.aj & 0x80000) > 0) {
                this.y = true;
            }
            if ((this.aj & 0x10000000) > 0) {
                this.z = true;
            }
            if ((this.aj & 0x8000000) > 0) {
                this.V = (byte)2;
            }
            this.ak = dataInputStream.readInt();
            this.al = dataInputStream.readInt();
            for (n2 = 0; n2 < 4; ++n2) {
                this.ap[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 4; ++n2) {
                this.aq[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.ar[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.as[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.at[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.au[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.av[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.aw[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.ax[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.ay[n2] = dataInputStream.readInt();
            }
            for (n2 = 0; n2 < 2; ++n2) {
                this.az[n2] = dataInputStream.readInt();
            }
            this.af = this.a();
            this.am = dataInputStream.readInt();
            this.an = dataInputStream.readInt();
            ((ByteArrayInputStream)object).reset();
            ((ByteArrayInputStream)object).close();
            dataInputStream.close();
            this.a = null;
            this.b = null;
            System.gc();
            return;
        }
        catch (Exception exception) {
            return;
        }
    }

    public final byte[] b() {
        ByteArrayOutputStream byteArrayOutputStream = new ByteArrayOutputStream();
        DataOutputStream dataOutputStream = new DataOutputStream(byteArrayOutputStream);
        byteArrayOutputStream.reset();
        try {
            int n2;
            dataOutputStream.writeInt(((h)object).i);
            dataOutputStream.writeInt(((h)object).j);
            dataOutputStream.writeBoolean(((h)object).q);
            dataOutputStream.writeBoolean(((h)object).r);
            dataOutputStream.writeBoolean(((h)object).s);
            dataOutputStream.writeBoolean(((h)object).t);
            dataOutputStream.writeBoolean(((h)object).u);
            dataOutputStream.writeBoolean(((h)object).v);
            dataOutputStream.writeInt(((h)object).H);
            dataOutputStream.writeInt(((h)object).I);
            dataOutputStream.writeInt(((h)object).J);
            dataOutputStream.writeInt(((h)object).K);
            dataOutputStream.writeInt(((h)object).N);
            dataOutputStream.writeInt(((h)object).O);
            dataOutputStream.writeInt(((h)object).P);
            dataOutputStream.write(((h)object).Q);
            dataOutputStream.write(((h)object).R);
            dataOutputStream.write(((h)object).S);
            dataOutputStream.write(((h)object).T);
            dataOutputStream.write(((h)object).U);
            dataOutputStream.write(((h)object).V);
            dataOutputStream.writeInt(((h)object).aj);
            dataOutputStream.writeInt(((h)object).ak);
            dataOutputStream.writeInt(((h)object).al);
            for (n2 = 0; n2 < 4; ++n2) {
                dataOutputStream.writeInt(((h)object).ap[n2]);
            }
            for (n2 = 0; n2 < 4; ++n2) {
                dataOutputStream.writeInt(((h)object).aq[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).ar[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).as[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).at[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).au[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).av[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).aw[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).ax[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).ay[n2]);
            }
            for (n2 = 0; n2 < 2; ++n2) {
                dataOutputStream.writeInt(((h)object).az[n2]);
            }
            dataOutputStream.writeInt(((h)object).am);
            dataOutputStream.writeInt(((h)object).an);
            dataOutputStream.flush();
            Object object = byteArrayOutputStream.toByteArray();
            byteArrayOutputStream.reset();
            byteArrayOutputStream.close();
            dataOutputStream.close();
            return object;
        }
        catch (Exception exception) {
            return null;
        }
    }

    final void a(Graphics graphics, int n2, int n3, int n4, int n5) {
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(this.i - n2, this.j - n3);
        graphics.setClip(0, 0, 24, 24);
        graphics.clipRect(0, 0, 24, 24);
        if (this.L % 2 == 1) {
            return;
        }
        if (this.ag) {
            return;
        }
        if (this.E) {
            if (this.aF == null) {
                try {
                    this.aF = Image.createImage("/Images/ItemGet.png");
                }
                catch (IOException iOException) {
                    System.out.println("error loadin raft image");
                    return;
                }
            }
            --this.ad;
            if (this.ad < 0) {
                this.E = false;
            }
            graphics.translate(2, -3);
            graphics.setClip(0, 0, 18, 27);
            graphics.clipRect(0, 0, 18, 27);
            if (this.ad > 10) {
                graphics.drawImage(this.aF, 0, -27 * this.V, 0);
                return;
            }
            graphics.drawImage(this.aF, -18, -27 * this.V, 0);
            return;
        }
        if (this.F) {
            if (this.aG == null) {
                try {
                    if (this.V == 0) {
                        this.aG = Image.createImage("/Images/Dig_Green.png");
                    }
                    if (this.V == 1) {
                        this.aG = Image.createImage("/Images/Dig_Blue.png");
                    }
                    if (this.V == 2) {
                        this.aG = Image.createImage("/Images/Dig_Red.png");
                    }
                }
                catch (IOException iOException) {
                    System.out.println("error loadin dig image");
                    this.aF = null;
                    this.aB = null;
                    return;
                }
            }
            --this.ae;
            if (this.ae <= 0) {
                this.F = false;
            }
            if (this.e == 0) {
                if (this.ae > 5) {
                    graphics.setClip(0, 0, 21, 24);
                    graphics.clipRect(0, 0, 21, 24);
                    graphics.drawImage(this.aG, 0, 0, 0);
                } else {
                    graphics.setClip(0, 0, 18, 24);
                    graphics.clipRect(0, 0, 18, 24);
                    graphics.drawImage(this.aG, -21, 0, 0);
                }
            }
            if (this.e == 1) {
                if (this.ae > 5) {
                    graphics.setClip(0, 0, 20, 25);
                    graphics.clipRect(0, 0, 20, 25);
                    graphics.drawImage(this.aG, 0, -24, 0);
                } else {
                    graphics.setClip(0, 0, 19, 25);
                    graphics.clipRect(0, 0, 19, 25);
                    graphics.drawImage(this.aG, -20, -24, 0);
                }
            }
            if (this.e == 2) {
                if (this.ae > 5) {
                    graphics.setClip(0, 0, 18, 27);
                    graphics.clipRect(0, 0, 18, 27);
                    graphics.drawImage(this.aG, 0, -49, 0);
                } else {
                    graphics.setClip(0, 0, 21, 27);
                    graphics.clipRect(0, 0, 21, 27);
                    graphics.drawImage(this.aG, -18, -49, 0);
                }
            }
            if (this.e == 3) {
                if (this.ae > 5) {
                    graphics.setClip(0, 0, 20, 26);
                    graphics.clipRect(0, 0, 20, 26);
                    graphics.drawImage(this.aG, 0, -76, 0);
                    return;
                }
                graphics.setClip(0, 0, 19, 26);
                graphics.clipRect(0, 0, 19, 26);
                graphics.drawImage(this.aG, -20, -76, 0);
            }
            return;
        }
        if (this.B) {
            if (this.aB == null) {
                try {
                    this.aB = Image.createImage("/Images/raft.png");
                }
                catch (IOException iOException) {
                    System.out.println("error loadin raft image");
                }
            }
            if (this.D) {
                if (this.aD == null || this.aE == null) {
                    try {
                        if (this.V == 0) {
                            this.aD = Image.createImage("/Images/GreenLinkFishing.png");
                        }
                        if (this.V == 1) {
                            this.aD = Image.createImage("/Images/BlueLinkFishing.png");
                        }
                        if (this.V == 2) {
                            this.aD = Image.createImage("/Images/RedLinkFishing.png");
                        }
                        this.aE = Image.createImage("/Images/RingsOnWater.png");
                    }
                    catch (IOException iOException) {
                        System.out.println("error loadin fishing image image");
                    }
                }
            } else {
                this.aD = null;
                this.aE = null;
            }
        } else {
            this.aB = null;
        }
        if (this.p) {
            if (this.aC == null) {
                try {
                    this.aC = Image.createImage("/Images/Shield.png");
                }
                catch (IOException iOException) {
                    System.out.println("error loadin raft image");
                }
            }
        } else {
            this.aC = null;
        }
        if (!this.g && !this.n) {
            if (this.B && this.aB != null) {
                this.f = 2;
                if (this.e == 0) {
                    graphics.translate(3, 0);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -32, 0, 0);
                    graphics.translate(0, 16);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -50, 0, 0);
                }
                if (this.e == 1) {
                    graphics.translate(-3, 12);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, 0, 0, 0);
                    graphics.translate(16, 0);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, -16, 0, 0);
                }
                if (this.e == 2) {
                    graphics.translate(3, 0);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -32, 0, 0);
                    graphics.translate(0, 16);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -50, 0, 0);
                }
                if (this.e == 3) {
                    graphics.translate(-3, 12);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, 0, 0, 0);
                    graphics.translate(16, 0);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, -16, 0, 0);
                }
                if (this.D) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    n5 = 0;
                    if (this.e == 2 || this.e == 3) {
                        graphics.translate(-4, 0);
                        n4 = -4;
                    } else {
                        graphics.translate(-4, 0);
                        n4 = 20;
                    }
                    if (this.ac == 1) {
                        graphics.setClip(0, 0, 28, 23);
                        graphics.clipRect(0, 0, 28, 23);
                        if (this.e == 0 || this.e == 1) {
                            graphics.drawRegion(this.aD, 27, 0, 28, 23, 2, 0, 0, 0);
                        } else {
                            graphics.drawRegion(this.aD, 27, 0, 28, 23, 0, 0, 0, 0);
                        }
                        n5 = 3;
                    }
                    if (this.ac == 2) {
                        graphics.setClip(0, 0, 28, 23);
                        graphics.clipRect(0, 0, 28, 23);
                        if (this.e == 0 || this.e == 1) {
                            graphics.drawRegion(this.aD, 55, 0, 28, 23, 2, 0, 0, 0);
                        } else {
                            graphics.drawRegion(this.aD, 55, 0, 28, 23, 0, 0, 0, 0);
                        }
                        n5 = 4;
                    }
                    if (this.ac == 3) {
                        graphics.setClip(0, 0, 28, 23);
                        graphics.clipRect(0, 0, 28, 23);
                        if (this.e == 0 || this.e == 1) {
                            graphics.drawRegion(this.aD, 83, 0, 28, 23, 2, 0, 0, 0);
                        } else {
                            graphics.drawRegion(this.aD, 83, 0, 28, 23, 0, 0, 0, 0);
                        }
                        n5 = 5;
                    }
                    if (this.ac == 4) {
                        graphics.setClip(0, 0, 29, 23);
                        graphics.clipRect(0, 0, 29, 23);
                        if (this.e == 0 || this.e == 1) {
                            graphics.drawRegion(this.aD, 111, 0, 29, 23, 2, 0, 0, 0);
                        } else {
                            graphics.drawRegion(this.aD, 111, 0, 29, 23, 0, 0, 0, 0);
                        }
                        n5 = 5;
                    }
                    if (this.ac == 5) {
                        graphics.translate(3, 0);
                        graphics.setClip(0, 0, 25, 23);
                        graphics.clipRect(0, 0, 25, 23);
                        if (this.e == 0 || this.e == 1) {
                            graphics.drawRegion(this.aD, 140, 0, 25, 23, 2, 0, 0, 0);
                            n4 = 16;
                        } else {
                            graphics.drawRegion(this.aD, 140, 0, 25, 23, 0, 0, 0, 0);
                            n4 = -1;
                        }
                        n5 = 6;
                    }
                    if (this.ac == 6) {
                        graphics.translate(3, 0);
                        graphics.setClip(0, 0, 25, 23);
                        graphics.clipRect(0, 0, 25, 23);
                        if (this.e == 0 || this.e == 1) {
                            n4 = 16;
                            graphics.drawRegion(this.aD, 165, 0, 25, 23, 2, 0, 0, 0);
                        } else {
                            n4 = 1;
                            graphics.drawRegion(this.aD, 165, 0, 25, 23, 0, 0, 0, 0);
                        }
                        n5 = 6;
                    }
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.aa - n2, this.ab - n3);
                    Random random = new Random();
                    if (random.nextInt() % 2 == 1) {
                        graphics.translate(-7, -4);
                        graphics.setClip(0, 0, 14, 8);
                        graphics.clipRect(0, 0, 14, 8);
                        graphics.drawImage(this.aE, 0, 0, 0);
                    } else {
                        graphics.translate(-6, -4);
                        graphics.setClip(0, 0, 12, 8);
                        graphics.clipRect(0, 0, 12, 8);
                        graphics.drawImage(this.aE, -14, 0, 0);
                    }
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.setClip(0, 0, 256, 176);
                    graphics.clipRect(0, 0, 256, 176);
                    graphics.setColor(0, 0, 0);
                    graphics.drawLine(this.aa - n2, this.ab - n3, this.i - n2 + n4, this.j - n3 + n5);
                    if (this.aI.d.a(this.aa + 1, this.ab, 1, 26, 0)) {
                        ++this.aa;
                    }
                    return;
                }
            }
            if (this.p && this.e == 0) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2 + 14, this.j - n3 + 8);
                graphics.setClip(0, 0, 10, 12);
                graphics.clipRect(0, 0, 10, 12);
                graphics.drawImage(this.aC, -33, 0, 0);
                if (this.f > 3) {
                    this.f = 0;
                }
            }
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.translate(this.i - n2, this.j - n3);
            graphics.setClip(0, 0, 24, 24);
            graphics.clipRect(0, 0, 24, 24);
            graphics.drawImage(this.a, -(this.f >> 1) * 24, -this.e * 24, 0);
            if (this.p) {
                if (this.e == 1) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2 + 17, this.j - n3 + 7 - (this.f >> 1));
                    graphics.setClip(0, 0, 6, 12);
                    graphics.clipRect(0, 0, 6, 12);
                    graphics.drawImage(this.aC, -18 - (this.f >> 1) * 6, -12, 0);
                }
                if (this.e == 2) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2 + 3, this.j - n3 + 12);
                    graphics.setClip(0, 0, 10, 12);
                    graphics.clipRect(0, 0, 10, 12);
                    graphics.drawImage(this.aC, -(this.f >> 1) * 12, 0, 0);
                }
                if (this.e == 3) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2 + 1, this.j - n3 + 7 - (this.f >> 1));
                    graphics.setClip(0, 0, 6, 12);
                    graphics.clipRect(0, 0, 6, 12);
                    graphics.drawImage(this.aC, -(this.f >> 1) * 6, -12, 0);
                    return;
                }
            }
        } else {
            if (this.e == 0) {
                if (this.B && this.aB != null) {
                    graphics.translate(3, -4);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -32, 0, 0);
                    graphics.translate(0, 16);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -50, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.setClip(0, 0, 24, 24);
                    graphics.clipRect(0, 0, 24, 24);
                }
                if (this.g) {
                    graphics.translate(6, -9);
                    graphics.setClip(0, 0, 9, 18);
                    graphics.clipRect(0, 0, 9, 18);
                    graphics.drawImage(this.b, 0, 0, 0);
                }
                if (this.n) {
                    for (n4 = 0; n4 < this.ai.size(); ++n4) {
                        g g2 = (g)this.ai.elementAt(n4);
                        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                        graphics.translate(g2.a - n2, g2.b - n3);
                        if (n4 == this.ai.size() - 1) {
                            graphics.translate(-2, 0);
                            graphics.setClip(0, 0, 12, 8);
                            graphics.clipRect(0, 0, 12, 8);
                            graphics.drawImage(this.c, 0, -16, 0);
                            continue;
                        }
                        graphics.setClip(0, 0, 8, 8);
                        graphics.clipRect(0, 0, 8, 8);
                        graphics.drawImage(this.c, 0, -(n4 % 2) << 3, 0);
                    }
                }
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.setClip(0, 0, 24, 24);
                graphics.clipRect(0, 0, 24, 24);
                graphics.drawImage(this.a, -72, -this.e * 24, 0);
            }
            if (this.e == 1) {
                if (this.B && this.aB != null) {
                    graphics.translate(-3, 12);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, 0, 0, 0);
                    graphics.translate(16, 0);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, -16, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.setClip(0, 0, 24, 24);
                    graphics.clipRect(0, 0, 24, 24);
                }
                if (this.g) {
                    graphics.translate(17, 12);
                    graphics.setClip(0, 0, 18, 8);
                    graphics.clipRect(0, 0, 18, 8);
                    graphics.drawImage(this.b, -18, 0, 0);
                }
                if (this.n) {
                    for (n4 = 0; n4 < this.ai.size(); ++n4) {
                        g g3 = (g)this.ai.elementAt(n4);
                        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                        graphics.translate(g3.a - n2, g3.b - n3);
                        if (n4 == this.ai.size() - 1) {
                            graphics.translate(0, -4);
                            graphics.setClip(0, 0, 8, 16);
                            graphics.clipRect(0, 0, 8, 16);
                            graphics.drawImage(this.c, -8, 0, 0);
                            continue;
                        }
                        graphics.setClip(0, 0, 8, 8);
                        graphics.clipRect(0, 0, 8, 8);
                        graphics.drawImage(this.c, 0, -(n4 % 2) << 3, 0);
                    }
                }
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.setClip(0, 0, 24, 24);
                graphics.clipRect(0, 0, 24, 24);
                graphics.drawImage(this.a, -72, -this.e * 24, 0);
            }
            if (this.e == 2) {
                if (this.B && this.aB != null) {
                    graphics.translate(3, -4);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -32, 0, 0);
                    graphics.translate(0, 16);
                    graphics.setClip(0, 0, 18, 16);
                    graphics.clipRect(0, 0, 18, 16);
                    graphics.drawImage(this.aB, -50, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.setClip(0, 0, 24, 24);
                    graphics.clipRect(0, 0, 24, 24);
                }
                if (this.g) {
                    graphics.translate(12, 20);
                    graphics.setClip(0, 0, 9, 18);
                    graphics.clipRect(0, 0, 9, 18);
                    graphics.drawImage(this.b, -9, 0, 0);
                }
                if (this.n) {
                    for (n4 = 0; n4 < this.ai.size(); ++n4) {
                        g g4 = (g)this.ai.elementAt(n4);
                        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                        graphics.translate(g4.a - n2, g4.b - n3);
                        if (n4 == this.ai.size() - 1) {
                            graphics.translate(-2, 0);
                            graphics.setClip(0, 0, 12, 8);
                            graphics.clipRect(0, 0, 12, 8);
                            graphics.drawImage(this.c, -12, -16, 0);
                            continue;
                        }
                        graphics.setClip(0, 0, 8, 8);
                        graphics.clipRect(0, 0, 8, 8);
                        graphics.drawImage(this.c, 0, -(n4 % 2) << 3, 0);
                    }
                }
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.setClip(0, 0, 24, 24);
                graphics.clipRect(0, 0, 24, 24);
                graphics.drawImage(this.a, -72, -this.e * 24, 0);
            }
            if (this.e == 3) {
                if (this.B && this.aB != null) {
                    graphics.translate(-3, 12);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, 0, 0, 0);
                    graphics.translate(16, 0);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.aB, -16, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.setClip(0, 0, 24, 24);
                    graphics.clipRect(0, 0, 24, 24);
                }
                graphics.drawImage(this.a, -72, -this.e * 24, 0);
                if (this.g) {
                    graphics.translate(-12, 12);
                    graphics.setClip(0, 0, 18, 9);
                    graphics.clipRect(0, 0, 18, 9);
                    graphics.drawImage(this.b, -18, -9, 0);
                }
                if (this.n) {
                    for (n4 = 0; n4 < this.ai.size(); ++n4) {
                        g g5 = (g)this.ai.elementAt(n4);
                        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                        graphics.translate(g5.a - n2, g5.b - n3);
                        if (n4 == this.ai.size() - 1) {
                            graphics.translate(0, -4);
                            graphics.setClip(0, 0, 8, 16);
                            graphics.clipRect(0, 0, 8, 16);
                            graphics.drawImage(this.c, -16, 0, 0);
                            continue;
                        }
                        graphics.setClip(0, 0, 8, 8);
                        graphics.clipRect(0, 0, 8, 8);
                        graphics.drawImage(this.c, 0, -(n4 % 2) << 3, 0);
                    }
                }
            }
        }
    }
}

