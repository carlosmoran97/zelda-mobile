/*
 * Decompiled with CFR 0.152.
 */
import java.io.DataInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.util.Random;
import java.util.Vector;
import javax.microedition.lcdui.Font;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public final class j {
    private Image t;
    public k a;
    o b;
    private int u;
    private int v;
    private Image w;
    private Image x;
    Image c;
    private Image y;
    Image d;
    private Image z;
    private byte[] A;
    private byte B;
    private byte C;
    private byte D;
    Random e;
    private int E;
    int f;
    int g;
    int h;
    int i;
    int j;
    private int F;
    private int G;
    int k;
    int l;
    boolean m;
    Vector n;
    Vector o;
    Vector p;
    Vector q;
    Vector r;
    Vector s;
    private Vector H;
    private Vector I;

    j(Graphics graphics, Graphics graphics2, h h2, k k2) {
        this.a = k2;
        this.D = 0;
        this.u = 0;
        this.v = 0;
        this.m = false;
        this.I = new Vector();
        new Vector();
        this.n = new Vector();
        this.o = new Vector();
        this.p = new Vector();
        this.q = new Vector();
        this.r = new Vector();
        this.s = new Vector();
        this.H = new Vector();
        this.e = new Random();
        this.t = null;
        this.g = 0;
        this.h = 0;
        this.b = new o(this);
        this.F = 0;
        System.out.println("Map Constructor");
        if (h2 != null) {
            if ((h2.an & 1 << h2.O) > 0) {
                this.g = 1;
            }
            if (h2.O == 9) {
                if (this.g == 0) {
                    this.a("Maps/Map" + h2.P + "-" + h2.O + ".xmap.xmob", graphics, h2, h2.P, h2.O, 0, 0, true);
                }
                if (this.g == 1) {
                    this.a("Maps/Map" + h2.P + "-" + h2.O + "-1.xmap.xmob", graphics, h2, h2.P, h2.O, 0, 0, true);
                }
            } else {
                this.a("Maps/Map" + h2.P + "-" + h2.O + ".xmap.xmob", graphics, h2, h2.P, h2.O, 0, 0, true);
            }
        } else {
            this.a("Maps/intro_new.xmap.xmob", graphics, h2, 0, 0, 0, 0, false);
            return;
        }
        this.h = this.g;
        this.E = h2.P;
        this.f = h2.O;
        this.G = h2.an;
        if ((this.G & 1 << this.f) > 0) {
            this.g = 1;
        }
        this.I.setSize(16);
    }

    final void a(Graphics graphics, int n2, int n3, int n4, int n5, h h2) {
        f f2;
        Graphics graphics2;
        int n6;
        int n7;
        int n8;
        for (n8 = 0; n8 < this.r.size(); ++n8) {
            n n9 = (n)this.r.elementAt(n8);
            ((n)this.r.elementAt(n8)).C = this.g;
            n9.H = h2.i;
            if (n9.a != 8) continue;
            if (n9.K != null) {
                boolean bl = this.a.a;
                int n10 = n5;
                int n11 = n4;
                n7 = n3;
                n6 = n2;
                graphics2 = graphics;
                f2 = n9;
                if (((n)f2).K == null || n11 + n6 < ((n)f2).i - 48 || ((n)f2).i + 16 + 48 < n6 || n10 + n7 < ((n)f2).j - 48 || ((n)f2).j + 16 - 32 + 48 < n7 || !((n)f2).w) continue;
                graphics2.translate(-graphics2.getTranslateX(), -graphics2.getTranslateY());
                graphics2.translate(((n)f2).i - n6, ((n)f2).j - n7);
                if (((n)f2).a != 8) continue;
                graphics2.translate(-40 + (((n)f2).B >> 3), -40 + (((n)f2).B >> 3));
                graphics2.setClip(0, 0, 96, 96);
                graphics2.clipRect(0, 0, 96, 96);
                if (!bl) continue;
                graphics2.drawImage(((n)f2).K, 0, 0, 0);
                continue;
            }
            if (this.c != null) {
                n9.K = this.c;
                continue;
            }
            try {
                if (this.a.a) {
                    this.c = Image.createImage("/Images/Glow.png");
                    continue;
                }
                this.c = Image.createImage("/Images/Glow2.png");
                continue;
            }
            catch (IOException iOException) {
                return;
            }
        }
        if (h2.A) {
            if (h2.d != null) {
                boolean bl = this.a.a;
                n7 = n3;
                n6 = n2;
                graphics2 = graphics;
                f2 = h2;
                graphics2.translate(-graphics2.getTranslateX(), -graphics2.getTranslateY());
                graphics2.translate(f2.i - n6, f2.j - n7);
                graphics2.translate(-32 + (f2.f >> 3), -38 + (f2.f >> 3));
                graphics2.setClip(0, 0, 96, 96);
                graphics2.clipRect(0, 0, 96, 96);
                if (bl) {
                    graphics2.drawImage(f2.d, 0, 0, 0);
                } else {
                    Random random = new Random();
                    graphics2.drawRegion(f2.d, 0, 0, 96, 96, Math.abs(random.nextInt() % 6), 0, 0, 0);
                }
            } else if (this.c != null) {
                h2.d = this.c;
            } else {
                try {
                    this.c = this.a.a ? Image.createImage("/Images/Glow.png") : Image.createImage("/Images/Glow2.png");
                }
                catch (IOException iOException) {
                    return;
                }
            }
        }
        for (n8 = 0; n8 < this.q.size(); ++n8) {
            l l2 = (l)this.q.elementAt(n8);
            if (l2.b == 23 && l2.z == 2) {
                if (this.y == null) {
                    try {
                        this.y = Image.createImage("/Images/GanonSlash.png");
                    }
                    catch (IOException iOException) {
                        return;
                    }
                } else {
                    l2.r = this.y;
                    n7 = n3;
                    n6 = n2;
                    graphics2 = graphics;
                    f2 = l2;
                    graphics2.translate(-graphics2.getTranslateX(), -graphics2.getTranslateY());
                    graphics2.translate(((l)f2).i - n6, ((l)f2).j - n7);
                    if (((l)f2).c == 1) {
                        graphics2.translate(-30, -12);
                        graphics2.setClip(0, 0, 89, 72);
                        graphics2.clipRect(0, 0, 89, 72);
                        graphics2.drawImage(((l)f2).r, 0, 0, 0);
                    }
                    if (((l)f2).c == 3) {
                        if (((l)f2).z == 2) {
                            graphics2.translate(30, 0);
                        }
                        graphics2.translate(-30, -12);
                        graphics2.setClip(0, 0, 89, 72);
                        graphics2.clipRect(0, 0, 89, 72);
                        graphics2.drawImage(((l)f2).r, -89, 0, 0);
                    }
                }
            }
            if (l2.b != 29 || l2.z != 7) continue;
            l2.a(graphics, n2, n3, h2);
        }
        for (n8 = 0; n8 < this.n.size(); ++n8) {
            p p2 = (p)this.n.elementAt(n8);
            if (p2.h != 11) continue;
            if (this.z == null) {
                try {
                    this.z = Image.createImage("/Images/ArrowLightEffect.png");
                    continue;
                }
                catch (IOException iOException) {
                    return;
                }
            }
            p2.d = this.z;
            n7 = n3;
            n6 = n2;
            graphics2 = graphics;
            f2 = p2;
            graphics2.translate(-graphics2.getTranslateX(), -graphics2.getTranslateY());
            graphics2.translate(((p)f2).i - n6 - ((p)f2).b * ((p)f2).n, ((p)f2).j - n7 - ((p)f2).c * ((p)f2).n);
            graphics2.translate(-2, 0);
            graphics2.setClip(0, 0, 13, 20);
            graphics2.clipRect(0, 0, 13, 20);
            if (((p)f2).a % 2 == 0) {
                graphics2.drawRegion(((p)f2).d, 0, 0, 13, 20, 0, 0, 0, 0);
                continue;
            }
            graphics2.drawRegion(((p)f2).d, 0, 0, 13, 20, 2, 0, 0, 0);
        }
    }

    final int a() {
        int n2 = 0;
        for (int i2 = 0; i2 < this.r.size(); ++i2) {
            n n3 = (n)this.r.elementAt(i2);
            if (n3.r == 0 && !n3.p) continue;
            if (n3.r == 3 && n3.s) {
                n2 += n3.t;
                continue;
            }
            if (n3.r == 2 && n3.s) {
                n2 += n3.t;
                continue;
            }
            if (!n3.p) continue;
            if (this.q.size() == 0) {
                n2 += n3.t;
                continue;
            }
            boolean bl = false;
            int n4 = 0;
            for (int i3 = 0; i3 < this.q.size(); ++i3) {
                l l2 = (l)this.q.elementAt(i3);
                if (l2.b != 9) {
                    bl = true;
                }
                if (!l2.g && l2.b != 36) continue;
                ++n4;
            }
            if (!bl) {
                n2 += n3.t;
            }
            if (this.q.size() != n4) continue;
            n2 += n3.t;
        }
        return n2;
    }

    final void a(h h2) {
        n n2;
        int n3;
        int n4 = h2.i;
        int n5 = h2.j - 32;
        n4 += 12;
        n5 += 16;
        for (n3 = 0; n3 < this.r.size(); ++n3) {
            n2 = (n)this.r.elementAt(n3);
            if (n2.a == 218) {
                for (int i2 = 0; i2 < this.p.size(); ++i2) {
                    c c2 = (c)this.p.elementAt(i2);
                    if (c2.i > n2.i || c2.i + 16 < n2.i || c2.j > n2.j || c2.j + 16 < n2.j) continue;
                    c2.f = false;
                }
            }
            if (n2.r == 0) continue;
            if (n2.r == 3) {
                n2.s = false;
                if (!this.a(n2.i, n2.j, 4, 100, n3)) {
                    n2.s = true;
                }
                if (n2.i > n4 || n2.i + 16 < n4 || n2.j > n5 + 32 || n2.j + 16 < n5 + 32) continue;
                n2.s = true;
                continue;
            }
            if (n2.r != 1) continue;
            if (this.a() >= n2.t) {
                n2.w = false;
                if (n2.c > 0) {
                    this.b(n2.c, this.E, h2);
                    this.a.getClass();
                    if (n2.y) {
                        int n6 = 7;
                        b b2 = this.a.H;
                        this.a.H.d = n6;
                    }
                }
                n2.y = false;
                continue;
            }
            n2.w = true;
            n2.y = true;
        }
        for (n3 = 0; n3 < this.s.size(); ++n3) {
            n2 = (n)this.s.elementAt(n3);
            if (n2.r != 1) continue;
            if (this.a() >= n2.t) {
                n2.w = false;
                if (n2.c > 0) {
                    this.b(n2.c, this.E, h2);
                    this.a.getClass();
                    if (n2.y) {
                        int n7 = 7;
                        b b3 = this.a.H;
                        this.a.H.d = n7;
                    }
                }
                n2.y = false;
                continue;
            }
            n2.w = true;
            n2.y = true;
        }
    }

    final boolean a(int n2, int n3, int n4, h h2, boolean bl) {
        n n5;
        n3 -= 32;
        int n6 = 0;
        boolean bl2 = false;
        n2 += 12;
        n3 += 16;
        if (this.F > 0) {
            --this.F;
        }
        if (h2.G != 0) {
            n n7;
            for (n6 = 0; n6 < this.s.size(); ++n6) {
                n7 = (n)this.s.elementAt(n6);
                if (n7.i + n7.D > n2 || n7.i + n7.F + n7.D < n2 || n7.j + n7.E > n3 + 32 || n7.j + n7.G + n7.E < n3 + 32) continue;
                System.out.println("Standing on something..");
                return true;
            }
            for (n6 = 0; n6 < this.r.size(); ++n6) {
                n7 = (n)this.r.elementAt(n6);
                ((n)this.r.elementAt(n6)).C = this.g;
                if (n7.i + n7.D > n2 || n7.i + n7.F + n7.D < n2 || n7.j + n7.E > n3 + 32 || n7.j + n7.G + n7.E < n3 + 32 || n7.a != 17) continue;
                h2.G = 0;
                return true;
            }
            return false;
        }
        n6 = (n3 >> 4) * this.B + (n2 >> 4);
        if (n6 <= this.B * this.C && n6 > 0) {
            if (((n6 = this.A[n6]) & 0x40) == 64) {
                return false;
            }
            if ((n6 & 0x80) == 128 && !h2.B) {
                return false;
            }
        }
        for (n6 = 0; n6 < this.r.size(); ++n6) {
            int n8;
            int n9;
            n5 = (n)this.r.elementAt(n6);
            if (!n5.e && n5.r < 2 && n5.a != 18 && n5.a != 185 && n5.a != 17 && (n5.a < 21 || n5.a > 24) && (n5.a < 208 || n5.a > 215) || !n5.w || n5.i + n5.D > n2 || n5.i + n5.F + n5.D < n2 || n5.j + n5.E > n3 + 32 || n5.j + n5.G + n5.E < n3 + 32) continue;
            if (n5.a == 18) {
                h2.G = 1;
                System.out.println("Layer changed to 1");
                return true;
            }
            if (n5.a == 17) {
                h2.G = 0;
                System.out.println("Layer changed to 0");
                return true;
            }
            if (n5.a >= 21 && n5.a <= 24) {
                System.out.println("standing on it..");
                n9 = 0;
                n8 = 0;
                if (h2.e == 0) {
                    n8 = -4;
                    if (h2.B) {
                        n8 = 0;
                    }
                }
                if (h2.e == 1) {
                    n9 = 4;
                    if (h2.B) {
                        n9 = 0;
                    }
                }
                if (h2.e == 2) {
                    n8 = 4;
                    if (h2.B) {
                        n8 = 0;
                    }
                }
                if (h2.e == 3) {
                    n9 = -4;
                    if (h2.B) {
                        n9 = 0;
                    }
                }
                if ((n6 = (n3 + n8 >> 4) * this.B + (n2 + n9 >> 4)) <= this.B * this.C && n6 > 0) {
                    byte by = this.A[n6];
                    if ((by & 0x80) == 128) {
                        if (h2.x) {
                            h2.B = true;
                        }
                        System.out.println("Rafting");
                        return true;
                    }
                    if (n5.i + n5.D - 6 <= n2 && n5.i + n5.F + n5.D + 6 >= n2 && n5.j + n5.E - 6 <= n3 + 32 && n5.j + n5.G + n5.E + 6 >= n3 + 32) {
                        h2.B = false;
                        System.out.println("NOT Rafting");
                        return true;
                    }
                    return false;
                }
                System.out.println("test");
                return true;
            }
            if (n5.a >= 212 && n5.a <= 215) {
                if (!h2.B || bl2) continue;
                if (n5.a == 212 && (n9 = (n3 + 2 >> 4) * this.B + (n2 >> 4)) <= this.B * this.C && n9 > 0 && ((n8 = this.A[n9]) & 0x80) == 128) {
                    h2.j += 2;
                    bl2 = true;
                }
                if (n5.a == 213 && (n9 = (n3 >> 4) * this.B + (n2 - 2 >> 4)) <= this.B * this.C && n9 > 0 && ((n8 = this.A[n9]) & 0x80) == 128) {
                    h2.i -= 2;
                    bl2 = true;
                }
                if (n5.a == 214 && (n9 = (n3 - 2 >> 4) * this.B + (n2 >> 4)) <= this.B * this.C && n9 > 0 && ((n8 = this.A[n9]) & 0x80) == 128) {
                    h2.j -= 2;
                    bl2 = true;
                }
                if (n5.a != 215 || (n9 = (n3 >> 4) * this.B + (n2 + 2 >> 4)) > this.B * this.C || n9 <= 0 || ((n8 = this.A[n9]) & 0x80) != 128) continue;
                h2.i += 2;
                bl2 = true;
                continue;
            }
            if (n5.a == 185) {
                if (h2.L <= 0) {
                    --h2.H;
                    if (h2.H < 0) {
                        h2.H = 0;
                    }
                    if (h2.e == 0) {
                        h2.j += 5;
                    }
                    if (h2.e == 1) {
                        h2.i -= 5;
                    }
                    if (h2.e == 2) {
                        h2.j -= 5;
                    }
                    if (h2.e == 3) {
                        h2.i += 5;
                    }
                    h2.L = 20;
                    this.a.g = true;
                }
                return false;
            }
            if (n5.v) {
                if (n5.a == 61 && !h2.p) {
                    if (h2.L <= 0) {
                        --h2.H;
                        if (h2.H < 0) {
                            h2.H = 0;
                        }
                        if (h2.e == 0) {
                            h2.j += 5;
                        }
                        if (h2.e == 1) {
                            h2.i -= 5;
                        }
                        if (h2.e == 2) {
                            h2.j -= 5;
                        }
                        if (h2.e == 3) {
                            h2.i += 5;
                        }
                        h2.L = 20;
                        this.a.g = true;
                    }
                    return false;
                }
                if (n4 == 0 && n5.j + n5.E < n3 + 36 && this.a(n5.i, n5.j - 1, h2.e, 100, n6)) {
                    if (bl) {
                        --n5.j;
                        --h2.j;
                    } else {
                        return true;
                    }
                }
                if (n4 == 1 && n5.i + n5.D + n5.F > n2 - 4 && this.a(n5.i, n5.j, h2.e, 100, n6)) {
                    if (bl) {
                        ++n5.i;
                        ++h2.i;
                    } else {
                        return true;
                    }
                }
                if (n4 == 2 && n5.j + n5.E + n5.G > n3 + 28 && this.a(n5.i, n5.j + 1, h2.e, 100, n6)) {
                    if (bl) {
                        ++n5.j;
                        ++h2.j;
                    } else {
                        return true;
                    }
                }
                if (n4 == 3 && n5.i + n5.F + n5.D < n2 + 4 && this.a(n5.i - 1, n5.j, h2.e, 100, n6)) {
                    if (bl) {
                        --n5.i;
                        --h2.i;
                    } else {
                        return true;
                    }
                }
            }
            if (n5.r == 2) {
                n5.s = true;
                if (!n5.e) continue;
                return false;
            }
            if (n5.r == 3) {
                n5.s = true;
                if (!n5.e) continue;
                return false;
            }
            return false;
        }
        if (h2.B) {
            for (n6 = 0; n6 < this.r.size(); ++n6) {
                n5 = (n)this.r.elementAt(n6);
                if (n5.i + n5.D > n2 + 16 || n5.i + n5.F + n5.D < n2 - 16 || n5.i + n5.D - 4 > n2 || n5.i + n5.F + n5.D + 4 < n2 || n5.j + n5.E - 4 > n3 + 32 || n5.j + n5.G + n5.E + 4 < n3 + 32) continue;
                return true;
            }
            if (h2.e == 0) {
                n3 -= 4;
            }
            if (h2.e == 1) {
                n2 += 4;
            }
            if (h2.e == 2) {
                n3 += 4;
            }
            if (h2.e == 3) {
                n2 -= 4;
            }
            if ((n6 = (n3 >> 4) * this.B + (n2 >> 4)) <= this.B * this.C && n6 > 0 && ((n6 = this.A[n6]) & 0x80) != 128) {
                return false;
            }
        }
        return true;
    }

    final int a(h h2, b b2, boolean bl) {
        f f2;
        int n2;
        int n3 = 0;
        int n4 = h2.i;
        int n5 = h2.j;
        int n6 = 0;
        int n7 = 0;
        if (h2.e == 0) {
            n4 += 6;
            n5 -= 9;
            n6 = 9;
            n7 = 18;
        }
        if (h2.e == 1) {
            n4 += 17;
            n5 += 12;
            n6 = 18;
            n7 = 9;
        }
        if (h2.e == 2) {
            n4 += 12;
            n5 += 20;
            n6 = 9;
            n7 = 18;
        }
        if (h2.e == 3) {
            n4 -= 12;
            n5 += 12;
            n6 = 18;
            n7 = 9;
        }
        for (n2 = 0; n2 < this.p.size(); ++n2) {
            f2 = (c)this.p.elementAt(n2);
            if (((c)f2).i > n4 + n6 || ((c)f2).i + (((c)f2).k << 1) < n4 || ((c)f2).j > n5 + n7 || ((c)f2).j + (((c)f2).l << 1) < n5 || ((c)f2).a == 128 || ((c)f2).a == 129 || (n3 = ((c)f2).a(h2, b2, this.a, true)) != 1) continue;
            this.p.removeElementAt(n2);
            this.a.g = true;
        }
        for (n2 = 0; n2 < this.q.size(); ++n2) {
            f2 = (l)this.q.elementAt(n2);
            if (!bl || !((l)f2).n && ((l)f2).b != 34 && ((l)f2).b != 135 && ((l)f2).b != 132) continue;
            if (((l)f2).b == 132) {
                if (this.a.e.x) {
                    if (this.a.e.z) {
                        this.a.j = true;
                        this.a.k = 32;
                        continue;
                    }
                    this.a.j = true;
                    this.a.k = 31;
                    this.a.e.aj |= 0x10000000;
                    this.a.e.z = true;
                    continue;
                }
                this.a.j = true;
                this.a.k = 30;
                continue;
            }
            if (((l)f2).b == 135) {
                if ((this.a.e.aj & 0x4000000) > 0 && this.a.e.V != 2) {
                    this.a.e.aj |= 0x8000000;
                    if (this.a.e.V > 2) continue;
                    this.a.j = true;
                    this.a.k = 27;
                    this.a.e.V = (byte)2;
                    this.a.e.M = 4;
                    this.a.e.a = null;
                    try {
                        this.a.e.a = null;
                        this.a.e.a = Image.createImage("/Images/LinkWalk_Red.png");
                        this.a.e.M = 4;
                        this.b.a[135] = null;
                        this.b.a[135] = Image.createImage("/Images/GreenTingle.png");
                        System.out.println("tingle test");
                        System.gc();
                    }
                    catch (IOException iOException) {}
                    continue;
                }
                if (this.a.e.V == 2) {
                    this.a.k = 29;
                    this.a.j = true;
                    continue;
                }
                this.a.k = 28;
                this.a.j = true;
                continue;
            }
            if (((l)f2).f || ((l)f2).i > n4 + n6 || ((l)f2).i + ((l)f2).B < n4 || ((l)f2).j > n5 + n7 || ((l)f2).j + ((l)f2).C < n5 || ((l)f2).b == 17) continue;
            if (((l)f2).b == 34) {
                this.a.j = true;
                this.a.k = 25;
                this.a.e.H = this.a.e.I / 4 << 2;
                this.a.g = true;
                continue;
            }
            if (((l)f2).t > 0) continue;
            boolean bl2 = false;
            if (((l)f2).b == 10 || ((l)f2).b == 27) {
                if (((l)f2).c == 0) {
                    if (h2.e != 2) {
                        ((l)f2).N -= h2.W;
                        bl2 = true;
                    } else {
                        ((l)f2).I = 5;
                        ((l)f2).H = 0;
                        ((l)f2).K = 2;
                    }
                }
                if (((l)f2).c == 1) {
                    if (h2.e != 3) {
                        ((l)f2).N -= h2.W;
                        bl2 = true;
                    } else {
                        ((l)f2).H = -5;
                        ((l)f2).I = 0;
                        ((l)f2).K = 2;
                    }
                }
                if (((l)f2).c == 2) {
                    if (h2.e != 0) {
                        ((l)f2).N -= h2.W;
                        bl2 = true;
                    } else {
                        ((l)f2).H = 0;
                        ((l)f2).I = -5;
                        ((l)f2).K = 2;
                    }
                }
                if (((l)f2).c == 3) {
                    if (h2.e != 1) {
                        ((l)f2).N -= h2.W;
                        bl2 = true;
                    } else {
                        ((l)f2).H = 5;
                        ((l)f2).I = 0;
                        ((l)f2).K = 2;
                    }
                }
            } else {
                ((l)f2).N -= h2.W;
                bl2 = true;
                if (((l)f2).b == 28 && ((l)f2).z == 3) {
                    ((l)f2).z = 4;
                    ((l)f2).F = 0;
                }
                if (((l)f2).b == 29 && ((l)f2).z == 3) {
                    ((l)f2).z = 4;
                    ((l)f2).F = 0;
                }
            }
            if (((l)f2).N <= 0) {
                ((l)f2).e = 0;
                ((l)f2).f = true;
                ((l)f2).q = this.w;
                ((l)f2).a = 0;
                continue;
            }
            if (!bl2) continue;
            ((l)f2).t = 5;
            if (((l)f2).b == 20 || ((l)f2).b == 21 || ((l)f2).b == 23 || ((l)f2).b == 24 || ((l)f2).b == 28 || ((l)f2).b == 29 || ((l)f2).b == 30 || ((l)f2).b == 31 || ((l)f2).b == 25) continue;
            if (h2.e == 0) {
                ((l)f2).I = -5;
                ((l)f2).H = 0;
            }
            if (h2.e == 1) {
                ((l)f2).H = 5;
                ((l)f2).I = 0;
            }
            if (h2.e == 2) {
                ((l)f2).H = 0;
                ((l)f2).I = 5;
            }
            if (h2.e != 3) continue;
            ((l)f2).H = -5;
            ((l)f2).I = 0;
        }
        for (n2 = 0; n2 < this.r.size(); ++n2) {
            f2 = (n)this.r.elementAt(n2);
            ((n)this.r.elementAt(n2)).C = this.g;
            if (!((n)f2).f && ((n)f2).u <= 0 || !((n)f2).w) continue;
            if (((n)f2).i + ((n)f2).D <= n4 + n6 && ((n)f2).i + ((n)f2).F + ((n)f2).D >= n4 && ((n)f2).j + ((n)f2).E <= n5 + n7 && ((n)f2).j + ((n)f2).G + ((n)f2).E >= n5) {
                if (((n)f2).u > 0) {
                    this.a.l = n2;
                    this.a.i = true;
                    h2.g = false;
                    return 0;
                }
                if (((n)f2).h) {
                    if (!bl) continue;
                    if (((n)f2).a == 11) {
                        ((n)f2).s = true;
                        ((n)f2).r = (byte)2;
                        ((n)f2).a = 8;
                        ((n)f2).J = this.b.c(8);
                        ((n)f2).e = true;
                        ((n)f2).f = false;
                        continue;
                    }
                }
                if (((n)f2).g) {
                    if (!bl) continue;
                    if (((n)f2).a == 3 || ((n)f2).a == 4 || ((n)f2).a == 15) {
                        if (!((n)f2).x) {
                            ((n)f2).x = true;
                            ((n)f2).A = 5;
                        }
                    } else {
                        this.r.removeElementAt(n2);
                    }
                }
                if (((n)f2).o) {
                    int n8;
                    if (((n)f2).a == 78 || ((n)f2).a == 77) {
                        if ((h2.aj & 0x100000) > 0 && ((n)f2).a == 77) {
                            int n9 = this.E / 32;
                            n3 = ((n)f2).c % 32;
                            int n10 = n9;
                            h2.ay[n10] = h2.ay[n10] | 1 << n3;
                            h2.g = false;
                            this.r.removeElementAt(n2);
                            n3 = 1;
                            this.a.g = true;
                            this.a.getClass();
                            n8 = 6;
                            b b3 = b2;
                            b2.d = n8;
                            continue;
                        }
                        if ((h2.aj & 0x200000) > 0 && ((n)f2).a == 78) {
                            int n11 = this.E / 32;
                            n3 = ((n)f2).c % 32;
                            int n12 = n11;
                            h2.ax[n12] = h2.ax[n12] | 1 << n3;
                            h2.g = false;
                            this.r.removeElementAt(n2);
                            n3 = 1;
                            this.a.g = true;
                            this.a.getClass();
                            n8 = 6;
                            b b4 = b2;
                            b2.d = n8;
                            continue;
                        }
                        this.a.k = 9;
                        this.a.j = true;
                        continue;
                    }
                    if (h2.N > 0) {
                        int n13 = this.E / 32;
                        n3 = ((n)f2).c % 32;
                        if (this.f == 0) {
                            int n14 = n13;
                            h2.ap[n14] = h2.ap[n14] | 1 << n3;
                        }
                        if (this.f == 1) {
                            int n15 = n13;
                            h2.aq[n15] = h2.aq[n15] | 1 << n3;
                        }
                        if (this.f == 3) {
                            int n16 = n13;
                            h2.ar[n16] = h2.ar[n16] | 1 << n3;
                        }
                        if (this.f == 4) {
                            int n17 = n13;
                            h2.as[n17] = h2.as[n17] | 1 << n3;
                        }
                        if (this.f == 5) {
                            int n18 = n13;
                            h2.at[n18] = h2.at[n18] | 1 << n3;
                        }
                        if (this.f == 6) {
                            int n19 = n13;
                            h2.au[n19] = h2.au[n19] | 1 << n3;
                            System.out.println("Curent worl set " + n3);
                        }
                        if (this.f == 7) {
                            int n20 = n13;
                            h2.av[n20] = h2.av[n20] | 1 << n3;
                        }
                        if (this.f == 8) {
                            int n21 = n13;
                            h2.aw[n21] = h2.aw[n21] | 1 << n3;
                        }
                        if (this.f == 9) {
                            int n22 = n13;
                            h2.ax[n22] = h2.ax[n22] | 1 << n3;
                        }
                        if (this.f == 10) {
                            int n23 = n13;
                            h2.ay[n23] = h2.ay[n23] | 1 << n3;
                        }
                        if (this.f == 11) {
                            int n24 = n13;
                            h2.az[n24] = h2.az[n24] | 1 << n3;
                        }
                        h2.g = false;
                        --h2.N;
                        this.r.removeElementAt(n2);
                        n3 = 1;
                        this.a.g = true;
                        this.a.getClass();
                        n8 = 6;
                        b b5 = b2;
                        b2.d = n8;
                    }
                }
                if (((n)f2).a == 30 && this.F == 0) {
                    ++this.g;
                    if (this.g > 1) {
                        this.g = 0;
                    }
                    if (this.g == 0) {
                        h2.an &= ~(1 << this.f);
                    }
                    if (this.g == 1) {
                        h2.an |= 1 << this.f;
                    }
                    ((n)f2).C = this.g;
                    this.F = 15;
                    System.out.println("p.WorldStates= " + h2.an);
                }
            }
            if (!bl) {
                return n3;
            }
            if (((n)f2).a != 3 && ((n)f2).a != 4 && ((n)f2).a != 15 || !((n)f2).x) continue;
            --((n)f2).A;
            if (((n)f2).A > 0) continue;
            this.a(((n)f2).i, ((n)f2).j, h2, ((n)f2).I);
            this.r.removeElementAt(n2);
        }
        return n3;
    }

    final boolean a(int n2, int n3, int n4, int n5, int n6) {
        f f2;
        n3 -= 32;
        int n7 = 0;
        int n8 = 0;
        int n9 = 0;
        if (n4 == 0) {
            n9 = -8;
        }
        if (n4 == 1) {
            n8 = 8;
        }
        if (n4 == 2) {
            n9 = 8;
        }
        if (n4 == 3) {
            n8 = -8;
        }
        if (n5 == 0) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 1) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 2) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 3) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 4) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 5) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 6) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 7) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 8) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 9) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 10) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 11) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 12) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 13) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 15) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 16) {
            n2 += 30;
            n3 += 32;
        }
        if (n5 == 19) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 25) {
            n2 += 48;
            n3 += 48;
        }
        if (n5 == 26) {
            n2 += 8;
            n3 += 8;
        }
        if (n5 == 28) {
            n2 += 16;
            n3 += 16;
        }
        if (n5 == 27) {
            n2 += 16;
            n3 += 16;
        }
        if (n5 == 29) {
            n2 += 48;
            n3 += 48;
        }
        if (n5 == 30) {
            n2 += 12;
            n3 += 12;
        }
        if (n5 == 31) {
            n2 += 12;
            n3 += 12;
        }
        if (n5 == 32) {
            n2 += 12;
            n3 += 12;
        }
        if (n5 == 100) {
            n2 += 8;
            n3 += 8;
        }
        if ((n7 = (n3 + n9 >> 4) * this.B + (n2 + n8 >> 4)) <= this.B * this.C && n7 > 0) {
            if (((n7 = this.A[n7]) & 0x40) == 64) {
                return false;
            }
            if ((n7 & 0x80) == 128 ? n5 != 18 && n5 != 26 && n5 != 33 : n5 == 26 || n5 == 33) {
                return false;
            }
        }
        n3 += 32;
        for (n7 = 0; n7 < this.r.size(); ++n7) {
            f2 = (n)this.r.elementAt(n7);
            if (!f2.e && f2.r < 2 || n6 == n7 && n5 == 100 || !f2.w || f2.i + f2.D > n2 + 6 || f2.i + f2.D + f2.F < n2 - 6 || f2.j + f2.E > n3 + 6 || f2.j + f2.E + f2.G < n3 - 6) continue;
            if (f2.r == 2) {
                f2.s = true;
                continue;
            }
            if (f2.r == 3) {
                f2.s = true;
                continue;
            }
            return false;
        }
        if (n5 == 100) {
            for (n7 = 0; n7 < this.q.size(); ++n7) {
                f2 = (l)this.q.elementAt(n7);
                if (((l)f2).i > n2 + n8 || ((l)f2).i + ((l)f2).B < n2 + n8 || ((l)f2).j > n3 + n9 || ((l)f2).j + ((l)f2).C < n3 + n9) continue;
                if (n4 == 4) {
                    return false;
                }
                if (n4 == 0) {
                    if (this.a(((l)f2).i, ((l)f2).j - 5, 0, ((l)f2).b, 0)) {
                        --((l)f2).j;
                        if (((l)f2).b == 9) {
                            ((l)f2).Q = true;
                        }
                    } else {
                        return false;
                    }
                }
                if (n4 == 1) {
                    if (this.a(((l)f2).i + 5, ((l)f2).j, 1, ((l)f2).b, 0)) {
                        ++((l)f2).i;
                        if (((l)f2).b == 9) {
                            ((l)f2).P = true;
                        }
                    } else {
                        return false;
                    }
                }
                if (n4 == 2) {
                    if (this.a(((l)f2).i, ((l)f2).j + 5, 2, ((l)f2).b, 0)) {
                        ++((l)f2).j;
                        if (((l)f2).b == 9) {
                            ((l)f2).Q = true;
                        }
                    } else {
                        return false;
                    }
                }
                if (n4 != 3) continue;
                if (this.a(((l)f2).i - 5, ((l)f2).j, 3, ((l)f2).b, 0)) {
                    --((l)f2).i;
                    if (((l)f2).b != 9) continue;
                    ((l)f2).P = true;
                    continue;
                }
                return false;
            }
        } else if (this.a(n2, n3, true) > 0) {
            return false;
        }
        return true;
    }

    final int a(h h2, b b2) {
        Object object;
        int n2;
        int n3 = 0;
        int n4 = h2.i;
        int n5 = h2.j;
        n4 += 12;
        n5 += 16;
        if (h2.L > 0) {
            --h2.L;
        }
        if (h2.L < 0) {
            h2.L = 0;
        }
        for (n2 = 0; n2 < this.p.size(); ++n2) {
            f f2;
            int n6;
            int n7;
            object = (c)this.p.elementAt(n2);
            if (((c)object).i + ((c)object).k <= n4 + 3 && ((c)object).i + ((c)object).k >= n4 - 10 && ((c)object).j + ((c)object).l <= n5 && ((c)object).j + ((c)object).l >= n5 - 16) {
                n7 = ((c)object).a(h2, b2, this.a, false);
                if (n7 == 1) {
                    this.p.removeElementAt(n2);
                    n3 = 1;
                    this.a.g = true;
                } else if (n7 == 2) {
                    n3 = 2;
                    this.a.g = true;
                }
            }
            if (((c)object).a != 4 || !((c)object).e) continue;
            --((c)object).b;
            if (((c)object).b > 0) continue;
            if (((c)object).b == 0) {
                ((c)object).g = 10;
                n7 = 4;
                object = b2;
                b2.d = n7;
                continue;
            }
            --((c)object).g;
            if (((c)object).g != 0) continue;
            n7 = 0;
            this.p.removeElementAt(n2);
            h h3 = h2;
            int n8 = ((c)object).j;
            n7 = ((c)object).i;
            object = this;
            int n9 = 0;
            for (n6 = 0; n6 < ((j)object).r.size(); ++n6) {
                f2 = (n)((j)object).r.elementAt(n6);
                if (f2.i > n7 + 24 || f2.i + f2.F < n7 - 24 || f2.j > n8 + 24 || f2.j + f2.G < n8 - 24) continue;
                if (f2.n) {
                    n9 = ((j)object).E / 32;
                    int n10 = f2.c % 32;
                    if (((j)object).f == 0) {
                        int n11 = n9;
                        h3.ap[n11] = h3.ap[n11] | 1 << n10;
                    }
                    if (((j)object).f == 1) {
                        int n12 = n9;
                        h3.aq[n12] = h3.aq[n12] | 1 << n10;
                    }
                    if (((j)object).f == 3) {
                        int n13 = n9;
                        h3.ar[n13] = h3.ar[n13] | 1 << n10;
                        System.out.println("Curent worl set");
                    }
                    if (((j)object).f == 4) {
                        int n14 = n9;
                        h3.as[n14] = h3.as[n14] | 1 << n10;
                    }
                    if (((j)object).f == 5) {
                        int n15 = n9;
                        h3.at[n15] = h3.at[n15] | 1 << n10;
                    }
                    if (((j)object).f == 6) {
                        int n16 = n9;
                        h3.au[n16] = h3.au[n16] | 1 << n10;
                        System.out.println("Curent worl set");
                    }
                    if (((j)object).f == 7) {
                        int n17 = n9;
                        h3.av[n17] = h3.av[n17] | 1 << n10;
                    }
                    if (((j)object).f == 8) {
                        int n18 = n9;
                        h3.aw[n18] = h3.aw[n18] | 1 << n10;
                    }
                    if (((j)object).f == 9) {
                        int n19 = n9;
                        h3.ax[n19] = h3.ax[n19] | 1 << n10;
                    }
                    if (((j)object).f == 10) {
                        int n20 = n9;
                        h3.ay[n20] = h3.ay[n20] | 1 << n10;
                    }
                    if (((j)object).f == 11) {
                        int n21 = n9;
                        h3.az[n21] = h3.az[n21] | 1 << n10;
                    }
                    ((j)object).r.removeElementAt(n6);
                    n9 = 1;
                }
                if (!f2.g) continue;
                if (f2.a == 3 || f2.a == 4 || f2.a == 15) {
                    if (f2.x) continue;
                    f2.x = true;
                    f2.A = 6;
                    continue;
                }
                ((j)object).r.removeElementAt(n6);
                super.a(f2.i, f2.j, h3, f2.I);
            }
            for (n6 = 0; n6 < ((j)object).q.size(); ++n6) {
                f2 = (l)((j)object).q.elementAt(n6);
                if (!((l)f2).n) {
                    if (((l)f2).b != 29) continue;
                    if (((l)f2).z == 8) {
                        ((l)f2).z = 9;
                        ((l)f2).F = 0;
                        continue;
                    }
                }
                if (((l)f2).f || ((l)f2).i > n7 + 24 || ((l)f2).i + ((l)f2).B < n7 - 24 || ((l)f2).j > n8 + 24 || ((l)f2).j + ((l)f2).C < n8 - 24 || ((l)f2).t > 0) continue;
                ((l)f2).N -= 2;
                if (((l)f2).N <= 0) {
                    ((l)f2).e = 0;
                    ((l)f2).f = true;
                    ((l)f2).q = ((j)object).w;
                    ((l)f2).a = 0;
                    continue;
                }
                ((l)f2).t = 5;
                if (((l)f2).b == 20) continue;
                if (((l)f2).i < n7) {
                    ((l)f2).I = -10;
                    ((l)f2).H = 0;
                }
                if (((l)f2).j > n8) {
                    ((l)f2).H = 10;
                    ((l)f2).I = 0;
                }
                if (((l)f2).i > n7) {
                    ((l)f2).H = 0;
                    ((l)f2).I = 10;
                }
                if (((l)f2).j >= n8) continue;
                ((l)f2).H = -10;
                ((l)f2).I = 0;
            }
            n7 = n9;
            if (n7 == 0) continue;
            n7 = 7;
            object = b2;
            b2.d = n7;
        }
        for (n2 = 0; n2 < this.r.size(); ++n2) {
            object = (n)this.r.elementAt(n2);
            if (((n)object).i <= n4 && ((n)object).i + ((n)object).F + ((n)object).D >= n4 && ((n)object).j <= n5 && ((n)object).j + ((n)object).G + ((n)object).E >= n5 && ((n)object).u == 1) {
                this.a.i = true;
                this.a.l = n2;
                continue;
            }
            if (((n)object).a != 3 && ((n)object).a != 4 && ((n)object).a != 15 || !((n)object).x) continue;
            --((n)object).A;
            if (((n)object).A >= 0) continue;
            this.a(((n)object).i, ((n)object).j, h2, ((n)object).I);
            this.r.removeElementAt(n2);
        }
        for (n2 = 0; n2 < this.q.size(); ++n2) {
            object = (l)this.q.elementAt(n2);
            if (((l)object).g || ((l)object).f || ((l)object).i > n4 || ((l)object).i + ((l)object).B < n4 || ((l)object).j > n5 || ((l)object).j + ((l)object).C < n5 || h2.L > 0) continue;
            h2.H -= ((l)object).O / h2.M;
            if (h2.H < 0) {
                h2.H = 0;
            }
            if (((l)object).b != 21) {
                if (h2.e == 0 && this.a(h2.i, h2.j + 10, h2.e, h2, true)) {
                    h2.j += 10;
                }
                if (h2.e == 1 && this.a(h2.i - 10, h2.j, h2.e, h2, true)) {
                    h2.i -= 10;
                }
                if (h2.e == 2 && this.a(h2.i, h2.j - 10, h2.e, h2, true)) {
                    h2.j -= 10;
                }
                if (h2.e == 3 && this.a(h2.i + 10, h2.j, h2.e, h2, true)) {
                    h2.i += 10;
                }
            } else if (this.a(h2.i + 50, h2.j, h2.e, h2, true)) {
                h2.i += 50;
            } else if (this.a(h2.i + 40, h2.j, h2.e, h2, true)) {
                h2.i += 40;
            } else if (this.a(h2.i + 30, h2.j, h2.e, h2, true)) {
                h2.i += 30;
            } else if (this.a(h2.i + 20, h2.j, h2.e, h2, true)) {
                h2.i += 20;
            } else if (this.a(h2.i + 10, h2.j, h2.e, h2, true)) {
                h2.i += 10;
            }
            n3 = 1;
            h2.L = 20;
        }
        return n3;
    }

    final int a(int n2, int n3, boolean n4) {
        n3 -= 32;
        if (n4 == 0) {
            n2 += 12;
            n3 += 16;
        }
        if (n4 != 0) {
            for (n4 = 0; n4 < this.o.size(); ++n4) {
                e e2 = (e)this.o.elementAt(n4);
                if ((e2.a << 4) - 4 > n2 || (e2.a + 1 << 4) + 4 < n2 || (e2.b << 4) - 4 > n3 || (e2.b + 1 << 4) + 4 < n3) continue;
                return n4 + 1;
            }
        } else {
            for (n4 = 0; n4 < this.o.size(); ++n4) {
                e e3 = (e)this.o.elementAt(n4);
                if (e3.a << 4 > n2 || e3.a + 1 << 4 < n2 || e3.b << 4 > n3 || e3.b + 1 << 4 < n3) continue;
                return n4 + 1;
            }
        }
        return -1;
    }

    private void a(int n2, int n3, h h2, int n4) {
        boolean bl = false;
        if (Math.abs(this.e.nextInt() % 4) == 3) {
            return;
        }
        int n5 = 0;
        switch (n4) {
            case 1: {
                n5 = Math.abs(this.e.nextInt() % 2);
                if (n5 <= 0) break;
                n5 = 3;
                break;
            }
            case 2: {
                n5 = Math.abs(this.e.nextInt() % 5);
                if (n5 != 2) break;
                n5 = 5;
                break;
            }
            case 3: {
                n5 = Math.abs(this.e.nextInt() % 6);
                if (n5 == 0) {
                    n5 = 1;
                }
                if (Math.abs(this.e.nextInt() % 6) != 2) break;
                bl = true;
                break;
            }
            default: {
                return;
            }
        }
        if (n5 == 6) {
            return;
        }
        if (n5 == 4 && !h2.s) {
            n5 = 1;
        }
        if (n5 == 5 && !h2.t) {
            n5 = 1;
        }
        c c2 = new c(this.b.a(n5), n5, n2, n3 - 32, 0, h2);
        this.p.addElement(c2);
        if (bl) {
            c2 = new c(this.b.a(n5), n5, n2 - 16, n3 - 32, 0, h2);
            this.p.addElement(c2);
            c2 = new c(this.b.a(n5), n5, n2 + 16, n3 - 32, 0, h2);
            this.p.addElement(c2);
            c2 = new c(this.b.a(n5), n5, n2, n3 - 16, 0, h2);
            this.p.addElement(c2);
            c c3 = new c(this.b.a(n5), n5, n2, n3, 0, h2);
            this.p.addElement(c3);
        }
    }

    /*
     * Unable to fully structure code
     * Could not resolve type clashes
     */
    final void b(h var1_1) {
        for (var2_2 = 0; var2_2 < this.q.size(); ++var2_2) {
            var3_3 = (l)this.q.elementAt(var2_2);
            var4_4 = 0;
            var5_5 = 0;
            var8_13 = this;
            var7_9 = var1_1;
            var6_6 /* !! */  = var3_3;
            if (var6_6 /* !! */ .t > 0 || var6_6 /* !! */ .K > 0) {
                --var6_6 /* !! */ .K;
                if (var6_6 /* !! */ .K < 0) {
                    var6_6 /* !! */ .K = 0;
                }
                if (var8_13.a(var6_6 /* !! */ .i + var6_6 /* !! */ .H, var6_6 /* !! */ .j + var6_6 /* !! */ .I, (int)var6_6 /* !! */ .c, var6_6 /* !! */ .b, 0) && var6_6 /* !! */ .b != 16 && var6_6 /* !! */ .b != 20 && var6_6 /* !! */ .b != 37) {
                    var6_6 /* !! */ .i += var6_6 /* !! */ .H;
                    var6_6 /* !! */ .j += var6_6 /* !! */ .I;
                }
            }
            switch (var6_6 /* !! */ .b) {
                case 7: {
                    if (var7_9.i + 24 > var6_6 /* !! */ .i && var7_9.i < var6_6 /* !! */ .i + 16) {
                        var6_6 /* !! */ .d = (byte)2;
                        if (var7_9.j > var6_6 /* !! */ .j && var6_6 /* !! */ .c == 2) {
                            var6_6 /* !! */ .d = (byte)3;
                        }
                        if (var7_9.j < var6_6 /* !! */ .j && var6_6 /* !! */ .c == 0) {
                            var6_6 /* !! */ .d = (byte)3;
                        }
                    }
                    if (var7_9.j + 24 <= var6_6 /* !! */ .j || var7_9.j >= var6_6 /* !! */ .j + 16) break;
                    var6_6 /* !! */ .d = (byte)2;
                    if (var7_9.i > var6_6 /* !! */ .i && var6_6 /* !! */ .c == 1) {
                        var6_6 /* !! */ .d = (byte)3;
                    }
                    if (var7_9.i >= var6_6 /* !! */ .i || var6_6 /* !! */ .c != 3) break;
                    var6_6 /* !! */ .d = (byte)3;
                    break;
                }
                case 8: {
                    var9_14 = new g();
                    new g().a = var6_6 /* !! */ .i;
                    var9_14.b = var6_6 /* !! */ .j;
                    var6_6 /* !! */ .T.addElement(var9_14);
                    var6_6 /* !! */ .T.removeElementAt(0);
                    for (var10_48 = 0; var10_48 < var6_6 /* !! */ .R.size(); ++var10_48) {
                        var11_81 = (g)var6_6 /* !! */ .R.elementAt(var10_48);
                        var12_102 = (g)var6_6 /* !! */ .T.elementAt(var10_48 * 5);
                        var11_81.a = var12_102.a;
                        var11_81.b = var12_102.b;
                        var6_6 /* !! */ .R.setElementAt(var11_81, var10_48);
                    }
                    break;
                }
                case 9: {
                    if (var7_9.i + 18 > var6_6 /* !! */ .i && var7_9.i + 6 < var6_6 /* !! */ .i + 16 && !var6_6 /* !! */ .P) {
                        if (var7_9.j > var6_6 /* !! */ .j && var8_13.a(var6_6 /* !! */ .i, var6_6 /* !! */ .j + 5, 2, 9, 0)) {
                            var6_6 /* !! */ .j += 3;
                            var6_6 /* !! */ .Q = true;
                        }
                        if (var7_9.j >= var6_6 /* !! */ .j || !var8_13.a(var6_6 /* !! */ .i, var6_6 /* !! */ .j - 5, 0, 9, 0)) break;
                        var6_6 /* !! */ .j -= 3;
                        var6_6 /* !! */ .Q = true;
                        break;
                    }
                    if (var7_9.j + 18 > var6_6 /* !! */ .j && var7_9.j + 6 < var6_6 /* !! */ .j + 16 && !var6_6 /* !! */ .Q) {
                        if (var7_9.i > var6_6 /* !! */ .i && var8_13.a(var6_6 /* !! */ .i + 5, var6_6 /* !! */ .j, 1, 9, 0)) {
                            var6_6 /* !! */ .i += 3;
                            var6_6 /* !! */ .P = true;
                        }
                        if (var7_9.i >= var6_6 /* !! */ .i || !var8_13.a(var6_6 /* !! */ .i - 5, var6_6 /* !! */ .j, 3, 9, 0)) break;
                        var6_6 /* !! */ .i -= 3;
                        var6_6 /* !! */ .P = true;
                        break;
                    }
                    if (var6_6 /* !! */ .j > var6_6 /* !! */ .E) {
                        if (var8_13.a(var6_6 /* !! */ .i, var6_6 /* !! */ .j - 3, 0, 9, 0)) {
                            --var6_6 /* !! */ .j;
                        } else if (var6_6 /* !! */ .j - 4 <= var6_6 /* !! */ .E) {
                            var6_6 /* !! */ .j = var6_6 /* !! */ .E;
                        }
                    }
                    if (var6_6 /* !! */ .j < var6_6 /* !! */ .E) {
                        if (var8_13.a(var6_6 /* !! */ .i, var6_6 /* !! */ .j + 3, 2, 9, 0)) {
                            ++var6_6 /* !! */ .j;
                        } else if (var6_6 /* !! */ .j + 4 >= var6_6 /* !! */ .E) {
                            var6_6 /* !! */ .j = var6_6 /* !! */ .E;
                        }
                    }
                    if (var6_6 /* !! */ .i > var6_6 /* !! */ .D) {
                        if (var8_13.a(var6_6 /* !! */ .i - 3, var6_6 /* !! */ .j, 3, 9, 0)) {
                            --var6_6 /* !! */ .i;
                        } else if (var6_6 /* !! */ .i - 4 <= var6_6 /* !! */ .D) {
                            var6_6 /* !! */ .i = var6_6 /* !! */ .D;
                        }
                    }
                    if (var6_6 /* !! */ .i < var6_6 /* !! */ .D) {
                        if (var8_13.a(var6_6 /* !! */ .i + 3, var6_6 /* !! */ .j, 1, 9, 0)) {
                            ++var6_6 /* !! */ .i;
                        } else if (var6_6 /* !! */ .i + 4 >= var6_6 /* !! */ .D) {
                            var6_6 /* !! */ .i = var6_6 /* !! */ .D;
                        }
                    }
                    if (var6_6 /* !! */ .j == var6_6 /* !! */ .E) {
                        var6_6 /* !! */ .Q = false;
                    }
                    if (var6_6 /* !! */ .i != var6_6 /* !! */ .D) break;
                    var6_6 /* !! */ .P = false;
                    break;
                }
                case 10: {
                    break;
                }
                case 11: {
                    if (var6_6 /* !! */ .z == 0) {
                        if (var7_9.i <= var6_6 /* !! */ .i - 48 || var7_9.i >= var6_6 /* !! */ .i + 48 || var7_9.j <= var6_6 /* !! */ .j - 48 || var7_9.j >= var6_6 /* !! */ .j + 48) break;
                        var6_6 /* !! */ .z = 1;
                        break;
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        if (var7_9.i > var6_6 /* !! */ .i - 24 && var7_9.i < var6_6 /* !! */ .i + 24) {
                            if (var7_9.j > var6_6 /* !! */ .j - 24 && var7_9.j < var6_6 /* !! */ .j + 24) {
                                var6_6 /* !! */ .z = 3;
                                var6_6 /* !! */ .h = false;
                                if (var7_9.i > var6_6 /* !! */ .i) {
                                    var6_6 /* !! */ .c = 1;
                                }
                                if (var7_9.i < var6_6 /* !! */ .i) {
                                    var6_6 /* !! */ .c = (byte)3;
                                }
                                if (var7_9.j > var6_6 /* !! */ .j) {
                                    var6_6 /* !! */ .c = (byte)2;
                                }
                                if (var7_9.j >= var6_6 /* !! */ .j) break;
                                var6_6 /* !! */ .c = 0;
                                break;
                            }
                            if (var7_9.j > var6_6 /* !! */ .j - 48 && var7_9.j < var6_6 /* !! */ .j + 48) break;
                            var6_6 /* !! */ .z = 0;
                            break;
                        }
                        if (var7_9.i > var6_6 /* !! */ .i - 48 && var7_9.i < var6_6 /* !! */ .i + 48) break;
                        var6_6 /* !! */ .z = 0;
                        break;
                    }
                    if (var6_6 /* !! */ .z != 3) break;
                    var9_15 = false;
                    var10_49 = false;
                    if (var7_9.i > var6_6 /* !! */ .i) {
                        var6_6 /* !! */ .c = 1;
                    }
                    if (var7_9.i < var6_6 /* !! */ .i) {
                        var6_6 /* !! */ .c = (byte)3;
                    }
                    if (var7_9.j > var6_6 /* !! */ .j) {
                        var6_6 /* !! */ .c = (byte)2;
                    }
                    if (var7_9.j >= var6_6 /* !! */ .j) break;
                    var6_6 /* !! */ .c = 0;
                    break;
                }
                case 13: {
                    if (var6_6 /* !! */ .z == 0) {
                        var9_16 = new Random();
                        var9_16.setSeed(var6_6 /* !! */ .a * 3456);
                        if (var9_16.nextInt() % 3 == 1) break;
                        var10_50 = var9_16.nextInt() % 4;
                        if (var10_50 == 0 && var8_13.a(var6_6 /* !! */ .i, var6_6 /* !! */ .j - 32, var10_50, 13, 0)) {
                            if (var6_6 /* !! */ .j - 32 < 32) break;
                            var6_6 /* !! */ .E = var6_6 /* !! */ .j - 32;
                            var6_6 /* !! */ .D = var6_6 /* !! */ .i;
                            var6_6 /* !! */ .c = (byte)var10_50;
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .A = 16;
                        }
                        if (var10_50 == 1 && var8_13.a(var6_6 /* !! */ .i + 32, var6_6 /* !! */ .j, var10_50, 13, 0)) {
                            if (var6_6 /* !! */ .i + 32 > 240) break;
                            var6_6 /* !! */ .E = var6_6 /* !! */ .j;
                            var6_6 /* !! */ .D = var6_6 /* !! */ .i + 32;
                            var6_6 /* !! */ .c = (byte)var10_50;
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .A = 16;
                        }
                        if (var10_50 == 2 && var8_13.a(var6_6 /* !! */ .i, var6_6 /* !! */ .j + 32, var10_50, 13, 0)) {
                            if (var6_6 /* !! */ .j + 32 > 176) break;
                            var6_6 /* !! */ .E = var6_6 /* !! */ .j + 32;
                            var6_6 /* !! */ .D = var6_6 /* !! */ .i;
                            var6_6 /* !! */ .c = (byte)var10_50;
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .A = 16;
                        }
                        if (var10_50 == 3 && var8_13.a(var6_6 /* !! */ .i - 32, var6_6 /* !! */ .j, var10_50, 13, 0)) {
                            if (var6_6 /* !! */ .i - 32 < 16) break;
                            var6_6 /* !! */ .E = var6_6 /* !! */ .j;
                            var6_6 /* !! */ .D = var6_6 /* !! */ .i - 32;
                            var6_6 /* !! */ .c = (byte)var10_50;
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .A = 16;
                        }
                    }
                    if (var6_6 /* !! */ .z != 1) break;
                    var6_6 /* !! */ .z = 1;
                    var6_6 /* !! */ .d = 1;
                    --var6_6 /* !! */ .A;
                    if (var6_6 /* !! */ .A < -16) {
                        var6_6 /* !! */ .A = -16;
                    }
                    var9_17 = false;
                    if (var6_6 /* !! */ .i < var6_6 /* !! */ .D) {
                        var6_6 /* !! */ .c = 1;
                        var9_17 = true;
                    }
                    if (var6_6 /* !! */ .i > var6_6 /* !! */ .D) {
                        var6_6 /* !! */ .c = (byte)3;
                        var9_17 = true;
                    }
                    if (var6_6 /* !! */ .j < var6_6 /* !! */ .E) {
                        var6_6 /* !! */ .c = (byte)2;
                        var9_17 = true;
                    }
                    if (var6_6 /* !! */ .j > var6_6 /* !! */ .E) {
                        var6_6 /* !! */ .c = 0;
                        var9_17 = true;
                    }
                    if (var9_17) break;
                    var6_6 /* !! */ .z = 0;
                    var6_6 /* !! */ .d = 0;
                    var6_6 /* !! */ .A = 16;
                    break;
                }
                case 14: {
                    var9_18 = 0;
                    var10_51 = 0;
                    if (var7_9.i > var6_6 /* !! */ .i) {
                        var9_18 = 1;
                        var6_6 /* !! */ .c = 1;
                    }
                    if (var7_9.i < var6_6 /* !! */ .i) {
                        var9_18 = -1;
                        var6_6 /* !! */ .c = (byte)3;
                    }
                    if (var7_9.j > var6_6 /* !! */ .j) {
                        var10_51 = 1;
                        var6_6 /* !! */ .c = (byte)2;
                    }
                    if (var7_9.j < var6_6 /* !! */ .j) {
                        var10_51 = -1;
                        var6_6 /* !! */ .c = 0;
                    }
                    if (var7_9.A && (var7_9.i - var6_6 /* !! */ .i) * (var7_9.i - var6_6 /* !! */ .i) + (var7_9.j - var6_6 /* !! */ .j) * (var7_9.j - var6_6 /* !! */ .j) <= 1024) {
                        if (var7_9.i >= var6_6 /* !! */ .i) {
                            var9_18 = -3;
                            var6_6 /* !! */ .c = (byte)3;
                        }
                        if (var7_9.i < var6_6 /* !! */ .i) {
                            var9_18 = 3;
                            var6_6 /* !! */ .c = 1;
                        }
                        if (var7_9.j >= var6_6 /* !! */ .j) {
                            var10_51 = -3;
                            var6_6 /* !! */ .c = 0;
                        }
                        if (var7_9.j < var6_6 /* !! */ .j) {
                            var10_51 = 3;
                            var6_6 /* !! */ .c = (byte)2;
                        }
                    }
                    for (var11_82 = 0; var11_82 < var8_13.r.size(); ++var11_82) {
                        var12_103 = (n)var8_13.r.elementAt(var11_82);
                        if (var12_103.a != 8 || (var12_103.i - var6_6 /* !! */ .i) * (var12_103.i - var6_6 /* !! */ .i) + (var12_103.i - var6_6 /* !! */ .j) * (var12_103.i - var6_6 /* !! */ .j) > 1024) continue;
                        if (var6_6 /* !! */ .c == 0) {
                            var10_51 = 3;
                            var9_18 = 0;
                        }
                        if (var6_6 /* !! */ .c == 1) {
                            var9_18 = -3;
                            var10_51 = 0;
                        }
                        if (var6_6 /* !! */ .c == 2) {
                            var10_51 = -3;
                            var9_18 = 0;
                        }
                        if (var6_6 /* !! */ .c != 3) continue;
                        var9_18 = 3;
                        var10_51 = 0;
                    }
                    var6_6 /* !! */ .j += var10_51;
                    var6_6 /* !! */ .i += var9_18;
                    break;
                }
                case 19: {
                    if (var6_6 /* !! */ .z == 0) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > var6_6 /* !! */ .G) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .G = 15;
                        }
                        var6_6 /* !! */ .d = (byte)4;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = false;
                        break;
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        if (var6_6 /* !! */ .F > var6_6 /* !! */ .G) {
                            var6_6 /* !! */ .z = 2;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .G = 50;
                        }
                        var6_6 /* !! */ .d = 0;
                        var9_19 = var7_9.i - var6_6 /* !! */ .i;
                        var10_52 = var7_9.j - var6_6 /* !! */ .j;
                        if (Math.abs(var9_19) <= Math.abs(var10_52)) {
                            if (var10_52 >= 0) {
                                var6_6 /* !! */ .c = (byte)2;
                                break;
                            }
                            var6_6 /* !! */ .c = 0;
                            break;
                        }
                        if (var9_19 < 0) {
                            var6_6 /* !! */ .c = (byte)3;
                            break;
                        }
                    } else {
                        if (var6_6 /* !! */ .z != 2) break;
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        if (var6_6 /* !! */ .F > var6_6 /* !! */ .G) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                            var9_20 = new Random();
                            var9_20.setSeed(var6_6 /* !! */ .i * var6_6 /* !! */ .j + var9_20.nextInt());
                            var6_6 /* !! */ .G = var9_20.nextInt() % 100 + 50;
                            var6_6 /* !! */ .c = (byte)(var9_20.nextInt() % 4);
                        }
                        if (var6_6 /* !! */ .F == 15) {
                            var9_21 = 0;
                            var10_53 = 0;
                            if (var6_6 /* !! */ .c == 0) {
                                var9_21 = -1;
                            }
                            if (var6_6 /* !! */ .c == 1) {
                                var10_53 = 1;
                            }
                            if (var6_6 /* !! */ .c == 2) {
                                var9_21 = 1;
                            }
                            if (var6_6 /* !! */ .c == 3) {
                                var10_53 = -1;
                            }
                            var11_83 = new p(var8_13.b.b(5), 5, var10_53, var9_21);
                            new p(var8_13.b.b(5), 5, var10_53, var9_21).i = var6_6 /* !! */ .i + var10_53 * 16;
                            var11_83.j = var6_6 /* !! */ .j + var9_21 * 16;
                            var11_83.p = true;
                            var8_13.n.addElement(var11_83);
                        }
                        var9_22 = var7_9.i - var6_6 /* !! */ .i;
                        var10_54 = var7_9.j - var6_6 /* !! */ .j;
                        if (Math.abs(var9_22) <= Math.abs(var10_54)) {
                            if (var10_54 >= 0) {
                                var6_6 /* !! */ .c = (byte)2;
                                break;
                            }
                            var6_6 /* !! */ .c = 0;
                            break;
                        }
                        if (var9_22 < 0) {
                            var6_6 /* !! */ .c = (byte)3;
                            break;
                        }
                    }
                    ** GOTO lbl1525
                }
                case 20: {
                    var9_23 = new Random();
                    var9_23.setSeed(var6_6 /* !! */ .i * var6_6 /* !! */ .j + var9_23.nextInt());
                    var10_55 = false;
                    var9_23.nextInt();
                    var11_84 = false;
                    var12_104 = false;
                    var6_6 /* !! */ .R.elementAt(var6_6 /* !! */ .R.size() - 1);
                    var11_85 = null;
                    for (var7_10 = var6_6 /* !! */ .R.size() - 1; var7_10 > 0; --var7_10) {
                        var10_56 = (g)var6_6 /* !! */ .R.elementAt(var7_10);
                        if (var7_10 == var6_6 /* !! */ .R.size() - 1) {
                            var10_56.b += var6_6 /* !! */ .H;
                            if (var10_56.a < var6_6 /* !! */ .i - 48) {
                                var10_56.a = var6_6 /* !! */ .i - 48;
                                var6_6 /* !! */ .H = -var6_6 /* !! */ .H;
                            }
                            if (var10_56.a > var6_6 /* !! */ .i + 48) {
                                var10_56.a = var6_6 /* !! */ .i + 48;
                                var6_6 /* !! */ .H = -var6_6 /* !! */ .H;
                            }
                            if (var10_56.b < var6_6 /* !! */ .E - 48) {
                                var10_56.b = var6_6 /* !! */ .E - 48;
                                var6_6 /* !! */ .H = -var6_6 /* !! */ .H;
                            }
                            if (var10_56.b > var6_6 /* !! */ .E + 48) {
                                var10_56.b = var6_6 /* !! */ .E + 48;
                                var6_6 /* !! */ .H = -var6_6 /* !! */ .H;
                            }
                            var6_6 /* !! */ .R.setElementAt(var10_56, var7_10);
                            continue;
                        }
                        var11_85 = (g)var6_6 /* !! */ .R.elementAt(var7_10 + 1);
                        var12_105 = (g)var6_6 /* !! */ .R.elementAt(var6_6 /* !! */ .R.size() - 1);
                        var10_56.a = var11_85.a + (var6_6 /* !! */ .i - var12_105.a) / 4;
                        var10_56.b = var11_85.b + (var6_6 /* !! */ .E - var12_105.b) / 4;
                        var6_6 /* !! */ .R.setElementAt(var10_56, var7_10);
                    }
                    var6_6 /* !! */ .S.elementAt(var6_6 /* !! */ .S.size() - 1);
                    for (var7_10 = var6_6 /* !! */ .S.size() - 1; var7_10 > 0; --var7_10) {
                        var10_57 = (g)var6_6 /* !! */ .S.elementAt(var7_10);
                        if (var7_10 == var6_6 /* !! */ .S.size() - 1) {
                            var10_57.b += var6_6 /* !! */ .I;
                            if (var10_57.a < var6_6 /* !! */ .i - 48) {
                                var10_57.a = var6_6 /* !! */ .i - 48;
                                var6_6 /* !! */ .I = -var6_6 /* !! */ .I;
                            }
                            if (var10_57.a > var6_6 /* !! */ .i + 48) {
                                var10_57.a = var6_6 /* !! */ .i + 48;
                                var6_6 /* !! */ .I = -var6_6 /* !! */ .I;
                            }
                            if (var10_57.b < var6_6 /* !! */ .E - 48) {
                                var10_57.b = var6_6 /* !! */ .E - 48;
                                var6_6 /* !! */ .I = -var6_6 /* !! */ .I;
                            }
                            if (var10_57.b > var6_6 /* !! */ .E + 48) {
                                var10_57.b = var6_6 /* !! */ .E + 48;
                                var6_6 /* !! */ .I = -var6_6 /* !! */ .I;
                            }
                            var6_6 /* !! */ .S.setElementAt(var10_57, var7_10);
                            continue;
                        }
                        var11_85 = (g)var6_6 /* !! */ .S.elementAt(var7_10 + 1);
                        var12_106 = (g)var6_6 /* !! */ .S.elementAt(var6_6 /* !! */ .S.size() - 1);
                        var10_57.a = var11_85.a + (var6_6 /* !! */ .i - var12_106.a) / 4;
                        var10_57.b = var11_85.b + (var6_6 /* !! */ .E - var12_106.b) / 4;
                        var6_6 /* !! */ .S.setElementAt(var10_57, var7_10);
                    }
                    var7_9 = (g)var6_6 /* !! */ .R.elementAt(var6_6 /* !! */ .R.size() - 1);
                    if (var9_23.nextInt() % 30 == 1) {
                        var10_58 = new p(var8_13.b.b(6), 6, -1, 0);
                        new p(var8_13.b.b(6), 6, -1, 0).i = var7_9.a;
                        var10_58.j = var7_9.b;
                        var7_9.c = -10;
                        var10_58.p = true;
                        var8_13.n.addElement(var10_58);
                    }
                    var7_9 = (g)var6_6 /* !! */ .S.elementAt(var6_6 /* !! */ .S.size() - 1);
                    if (Math.abs(var9_23.nextInt() % 30) != 5) break;
                    var10_59 = new p(var8_13.b.b(6), 6, -1, 0);
                    new p(var8_13.b.b(6), 6, -1, 0).i = var7_9.a;
                    var10_59.j = var7_9.b;
                    var7_9.c = -10;
                    var10_59.p = true;
                    var8_13.n.addElement(var10_59);
                    break;
                }
                case 21: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .i = var7_9.i - 20;
                        var6_6 /* !! */ .j = var7_9.j - 90;
                        var6_6 /* !! */ .B = 55;
                        var6_6 /* !! */ .C = 90;
                        var6_6 /* !! */ .k = 25;
                        var6_6 /* !! */ .l = 45;
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .o = false;
                        if (var6_6 /* !! */ .F == 50) {
                            var6_6 /* !! */ .z = 1;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .n = true;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .o = true;
                        var6_6 /* !! */ .B = 55;
                        var6_6 /* !! */ .C = 90;
                        var6_6 /* !! */ .k = 25;
                        var6_6 /* !! */ .l = 45;
                        if (var6_6 /* !! */ .F == 60) {
                            var6_6 /* !! */ .z = 2;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        var6_6 /* !! */ .n = true;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .o = true;
                        if (var6_6 /* !! */ .F == 70) {
                            var6_6 /* !! */ .j += 37;
                            var6_6 /* !! */ .B = 68;
                            var6_6 /* !! */ .C = 60;
                            var6_6 /* !! */ .k = 34;
                            var6_6 /* !! */ .l = 30;
                        }
                        if (var6_6 /* !! */ .F == 75) {
                            var6_6 /* !! */ .i += 20;
                            var6_6 /* !! */ .B = 68;
                            var6_6 /* !! */ .C = 80;
                            var6_6 /* !! */ .k = 34;
                            var6_6 /* !! */ .l = 30;
                        }
                        if (var6_6 /* !! */ .F == 80) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var7_9.A && var6_6 /* !! */ .z != 0) {
                        var6_6 /* !! */ .o = true;
                        var6_6 /* !! */ .n = true;
                    } else {
                        var6_6 /* !! */ .o = false;
                        var6_6 /* !! */ .n = false;
                    }
                    ++var6_6 /* !! */ .F;
                    break;
                }
                case 22: {
                    if (var6_6 /* !! */ .z == 0) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > var6_6 /* !! */ .G) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .G = 15;
                            ++var6_6 /* !! */ .J;
                            if (var6_6 /* !! */ .J == 4) {
                                var6_6 /* !! */ .J = 0;
                                var6_6 /* !! */ .z = 3;
                            }
                        }
                        var6_6 /* !! */ .d = (byte)10;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = false;
                        break;
                    }
                    if (var6_6 /* !! */ .z != 1) ** GOTO lbl464
                    ++var6_6 /* !! */ .F;
                    var6_6 /* !! */ .g = false;
                    var6_6 /* !! */ .h = true;
                    if (var6_6 /* !! */ .F > var6_6 /* !! */ .G) {
                        var6_6 /* !! */ .z = 2;
                        var6_6 /* !! */ .F = 0;
                        var6_6 /* !! */ .G = 50;
                    }
                    var6_6 /* !! */ .d = 0;
                    var9_24 = var7_9.i - var6_6 /* !! */ .i;
                    var10_60 = false;
                    if (var9_24 < 0) {
                        var6_6 /* !! */ .c = (byte)3;
                        break;
                    }
                    ** GOTO lbl1525
lbl464:
                    // 1 sources

                    if (var6_6 /* !! */ .z != 2) ** GOTO lbl498
                    ++var6_6 /* !! */ .F;
                    var6_6 /* !! */ .d = 0;
                    var6_6 /* !! */ .g = false;
                    var6_6 /* !! */ .h = true;
                    if (var6_6 /* !! */ .F > var6_6 /* !! */ .G) {
                        var6_6 /* !! */ .z = 0;
                        var6_6 /* !! */ .F = 0;
                        var9_25 = new Random();
                        var9_25.setSeed(var6_6 /* !! */ .i * var6_6 /* !! */ .j + var9_25.nextInt());
                        var6_6 /* !! */ .G = var9_25.nextInt() % 100 + 50;
                        var6_6 /* !! */ .c = (byte)(var9_25.nextInt() % 4);
                    }
                    if (var6_6 /* !! */ .F == 15) {
                        var9_26 = 0;
                        var10_61 = 0;
                        if (var6_6 /* !! */ .c == 0) {
                            var9_26 = -1;
                        }
                        if (var6_6 /* !! */ .c == 1) {
                            var10_61 = 1;
                        }
                        if (var6_6 /* !! */ .c == 2) {
                            var9_26 = 1;
                        }
                        if (var6_6 /* !! */ .c == 3) {
                            var10_61 = -1;
                        }
                        var11_86 = new p(var8_13.b.b(5), 5, var10_61, var9_26);
                        new p(var8_13.b.b(5), 5, var10_61, var9_26).i = var6_6 /* !! */ .i + var10_61 * 16;
                        var11_86.j = var6_6 /* !! */ .j + var9_26 * 16 + 16;
                        var11_86.p = true;
                        var8_13.n.addElement(var11_86);
                    }
                    var9_27 = var7_9.i - var6_6 /* !! */ .i;
                    var10_62 = false;
                    if (var9_27 < 0) {
                        var6_6 /* !! */ .c = (byte)3;
                        break;
                    }
                    ** GOTO lbl1525
lbl498:
                    // 1 sources

                    if (var6_6 /* !! */ .z != 3) break;
                    ++var6_6 /* !! */ .F;
                    var6_6 /* !! */ .d = 0;
                    var6_6 /* !! */ .g = false;
                    var6_6 /* !! */ .h = true;
                    var6_6 /* !! */ .i = 128;
                    var6_6 /* !! */ .j = 90;
                    var6_6 /* !! */ .c = 1;
                    if (var6_6 /* !! */ .F == 20) {
                        var9_28 = new Random();
                        var9_28.setSeed(var6_6 /* !! */ .i * var6_6 /* !! */ .j + var9_28.nextInt());
                        var10_63 = new l(var8_13.b.d(19), var6_6 /* !! */ .i + 16, var6_6 /* !! */ .j - 16, 1, 19);
                        var8_13.q.addElement(var10_63);
                    }
                    if (var6_6 /* !! */ .F == 30) {
                        var9_29 = new Random();
                        var9_29.setSeed(var6_6 /* !! */ .i * var6_6 /* !! */ .j + var9_29.nextInt());
                        var10_64 = new l(var8_13.b.d(19), var6_6 /* !! */ .i + 16, var6_6 /* !! */ .j, 1, 19);
                        var8_13.q.addElement(var10_64);
                    }
                    if (var6_6 /* !! */ .F == 40) {
                        var9_30 = new Random();
                        var9_30.setSeed(var6_6 /* !! */ .i * var6_6 /* !! */ .j + var9_30.nextInt());
                        var10_65 = new l(var8_13.b.d(19), var6_6 /* !! */ .i + 16, var6_6 /* !! */ .j + 16, 1, 19);
                        var8_13.q.addElement(var10_65);
                    }
                    if (var6_6 /* !! */ .F != 60) break;
                    var6_6 /* !! */ .z = 0;
                    var6_6 /* !! */ .F = 0;
                    break;
                }
                case 23: {
                    if (var6_6 /* !! */ .z == 10) {
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .i += 5;
                        if (var6_6 /* !! */ .i > 200) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .c = (byte)3;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 11) {
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .i -= 5;
                        if (var6_6 /* !! */ .i < 20) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .c = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .c == 1) {
                            if (var7_9.i > 60) {
                                var6_6 /* !! */ .j = 176 - var7_9.j;
                            } else if (var6_6 /* !! */ .j + 50 > var7_9.j && var6_6 /* !! */ .j - 50 < var7_9.j) {
                                var6_6 /* !! */ .z = 1;
                                var6_6 /* !! */ .F = 0;
                            } else {
                                var6_6 /* !! */ .z = 10;
                            }
                        }
                        if (var6_6 /* !! */ .c == 3) {
                            if (var7_9.i < 170) {
                                var6_6 /* !! */ .j = 176 - var7_9.j;
                            } else if (var6_6 /* !! */ .j + 50 > var7_9.j && var6_6 /* !! */ .j - 50 < var7_9.j) {
                                var6_6 /* !! */ .z = 1;
                                var6_6 /* !! */ .F = 0;
                            } else {
                                var6_6 /* !! */ .z = 11;
                            }
                        }
                        if (var6_6 /* !! */ .F == 50) {
                            var6_6 /* !! */ .z = 3;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = 2;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .O = 12;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F == 1 && var6_6 /* !! */ .c == 3) {
                            var6_6 /* !! */ .i -= 30;
                        }
                        var6_6 /* !! */ .B = 60;
                        var6_6 /* !! */ .C = 64;
                        if (var6_6 /* !! */ .F >= 10) {
                            var6_6 /* !! */ .O = 4;
                            var6_6 /* !! */ .F = 0;
                            if (var6_6 /* !! */ .c == 3 && var6_6 /* !! */ .F == 10) {
                                var6_6 /* !! */ .i += 30;
                            }
                            if (var7_9.i <= 100) {
                                var6_6 /* !! */ .B = 35;
                                var6_6 /* !! */ .C = 43;
                                var6_6 /* !! */ .z = 10;
                                break;
                            }
                            if (var7_9.i >= 120) {
                                var6_6 /* !! */ .B = 35;
                                var6_6 /* !! */ .C = 43;
                                var6_6 /* !! */ .z = 11;
                                break;
                            }
                            var6_6 /* !! */ .B = 35;
                            var6_6 /* !! */ .C = 43;
                            var6_6 /* !! */ .z = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 3) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = 4;
                        }
                    }
                    if (var6_6 /* !! */ .z == 4) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .z = 5;
                            var6_6 /* !! */ .F = 0;
                        }
                        if (var6_6 /* !! */ .F == 2) {
                            var6_6 /* !! */ .n = false;
                            var9_31 = 0;
                            var10_66 = false;
                            if (var6_6 /* !! */ .c == 1) {
                                var9_31 = 1;
                            }
                            if (var6_6 /* !! */ .c == 3) {
                                var9_31 = -1;
                            }
                            var11_87 = new p(var8_13.b.b(7), 7, var9_31, 0);
                            new p(var8_13.b.b(7), 7, var9_31, 0).i = var6_6 /* !! */ .i + var9_31 * 16;
                            var11_87.j = var6_6 /* !! */ .j + 16;
                            var11_87.p = true;
                            var8_13.n.addElement(var11_87);
                        }
                    }
                    if (var6_6 /* !! */ .z == 5) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 70) {
                            var6_6 /* !! */ .F = 0;
                            var9_32 = new Random();
                            var10_67 = Math.abs(var9_32.nextInt()) % 3;
                            if (var10_67 == 0) {
                                var6_6 /* !! */ .z = 4;
                            }
                            if (var10_67 == 1) {
                                var6_6 /* !! */ .z = 2;
                            }
                            if (var10_67 == 2) {
                                var6_6 /* !! */ .z = var7_9.i > var6_6 /* !! */ .i ? 10 : 11;
                            }
                        }
                        if (var6_6 /* !! */ .c == 1 && var7_9.i <= 60) {
                            if (var6_6 /* !! */ .j + 50 > var7_9.j && var6_6 /* !! */ .j - 50 < var7_9.j) {
                                var6_6 /* !! */ .z = 1;
                                var6_6 /* !! */ .F = 0;
                            } else {
                                var6_6 /* !! */ .z = 10;
                            }
                        }
                        if (var6_6 /* !! */ .c == 3 && var7_9.i >= 170) {
                            if (var6_6 /* !! */ .j + 50 > var7_9.j && var6_6 /* !! */ .j - 50 < var7_9.j) {
                                var6_6 /* !! */ .z = 1;
                                var6_6 /* !! */ .F = 0;
                            } else {
                                var6_6 /* !! */ .z = 11;
                            }
                        }
                    }
                    if (var6_6 /* !! */ .z == 6) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = 5;
                        }
                    }
                    if (var6_6 /* !! */ .z == 7) {
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = 8;
                            var6_6 /* !! */ .n = false;
                        }
                    }
                    if (var6_6 /* !! */ .z != 8) break;
                    var6_6 /* !! */ .n = false;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 10) break;
                    var6_6 /* !! */ .F = 0;
                    var6_6 /* !! */ .z = 0;
                    break;
                }
                case 24: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .j > 50) {
                            var6_6 /* !! */ .j -= 4;
                            var6_6 /* !! */ .F = 0;
                        } else {
                            if (var6_6 /* !! */ .i < 256 - var7_9.i) {
                                var6_6 /* !! */ .i += 4;
                            }
                            if (var6_6 /* !! */ .i > 256 - var7_9.i) {
                                var6_6 /* !! */ .i -= 4;
                            }
                        }
                        if (var6_6 /* !! */ .F > 20) {
                            var9_33 = new Random();
                            var10_68 = Math.abs(var9_33.nextInt(Math.abs((int)System.currentTimeMillis()))) % 3;
                            if (var6_6 /* !! */ .N > 20) {
                                if (var10_68 == 0) {
                                    var6_6 /* !! */ .z = 2;
                                    var6_6 /* !! */ .F = 0;
                                    var6_6 /* !! */ .w = 4.0f;
                                    var6_6 /* !! */ .v = (float)(var7_9.i - var6_6 /* !! */ .i + 1) / ((float)(var7_9.j - var6_6 /* !! */ .j + 1) / var6_6 /* !! */ .w);
                                    var6_6 /* !! */ .x = var6_6 /* !! */ .i;
                                    var6_6 /* !! */ .y = var6_6 /* !! */ .j;
                                }
                                if (var10_68 == 1) {
                                    var6_6 /* !! */ .z = 3;
                                    var6_6 /* !! */ .F = 0;
                                }
                                if (var10_68 != 2) break;
                                var6_6 /* !! */ .z = 4;
                                var6_6 /* !! */ .F = 0;
                                break;
                            }
                            if (var10_68 == 0) {
                                var6_6 /* !! */ .z = 5;
                                var6_6 /* !! */ .F = 0;
                                var6_6 /* !! */ .w = 5.0f;
                                var6_6 /* !! */ .v = (float)(var7_9.i - var6_6 /* !! */ .i + 1) / ((float)(var7_9.j - var6_6 /* !! */ .j + 1) / var6_6 /* !! */ .w);
                                var6_6 /* !! */ .x = var6_6 /* !! */ .i;
                                var6_6 /* !! */ .y = var6_6 /* !! */ .j;
                            }
                            if (var10_68 == 1) {
                                var6_6 /* !! */ .z = 6;
                                var6_6 /* !! */ .F = 0;
                            }
                            if (var10_68 != 2) break;
                            var6_6 /* !! */ .z = 7;
                            var6_6 /* !! */ .F = 0;
                            break;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2 || var6_6 /* !! */ .z == 5) {
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .x += var6_6 /* !! */ .v;
                        var6_6 /* !! */ .y += var6_6 /* !! */ .w;
                        var6_6 /* !! */ .i = (int)var6_6 /* !! */ .x;
                        var6_6 /* !! */ .j = (int)var6_6 /* !! */ .y;
                        if (var6_6 /* !! */ .j > 150) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                        if (var6_6 /* !! */ .i > 220) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                        if (var6_6 /* !! */ .i < 20) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 3) {
                        var6_6 /* !! */ .n = false;
                        if (var7_9.i > var6_6 /* !! */ .i) {
                            ++var6_6 /* !! */ .i;
                        }
                        if (var7_9.i < var6_6 /* !! */ .i) {
                            --var6_6 /* !! */ .i;
                        }
                        if (var7_9.j > var6_6 /* !! */ .j) {
                            ++var6_6 /* !! */ .j;
                        }
                        if (var7_9.j < var6_6 /* !! */ .j) {
                            --var6_6 /* !! */ .j;
                        }
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 4) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F == 5) {
                            var9_34 = false;
                            var10_69 = false;
                            var11_88 = new p(var8_13.b.b(7), 7, 0, 1);
                            new p(var8_13.b.b(7), 7, 0, 1).i = var6_6 /* !! */ .i + 16;
                            var11_88.j = var6_6 /* !! */ .j + 16 + 16;
                            var11_88.p = true;
                            var8_13.n.addElement(var11_88);
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 6) {
                        var6_6 /* !! */ .n = false;
                        if (var7_9.i - 16 > var6_6 /* !! */ .i) {
                            var6_6 /* !! */ .i += 4;
                        }
                        if (var7_9.i - 16 < var6_6 /* !! */ .i) {
                            var6_6 /* !! */ .i -= 4;
                        }
                        if (var7_9.j - 45 > var6_6 /* !! */ .j) {
                            var6_6 /* !! */ .j += 4;
                        }
                        if (var7_9.j - 45 < var6_6 /* !! */ .j) {
                            var6_6 /* !! */ .j -= 4;
                        }
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 8;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 7) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F == 5) {
                            var9_35 = false;
                            var10_70 = false;
                            var11_89 = new p(var8_13.b.b(7), 7, 0, 1);
                            new p(var8_13.b.b(7), 7, 0, 1).i = var6_6 /* !! */ .i + 16;
                            var11_89.j = var6_6 /* !! */ .j + 16 + 16;
                            var11_89.p = true;
                            var8_13.n.addElement(var11_89);
                            var12_107 = new p(var8_13.b.b(7), 7, 1, 1);
                            new p(var8_13.b.b(7), 7, 1, 1).i = var6_6 /* !! */ .i + 16 + 16;
                            var12_107.j = var6_6 /* !! */ .j + 16 + 16;
                            var12_107.p = true;
                            var8_13.n.addElement(var12_107);
                            var11_89 = new p(var8_13.b.b(7), 7, -1, 1);
                            new p(var8_13.b.b(7), 7, -1, 1).i = var6_6 /* !! */ .i + -16 + 16;
                            var11_89.j = var6_6 /* !! */ .j + 16 + 16;
                            var11_89.p = true;
                            var8_13.n.addElement(var11_89);
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 8) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F == 8) {
                            var6_6 /* !! */ .z = 9;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 9) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .C = 64;
                        if (var6_6 /* !! */ .F == 3) {
                            var6_6 /* !! */ .z = 10;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 10) {
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .C = 64;
                        if (var6_6 /* !! */ .F == 5) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .C = 48;
                        }
                    }
                    if (var6_6 /* !! */ .z == 20) {
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .J == 3) {
                            var6_6 /* !! */ .z = 21;
                        }
                        if (var6_6 /* !! */ .F == 70) {
                            var6_6 /* !! */ .z = 21;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z != 21) break;
                    var6_6 /* !! */ .n = false;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F == 5) {
                        for (var9_36 = 0; var9_36 < var8_13.q.size(); ++var9_36) {
                            var10_71 = (l)var8_13.q.elementAt(var9_36);
                            if (var10_71.b != 134) continue;
                            var11_90 = new l(var8_13.b.d(36), var10_71.i, var10_71.j - 24, 1, 36);
                            var8_13.q.addElement(var11_90);
                            var10_71.z = 1;
                        }
                        var9_36 = Math.abs(var6_6 /* !! */ .U.nextInt() % 10);
                        for (var10_72 = 0; var10_72 < var9_36; ++var10_72) {
                            var11_91 = Math.abs(var6_6 /* !! */ .U.nextInt() % 200);
                            var12_108 = Math.abs(var6_6 /* !! */ .U.nextInt() % 100);
                            var11_92 = new l(var8_13.b.d(36), var11_91 + 50, var12_108 + 50, 1, 36);
                            var8_13.q.addElement(var11_92);
                        }
                    }
                    if (var6_6 /* !! */ .F != 30) break;
                    var6_6 /* !! */ .z = 1;
                    var6_6 /* !! */ .F = 0;
                    var6_6 /* !! */ .J = 0;
                    break;
                }
                case 25: {
                    break;
                }
                case 26: {
                    if (var6_6 /* !! */ .z == 0) {
                        var9_37 = Math.abs(var8_13.e.nextInt() % 20);
                        if (var9_37 == 0) {
                            var6_6 /* !! */ .z = 2;
                        }
                        var6_6 /* !! */ .F = 0;
                        var6_6 /* !! */ .d = 0;
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .d = (byte)2;
                        if (var6_6 /* !! */ .F >= 30) {
                            var6_6 /* !! */ .z = 0;
                        }
                        if (var6_6 /* !! */ .F >= 20) {
                            var6_6 /* !! */ .d = 1;
                        }
                    }
                    if (var6_6 /* !! */ .z != 2) break;
                    ++var6_6 /* !! */ .F;
                    var6_6 /* !! */ .d = (byte)4;
                    if (var6_6 /* !! */ .F > 10) {
                        var6_6 /* !! */ .z = 1;
                        var6_6 /* !! */ .F = 0;
                    }
                    if ((var9_38 = Math.abs(var8_13.e.nextInt() % 10)) <= 3) {
                        var10_73 = Math.abs(var7_9.i - var6_6 /* !! */ .i);
                        if (var10_73 >= (var11_93 = Math.abs(var7_9.j - var6_6 /* !! */ .j))) {
                            if (var7_9.i > var6_6 /* !! */ .i) {
                                var6_6 /* !! */ .c = 1;
                            }
                            if (var7_9.i >= var6_6 /* !! */ .i) break;
                            var6_6 /* !! */ .c = (byte)3;
                            break;
                        }
                        if (var7_9.j > var6_6 /* !! */ .j) {
                            var6_6 /* !! */ .c = (byte)2;
                        }
                        if (var7_9.j >= var6_6 /* !! */ .j) break;
                        var6_6 /* !! */ .c = 0;
                        break;
                    }
                    if (var9_38 >= 6) break;
                    var9_38 = Math.abs(var8_13.e.nextInt() % 4);
                    var6_6 /* !! */ .c = (byte)var9_38;
                    break;
                }
                case 27: {
                    var6_6 /* !! */ .d = 1;
                    if (var6_6 /* !! */ .c == 0 && var6_6 /* !! */ .i > var7_9.i && var6_6 /* !! */ .i < var7_9.i + 32 && var6_6 /* !! */ .j - var7_9.j - 32 > -50) {
                        var6_6 /* !! */ .d = (byte)3;
                    }
                    if (var6_6 /* !! */ .c == 1 && var6_6 /* !! */ .j > var7_9.j - 32 && var6_6 /* !! */ .j < var7_9.j && var6_6 /* !! */ .i - var7_9.i < 50) {
                        var6_6 /* !! */ .d = (byte)3;
                    }
                    if (var6_6 /* !! */ .c == 2 && var6_6 /* !! */ .i > var7_9.i && var6_6 /* !! */ .i < var7_9.i + 32 && var6_6 /* !! */ .j - var7_9.j - 32 < 50) {
                        var6_6 /* !! */ .d = (byte)3;
                    }
                    if (var6_6 /* !! */ .c != 3 || var6_6 /* !! */ .j <= var7_9.j - 32 || var6_6 /* !! */ .j >= var7_9.j || var6_6 /* !! */ .i - var7_9.i <= -50) break;
                    var6_6 /* !! */ .d = (byte)3;
                    break;
                }
                case 28: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .d = (byte)10;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = false;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 2;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 3) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 4) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 24) {
                            var6_6 /* !! */ .z = 5;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 5) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 6;
                            var6_6 /* !! */ .F = 24;
                        }
                        var9_39 = var7_9.i - var6_6 /* !! */ .i;
                        var10_74 = var7_9.j - var6_6 /* !! */ .j;
                        if (Math.abs(var9_39) > Math.abs(var10_74)) {
                            if (var6_6 /* !! */ .j < var7_9.j - 32) {
                                if (var6_6 /* !! */ .i < var7_9.i) {
                                    var6_6 /* !! */ .c = 1;
                                }
                                if (var6_6 /* !! */ .i > var7_9.i) {
                                    var6_6 /* !! */ .c = (byte)3;
                                }
                            }
                        } else {
                            var6_6 /* !! */ .c = 0;
                        }
                        if (var6_6 /* !! */ .U.nextInt() % 10 == 1) {
                            var11_94 = false;
                            var12_109 = 0;
                            if (Math.abs(var9_39) > 30) {
                                var12_109 = var9_39 > 0 ? 1 : -1;
                            }
                            var11_95 = new p(var8_13.b.b(0), 0, var12_109, 1);
                            new p(var8_13.b.b(0), 0, var12_109, 1).i = var6_6 /* !! */ .i + var12_109 * 16 + 5 - var6_6 /* !! */ .V[0] + 5 - var6_6 /* !! */ .V[1] + 5 - var6_6 /* !! */ .V[2] + 5 - var6_6 /* !! */ .V[3] + 5 - var6_6 /* !! */ .V[4] + 5 - var6_6 /* !! */ .V[5];
                            var11_95.j = var6_6 /* !! */ .j + 16 - 60;
                            var11_95.p = true;
                            var8_13.n.addElement(var11_95);
                        }
                    }
                    if (var6_6 /* !! */ .z != 6) break;
                    var6_6 /* !! */ .d = 0;
                    var6_6 /* !! */ .g = false;
                    var6_6 /* !! */ .h = true;
                    var6_6 /* !! */ .n = true;
                    --var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F >= 0) break;
                    var6_6 /* !! */ .z = 0;
                    var6_6 /* !! */ .F = 0;
                    break;
                }
                case 29: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .z = 2;
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 2;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 4;
                            var6_6 /* !! */ .F = 5;
                        }
                    }
                    if (var6_6 /* !! */ .z == 4) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 24) {
                            var6_6 /* !! */ .z = 5;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 5) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 7;
                            var6_6 /* !! */ .F = 0;
                        }
                        var9_40 = var7_9.i - var6_6 /* !! */ .i;
                        var10_75 = var7_9.j - var6_6 /* !! */ .j;
                        if (Math.abs(var9_40) > Math.abs(var10_75)) {
                            if (var6_6 /* !! */ .j < var7_9.j - 32) {
                                if (var6_6 /* !! */ .i < var7_9.i) {
                                    var6_6 /* !! */ .c = 1;
                                }
                                if (var6_6 /* !! */ .i > var7_9.i) {
                                    var6_6 /* !! */ .c = (byte)3;
                                }
                            }
                        } else {
                            var6_6 /* !! */ .c = 0;
                        }
                        if (var6_6 /* !! */ .U.nextInt() % 15 == 1) {
                            var11_96 = false;
                            var12_110 = 0;
                            if (Math.abs(var9_40) > 30) {
                                var12_110 = var9_40 > 0 ? 1 : -1;
                            }
                            var11_97 = new p(var8_13.b.b(0), 0, var12_110, 1);
                            new p(var8_13.b.b(0), 0, var12_110, 1).i = var6_6 /* !! */ .i + var12_110 * 16 + 5 - var6_6 /* !! */ .V[0] + 5 - var6_6 /* !! */ .V[1] + 5 - var6_6 /* !! */ .V[2] + 5 - var6_6 /* !! */ .V[3] + 5 - var6_6 /* !! */ .V[4] + 5 - var6_6 /* !! */ .V[5];
                            var11_97.j = var6_6 /* !! */ .j + 16 - 60;
                            var11_97.p = true;
                            var11_97.o = 3;
                            var8_13.n.addElement(var11_97);
                        }
                    }
                    if (var6_6 /* !! */ .z == 6) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        --var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F < 0) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 7) {
                        var6_6 /* !! */ .d = 0;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 6;
                            var6_6 /* !! */ .F = 24;
                        }
                    }
                    if (var6_6 /* !! */ .z == 8) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 160) {
                            var6_6 /* !! */ .z = 6;
                            var6_6 /* !! */ .F = 24;
                            if (var7_9.ag) {
                                var7_9.H = 0;
                            }
                        }
                    }
                    if (var6_6 /* !! */ .z == 9) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 32) {
                            var6_6 /* !! */ .z = 10;
                            var6_6 /* !! */ .F = 0;
                            var7_9.i = 110;
                            var7_9.j = 130;
                            var7_9.L = 10;
                            var7_9.ag = false;
                        }
                    }
                    if (var6_6 /* !! */ .z != 10) break;
                    var6_6 /* !! */ .d = 0;
                    var6_6 /* !! */ .g = true;
                    var6_6 /* !! */ .h = true;
                    var6_6 /* !! */ .n = true;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 100) break;
                    var6_6 /* !! */ .z = 6;
                    var6_6 /* !! */ .F = 24;
                    var6_6 /* !! */ .d = 0;
                    var6_6 /* !! */ .g = true;
                    var6_6 /* !! */ .h = true;
                    var6_6 /* !! */ .n = false;
                    break;
                }
                case 30: 
                case 31: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .n = true;
                        var6_6 /* !! */ .d = (byte)4;
                        var6_6 /* !! */ .c = var7_9.i < var6_6 /* !! */ .i ? (byte)3 : 1;
                        ++var6_6 /* !! */ .F;
                        var9_41 = var7_9.i - var6_6 /* !! */ .i;
                        var10_76 = var7_9.j - var6_6 /* !! */ .j;
                        if (var6_6 /* !! */ .F == 25) {
                            if (var6_6 /* !! */ .b == 30) {
                                var11_98 = new p(var8_13.b.b(9), 1, -1, 0);
                                new p(var8_13.b.b(9), 1, -1, 0).i = var6_6 /* !! */ .i + 4;
                                var11_98.j = var6_6 /* !! */ .j + 4;
                                var11_98.h = 9;
                                var11_98.c = 0;
                                var11_98.b = 0;
                                if (Math.abs(var9_41) < Math.abs(var10_76)) {
                                    var11_98.c = var10_76 > 0 ? 1 : -1;
                                    if (Math.abs(var9_41) > 50) {
                                        var11_98.b = var9_41 > 0 ? 1 : -1;
                                    }
                                } else {
                                    var11_98.b = var9_41 > 0 ? 1 : -1;
                                    if (Math.abs(var10_76) > 50) {
                                        var11_98.c = var10_76 > 0 ? 1 : -1;
                                    }
                                }
                                var11_98.p = true;
                                var8_13.n.addElement(var11_98);
                            }
                            if (var6_6 /* !! */ .b == 31) {
                                var11_99 = new p(var8_13.b.b(10), 1, -1, 0);
                                new p(var8_13.b.b(10), 1, -1, 0).i = var6_6 /* !! */ .i + 4;
                                var11_99.j = var6_6 /* !! */ .j + 4;
                                var11_99.h = 10;
                                var11_99.c = 0;
                                var11_99.b = 0;
                                if (Math.abs(var9_41) < Math.abs(var10_76)) {
                                    var11_99.c = var10_76 > 0 ? 1 : -1;
                                    if (Math.abs(var9_41) > 50) {
                                        var11_99.b = var9_41 > 0 ? 1 : -1;
                                    }
                                } else {
                                    var11_99.b = var9_41 > 0 ? 1 : -1;
                                    if (Math.abs(var10_76) > 50) {
                                        var11_99.c = var10_76 > 0 ? 1 : -1;
                                    }
                                }
                                var11_99.p = true;
                                var8_13.n.addElement(var11_99);
                            }
                            var6_6 /* !! */ .F = 0;
                        }
                        if (Math.abs(var9_41) + Math.abs(var10_76) < 50) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .F += Math.abs(var6_6 /* !! */ .U.nextInt() % 5);
                        var6_6 /* !! */ .h = false;
                        if (var6_6 /* !! */ .F > 200) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                        if ((var9_42 = var7_9.i - var6_6 /* !! */ .i) * var9_42 + (var10_77 = var7_9.j - var6_6 /* !! */ .j) * var10_77 < 1024) {
                            var11_100 = 0;
                            if (var6_6 /* !! */ .c == 0) {
                                var6_6 /* !! */ .c = (byte)2;
                                ++var11_100;
                            }
                            if (var6_6 /* !! */ .c == 1) {
                                var6_6 /* !! */ .c = (byte)3;
                                ++var11_100;
                            }
                            if (var6_6 /* !! */ .c == 2) {
                                var6_6 /* !! */ .c = 0;
                                ++var11_100;
                            }
                            if (var6_6 /* !! */ .c == 3) {
                                var6_6 /* !! */ .c = 1;
                                ++var11_100;
                            }
                            if (var11_100 > 1) {
                                var6_6 /* !! */ .z = 2;
                                var6_6 /* !! */ .F = 0;
                            }
                            var6_6 /* !! */ .M = -var6_6 /* !! */ .M;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 3;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 3) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                        var6_6 /* !! */ .d = (byte)10;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = false;
                        var6_6 /* !! */ .n = false;
                    }
                    if (var6_6 /* !! */ .z != 4) break;
                    var6_6 /* !! */ .h = true;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 50) break;
                    var6_6 /* !! */ .z = 2;
                    var6_6 /* !! */ .F = 0;
                    break;
                }
                case 32: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .d = (byte)10;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = false;
                        var6_6 /* !! */ .n = false;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .d = 1;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 2;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .a = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        var6_6 /* !! */ .d = 1;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 12) {
                            var6_6 /* !! */ .z = 3;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 3) {
                        var6_6 /* !! */ .d = 1;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = false;
                        var6_6 /* !! */ .n = true;
                        var6_6 /* !! */ .F += Math.abs(var6_6 /* !! */ .U.nextInt() % 3) + 1;
                        if (var6_6 /* !! */ .F > 200) {
                            var6_6 /* !! */ .z = 4;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .a = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z != 4) break;
                    var6_6 /* !! */ .d = 1;
                    var6_6 /* !! */ .g = false;
                    var6_6 /* !! */ .h = true;
                    var6_6 /* !! */ .n = true;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 12) break;
                    var6_6 /* !! */ .z = 0;
                    var6_6 /* !! */ .F = 0;
                    break;
                }
                case 33: {
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .d = (byte)10;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .h = false;
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .F += Math.abs(var6_6 /* !! */ .U.nextInt() % 3) + 1;
                        if (var6_6 /* !! */ .F > 150) {
                            var6_6 /* !! */ .z = 1;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .d = 1;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .z = 2;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .a = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        var6_6 /* !! */ .d = 1;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 3;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .a = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 3) {
                        var6_6 /* !! */ .d = 1;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = true;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F == 5) {
                            var9_43 = new p(var8_13.b.b(1), 1, -1, 0);
                            new p(var8_13.b.b(1), 1, -1, 0).i = var6_6 /* !! */ .i + 4;
                            var9_43.j = var6_6 /* !! */ .j + 4;
                            var9_43.c = var7_9.j < var6_6 /* !! */ .j ? -1 : 1;
                            var9_43.b = var7_9.i < var6_6 /* !! */ .i ? -1 : 1;
                            var9_43.p = true;
                            var8_13.n.addElement(var9_43);
                        }
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .z = 4;
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .a = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z != 4) break;
                    var6_6 /* !! */ .d = 1;
                    var6_6 /* !! */ .g = false;
                    var6_6 /* !! */ .h = true;
                    var6_6 /* !! */ .n = true;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 10) break;
                    var6_6 /* !! */ .z = 0;
                    var6_6 /* !! */ .F = 0;
                    var6_6 /* !! */ .a = 0;
                    break;
                }
                case 35: {
                    break;
                }
                case 36: {
                    if (var6_6 /* !! */ .z != 0) break;
                    var6_6 /* !! */ .j += 4;
                    var6_6 /* !! */ .g = true;
                    if (var6_6 /* !! */ .j < var6_6 /* !! */ .s) break;
                    var6_6 /* !! */ .j = var6_6 /* !! */ .s;
                    var6_6 /* !! */ .z = 1;
                    var6_6 /* !! */ .F = 0;
                    var6_6 /* !! */ .g = false;
                    break;
                }
                case 37: {
                    var9_44 = var7_9.i + 8 - (var6_6 /* !! */ .i + 16);
                    var10_78 = var7_9.j + 12 - (var6_6 /* !! */ .j + 16);
                    if (var6_6 /* !! */ .z == 0) {
                        var6_6 /* !! */ .h = true;
                        if (var6_6 /* !! */ .c == 2 || var6_6 /* !! */ .c == 3) {
                            var6_6 /* !! */ .c = 0;
                        }
                        var6_6 /* !! */ .i = var6_6 /* !! */ .c == 0 ? ++var6_6 /* !! */ .i : --var6_6 /* !! */ .i;
                        if (var6_6 /* !! */ .i < 50) {
                            var6_6 /* !! */ .c = 0;
                        }
                        if (var6_6 /* !! */ .i > 200) {
                            var6_6 /* !! */ .c = 1;
                        }
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = var6_6 /* !! */ .U.nextInt() % 2 == 0 ? 1 : 2;
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .d = 1;
                        if (var6_6 /* !! */ .F % 9 == 1) {
                            for (var11_101 = 0; var11_101 < 3; ++var11_101) {
                                var12_111 = new p(var8_13.b.b(1), 1, -1, 0);
                                new p(var8_13.b.b(1), 1, -1, 0).i = var6_6 /* !! */ .i + 14;
                                var12_111.j = var6_6 /* !! */ .j + 16;
                                var12_111.c = 1;
                                var12_111.b = 1 - var11_101;
                                var12_111.p = true;
                                var8_13.n.addElement(var12_111);
                            }
                        }
                        if (var6_6 /* !! */ .F > 20) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = var6_6 /* !! */ .U.nextInt() % 2 == 0 ? 0 : 2;
                        }
                    }
                    if (var6_6 /* !! */ .z == 2) {
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .d = 1;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .z = 4;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 4) {
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .d = 1;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .z = 5;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 5) {
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .g = true;
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .d = (byte)2;
                        if (var6_6 /* !! */ .F < 80) {
                            if (var7_9.i > var6_6 /* !! */ .i - 16) {
                                ++var6_6 /* !! */ .i;
                            }
                            if (var7_9.i < var6_6 /* !! */ .i + 16) {
                                --var6_6 /* !! */ .i;
                            }
                            if (var7_9.j > var6_6 /* !! */ .j - 16) {
                                ++var6_6 /* !! */ .j;
                            }
                            if (var7_9.j < var6_6 /* !! */ .j + 16) {
                                --var6_6 /* !! */ .j;
                            }
                        }
                        if (var6_6 /* !! */ .F > 100) {
                            var6_6 /* !! */ .z = 6;
                            var6_6 /* !! */ .F = 0;
                            var8_13.a.F = 10;
                            var6_6 /* !! */ .h = true;
                            var6_6 /* !! */ .g = false;
                            var6_6 /* !! */ .n = true;
                            if (Math.abs(var9_44) + Math.abs(var10_78) < 80) {
                                var7_9.H += 4;
                                var7_9.L = 10;
                            }
                        }
                    }
                    if (var6_6 /* !! */ .z == 6) {
                        var6_6 /* !! */ .d = 1;
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .z = 8;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z == 7) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 10) {
                            var6_6 /* !! */ .z = 0;
                            var6_6 /* !! */ .F = 0;
                        }
                    }
                    if (var6_6 /* !! */ .z != 8) break;
                    if (128 > var6_6 /* !! */ .i) {
                        var6_6 /* !! */ .c = 1;
                    }
                    if (128 < var6_6 /* !! */ .i) {
                        var6_6 /* !! */ .c = (byte)3;
                    }
                    if (50 > var6_6 /* !! */ .j) {
                        var6_6 /* !! */ .c = (byte)2;
                    }
                    if (50 < var6_6 /* !! */ .j) {
                        var6_6 /* !! */ .c = 0;
                    }
                    var6_6 /* !! */ .d = (byte)5;
                    var6_6 /* !! */ .h = false;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 20) break;
                    var6_6 /* !! */ .d = 1;
                    var6_6 /* !! */ .z = 0;
                    break;
                }
                case 38: {
                    if (var6_6 /* !! */ .z == 0 || var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .h = true;
                        var6_6 /* !! */ .n = false;
                        var6_6 /* !! */ .g = false;
                        if (var6_6 /* !! */ .c == 2 || var6_6 /* !! */ .c == 3) {
                            var6_6 /* !! */ .c = 0;
                        }
                        if (var6_6 /* !! */ .c == 0) {
                            if (var6_6 /* !! */ .a % 2 == 1) {
                                var6_6 /* !! */ .i += 2;
                            }
                        } else if (var6_6 /* !! */ .a % 2 == 1) {
                            var6_6 /* !! */ .i -= 2;
                        }
                        if (var6_6 /* !! */ .i < 80) {
                            var6_6 /* !! */ .c = 0;
                        }
                        if (var6_6 /* !! */ .i > 130) {
                            var6_6 /* !! */ .c = 1;
                        }
                        ++var6_6 /* !! */ .F;
                        var6_6 /* !! */ .g = false;
                        var6_6 /* !! */ .h = true;
                        if (var6_6 /* !! */ .z == 0) {
                            if (var6_6 /* !! */ .F > 50) {
                                var6_6 /* !! */ .z = 1;
                                var6_6 /* !! */ .F = 0;
                            }
                        } else if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .F = 0;
                            var6_6 /* !! */ .z = 2;
                        }
                    }
                    if (var6_6 /* !! */ .z != 2) break;
                    if (var6_6 /* !! */ .F % 9 == 1) {
                        for (var9_45 = 0; var9_45 < 3; ++var9_45) {
                            var10_79 = new p(var8_13.b.b(1), 1, -1, 0);
                            new p(var8_13.b.b(1), 1, -1, 0).i = var6_6 /* !! */ .i + 24;
                            var10_79.j = var6_6 /* !! */ .j + 12;
                            var10_79.c = 1;
                            var10_79.b = 1 - var9_45;
                            var10_79.p = true;
                            var8_13.n.addElement(var10_79);
                        }
                    }
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F <= 50) break;
                    var6_6 /* !! */ .z = 1;
                    var6_6 /* !! */ .F = 0;
                    break;
                }
                case 135: {
                    if (var6_6 /* !! */ .z > 0) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F == 20) {
                            var6_6 /* !! */ .z = 2;
                        }
                        if (var6_6 /* !! */ .F == 50) {
                            var6_6 /* !! */ .z = 3;
                        }
                        if (var6_6 /* !! */ .F != 70) break;
                        var6_6 /* !! */ .z = 0;
                        break;
                    }
                    if (var6_6 /* !! */ .U.nextInt() % 100 != 67) break;
                    var6_6 /* !! */ .z = 1;
                    var6_6 /* !! */ .F = 0;
                    break;
                }
                case 133: {
                    var9_46 = new g();
                    new g().a = var7_9.i;
                    var9_46.b = var7_9.j;
                    var9_46.c = var7_9.e;
                    var6_6 /* !! */ .R.addElement(var9_46);
                    if (var6_6 /* !! */ .R.size() <= 30) break;
                    var10_80 = (g)var6_6 /* !! */ .R.firstElement();
                    if (var6_6 /* !! */ .i == var10_80.a && var6_6 /* !! */ .j == var9_46.b) {
                        var6_6 /* !! */ .a = 0;
                    }
                    if (var10_80.a == var7_9.i && var10_80.b == var7_9.j) {
                        if (var6_6 /* !! */ .c == 0) {
                            var6_6 /* !! */ .i = var10_80.a;
                            var6_6 /* !! */ .j = var10_80.b + 8;
                            var6_6 /* !! */ .a = 0;
                        }
                        if (var6_6 /* !! */ .c == 1) {
                            var6_6 /* !! */ .i = var10_80.a - 8;
                            var6_6 /* !! */ .j = var10_80.b;
                            var6_6 /* !! */ .a = 0;
                        }
                        if (var6_6 /* !! */ .c == 2) {
                            var6_6 /* !! */ .i = var10_80.a;
                            var6_6 /* !! */ .j = var10_80.b - 8;
                            var6_6 /* !! */ .a = 0;
                        }
                        if (var6_6 /* !! */ .c == 3) {
                            var6_6 /* !! */ .i = var10_80.a + 8;
                            var6_6 /* !! */ .j = var10_80.b;
                            var6_6 /* !! */ .a = 0;
                        }
                    } else {
                        var6_6 /* !! */ .i = var10_80.a;
                        var6_6 /* !! */ .j = var10_80.b;
                        var6_6 /* !! */ .c = (byte)var10_80.c;
                    }
                    var6_6 /* !! */ .R.removeElementAt(0);
                    break;
                }
                case 134: {
                    if (var6_6 /* !! */ .z == 0) {
                        ++var6_6 /* !! */ .F;
                        if (var6_6 /* !! */ .F > 50) {
                            var6_6 /* !! */ .F = 0;
                            var9_47 = new p(var8_13.b.b(11), 11, -1, 0);
                            new p(var8_13.b.b(11), 11, -1, 0).i = var6_6 /* !! */ .i + 4;
                            var9_47.j = var6_6 /* !! */ .j + 4;
                            if (var6_6 /* !! */ .c == 1) {
                                var9_47.c = -1;
                                var9_47.b = -1;
                            } else {
                                var9_47.c = -1;
                                var9_47.b = 1;
                            }
                            var9_47.e = 0;
                            var9_47.p = false;
                            var8_13.n.addElement(var9_47);
                        }
                    }
                    if (var6_6 /* !! */ .z == 1) {
                        var6_6 /* !! */ .F = 0;
                        var6_6 /* !! */ .z = 2;
                    }
                    if (var6_6 /* !! */ .z != 2) break;
                    ++var6_6 /* !! */ .F;
                    if (var6_6 /* !! */ .F < 75) {
                        if (var6_6 /* !! */ .c == 1) {
                            var6_6 /* !! */ .i -= 2;
                            break;
                        }
                        var6_6 /* !! */ .i += 2;
                        break;
                    }
                    var6_6 /* !! */ .z = 0;
                    var6_6 /* !! */ .F = 0;
                    if (var6_6 /* !! */ .c == 1) {
                        var6_6 /* !! */ .c = (byte)3;
                        break;
                    }
lbl1525:
                    // 6 sources

                    var6_6 /* !! */ .c = 1;
                }
            }
            if (var3_3.f) {
                var3_3.e = (byte)(var3_3.e + 1);
                if (var3_3.e == 5) {
                    if (var3_3.b == 16 || var3_3.b == 25) {
                        if (var3_3.b == 25) {
                            if ((var1_1.aj & 0x200000) <= 0) {
                                var6_6 /* !! */  = new c(this.b.a(154), 154, var3_3.i + (var3_3.k >> 1), var3_3.j + 16 - 32, 0, var1_1);
                                new c(this.b.a(154), 154, var3_3.i + (var3_3.k >> 1), var3_3.j + 16 - 32, 0, var1_1).c = 21;
                                this.p.addElement(var6_6 /* !! */ );
                            } else {
                                this.a(var3_3.i + (var3_3.k >> 1), var3_3.j + 16, var1_1, var3_3.L);
                            }
                        }
                        if (var3_3.b == 16) {
                            this.a(var3_3.i + (var3_3.k >> 1), var3_3.j + 16, var1_1, var3_3.L);
                        }
                    } else {
                        this.a(var3_3.i + (var3_3.k >> 1), var3_3.j, var1_1, var3_3.L);
                    }
                }
                if (var3_3.e == 7) {
                    this.q.removeElementAt(var2_2);
                    return;
                }
                this.q.setElementAt(var3_3, var2_2);
                continue;
            }
            if (var3_3.h) continue;
            if (var3_3.c == 0) {
                var4_4 = 0;
                var5_5 = -var3_3.d;
            }
            if (var3_3.c == 1) {
                var4_4 = var3_3.d;
                var5_5 = 0;
            }
            if (var3_3.c == 2) {
                var4_4 = 0;
                var5_5 = var3_3.d;
            }
            if (var3_3.c == 3) {
                var4_4 = -var3_3.d;
                var5_5 = 0;
            }
            if (this.a(var3_3.i + var4_4, var3_3.j + var5_5, (int)var3_3.c, var3_3.b, 0)) {
                var3_3.i += var4_4;
                var3_3.j += var5_5;
                if (var3_3.i > 240) {
                    var3_3.c = (byte)3;
                }
                if (var3_3.i < 16) {
                    var3_3.c = 1;
                }
                if (var3_3.j > 176) {
                    var3_3.c = 0;
                }
                if (var3_3.j < 32) {
                    var3_3.c = (byte)2;
                }
                if (Math.abs(this.e.nextInt() % 30) == 0) {
                    var6_7 = (byte)Math.abs(this.e.nextInt() % 4);
                    var7_11 = false;
                    if (var3_3.b == 8) {
                        var7_11 = true;
                        if (var3_3.c % 2 == var6_7 % 2) {
                            var3_3.c = (byte)(var3_3.c + 1);
                            if (var3_3.c > 3) {
                                var3_3.c = 0;
                            }
                        } else {
                            var3_3.c = var6_7;
                        }
                    }
                    if (var3_3.b == 10) {
                        var7_11 = true;
                        if (var3_3.c % 2 == var6_7 % 2) {
                            var3_3.c = (byte)(var3_3.c + 1);
                            if (var3_3.c > 3) {
                                var3_3.c = 0;
                            }
                        } else {
                            var3_3.c = var6_7;
                        }
                    }
                    if (var3_3.b == 15) {
                        var7_11 = true;
                        if (var3_3.c % 2 == var6_7 % 2) {
                            var3_3.c = (byte)(var3_3.c + 1);
                            if (var3_3.c > 3) {
                                var3_3.c = 0;
                            }
                        } else {
                            var3_3.c = var6_7;
                        }
                    }
                    if (var3_3.b == 17) {
                        var7_11 = true;
                    }
                    if (var3_3.b == 12) {
                        var7_11 = true;
                        if (var3_3.c % 2 == var6_7 % 2) {
                            var3_3.c = (byte)(var3_3.c + 1);
                            if (var3_3.c > 3) {
                                var3_3.c = 0;
                            }
                        } else {
                            var3_3.c = var6_7;
                        }
                    }
                    if (var3_3.b == 19) {
                        var7_11 = true;
                        if (var3_3.c % 2 == var6_7 % 2) {
                            var3_3.c = (byte)(var3_3.c + 1);
                            if (var3_3.c > 3) {
                                var3_3.c = 0;
                            }
                        } else {
                            var3_3.c = var6_7;
                        }
                    }
                    if (var3_3.b == 22) {
                        var7_11 = true;
                        if (var3_3.c % 2 == var6_7 % 2) {
                            var3_3.c = (byte)(var3_3.c + 1);
                            if (var3_3.c > 3) {
                                var3_3.c = 0;
                            }
                        } else {
                            var3_3.c = var6_7;
                        }
                    }
                    if (var3_3.b == 30) {
                        var7_11 = true;
                    }
                    if (var3_3.b == 31) {
                        var7_11 = true;
                    }
                    if (!var7_11) {
                        var3_3.c = var6_7;
                    }
                }
                if (var3_3.b == 16) {
                    if (var3_3.c == 1) {
                        var3_3.c = 0;
                    }
                    if (var3_3.c == 3) {
                        var3_3.c = (byte)2;
                    }
                }
                if (var3_3.b == 0 && Math.abs(this.e.nextInt() % 100) == 5 && var3_3.u <= 0) {
                    var6_6 /* !! */  = new p(this.b.b(0), 0, var4_4, var5_5);
                    new p(this.b.b(0), 0, var4_4, var5_5).i = var3_3.i + 4;
                    var6_6 /* !! */ .j = var3_3.j + 4;
                    var6_6 /* !! */ .p = true;
                    this.n.addElement(var6_6 /* !! */ );
                    var3_3.u = 100;
                }
                if (var3_3.b == 6 && Math.abs(this.e.nextInt() % 50) == 5 && !var3_3.p) {
                    var6_6 /* !! */  = new p(this.b.b(2), 2, var4_4, var5_5);
                    new p(this.b.b(2), 2, var4_4, var5_5).i = var3_3.i + var4_4 * 8;
                    var6_6 /* !! */ .j = var3_3.j + var5_5 * 8;
                    var6_6 /* !! */ .q = var2_2;
                    var6_6 /* !! */ .p = true;
                    this.n.addElement(var6_6 /* !! */ );
                    var3_3.p = true;
                }
                if (var3_3.b == 12 && Math.abs(this.e.nextInt() % 100) == 5 && var3_3.u <= 0) {
                    var6_6 /* !! */  = new p(this.b.b(3), 3, var4_4, var5_5);
                    new p(this.b.b(3), 3, var4_4, var5_5).i = var3_3.i + 4;
                    var6_6 /* !! */ .j = var3_3.j + 4;
                    var6_6 /* !! */ .p = true;
                    this.n.addElement(var6_6 /* !! */ );
                    var3_3.u = 100;
                }
                if (var3_3.b == 16 && Math.abs(this.e.nextInt() % 20) <= 3 && var3_3.u <= 0) {
                    var6_6 /* !! */  = new p(this.b.b(1), 1, -1, 0);
                    new p(this.b.b(1), 1, -1, 0).i = var3_3.i + 4;
                    var6_6 /* !! */ .j = var3_3.j + 4;
                    if (Math.abs(this.e.nextInt() % 10) <= 4) {
                        var6_6 /* !! */ .c = var1_1.j < var3_3.j ? -1 : 1;
                    }
                    var6_6 /* !! */ .p = true;
                    this.n.addElement(var6_6 /* !! */ );
                    var3_3.u = 10;
                }
            } else {
                var6_8 = (byte)Math.abs(this.e.nextInt() % 4);
                var7_12 = false;
                if (var3_3.b == 8) {
                    var7_12 = true;
                    if (var3_3.c % 2 == var6_8 % 2) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = (byte)2;
                        }
                    } else {
                        var3_3.c = var6_8;
                    }
                }
                if (var3_3.b == 10) {
                    var7_12 = true;
                    if (var3_3.c % 2 == var6_8 % 2) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = (byte)2;
                        }
                    } else {
                        var3_3.c = var6_8;
                    }
                }
                if (var3_3.b == 15) {
                    var7_12 = true;
                    if (var3_3.c % 2 == var6_8 % 2) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = (byte)2;
                        }
                    } else {
                        var3_3.c = var6_8;
                    }
                }
                if (var3_3.b == 12) {
                    var7_12 = true;
                    if (var3_3.c % 2 == var6_8 % 2) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = (byte)2;
                        }
                    } else {
                        var3_3.c = var6_8;
                    }
                }
                if (var3_3.b == 13) {
                    var3_3.i += var4_4;
                    var3_3.j += var5_5;
                    if (var3_3.i > 240) {
                        var3_3.c = (byte)3;
                    }
                    if (var3_3.i < 16) {
                        var3_3.c = 1;
                    }
                    if (var3_3.j > 176) {
                        var3_3.c = 0;
                    }
                    if (var3_3.j < 32) {
                        var3_3.c = (byte)2;
                    }
                    var7_12 = true;
                }
                if (var3_3.b == 17) {
                    var7_12 = true;
                    var3_3.c = (byte)(var3_3.c + 1);
                    if (var3_3.c > 3) {
                        var3_3.c = 0;
                    }
                }
                if (var3_3.b == 19) {
                    var7_12 = true;
                    if (var3_3.c % 2 == var6_8 % 2) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = (byte)2;
                        }
                    } else {
                        var3_3.c = var6_8;
                    }
                }
                if (var3_3.b == 22) {
                    var7_12 = true;
                    if (var3_3.c % 2 == var6_8 % 2) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = (byte)2;
                        }
                    } else {
                        var3_3.c = var6_8;
                    }
                }
                if (var3_3.b == 30 || var3_3.b == 31) {
                    var7_12 = true;
                    if (var3_3.M > 0) {
                        var3_3.c = (byte)(var3_3.c + 1);
                        if (var3_3.c > 3) {
                            var3_3.c = 0;
                        }
                    } else {
                        var7_12 = true;
                        var3_3.c = (byte)(var3_3.c - 1);
                        if (var3_3.c < 0) {
                            var3_3.c = (byte)3;
                        }
                    }
                }
                if (!var7_12) {
                    var3_3.c = var6_8;
                }
            }
            this.q.setElementAt(var3_3, var2_2);
        }
    }

    final void c(h h2) {
        block23: {
            block26: {
                block24: {
                    int n2;
                    block25: {
                        g g2;
                        boolean bl = false;
                        if (h2.X != 0) {
                            return;
                        }
                        if (!h2.n) break block23;
                        if (h2.o) break block24;
                        g g3 = new g();
                        if (h2.ai.size() == 0) {
                            g2 = new g();
                            if (h2.e == 0) {
                                g2.a = h2.i + 8;
                                g2.b = h2.j + 8;
                            }
                            if (h2.e == 1) {
                                g2.a = h2.i + 12;
                                g2.b = h2.j + 12;
                            }
                            if (h2.e == 2) {
                                g2.a = h2.i + 8;
                                g2.b = h2.j + 12;
                            }
                            if (h2.e == 3) {
                                g2.a = h2.i + 12;
                                g2.b = h2.j + 8;
                            }
                        } else {
                            g2 = (g)h2.ai.elementAt(h2.ai.size() - 1);
                        }
                        if (h2.e == 0) {
                            g3.b = g2.b - 8;
                            g3.a = g2.a;
                        }
                        if (h2.e == 1) {
                            g3.b = g2.b;
                            g3.a = g2.a + 8;
                        }
                        if (h2.e == 2) {
                            g3.b = g2.b + 8;
                            g3.a = g2.a;
                        }
                        if (h2.e == 3) {
                            g3.b = g2.b;
                            g3.a = g2.a - 8;
                        }
                        if ((n2 = this.a(g3.a, g3.b, h2)) != 0) break block25;
                        h2.ai.addElement(g3);
                        if (h2.ai.size() > 12) {
                            h2.o = true;
                            return;
                        }
                        break block23;
                    }
                    if (n2 == 2) {
                        h2.ah = true;
                    }
                    h2.o = true;
                    return;
                }
                if (h2.ai.size() < 1) break block26;
                g g4 = (g)h2.ai.elementAt(h2.ai.size() - 1);
                this.a(g4.a, g4.b, h2);
                if (!h2.ah) {
                    h2.ai.removeElementAt(h2.ai.size() - 1);
                } else {
                    g g5 = (g)h2.ai.elementAt(0);
                    h2.i = g5.a;
                    h2.j = g5.b;
                    if (h2.e == 0) {
                        h2.i -= 8;
                        h2.j -= 8;
                    }
                    if (h2.e == 1) {
                        h2.i -= 4;
                        h2.j -= 12;
                    }
                    if (h2.e == 2) {
                        h2.i -= 8;
                        h2.j -= 12;
                    }
                    if (h2.e == 3) {
                        h2.i -= 12;
                        h2.j -= 8;
                    }
                    h2.ai.removeElementAt(0);
                }
                if (h2.ai.size() != 0) break block23;
            }
            h2.n = false;
            h2.ai.removeAllElements();
            h2.o = false;
            h2.X = 0;
            h2.ah = false;
        }
    }

    private int a(int n2, int n3, h h2) {
        f f2;
        int n4 = 0;
        int n5 = 0;
        int n6 = 0;
        int n7 = 0;
        int n8 = 0;
        int n9 = 0;
        if (h2.e == 0) {
            n3 -= 4;
            n6 = 12;
            n7 = 8;
            n9 = 8;
        }
        if (h2.e == 1) {
            n2 += 4;
            n6 = 8;
            n7 = 12;
            n8 = -8;
        }
        if (h2.e == 2) {
            n3 += 4;
            n6 = 12;
            n7 = 8;
            n9 = -8;
        }
        if (h2.e == 3) {
            n2 -= 4;
            n6 = 8;
            n7 = 12;
            n8 = 8;
        }
        if ((n5 = (n3 - 32 >> 4) * this.B + (n2 >> 4)) <= this.B * this.C && n5 > 0 && ((n5 = this.A[n5]) & 0x40) == 64) {
            return 1;
        }
        for (n5 = 0; n5 < this.p.size(); ++n5) {
            f2 = (c)this.p.elementAt(n5);
            if (f2.i > n2 + n6 || f2.i + (f2.k << 1) < n2 || f2.j > n3 + n7 || f2.j + (f2.l << 1) < n3 || f2.a >= 128) continue;
            f2.i += n8;
            f2.j += n9;
            n4 = 1;
        }
        for (n5 = 0; n5 < this.q.size(); ++n5) {
            f2 = (l)this.q.elementAt(n5);
            if (!((l)f2).n || ((l)f2).f || ((l)f2).i > n2 + n6 || ((l)f2).i + ((l)f2).B < n2 || ((l)f2).j > n3 + n7 || ((l)f2).j + ((l)f2).C < n3) continue;
            if (((l)f2).b != 16 && ((l)f2).b != 17 && ((l)f2).b != 18 && ((l)f2).b != 19 && ((l)f2).b != 20 && ((l)f2).b != 21) {
                ((l)f2).i += n8;
                ((l)f2).j += n9;
                n4 = 1;
                continue;
            }
            n4 = 2;
        }
        for (n5 = 0; n5 < this.r.size(); ++n5) {
            f2 = (n)this.r.elementAt(n5);
            if (((n)f2).i > n2 + n6 || ((n)f2).i + 16 < n2 || ((n)f2).j > n3 + n7 || ((n)f2).j + 16 < n3) continue;
            if (((n)f2).a == 20) {
                n4 = 2;
                if (h2.ai.size() < 1) {
                    return 2;
                }
                g g2 = (g)h2.ai.elementAt(h2.ai.size() - 1);
                if (h2.e == 0) {
                    g2.b = ((n)f2).j + 16;
                }
                if (h2.e == 1) {
                    g2.a = ((n)f2).i - 16;
                }
                if (h2.e == 2) {
                    g2.b = ((n)f2).j - 16;
                }
                if (h2.e != 3) continue;
                g2.a = ((n)f2).i + 20;
                continue;
            }
            if (!((n)f2).e || !((n)f2).w) continue;
            return 1;
        }
        return n4;
    }

    final boolean a(Graphics graphics, int n2, int n3, int n4, int n5, h h2, b b2) {
        boolean bl = false;
        if (h2.ao > 0) {
            --h2.ao;
            if (h2.ao < 0) {
                h2.ao = 0;
            }
        }
        for (int i2 = 0; i2 < this.n.size(); ++i2) {
            f f2;
            p p2 = (p)this.n.elementAt(i2);
            if (p2.i > 256) {
                if (p2.h == 2 || p2.h == 8) {
                    if (p2.p) {
                        ((l)this.q.elementAt((int)p2.q)).p = false;
                        this.n.removeElementAt(i2);
                        continue;
                    }
                    p2.r = true;
                } else {
                    this.n.removeElementAt(i2);
                    continue;
                }
            }
            if (p2.i < 0) {
                if (p2.h == 2 || p2.h == 8 || p2.h == 0) {
                    if (p2.p) {
                        ((l)this.q.elementAt((int)p2.q)).p = false;
                        this.n.removeElementAt(i2);
                        continue;
                    }
                    p2.r = true;
                } else {
                    this.n.removeElementAt(i2);
                    continue;
                }
            }
            if (p2.j > 240) {
                if (p2.h == 2 || p2.h == 8) {
                    if (p2.p) {
                        ((l)this.q.elementAt((int)p2.q)).p = false;
                        this.n.removeElementAt(i2);
                        continue;
                    }
                    p2.r = true;
                } else {
                    this.n.removeElementAt(i2);
                    continue;
                }
            }
            if (p2.j < 32) {
                if (p2.h == 2 || p2.h == 8) {
                    if (p2.p) {
                        ((l)this.q.elementAt((int)p2.q)).p = false;
                        this.n.removeElementAt(i2);
                        continue;
                    }
                    p2.r = true;
                } else {
                    this.n.removeElementAt(i2);
                    continue;
                }
            }
            int n6 = 0;
            n6 = (p2.j - 32 >> 4) * this.B + (p2.i >> 4);
            if (n6 <= this.B * this.C && n6 > 0 && ((n6 = this.A[n6]) & 0x40) == 64) {
                if (p2.h != 2 && p2.h != 8) {
                    if (p2.h != 3 && p2.h != 5 && p2.h != 0 && p2.h != 11) {
                        this.n.removeElementAt(i2);
                    }
                } else if (!p2.r) {
                    p2.b = -p2.b;
                    p2.c = -p2.c;
                    p2.r = true;
                }
            }
            if (p2.r) {
                if (p2.p) {
                    try {
                        if (((l)this.q.elementAt((int)p2.q)).i <= p2.i + 5 && ((l)this.q.elementAt((int)p2.q)).i + 16 >= p2.i + 5 && ((l)this.q.elementAt((int)p2.q)).j <= p2.j + 5 && ((l)this.q.elementAt((int)p2.q)).j + 16 >= p2.j + 5) {
                            this.n.removeElementAt(i2);
                            ((l)this.q.elementAt((int)p2.q)).p = false;
                        }
                        if (((l)this.q.elementAt((int)p2.q)).i + 8 > p2.i + 5) {
                            p2.b = 1;
                        }
                        if (((l)this.q.elementAt((int)p2.q)).j + 8 > p2.j + 5) {
                            p2.c = 1;
                        }
                        if (((l)this.q.elementAt((int)p2.q)).i + 8 < p2.i + 5) {
                            p2.b = -1;
                        }
                        if (((l)this.q.elementAt((int)p2.q)).j + 8 < p2.j + 5) {
                            p2.c = -1;
                        }
                        if (((l)this.q.elementAt((int)p2.q)).i + 8 == p2.i + 5) {
                            p2.b = 0;
                        }
                        if (((l)this.q.elementAt((int)p2.q)).j + 8 == p2.j + 5) {
                            p2.c = 0;
                        }
                    }
                    catch (ArrayIndexOutOfBoundsException arrayIndexOutOfBoundsException) {
                        this.n.removeElementAt(i2);
                    }
                } else {
                    if (h2.i - 1 <= p2.i + 5 && h2.i + 16 >= p2.i + 5 && h2.j - 1 <= p2.j + 5 && h2.j + 24 >= p2.j + 5) {
                        h2.h = false;
                        this.n.removeElementAt(i2);
                    }
                    if (h2.i + 8 > p2.i + 5) {
                        p2.b = 1;
                    }
                    if (h2.j + 8 > p2.j + 5) {
                        p2.c = 1;
                    }
                    if (h2.i + 8 < p2.i + 5) {
                        p2.b = -1;
                    }
                    if (h2.j + 8 < p2.j + 5) {
                        p2.c = -1;
                    }
                    if (h2.i + 8 == p2.i + 5) {
                        p2.b = 0;
                    }
                    if (h2.j + 8 == p2.j + 5) {
                        p2.c = 0;
                    }
                }
            }
            p2.a(graphics, n2, n3, n4, n5);
            if (p2.p) {
                if (h2.i + 20 < p2.i || h2.i + 4 > p2.i + p2.f || h2.j + h2.Z < p2.j || h2.j > p2.j + p2.g) continue;
                if (h2.p) {
                    if (p2.e == 0 && h2.e == 2) {
                        if (p2.h != 5) {
                            this.n.removeElementAt(i2);
                            continue;
                        }
                        p2.p = false;
                        p2.b = -p2.b;
                        p2.c = -p2.c;
                        p2.o <<= 1;
                        continue;
                    }
                    if (p2.e == 1 && h2.e == 3) {
                        if (p2.h != 5) {
                            this.n.removeElementAt(i2);
                            continue;
                        }
                        p2.p = false;
                        p2.b = -p2.b;
                        p2.c = -p2.c;
                        p2.o <<= 1;
                        continue;
                    }
                    if (p2.e == 2 && h2.e == 0) {
                        if (p2.h != 5) {
                            this.n.removeElementAt(i2);
                            continue;
                        }
                        p2.p = false;
                        p2.b = -p2.b;
                        p2.c = -p2.c;
                        p2.o <<= 1;
                        continue;
                    }
                    if (p2.e == 3 && h2.e == 1) {
                        if (p2.h != 5) {
                            this.n.removeElementAt(i2);
                            continue;
                        }
                        p2.p = false;
                        p2.o <<= 1;
                        p2.b = -p2.b;
                        p2.c = -p2.c;
                        continue;
                    }
                }
                if (h2.g && p2.h == 7) {
                    p2.p = false;
                    ++p2.o;
                    p2.b = -p2.b;
                    p2.c = -p2.c;
                    continue;
                }
                if (h2.L > 0) continue;
                h2.L = 20;
                h2.H -= p2.o / h2.M + 1;
                if (h2.H < 0) {
                    h2.H = 0;
                }
                if (p2.h == 7) {
                    for (n6 = 0; n6 < this.q.size(); ++n6) {
                        f2 = (l)this.q.elementAt(n6);
                        if (((l)f2).b != 23 || ((l)f2).z != 5) continue;
                        ((l)f2).F = 60;
                    }
                }
                if (p2.h != 2) {
                    this.n.removeElementAt(i2);
                } else if (!p2.r) {
                    p2.b = -p2.b;
                    p2.c = -p2.c;
                    p2.r = true;
                }
                bl = true;
                continue;
            }
            for (n6 = 0; n6 < this.q.size(); ++n6) {
                f2 = (l)this.q.elementAt(n6);
                if (!((l)f2).n && ((l)f2).b != 22 && ((l)f2).b != 23 && ((l)f2).b != 24 && ((l)f2).b != 38 || ((l)f2).f || ((l)f2).i + ((l)f2).B < p2.i || ((l)f2).i > p2.i + p2.f || ((l)f2).j + ((l)f2).C < p2.j || ((l)f2).j > p2.j + p2.g) continue;
                if (((l)f2).t <= 0) {
                    if ((((l)f2).b == 30 || ((l)f2).b == 31) && p2.h == 8) {
                        ((l)f2).z = 4;
                        ((l)f2).F = 0;
                        if (p2.r || ((l)f2).f) continue;
                        p2.b = -p2.b;
                        p2.c = -p2.c;
                        p2.r = true;
                        continue;
                    }
                    if (((l)f2).b == 24) {
                        if (p2.h != 11 && p2.h != 7 || ((l)f2).z == 20 || ((l)f2).z == 3 || ((l)f2).z == 6) continue;
                        ++((l)f2).J;
                        this.n.removeElementAt(i2);
                        ((l)f2).t = 10;
                        if (((l)f2).J != 3) continue;
                        ((l)f2).J = 0;
                        ((l)f2).z = 20;
                        ((l)f2).t = 0;
                        continue;
                    }
                    if (((l)f2).b == 38) {
                        if (p2.h != 3 || p2.i <= ((l)f2).i + 16 || p2.i >= ((l)f2).i + 32 || ((l)f2).z <= 0) continue;
                        --((l)f2).N;
                        if (((l)f2).N <= 0) {
                            ((l)f2).e = 0;
                            ((l)f2).f = true;
                            ((l)f2).q = this.w;
                            ((l)f2).a = 0;
                            continue;
                        }
                        ((l)f2).z = 0;
                        ((l)f2).t = 10;
                        continue;
                    }
                    if (((l)f2).b == 23) {
                        if (p2.h != 7 || ((l)f2).z != 5) continue;
                        Random random = new Random();
                        if (Math.abs(random.nextInt() % 2) != 1) {
                            ((l)f2).z = 6;
                            p2.p = true;
                            ++p2.o;
                            p2.b = -p2.b;
                            p2.c = -p2.c;
                            continue;
                        }
                        ((l)f2).z = 7;
                        this.n.removeElementAt(i2);
                        continue;
                    }
                    if (((l)f2).b == 10) {
                        if (p2.h == 3) {
                            --((l)f2).N;
                        }
                    } else {
                        if (((l)f2).b == 17 || ((l)f2).b == 38) continue;
                        ((l)f2).N -= p2.o;
                    }
                    if (((l)f2).N <= 0) {
                        ((l)f2).e = 0;
                        ((l)f2).f = true;
                        ((l)f2).q = this.w;
                        ((l)f2).a = 0;
                    } else {
                        ((l)f2).t = 10;
                    }
                }
                if (p2.h != 2 && p2.h != 8) {
                    if (p2.h == 3 || p2.h == 5) continue;
                    this.n.removeElementAt(i2);
                    continue;
                }
                if (p2.r || ((l)f2).f) continue;
                p2.b = -p2.b;
                p2.c = -p2.c;
                p2.r = true;
            }
            for (n6 = 0; n6 < this.p.size(); ++n6) {
                f2 = (c)this.p.elementAt(n6);
                if (((c)f2).i > p2.i + 10 || ((c)f2).i + 5 < p2.i || ((c)f2).j > p2.j + 10 || ((c)f2).j + 5 < p2.j || ((c)f2).a >= 100 || ((c)f2).a(h2, b2, this.a, false) != 1) continue;
                this.p.removeElementAt(n6);
                bl = true;
                this.a.g = true;
            }
            if (p2.h != 3 && p2.h != 8) continue;
            for (n6 = 0; n6 < this.r.size(); ++n6) {
                f2 = (n)this.r.elementAt(n6);
                if (((n)f2).i > p2.i + 10 || ((n)f2).i + 5 < p2.i || ((n)f2).j > p2.j + 10 || ((n)f2).j + 5 < p2.j) continue;
                if (((n)f2).a == 60) {
                    ((n)f2).s = true;
                }
                if (((n)f2).a != 30 || p2.h != 8 || this.F != 0) continue;
                ++this.g;
                if (this.g > 1) {
                    this.g = 0;
                }
                if (this.g == 0) {
                    h2.an &= ~(1 << this.f);
                }
                if (this.g == 1) {
                    h2.an |= 1 << this.f;
                }
                ((n)f2).C = this.g;
                this.F = 15;
            }
        }
        return bl;
    }

    final boolean a(int n2, Graphics graphics, int n3, int n4) {
        Object object = (n)this.r.elementAt(n2);
        if (n3 > 256) {
            n3 = 256;
        }
        if (n4 > 208) {
            n4 = 208;
        }
        if (((n)object).u > 0) {
            int n5;
            object = (String)this.H.elementAt(((n)object).b);
            ++this.v;
            Vector<String> vector = new Vector<String>();
            Object object2 = object;
            int n6 = 0;
            n6 = 0;
            Font font = graphics.getFont();
            for (n5 = 0; n5 < ((String)object).length(); ++n5) {
                if (font.substringWidth((String)object2, 0, n6) > n3 - 32) {
                    vector.addElement(((String)object2).substring(0, n6));
                    object2 = ((String)object2).substring(n6);
                    n6 = 0;
                }
                ++n6;
            }
            vector.addElement(((String)object2).substring(0, n6) + " ");
            n5 = vector.size();
            if (n5 > 7) {
                ++this.u;
                n5 = 7;
                if (vector.size() - this.u / font.getHeight() < 7) {
                    --this.u;
                }
            }
            int n7 = font.getHeight() * n5;
            graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
            graphics.setColor(0);
            graphics.setClip(0, 0, n3, n4);
            graphics.fillRect(10, (n4 / 3 << 1) - 48 + 32 - n7 / 2, n3 - 20, n7 + 10);
            graphics.setColor(0xFFFFFF);
            graphics.drawRect(10, (n4 / 3 << 1) - 48 + 32 - n7 / 2, n3 - 20, n7 + 10);
            graphics.setColor(0xAAAAAA);
            graphics.drawRect(11, (n4 / 3 << 1) - 47 + 32 - n7 / 2, n3 - 22, n7 + 8);
            graphics.setColor(0xFFFFFF);
            graphics.translate(15, (n4 / 3 << 1) - 44 + 32 - n7 / 2);
            if (this.u > 0) {
                graphics.translate(0, font.getHeight());
                graphics.translate(0, -(this.u % font.getHeight()));
            }
            n6 = 0;
            n7 = 1;
            for (n4 = this.u / font.getHeight(); n4 < vector.size(); ++n4) {
                graphics.drawString((String)vector.elementAt(n4), 0, 0, 20);
                graphics.translate(0, font.getHeight());
                if (++n6 <= 7) continue;
                n7 = 0;
                break;
            }
            if (this.v % 8 >= 4 && n7 != 0) {
                graphics.translate(n3 - 38, -font.getHeight() + 8);
                graphics.setColor(33274);
                graphics.fillRect(0, 0, 10, 10);
                graphics.setColor(0xFFFFFF);
            }
        }
        return true;
    }

    final boolean a(Graphics graphics, int n2, int n3, int n4) {
        int n5;
        String string = this.a.I[n4];
        if (n2 > 256) {
            n2 = 256;
        }
        if (n3 > 208) {
            n3 = 208;
        }
        ++this.v;
        Vector<String> vector = new Vector<String>();
        String string2 = string;
        int n6 = 0;
        n6 = 0;
        Font font = graphics.getFont();
        for (n5 = 0; n5 < string.length(); ++n5) {
            if (font.substringWidth(string2, 0, n6) > n2 - 32) {
                vector.addElement(string2.substring(0, n6));
                string2 = string2.substring(n6);
                n6 = 0;
            }
            ++n6;
        }
        vector.addElement(string2.substring(0, n6) + " ");
        n5 = vector.size();
        if (n5 > 7) {
            ++this.u;
            n5 = 7;
            if (vector.size() - this.u / font.getHeight() < 7) {
                --this.u;
            }
        }
        int n7 = font.getHeight() * n5;
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.setColor(0);
        graphics.setClip(0, 0, n2, n3);
        graphics.fillRect(10, (n3 / 3 << 1) - 48 + 32 - n7 / 2, n2 - 20, n7 + 10);
        graphics.setColor(0xFFFFFF);
        graphics.drawRect(10, (n3 / 3 << 1) - 48 + 32 - n7 / 2, n2 - 20, n7 + 10);
        graphics.setColor(0xAAAAAA);
        graphics.drawRect(11, (n3 / 3 << 1) - 47 + 32 - n7 / 2, n2 - 22, n7 + 8);
        graphics.setColor(0xFFFFFF);
        graphics.translate(15, (n3 / 3 << 1) - 44 + 32 - n7 / 2);
        if (this.u > 0) {
            graphics.translate(0, font.getHeight());
            graphics.translate(0, -(this.u % font.getHeight()));
        }
        n6 = 0;
        n3 = 1;
        for (n7 = this.u / font.getHeight(); n7 < vector.size(); ++n7) {
            graphics.drawString((String)vector.elementAt(n7), 0, 0, 20);
            graphics.translate(0, font.getHeight());
            if (++n6 <= 7) continue;
            n3 = 0;
            break;
        }
        if (this.v % 8 >= 4 && n3 != 0) {
            graphics.translate(n2 - 38, -font.getHeight() + 8);
            graphics.setColor(33274);
            graphics.fillRect(0, 0, 10, 10);
            graphics.setColor(0xFFFFFF);
        }
        return true;
    }

    final boolean a(String object, Graphics graphics, h h2, int n2, int n3, int n4, int n5, boolean bl) {
        this.j = n5;
        this.i = n4;
        System.out.println("Load Map" + (String)object);
        this.h = this.g;
        try {
            Object object2 = new byte[4];
            n5 = 0;
            object = this.getClass().getResourceAsStream((String)object);
            if (object == null) {
                System.out.println("File Not Found");
                return false;
            }
            object = new DataInputStream((InputStream)object);
            ((DataInputStream)object).read((byte[])object2, 0, 4);
            if (object2[0] == 88) {
                int n6;
                int n7;
                int n8;
                int n9;
                int n10;
                object2 = new i();
                new i().a = this.f;
                ((i)object2).b = this.E;
                ((i)object2).c = this.q.size();
                this.I.addElement(object2);
                if (this.I.size() > 16) {
                    this.I.removeElementAt(0);
                }
                this.q.removeAllElements();
                this.p.removeAllElements();
                this.o.removeAllElements();
                this.r.removeAllElements();
                this.s.removeAllElements();
                this.H.removeAllElements();
                this.n.removeAllElements();
                this.E = n2;
                this.f = n3;
                if (bl) {
                    h2.h = false;
                }
                o o2 = this.b;
                object2 = o2;
                o2.a();
                ((o)object2).b();
                ((o)object2).c();
                ((o)object2).d();
                this.x = null;
                this.c = null;
                this.y = null;
                this.d = null;
                this.z = null;
                this.B = ((DataInputStream)object).readByte();
                this.C = ((DataInputStream)object).readByte();
                byte by = ((DataInputStream)object).readByte();
                n5 = by;
                if (by != this.D) {
                    this.t = null;
                    this.D = (byte)n5;
                }
                this.A = null;
                System.gc();
                System.out.println("Tile set " + this.D);
                this.A = new byte[this.B * this.C];
                for (n10 = 0; n10 < this.B * this.C; ++n10) {
                    this.A[n10] = ((DataInputStream)object).readByte();
                }
                byte by2 = ((DataInputStream)object).readByte();
                n10 = by2;
                if (by2 > 0) {
                    for (n5 = 0; n5 < n10; ++n5) {
                        e e2 = new e();
                        new e().c = ((DataInputStream)object).readUnsignedByte();
                        e2.d = ((DataInputStream)object).readUnsignedByte();
                        e2.e = ((DataInputStream)object).readUnsignedByte();
                        e2.f = ((DataInputStream)object).readUnsignedByte();
                        e2.a = ((DataInputStream)object).readUnsignedByte();
                        e2.b = ((DataInputStream)object).readUnsignedByte();
                        this.o.addElement(e2);
                    }
                }
                try {
                    if (this.t == null) {
                        if (this.D == 0) {
                            this.t = Image.createImage("/Images/ZeldaTileSet01.png");
                        }
                        if (this.D == 1) {
                            this.t = Image.createImage("/Images/ZeldaTileSet02.png");
                        }
                        if (this.D == 2) {
                            this.t = Image.createImage("/Images/Zelda1Minish05.png");
                        }
                        if (this.D == 3) {
                            this.t = Image.createImage("/Images/Zelda1Minish01.png");
                        }
                        if (this.D == 4) {
                            this.t = Image.createImage("/Images/Zelda1Minish02.png");
                        }
                        if (this.D == 5) {
                            this.t = Image.createImage("/Images/Zelda1Minish03.png");
                        }
                        if (this.D == 6) {
                            this.t = Image.createImage("/Images/Zelda1Minish04.png");
                        }
                        if (this.D == 7) {
                            this.t = Image.createImage("/Images/Zelda1Minish06.png");
                        }
                        if (this.D == 8) {
                            this.t = Image.createImage("/Images/Zelda1Minish07.png");
                        }
                        if (this.D == 9) {
                            this.t = Image.createImage("/Images/Zelda1Minish08.png");
                        }
                        if (this.D == 10) {
                            this.t = Image.createImage("/Images/Zelda1Minish09.png");
                        }
                        if (this.D == 11) {
                            this.t = Image.createImage("/Images/Zelda1Minish10.png");
                        }
                        if (this.D == 12) {
                            this.t = Image.createImage("/Images/Zelda1Minish11.png");
                        }
                        if (this.D == 13) {
                            this.t = Image.createImage("/Images/Zelda1Minish12.png");
                        }
                        if (this.D == 14) {
                            this.t = Image.createImage("/Images/Zelda1Minish13.png");
                        }
                        if (this.D == 15) {
                            this.t = Image.createImage("/Images/Zelda1Minish14.png");
                        }
                        if (this.D == 16) {
                            this.t = Image.createImage("/Images/Zelda1Minish15.png");
                        }
                        if (this.D == 17) {
                            this.t = Image.createImage("/Images/Zelda1Minish16.png");
                        }
                        if (this.D == 18) {
                            this.t = Image.createImage("/Images/Zelda1Minish17.png");
                        }
                        if (this.D == 19) {
                            this.t = Image.createImage("/Images/Zelda1Minish18.png");
                        }
                        if (this.D == 20) {
                            this.t = Image.createImage("/Images/Zelda1Minish19.png");
                        }
                        if (this.D == 21) {
                            this.t = Image.createImage("/Images/Zelda1Minish20.png");
                        }
                        if (this.D == 22) {
                            this.t = Image.createImage("/Images/Zelda1Minish21.png");
                        }
                        if (this.D == 23) {
                            this.t = Image.createImage("/Images/Zelda1Minish22.png");
                        }
                        if (this.D == 24) {
                            this.t = Image.createImage("/Images/Zelda1Minish23.png");
                        }
                        if (this.D == 25) {
                            this.t = Image.createImage("/Images/Zelda1Minish24.png");
                        }
                        if (this.D == 26) {
                            this.t = Image.createImage("/Images/Zelda1Minish25.png");
                        }
                    }
                }
                catch (IOException iOException) {
                    return false;
                }
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.setColor(0);
                graphics.fillRect(0, 0, 255, 176);
                for (n5 = 0; n5 < this.C; ++n5) {
                    for (int i2 = 0; i2 < this.B; ++i2) {
                        n10 = this.A[n5 * this.B + i2] & 0x3F;
                        if (n10 == 0) {
                            graphics.translate(16, 0);
                            continue;
                        }
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        int n11 = n10 & 3;
                        n9 = n10 >> 2;
                        graphics.drawImage(this.t, -(n11 << 4), -(n9 << 4), 0);
                        graphics.translate(16, 0);
                    }
                    graphics.translate(-graphics.getTranslateX(), 16);
                }
                if (this.f == 20 && this.E == 2) {
                    this.t = null;
                    System.gc();
                    try {
                        this.t = Image.createImage("/Images/ganonbattleBG.png");
                    }
                    catch (IOException iOException) {}
                    if (this.t != null) {
                        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                        graphics.setClip(0, 0, 256, 176);
                        graphics.clipRect(0, 0, 256, 176);
                        graphics.drawImage(this.t, 0, 0, 0);
                    }
                }
                if (this.f == 20 && this.E == 3) {
                    this.a.p = 100;
                    return true;
                }
                int n12 = ((DataInputStream)object).readByte();
                n5 = n12;
                int n13 = n12;
                for (n10 = 0; n10 < this.I.size(); ++n10) {
                    i i3 = (i)this.I.elementAt(n10);
                    if (i3 == null || i3.b != this.E || i3.a != this.f) continue;
                    n13 = i3.c;
                }
                if (this.E == 2 && this.f == 20 && this.a.t) {
                    this.a.G = true;
                }
                if (n5 > 0) {
                    for (n10 = 0; n10 < n5; ++n10) {
                        int n14 = ((DataInputStream)object).readByte();
                        n9 = ((DataInputStream)object).readByte();
                        n14 <<= 4;
                        n9 <<= 4;
                        n8 = ((DataInputStream)object).readUnsignedByte();
                        if (n8 == 133 && (h2.aj & 0x1000000) > 0 || n10 >= n13) continue;
                        l l2 = new l(this.b.d(n8), n14, n9, 1, n8);
                        this.q.addElement(l2);
                    }
                    this.w = Image.createImage("/Images/Death.png");
                }
                byte by3 = ((DataInputStream)object).readByte();
                n10 = by3;
                if (by3 > 0) {
                    for (int i4 = 0; i4 < n10; ++i4) {
                        n9 = ((DataInputStream)object).readByte();
                        n8 = ((DataInputStream)object).readByte();
                        int n15 = ((DataInputStream)object).readUnsignedByte();
                        n5 = ((DataInputStream)object).readUnsignedByte();
                        c c2 = new c(this.b.a(n15), n15, n9 <<= 4, n8 <<= 4, n5, h2);
                        if (bl) {
                            n7 = 0;
                            if ((h2.aj & 1 << n5) <= 0) {
                                n7 = 1;
                            }
                            if (n15 == 128 || n15 == 129) {
                                n7 = (h2.ak & 1 << n5) > 0 ? 0 : 1;
                            }
                            if (n15 == 131) {
                                n7 = (h2.al & 1 << n5) > 0 ? 0 : 1;
                            }
                            if (n15 == 147 || n15 == 148 || n15 == 149) {
                                n7 = (h2.am & 1 << n5) > 0 ? 0 : 1;
                            }
                            if (n15 > 128) {
                                if (this.x == null) {
                                    try {
                                        this.x = Image.createImage("/Images/BigChest.png");
                                    }
                                    catch (IOException iOException) {
                                        System.out.println("error loading big chest");
                                    }
                                }
                                c2.d = this.x;
                            }
                            c2.h = n7 == 0;
                        }
                        this.p.addElement(c2);
                    }
                }
                if ((n6 = ((DataInputStream)object).readUnsignedByte()) > 0) {
                    for (n9 = 0; n9 < n6; ++n9) {
                        n8 = ((DataInputStream)object).readUnsignedByte();
                        ((DataInputStream)object).readUnsignedByte();
                        int n16 = 0;
                        n5 = ((DataInputStream)object).readUnsignedByte();
                        int n17 = ((DataInputStream)object).readByte();
                        n7 = ((DataInputStream)object).readByte();
                        n10 = ((DataInputStream)object).readUnsignedByte();
                        n16 = ((DataInputStream)object).readUnsignedByte();
                        ((DataInputStream)object).readUnsignedByte();
                        n17 <<= 4;
                        n7 <<= 4;
                        int n18 = ((DataInputStream)object).readUnsignedByte();
                        int n19 = ((DataInputStream)object).readUnsignedByte();
                        n n20 = new n(n8);
                        new n(n8).w = true;
                        n20.t = n16;
                        n20.i = n17;
                        n20.j = n7 + 32;
                        n20.a = n8;
                        n20.b = n5;
                        n20.c = n10;
                        n20.d = n18 & 3;
                        n20.e = (n18 & 4) > 0;
                        n20.f = (n18 & 8) > 0;
                        n20.g = (n18 & 0x10) > 0;
                        n20.h = (n18 & 0x20) > 0;
                        n20.n = (n18 & 0x40) > 0;
                        n20.o = (n18 & 0x80) > 0;
                        n20.p = (n19 & 1) > 0;
                        n20.q = (n19 & 2) > 0;
                        n20.u = (n19 & 0xC) >> 2;
                        n20.v = (n19 & 0x10) > 0;
                        n20.r = (byte)((n19 & 0xC0) >> 6);
                        if (n20.r == 3 && n20.a == 0) {
                            n20.r = 0;
                        }
                        n20.q = !n20.q;
                        System.out.println("type= " + n20.a + " " + n20.q);
                        if (n20.a > 0) {
                            if (n20.a != 17 && n20.a != 18) {
                                n20.J = this.b.c(n20.a);
                            } else {
                                n20.q = false;
                            }
                        }
                        n5 = 1;
                        n17 = n10 / 32;
                        n10 %= 32;
                        if (bl && n20.c != 0) {
                            if (n3 == 0 && (h2.ap[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 1 && (h2.aq[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 3 && (h2.ar[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 4 && (h2.as[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 5 && (h2.at[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 6 && (h2.au[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 7 && (h2.av[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 8 && (h2.aw[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 9 && (h2.ax[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 10 && (h2.ay[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                            if (n3 == 11 && (h2.az[n17] & 1 << n10) > 0) {
                                n5 = 0;
                            }
                        }
                        if (n5 == 0) continue;
                        if (n20.d <= 1) {
                            this.r.addElement(n20);
                            continue;
                        }
                        if (n20.a == 18) {
                            this.r.addElement(n20);
                        }
                        this.s.addElement(n20);
                    }
                }
                byte by4 = ((DataInputStream)object).readByte();
                n9 = by4;
                if (by4 > 0) {
                    for (n8 = 0; n8 < n9; ++n8) {
                        int n21 = ((DataInputStream)object).readUnsignedByte();
                        char[] cArray = new char[n21 + 1];
                        for (int i5 = 0; i5 < n21; ++i5) {
                            cArray[i5] = (char)((DataInputStream)object).readUnsignedByte();
                        }
                        cArray[n21] = 32;
                        String string = new String(cArray);
                        this.H.addElement(string);
                        System.out.println(string);
                    }
                }
                for (n8 = 0; n8 < this.r.size(); ++n8) {
                    if (((n)this.r.elementAt((int)n8)).a == 204) {
                        ((n)this.r.elementAt(n8)).a(graphics, 0, 0, 256, 256);
                        ((n)this.r.elementAt((int)n8)).z = true;
                    }
                    if (((n)this.r.elementAt((int)n8)).d == 0) {
                        ((n)this.r.elementAt(n8)).a(graphics, 0, 32, 256, 256);
                        ((n)this.r.elementAt((int)n8)).z = true;
                    }
                    if (this.a.x) continue;
                    if (((n)this.r.elementAt((int)n8)).a >= 192 && ((n)this.r.elementAt((int)n8)).a <= 203) {
                        ((n)this.r.elementAt(n8)).a(graphics, 0, 32, 256, 256);
                        ((n)this.r.elementAt((int)n8)).z = true;
                    }
                    if (((n)this.r.elementAt((int)n8)).a != 28) continue;
                    ((n)this.r.elementAt(n8)).a(graphics, 0, 32, 256, 256);
                    ((n)this.r.elementAt((int)n8)).z = true;
                }
                if (n3 == 3 && n2 == 11) {
                    this.m = true;
                }
                if (n3 == 4 && n2 == 3) {
                    this.m = true;
                }
                if (n3 == 5 && n2 == 15) {
                    this.m = true;
                }
                if (n3 == 6 && n2 == 8) {
                    this.m = true;
                }
                if (n3 == 7 && n2 == 9) {
                    this.m = true;
                }
                if (n3 == 8 && n2 == 11) {
                    this.m = true;
                }
                if (n3 == 9 && n2 == 15) {
                    this.m = true;
                }
                if (n3 == 10 && n2 == 17) {
                    this.m = true;
                }
                if (n3 == 12) {
                    n8 = 0;
                    if (n2 == 20 && (h2.aj & 0x2000000) == 0) {
                        n8 = (h2.aj & 0x1000000) > 0 ? 0 : 1;
                        h2.aj |= 0x1000000;
                    }
                    if ((h2.aj & 0x1000000) > 0 && (h2.aj & 0x2000000) == 0 && n8 == 0) {
                        l l3 = new l(this.b.d(133), 1000, 1000, 1, 133);
                        this.q.addElement(l3);
                    }
                }
            } else {
                return false;
            }
            return true;
        }
        catch (IOException iOException) {
            System.out.println("File Not Found");
            return true;
        }
    }

    final boolean a(int n2, int n3) {
        for (int i2 = 0; i2 < this.n.size(); ++i2) {
            p p2 = (p)this.n.elementAt(i2);
            if (p2.h != 8) continue;
            if (!p2.r) {
                p2.b = n2;
                p2.c = n3;
                this.k = p2.i;
                this.l = p2.j;
                return true;
            }
            this.k = p2.i;
            this.l = p2.j;
            return false;
        }
        return false;
    }

    private void b(int n2, int n3, h h2) {
        n3 /= 32;
        int n4 = n2 % 32;
        this.v = 0;
        if (n2 == 0) {
            return;
        }
        if (this.f == 0) {
            int n5 = n3;
            h2.ap[n5] = h2.ap[n5] | 1 << n4;
        }
        if (this.f == 1) {
            int n6 = n3;
            h2.aq[n6] = h2.aq[n6] | 1 << n4;
        }
        if (this.f == 3) {
            int n7 = n3;
            h2.ar[n7] = h2.ar[n7] | 1 << n4;
        }
        if (this.f == 4) {
            int n8 = n3;
            h2.as[n8] = h2.as[n8] | 1 << n4;
        }
        if (this.f == 5) {
            int n9 = n3;
            h2.at[n9] = h2.at[n9] | 1 << n4;
        }
        if (this.f == 6) {
            int n10 = n3;
            h2.au[n10] = h2.au[n10] | 1 << n4;
        }
        if (this.f == 7) {
            int n11 = n3;
            h2.av[n11] = h2.av[n11] | 1 << n4;
        }
        if (this.f == 8) {
            int n12 = n3;
            h2.aw[n12] = h2.aw[n12] | 1 << n4;
        }
        if (this.f == 9) {
            int n13 = n3;
            h2.ax[n13] = h2.ax[n13] | 1 << n4;
        }
        if (this.f == 10) {
            int n14 = n3;
            h2.ay[n14] = h2.ay[n14] | 1 << n4;
        }
        if (this.f == 11 || this.f == 20) {
            int n15 = n3;
            h2.az[n15] = h2.az[n15] | 1 << n4;
        }
    }

    final void d(h h2) {
        f f2;
        int n2;
        int n3 = 0;
        int n4 = 0;
        if (h2.ae <= 0) {
            h2.F = true;
            h2.ae = 10;
        }
        if (h2.ae != 6) {
            return;
        }
        if (h2.e == 0) {
            n3 = h2.i + 12;
            n4 = h2.j - 26;
        }
        if (h2.e == 1) {
            n3 = h2.i + 16 + 12;
            n4 = h2.j - 16;
        }
        if (h2.e == 2) {
            n3 = h2.i + 12;
            n4 = h2.j;
        }
        if (h2.e == 3) {
            n3 = h2.i - 10;
            n4 = h2.j - 16;
        }
        for (n2 = 0; n2 < this.r.size(); ++n2) {
            f2 = (n)this.r.elementAt(n2);
            if (f2.i > n3 || f2.i + f2.F < n3 || f2.j > n4 + 32 || f2.j + f2.G < n4 + 32 || f2.a != 218) continue;
            for (n3 = 0; n3 < this.p.size(); ++n3) {
                c c2 = (c)this.p.elementAt(n3);
                if (c2.i > f2.i || c2.i + 16 < f2.i || c2.j > f2.j || c2.j + 16 < f2.j) continue;
                c2.f = true;
            }
            this.r.removeElementAt(n2);
            return;
        }
        for (n2 = 0; n2 < this.q.size(); ++n2) {
            f2 = (l)this.q.elementAt(n2);
            if (((l)f2).b != 28 && ((l)f2).b != 29 || ((l)f2).i > n3 || ((l)f2).i + ((l)f2).B < n3 || ((l)f2).j > n4 + 32 || ((l)f2).j + ((l)f2).C < n4 + 32) continue;
            ((l)f2).z = 3;
        }
    }
}

