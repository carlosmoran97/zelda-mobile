/*
 * Decompiled with CFR 0.152.
 */
import java.io.IOException;
import java.util.Random;
import java.util.Vector;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public final class l
extends f {
    public byte a;
    public int b;
    public byte c;
    public byte d;
    public byte e;
    public boolean f;
    public boolean g;
    public boolean h;
    public boolean n;
    public boolean o;
    boolean p;
    Image q;
    Image r;
    private Image W;
    private int X;
    int s;
    int t;
    int u;
    float v;
    float w;
    float x;
    float y;
    int z;
    int A;
    int B;
    int C;
    int D;
    int E;
    int F;
    int G;
    int H;
    int I;
    int J;
    int K;
    int L;
    int M;
    int N;
    int O;
    boolean P;
    boolean Q;
    Vector R;
    Vector S;
    private Vector Y;
    private Vector Z;
    Vector T;
    private Vector aa;
    Random U;
    int[] V;
    private int[] ab;

    l(Image object, int n2, int n3, byte by, int n4) {
        this.i = n2;
        this.j = n3 + 32;
        this.c = 1;
        this.b = n4;
        this.q = object;
        this.k = 8;
        this.l = 8;
        this.B = 16;
        this.C = 16;
        this.m = 0;
        this.d = 1;
        this.N = 1;
        this.O = 2;
        this.K = 0;
        this.t = 0;
        this.n = true;
        this.g = false;
        this.h = false;
        this.o = true;
        this.z = 0;
        this.U = new Random();
        this.U.setSeed(this.i * this.j + this.U.nextInt());
        this.c = (byte)Math.abs(this.U.nextInt() % 4);
        this.W = null;
        if (this.b == 0) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.d = (byte)2;
            this.L = 1;
        }
        if (this.b == 1) {
            this.n = false;
            this.g = false;
            this.h = true;
            this.L = 1;
        }
        if (this.b == 3) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.N = 2;
            this.L = 2;
        }
        if (this.b == 4) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.L = 0;
        }
        if (this.b == 5) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.N = 2;
            this.L = 1;
        }
        if (this.b == 6) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.N = 3;
            this.L = 2;
        }
        if (this.b == 7) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.d = (byte)3;
            this.L = 1;
        }
        if (this.b == 8) {
            this.L = 3;
            this.n = true;
            this.g = false;
            this.h = false;
            this.d = (byte)2;
            this.O = 4;
            this.N = 4;
            this.T = new Vector();
            for (int i2 = 0; i2 < 22; ++i2) {
                g g2 = new g();
                new g().a = this.i;
                g2.b = this.j;
                g2.c = 3;
                this.T.addElement(g2);
            }
            this.R = new Vector();
            object = new g();
            g g3 = new g();
            g g4 = new g();
            g g5 = new g();
            g g6 = new g();
            ((g)object).a = this.i;
            ((g)object).b = this.j;
            ((g)object).c = 3;
            this.R.addElement(object);
            g3.a = ((g)object).a - 12;
            g3.b = ((g)object).b;
            g3.c = ((g)object).c;
            this.R.addElement(g3);
            g4.a = g3.a - 12;
            g4.b = g3.b;
            g4.c = g3.c;
            this.R.addElement(g4);
            g5.a = g4.a - 12;
            g5.b = g4.b;
            g5.c = g4.c;
            this.R.addElement(g5);
            g6.a = g5.a - 12;
            g6.b = g5.b;
            g6.c = g5.c;
            this.R.addElement(g6);
        }
        if (this.b == 9) {
            this.n = false;
            this.g = false;
            this.h = true;
            this.D = this.i;
            this.E = this.j;
            this.O = 4;
        }
        if (this.b == 10) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.D = this.i;
            this.E = this.j;
            this.O = 2;
            this.N = 4;
            this.L = 2;
        }
        if (this.b == 11) {
            this.n = true;
            this.g = false;
            this.h = true;
            this.D = this.i;
            this.E = this.j;
            this.O = 1;
            this.N = 1;
            this.L = 1;
        }
        if (this.b == 12) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.D = this.i;
            this.E = this.j;
            this.O = 2;
            this.N = 2;
            this.L = 2;
        }
        if (this.b == 13) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.D = this.i;
            this.E = this.j;
            this.O = 1;
            this.N = 3;
            this.d = 0;
            this.A = 16;
            this.L = 1;
        }
        if (this.b == 14) {
            this.n = false;
            this.g = false;
            this.h = true;
            this.D = this.i;
            this.E = this.j;
            this.O = 2;
            this.N = 3;
            this.d = 1;
            this.L = 0;
        }
        if (this.b == 15) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.D = this.i;
            this.E = this.j;
            this.O = 4;
            this.N = 8;
            this.B = 16;
            this.C = 26;
            this.k = 8;
            this.l = 12;
            this.L = 2;
        }
        if (this.b == 16) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.N = 10;
            this.B = 32;
            this.C = 32;
            this.O = 4;
            this.k = 16;
            this.l = 16;
            this.L = 3;
        }
        if (this.b == 17) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.N = 10;
            this.B = 16;
            this.C = 16;
            this.O = 4;
            this.k = 20;
            this.l = 20;
            this.d = (byte)4;
            this.B = 20;
            this.C = 20;
            this.L = 3;
        }
        if (this.b == 18) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.N = 10;
            this.d = (byte)2;
            this.B = 32;
            this.C = 32;
            this.O = 4;
            this.k = 8;
            this.l = 8;
            this.L = 3;
        }
        if (this.b == 20) {
            this.L = 3;
            this.n = true;
            this.g = false;
            this.h = true;
            this.N = 10;
            this.d = (byte)2;
            this.B = 48;
            this.C = 64;
            this.O = 4;
            this.k = 24;
            this.l = 32;
            this.D = this.i;
            this.E = this.j + 24;
            this.H = this.d;
            this.I = -this.d;
            this.R = new Vector();
            this.S = new Vector();
            this.Y = new Vector();
            this.Z = new Vector();
            for (int i3 = 0; i3 < 5; ++i3) {
                g g7 = new g();
                g g8 = new g();
                g g9 = new g();
                g g10 = new g();
                g7.a = this.i - i3 * 10;
                g7.b = this.j - i3 * 6 + 24;
                g7.c = 3;
                this.R.addElement(g7);
                g8.a = this.i - (i3 << 3);
                g8.b = this.j + (i3 << 1) + 24;
                g8.c = 3;
                this.S.addElement(g8);
                g9.a = this.i - (i3 << 2);
                g9.b = this.j + (i3 << 1);
                g9.c = 3;
                this.Y.addElement(g9);
                g10.a = this.i - i3 * 6;
                g10.b = this.j + (i3 << 3);
                g10.c = 3;
                this.Z.addElement(g10);
            }
        }
        if (this.b == 19) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.D = this.i;
            this.E = this.j;
            this.O = 1;
            this.N = 3;
            this.d = 0;
            this.z = 0;
            this.F = 0;
            Random random = new Random();
            random.setSeed(this.i * this.j + random.nextInt());
            this.G = random.nextInt() % 100;
            this.L = 2;
        }
        if (this.b == 21) {
            this.n = true;
            this.g = false;
            this.h = true;
            this.N = 20;
            this.d = (byte)2;
            this.B = 55;
            this.C = 90;
            this.k = 25;
            this.l = 45;
            this.j -= 50;
            this.i -= 40;
            this.m = 80;
            this.O = 10;
            this.D = this.i;
            this.E = this.j + 100;
            this.o = false;
            this.n = false;
            this.z = 0;
            this.F = 0;
            this.L = 3;
        }
        if (this.b == 22) {
            this.n = false;
            this.g = false;
            this.h = true;
            this.N = 50;
            this.d = (byte)10;
            this.z = 0;
            this.F = 0;
            this.G = 50;
            this.B = 21;
            this.C = 37;
            this.k = 10;
            this.l = 18;
            this.O = 5;
            this.o = true;
            this.F = 0;
            this.L = 3;
            this.J = 0;
        }
        if (this.b == 23) {
            this.n = true;
            this.g = false;
            this.h = true;
            this.O = 4;
            this.z = 0;
            this.N = 50;
            this.B = 35;
            this.C = 43;
            this.k = 17;
            this.l = 21;
            this.c = 1;
        }
        if (this.b == 24) {
            this.n = true;
            this.g = false;
            this.h = true;
            this.O = 4;
            this.z = 0;
            this.N = 40;
            this.B = 56;
            this.C = 48;
            this.k = 27;
            this.l = 24;
            this.c = (byte)2;
            this.j = 50;
            this.J = 0;
        }
        if (this.b == 25) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 4;
            this.z = 0;
            this.N = 40;
            this.B = 72;
            this.C = 72;
            this.J = 0;
            this.c = 0;
            this.d = (byte)4;
        }
        if (this.b == 26) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 2;
            this.z = 0;
            this.N = 4;
            this.J = 0;
            this.c = 0;
        }
        if (this.b == 27) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 2;
            this.z = 0;
            this.N = 12;
            this.J = 0;
            this.c = 0;
            this.k = 16;
            this.l = 16;
            this.B = 32;
            this.C = 32;
        }
        if (this.b == 28) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.c = 0;
            this.O = 2;
            this.z = 0;
            this.N = 24;
            this.c = 0;
            this.k = 16;
            this.l = 16;
            this.B = 32;
            this.C = 32;
            this.J = 1;
            this.G = 0;
            this.V = new int[6];
            this.ab = new int[6];
            this.V[0] = 2;
            this.V[1] = 3;
            this.V[2] = 4;
            this.V[3] = 5;
            this.V[4] = 6;
            this.V[5] = 7;
            this.ab[0] = 1;
            this.ab[1] = 1;
            this.ab[2] = 1;
            this.ab[3] = 1;
            this.ab[4] = 1;
            this.ab[5] = 1;
        }
        if (this.b == 29) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.c = 0;
            this.O = 2;
            this.z = 0;
            this.N = 58;
            this.c = 0;
            this.k = 32;
            this.l = 32;
            this.B = 64;
            this.C = 64;
            this.J = 1;
            this.G = 0;
            this.V = new int[6];
            this.ab = new int[6];
            this.V[0] = 2;
            this.V[1] = 3;
            this.V[2] = 4;
            this.V[3] = 5;
            this.V[4] = 6;
            this.V[5] = 7;
            this.ab[0] = 1;
            this.ab[1] = 1;
            this.ab[2] = 1;
            this.ab[3] = 1;
            this.ab[4] = 1;
            this.ab[5] = 1;
            this.aa = new Vector();
        }
        if (this.b == 30) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 2;
            this.d = (byte)4;
            this.z = 0;
            this.N = 32;
            this.J = 0;
            this.c = 0;
            this.k = 10;
            this.l = 10;
            this.B = 20;
            this.C = 20;
            this.M = -1;
        }
        if (this.b == 31) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 2;
            this.d = (byte)4;
            this.z = 0;
            this.N = 32;
            this.J = 0;
            this.c = 0;
            this.k = 10;
            this.l = 10;
            this.B = 20;
            this.C = 20;
            this.M = 1;
        }
        if (this.b == 32) {
            this.n = false;
            this.g = true;
            this.h = false;
            this.O = 4;
            this.d = (byte)4;
            this.z = 0;
            this.N = 8;
            this.J = 0;
            this.c = 0;
            this.k = 8;
            this.l = 9;
            this.B = 16;
            this.C = 18;
            this.F = 30;
        }
        if (this.b == 33) {
            this.n = false;
            this.g = true;
            this.h = false;
            this.O = 4;
            this.d = (byte)4;
            this.z = 0;
            this.N = 2;
            this.J = 0;
            this.c = 0;
            this.k = 8;
            this.l = 8;
            this.B = 16;
            this.C = 16;
            this.F = 100;
        }
        if (this.b == 34) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.k = 32;
            this.l = 32;
            this.B = 64;
            this.C = 64;
        }
        if (this.b == 35) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 4;
            this.d = 1;
            this.z = 0;
            this.N = 16;
            this.k = 22;
            this.l = 22;
            this.B = 44;
            this.C = 40;
        }
        if (this.b == 36) {
            this.n = false;
            this.g = false;
            this.h = true;
            this.L = 0;
            this.O = 4;
            this.B = 23;
            this.C = 30;
            this.k = 11;
            this.l = 15;
            this.A = 100;
            this.X = this.i;
            this.s = this.j;
            Random random = new Random();
            random.setSeed(this.i * this.j + random.nextInt());
            this.j -= 100 + (Math.abs(random.nextInt() % 30) << 2);
            this.z = 0;
        }
        if (this.b == 37) {
            this.n = true;
            this.g = false;
            this.h = false;
            this.O = 4;
            this.d = 1;
            this.z = 0;
            this.N = 128;
            this.k = 22;
            this.l = 22;
            this.B = 44;
            this.C = 40;
            this.D = this.i;
            this.E = this.j + 24;
        }
        if (this.b == 38) {
            this.n = false;
            this.g = false;
            this.h = true;
            this.O = 10;
            this.d = 1;
            this.z = 0;
            this.N = 8;
            this.k = 24;
            this.l = 8;
            this.B = 48;
            this.C = 16;
        }
        if (this.b == 128) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.l += 4;
        }
        if (this.b == 129) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.l += 4;
        }
        if (this.b == 130) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.l += 4;
        }
        if (this.b == 131) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.l += 4;
        }
        if (this.b == 132) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.k = 12;
            this.l = 16;
            this.B = 24;
            this.C = 32;
            this.X = 200;
            this.s = 210;
            this.F = -1;
            this.G = 1;
        }
        if (this.b == 133) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.k = 13;
            this.l = 14;
            this.B = 16;
            this.C = 28;
            this.c = 1;
            this.R = new Vector();
            this.m = 8;
        }
        if (this.b == 134) {
            this.n = false;
            this.g = true;
            this.h = true;
            this.k = 13;
            this.l = 14;
            this.B = 16;
            this.C = 28;
            this.c = 1;
            this.R = new Vector();
            this.m = 8;
            this.F = 0;
        }
        if (this.b == 135) {
            this.n = false;
            this.g = true;
            this.h = true;
        }
    }

    final void a(Graphics graphics, int n2, int n3, int n4, int n5) {
        if (n4 + n2 < this.i && this.b != 8 && this.b != 20 && this.b != 21) {
            return;
        }
        if (this.i + this.k + this.k < n2 && this.b != 8 && this.b != 20 && this.b != 21) {
            return;
        }
        if (n5 + n3 < this.j && this.b != 8 && this.b != 20 && this.b != 21) {
            return;
        }
        if (this.j + this.l + this.l - 32 < n3 && this.b != 8 && this.b != 20 && this.b != 21) {
            return;
        }
        if (this.t % 3 == 1 && this.z != 20 && this.b != 29) {
            this.a();
            return;
        }
        this.a();
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(this.i - n2, this.j - n3);
        graphics.setClip(0, 0, 16, 16);
        graphics.clipRect(0, 0, 16, 16);
        if (!this.f) {
            if (this.b == 0) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 1) {
                graphics.drawImage(this.q, -16 * (this.a >> 2), 0, 0);
            }
            if (this.b == 2) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), 0, 0);
            }
            if (this.b == 3) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), 0, 0);
            }
            if (this.b == 4) {
                graphics.translate(0, 4);
                graphics.setClip(0, 0, 16, 8);
                graphics.clipRect(0, 0, 16, 8);
                graphics.drawImage(this.q, 0, -8 * (this.a >> 3), 0);
            }
            if (this.b == 5) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 6) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 7) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 8) {
                for (n4 = 0; n4 < this.R.size(); ++n4) {
                    g g2 = (g)this.R.elementAt(n4);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(g2.a - n2, g2.b - n3);
                    if (n4 == this.R.size() - 1) {
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        if (this.c == 0) {
                            graphics.drawImage(this.q, 0, 0, 0);
                        }
                        if (this.c == 1) {
                            graphics.drawImage(this.q, -16, 0, 0);
                        }
                        if (this.c == 2) {
                            graphics.drawImage(this.q, 0, -16, 0);
                        }
                        if (this.c != 3) continue;
                        graphics.drawImage(this.q, -16, -16, 0);
                        continue;
                    }
                    if (n4 == 0) {
                        graphics.translate(7, 7);
                        graphics.setClip(0, 0, 6, 6);
                        graphics.clipRect(0, 0, 6, 6);
                        graphics.drawImage(this.q, -12, -32, 0);
                        continue;
                    }
                    graphics.translate(2, 2);
                    graphics.setClip(0, 0, 12, 12);
                    graphics.clipRect(0, 0, 12, 12);
                    graphics.drawImage(this.q, 0, -32, 0);
                }
            }
            if (this.b == 9) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.q, 0, 0, 0);
            }
            if (this.b == 10) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 11) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                if (this.z == 1) {
                    graphics.drawImage(this.q, 0, -32, 0);
                }
                if (this.z == 2) {
                    graphics.drawImage(this.q, -16, -32, 0);
                    graphics.translate(0, -this.A);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, 0, -(this.c >> 1) << 4, 0);
                    graphics.translate(0, this.A);
                }
                if (this.z == 3) {
                    graphics.drawImage(this.q, -(this.a >> 3) << 4, -(this.c >> 1) << 4, 0);
                }
            }
            if (this.b == 12) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 13) {
                graphics.drawImage(this.q, 0, -64, 0);
                if (this.A > 0) {
                    graphics.translate(0, -16 + this.A);
                } else {
                    graphics.translate(0, -16 - this.A);
                }
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
            }
            if (this.b == 14) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.q, -16 * this.c, 0, 0);
            }
            if (this.b == 15) {
                graphics.setClip(0, 0, 16, 26);
                graphics.clipRect(0, 0, 16, 26);
                graphics.drawImage(this.q, -16 * (this.a >> 3), -26 * this.c, 0);
            }
            if (this.b == 16) {
                graphics.setClip(0, 0, 32, 32);
                graphics.clipRect(0, 0, 32, 32);
                graphics.drawImage(this.q, -32 * (this.a >> 3), 0, 0);
            }
            if (this.b == 17) {
                Random random = new Random();
                random.setSeed(this.a * 3456);
                n5 = random.nextInt() % 2;
                n4 = random.nextInt() % 2;
                if (this.c == 0) {
                    this.k = 20;
                    this.l = 20;
                    this.B = 20;
                    this.C = 20;
                    graphics.translate(-8, 0);
                    graphics.translate(0, 0);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, n5 * -15, -85, 0);
                    graphics.translate(0, 8);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -15 - n4 * 15, -85, 0);
                    graphics.translate(0 - (n5 << 1), 8 - (n4 << 1));
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -30, -85, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-8, 0);
                    graphics.setClip(0, 0, 23, 45);
                    graphics.clipRect(0, 0, 23, 45);
                    graphics.drawImage(this.q, -23 * (this.a >> 3), 0, 0);
                    this.k = 12;
                    this.l = 22;
                    this.B = 23;
                    this.C = 40;
                }
                if (this.c == 1) {
                    graphics.translate(0, -8);
                    graphics.translate(0, 0);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, n5 * -15, -85, 0);
                    graphics.translate(-8, 0);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -15 - n4 * 15, -85, 0);
                    graphics.translate(-8 - n5 * n5, -n4 << 1);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -30, -85, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(0, -8);
                    graphics.setClip(0, 0, 44, 20);
                    graphics.clipRect(0, 0, 44, 20);
                    graphics.drawImage(this.q, -44, -45 - 20 * (this.a >> 3), 0);
                    this.k = 22;
                    this.l = 10;
                    this.B = 44;
                    this.C = 20;
                }
                if (this.c == 2) {
                    graphics.translate(-8, 0);
                    graphics.translate(0, 0);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, n5 * -15, -85, 0);
                    graphics.translate(0, -8);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -15 - n4 * 15, -85, 0);
                    graphics.translate(0 - n5 * n5, -8 - (n4 << 1));
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -30, -85, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-8, 0);
                    graphics.setClip(0, 0, 23, 45);
                    graphics.clipRect(0, 0, 23, 45);
                    graphics.drawImage(this.q, -23 * (this.a >> 3) - 46, 0, 0);
                    this.k = 12;
                    this.l = 22;
                    this.B = 23;
                    this.C = 40;
                }
                if (this.c == 3) {
                    graphics.translate(0, -8);
                    graphics.translate(35, 0);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, n5 * -15, -85, 0);
                    graphics.translate(8, 0);
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -15 - n4 * 15, -85, 0);
                    graphics.translate(8 - n5 * n5, 0 - (n4 << 1));
                    graphics.setClip(0, 0, 15, 15);
                    graphics.clipRect(0, 0, 15, 15);
                    graphics.drawImage(this.q, -30, -85, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(0, -8);
                    graphics.setClip(0, 0, 44, 20);
                    graphics.clipRect(0, 0, 44, 20);
                    graphics.drawImage(this.q, 0, -45 - 20 * (this.a >> 3), 0);
                    this.k = 22;
                    this.l = 10;
                    this.B = 44;
                    this.C = 20;
                }
                this.a = (byte)(this.a + 1);
            }
            if (this.b == 18) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.q, 0, 0, 0);
                if (this.N > 1) {
                    graphics.translate(12, 0);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -16 + 16 * (this.a >> 3), -32, 0);
                    graphics.translate(-12, 0);
                }
                if (this.N > 3) {
                    graphics.translate(-12, 0);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -16 * (this.a >> 3), -16, 0);
                    graphics.translate(12, 0);
                }
                if (this.N > 5) {
                    graphics.translate(0, -12);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -32, -32 + 16 * (this.a >> 3), 0);
                    graphics.translate(0, 12);
                }
                if (this.N > 7) {
                    graphics.translate(0, 12);
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -16 - 16 * (this.a >> 3), 0, 0);
                }
            }
            if (this.b == 19) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                if (this.z != 0) {
                    if (this.z == 1 && this.a % 2 == 0) {
                        graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
                    }
                    if (this.z == 2) {
                        graphics.drawImage(this.q, -16 * (this.a >> 3), -16 * this.c, 0);
                    }
                }
            }
            if (this.b == 20) {
                for (n4 = 0; n4 < this.R.size(); ++n4) {
                    g g3 = (g)this.R.elementAt(n4);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(g3.a - n2, g3.b - n3);
                    if (n4 < this.R.size() - 1) {
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        graphics.drawImage(this.q, -48, -50, 0);
                        continue;
                    }
                    if (g3.c < 0) {
                        ++g3.c;
                        graphics.setClip(0, 0, 33, 26);
                        graphics.clipRect(0, 0, 33, 26);
                        graphics.drawImage(this.q, -47, 0, 0);
                        continue;
                    }
                    graphics.setClip(0, 0, 33, 24);
                    graphics.clipRect(0, 0, 33, 24);
                    graphics.drawImage(this.q, -47, -26, 0);
                }
                for (n4 = 0; n4 < this.S.size(); ++n4) {
                    g g4 = (g)this.S.elementAt(n4);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(g4.a - n2, g4.b - n3);
                    if (n4 < this.S.size() - 1) {
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        graphics.drawImage(this.q, -48, -50, 0);
                        continue;
                    }
                    if (g4.c < 0) {
                        ++g4.c;
                        graphics.setClip(0, 0, 33, 26);
                        graphics.clipRect(0, 0, 33, 26);
                        graphics.drawImage(this.q, -47, 0, 0);
                        continue;
                    }
                    graphics.setClip(0, 0, 33, 24);
                    graphics.clipRect(0, 0, 33, 24);
                    graphics.drawImage(this.q, -47, -26, 0);
                }
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.setClip(0, 0, 47, 64);
                graphics.clipRect(0, 0, 47, 64);
                graphics.drawImage(this.q, 0, 0, 0);
            }
            if (this.b == 21 && this.o) {
                if (this.z == 1 && this.F % 2 == 0) {
                    graphics.setClip(0, 0, 56, 90);
                    graphics.clipRect(0, 0, 56, 90);
                    graphics.drawImage(this.q, 0, 0, 0);
                }
                if (this.z == 2) {
                    if (this.F < 70) {
                        graphics.setClip(0, 0, 56, 90);
                        graphics.clipRect(0, 0, 56, 90);
                        graphics.drawImage(this.q, 0, 0, 0);
                    }
                    if (this.F >= 70 && this.F < 75) {
                        graphics.setClip(0, 0, 49, 60);
                        graphics.clipRect(0, 0, 49, 60);
                        graphics.drawImage(this.q, -56, -58, 0);
                    }
                    if (this.F >= 75) {
                        graphics.setClip(0, 0, 68, 59);
                        graphics.clipRect(0, 0, 68, 59);
                        graphics.drawImage(this.q, -56, 0, 0);
                    }
                }
            }
            if (this.b == 22) {
                if (this.z == 0) {
                    return;
                }
                if (this.z == 1 && this.a % 2 == 0) {
                    if (this.c == 1) {
                        graphics.translate(0, 0);
                        graphics.setClip(0, 0, 22, 36);
                        graphics.clipRect(0, 0, 22, 36);
                        graphics.drawImage(this.q, -80, 0, 0);
                    }
                    if (this.c == 3) {
                        graphics.translate(0, 0);
                        graphics.setClip(0, 0, 21, 36);
                        graphics.clipRect(0, 0, 21, 36);
                        graphics.drawImage(this.q, 0, 0, 0);
                    }
                }
                if (this.z == 2) {
                    if (this.c == 1) {
                        graphics.translate(0, 0);
                        graphics.setClip(0, 0, 28, 36);
                        graphics.clipRect(0, 0, 28, 36);
                        graphics.drawImage(this.q, -50, 0, 0);
                    }
                    if (this.c == 3) {
                        graphics.translate(0, 0);
                        graphics.setClip(0, 0, 21, 36);
                        graphics.clipRect(0, 0, 21, 36);
                        graphics.drawImage(this.q, -21, 0, 0);
                    }
                }
                if (this.z == 3) {
                    if (this.c == 1) {
                        if (this.a >> 3 < 1) {
                            graphics.translate(0, 0);
                            graphics.setClip(0, 0, 22, 36);
                            graphics.clipRect(0, 0, 22, 36);
                            graphics.drawImage(this.q, -80, 0, 0);
                        } else {
                            graphics.translate(0, 0);
                            graphics.setClip(0, 0, 28, 36);
                            graphics.clipRect(0, 0, 28, 36);
                            graphics.drawImage(this.q, -50, 0, 0);
                        }
                    }
                    if (this.c == 3) {
                        if (this.a >> 3 < 1) {
                            graphics.translate(0, 0);
                            graphics.setClip(0, 0, 21, 36);
                            graphics.clipRect(0, 0, 21, 36);
                            graphics.drawImage(this.q, 0, 0, 0);
                        } else {
                            graphics.translate(0, 0);
                            graphics.setClip(0, 0, 21, 36);
                            graphics.clipRect(0, 0, 21, 36);
                            graphics.drawImage(this.q, -21, 0, 0);
                        }
                    }
                }
            }
            if (this.b == 23) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.z == 0 || this.z == 11 || this.z == 10 || this.z == 1 || this.z == 3 || this.z == 5) {
                    if ((this.z == 1 || this.z == 3) && this.a % 2 == 1) {
                        return;
                    }
                    if (this.c == 1) {
                        graphics.translate(7, 47);
                        graphics.setClip(0, 0, 27, 7);
                        graphics.clipRect(0, 0, 27, 7);
                        graphics.drawImage(this.q, 0, -107, 0);
                        graphics.translate(-7, -47 + (this.a >> 3));
                        graphics.setClip(0, 0, 35, 42);
                        graphics.clipRect(0, 0, 35, 42);
                        graphics.drawImage(this.q, 0, 0, 0);
                    }
                    if (this.c == 3) {
                        graphics.translate(7, 47);
                        graphics.setClip(0, 0, 27, 7);
                        graphics.clipRect(0, 0, 27, 7);
                        graphics.drawImage(this.q, 0, -107, 0);
                        graphics.translate(-7, -47 + (this.a >> 3));
                        graphics.setClip(0, 0, 35, 42);
                        graphics.clipRect(0, 0, 35, 42);
                        graphics.drawImage(this.q, -35, 0, 0);
                    }
                }
                if (this.z == 2 || this.z == 4 || this.z == 6) {
                    if (this.c == 1) {
                        graphics.translate(7, 47);
                        graphics.setClip(0, 0, 27, 7);
                        graphics.clipRect(0, 0, 27, 7);
                        graphics.drawImage(this.q, 0, -107, 0);
                        graphics.translate(-7, -69 + (this.a >> 3));
                        graphics.setClip(0, 0, 34, 64);
                        graphics.clipRect(0, 0, 34, 64);
                        graphics.drawImage(this.q, 0, -43, 0);
                    }
                    if (this.c == 3) {
                        if (this.z == 2) {
                            graphics.translate(30, 0);
                        }
                        graphics.translate(7, 47);
                        graphics.setClip(0, 0, 27, 7);
                        graphics.clipRect(0, 0, 27, 7);
                        graphics.drawImage(this.q, 0, -107, 0);
                        graphics.translate(-7, -69 + (this.a >> 3));
                        graphics.setClip(0, 0, 34, 64);
                        graphics.clipRect(0, 0, 34, 64);
                        graphics.drawImage(this.q, -34, -43, 0);
                    }
                }
                if (this.z == 7) {
                    graphics.setClip(0, 0, 38, 40);
                    graphics.clipRect(0, 0, 38, 40);
                    graphics.drawImage(this.q, -70, -40 * ((this.a >> 2) % 2), 0);
                }
                if (this.z == 8) {
                    if (this.a % 2 == 1) {
                        return;
                    }
                    graphics.setClip(0, 0, 38, 40);
                    graphics.clipRect(0, 0, 38, 40);
                    graphics.drawImage(this.q, -70, -80, 0);
                }
            }
            if (this.b == 24) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.z == 0) {
                    graphics.translate(9, 38);
                    graphics.setClip(0, 0, 36, 14);
                    graphics.clipRect(0, 0, 36, 14);
                    graphics.drawImage(this.q, -1, -152, 0);
                    graphics.translate(-9, -38);
                    graphics.setClip(0, 0, 56, 48);
                    graphics.clipRect(0, 0, 56, 48);
                    graphics.drawImage(this.q, 0, 0, 0);
                }
                if (this.z == 1 || this.z == 3 || this.z == 4 || this.z == 7 || this.z == 6) {
                    graphics.translate(9, 40);
                    graphics.setClip(0, 0, 36, 14);
                    graphics.clipRect(0, 0, 36, 14);
                    graphics.drawImage(this.q, -1, -152, 0);
                    if (this.z == 1) {
                        graphics.translate(-9, -40 - (this.a >> 3));
                        graphics.setClip(0, 0, 47, 46);
                        graphics.clipRect(0, 0, 47, 46);
                        graphics.drawImage(this.q, -57, -1, 0);
                    }
                    if (this.z == 4 || this.z == 7) {
                        graphics.translate(-9, -40 - (this.a >> 3));
                        graphics.setClip(0, 0, 51, 48);
                        graphics.clipRect(0, 0, 51, 48);
                        graphics.drawImage(this.q, -105, 0, 0);
                    }
                }
                if (this.z == 2 || this.z == 5) {
                    graphics.setClip(0, 0, 43, 48);
                    graphics.clipRect(0, 0, 43, 48);
                    graphics.drawImage(this.q, -157, 0, 0);
                }
                if (this.z == 8) {
                    if (this.a % 2 == 1) {
                        return;
                    }
                    graphics.translate(9, 50);
                    graphics.setClip(0, 0, 36, 14);
                    graphics.clipRect(0, 0, 36, 14);
                    graphics.drawImage(this.q, -1, -152, 0);
                    graphics.translate(0, -10);
                    graphics.translate(-9, -50);
                    graphics.setClip(0, 0, 51, 64);
                    graphics.clipRect(0, 0, 51, 64);
                    graphics.drawImage(this.q, -114, -49, 0);
                }
                if (this.z == 9) {
                    graphics.translate(9, 50);
                    graphics.setClip(0, 0, 36, 14);
                    graphics.clipRect(0, 0, 36, 14);
                    graphics.drawImage(this.q, -1, -152, 0);
                    graphics.translate(-9, -50);
                    graphics.setClip(0, 0, 54, 57);
                    graphics.clipRect(0, 0, 54, 57);
                    graphics.drawImage(this.q, -112, -114, 0);
                }
                if (this.z == 10) {
                    graphics.translate(9, 50);
                    graphics.setClip(0, 0, 36, 14);
                    graphics.clipRect(0, 0, 36, 14);
                    graphics.drawImage(this.q, -1, -152, 0);
                    graphics.translate(-9, -50);
                    graphics.setClip(0, 0, 48, 69);
                    graphics.clipRect(0, 0, 48, 69);
                    graphics.drawImage(this.q, -167, -70, 0);
                    graphics.translate(48, 0);
                    graphics.setClip(0, 0, 28, 69);
                    graphics.clipRect(0, 0, 28, 69);
                    graphics.drawImage(this.q, -201, 0, 0);
                }
                if (this.z == 20) {
                    if (this.t % 2 == 1) {
                        graphics.setClip(0, 0, 56, 53);
                        graphics.clipRect(0, 0, 56, 53);
                        graphics.drawImage(this.q, -57, -49, 0);
                    } else {
                        graphics.setClip(0, 0, 56, 53);
                        graphics.clipRect(0, 0, 56, 53);
                        graphics.drawImage(this.q, 0, -49, 0);
                    }
                }
                if (this.z == 21) {
                    if (this.a % 3 != 1) {
                        graphics.setClip(0, 0, 55, 48);
                        graphics.clipRect(0, 0, 55, 48);
                        graphics.drawImage(this.q, -56, -103, 0);
                    } else {
                        graphics.setClip(0, 0, 55, 48);
                        graphics.clipRect(0, 0, 55, 48);
                        graphics.drawImage(this.q, 0, -103, 0);
                    }
                }
            }
            if (this.b == 25) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.translate(0, 0);
                graphics.setClip(0, 0, 38, 33);
                graphics.clipRect(0, 0, 38, 33);
                n4 = 0;
                n5 = 0;
                if (this.c == 0) {
                    n4 = 0;
                    n5 = -16;
                }
                if (this.c == 1) {
                    n4 = -16;
                    n5 = 0;
                }
                if (this.c == 2) {
                    n4 = 0;
                    n5 = 16;
                }
                if (this.c == 3) {
                    n4 = 16;
                    n5 = 0;
                }
                graphics.translate(32, 32);
                if (this.a > 3 && this.a < 10) {
                    graphics.translate(n4, n5);
                }
                graphics.setClip(0, 0, 38, 33);
                graphics.clipRect(0, 0, 38, 33);
                graphics.drawRegion(this.q, 64, 0, 38, 33, 0, 0, 0, 0);
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.a > 1 && this.a < 5) {
                    graphics.translate(n4, n5);
                }
                graphics.translate(0, 32);
                graphics.setClip(0, 0, 33, 38);
                graphics.clipRect(0, 0, 33, 38);
                graphics.drawRegion(this.q, 64, 0, 38, 33, 5, 0, 0, 0);
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.a > 7 && this.a < 12) {
                    graphics.translate(n4, n5);
                }
                graphics.translate(32, 0);
                graphics.setClip(0, 0, 38, 33);
                graphics.clipRect(0, 0, 38, 33);
                graphics.drawRegion(this.q, 64, 0, 38, 33, 6, 0, 0, 0);
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.a > 8 && this.a < 15) {
                    graphics.translate(n4, n5);
                }
                graphics.translate(0, 0);
                graphics.setClip(0, 0, 33, 38);
                graphics.clipRect(0, 0, 33, 38);
                graphics.drawRegion(this.q, 64, 0, 38, 33, 3, 0, 0, 0);
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.translate(0, 0);
                if (this.c == 0) {
                    graphics.translate(12, -16 - Math.abs(this.a - 7));
                    graphics.setClip(0, 0, 42, 46);
                    graphics.clipRect(0, 0, 42, 46);
                    graphics.drawRegion(this.q, 64, 33, 42, 46, 3, 0, 0, 0);
                }
                if (this.c == 1) {
                    graphics.translate(32 + Math.abs(this.a - 7), 12);
                    graphics.setClip(0, 0, 46, 42);
                    graphics.clipRect(0, 0, 46, 42);
                    graphics.drawRegion(this.q, 64, 33, 42, 46, 6, 0, 0, 0);
                }
                if (this.c == 2) {
                    graphics.translate(12, 32 + Math.abs(this.a - 7));
                    graphics.setClip(0, 0, 42, 46);
                    graphics.clipRect(0, 0, 42, 46);
                    graphics.drawRegion(this.q, 64, 33, 42, 46, 0, 0, 0, 0);
                }
                if (this.c == 3) {
                    graphics.translate(-32 - Math.abs(this.a - 7), 12);
                    graphics.setClip(0, 0, 46, 42);
                    graphics.clipRect(0, 0, 46, 42);
                    graphics.drawRegion(this.q, 64, 33, 42, 46, 5, 0, 0, 0);
                }
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.translate(0, 3);
                graphics.setClip(0, 0, 64, 64);
                graphics.clipRect(0, 0, 64, 64);
                if (this.c == 0) {
                    graphics.drawRegion(this.q, 0, 0, 64, 64, 3, 0, 0, 0);
                }
                if (this.c == 1) {
                    graphics.drawRegion(this.q, 0, 0, 64, 64, 6, 0, 0, 0);
                }
                if (this.c == 2) {
                    graphics.drawRegion(this.q, 0, 0, 64, 64, 0, 0, 0, 0);
                }
                if (this.c == 3) {
                    graphics.drawRegion(this.q, 0, 0, 64, 64, 5, 0, 0, 0);
                }
            }
            if (this.b == 26) {
                if (this.z == 0) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    if (this.c == 0) {
                        graphics.drawImage(this.q, -16, 0, 0);
                    }
                    if (this.c == 1) {
                        graphics.drawImage(this.q, -16, -16, 0);
                    }
                    if (this.c == 2) {
                        graphics.drawImage(this.q, -16, -16, 0);
                    }
                    if (this.c == 3) {
                        graphics.drawImage(this.q, -16, 0, 0);
                    }
                }
                if (this.z == 1) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    if (this.c == 0) {
                        graphics.drawImage(this.q, -16, 0, 0);
                    }
                    if (this.c == 1) {
                        graphics.drawImage(this.q, -16, -16, 0);
                    }
                    if (this.c == 2) {
                        graphics.drawImage(this.q, -16, -16, 0);
                    }
                    if (this.c == 3) {
                        graphics.drawImage(this.q, -16, 0, 0);
                    }
                }
                if (this.z == 2) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    if (this.c == 0) {
                        graphics.drawImage(this.q, -16 * (this.a % 2), 0, 0);
                    }
                    if (this.c == 1) {
                        graphics.drawImage(this.q, -16 * (this.a % 2), -16, 0);
                    }
                    if (this.c == 2) {
                        graphics.drawImage(this.q, -16 * (this.a % 2), -16, 0);
                    }
                    if (this.c == 3) {
                        graphics.drawImage(this.q, -16 * (this.a % 2), 0, 0);
                    }
                }
            }
            if (this.b == 27) {
                if (this.a >= 15) {
                    this.a = 0;
                }
                if (this.c == 0) {
                    if (this.a / 5 == 0) {
                        graphics.setClip(0, 0, 22, 26);
                        graphics.clipRect(0, 0, 22, 26);
                        graphics.drawImage(this.q, -55, 0, 0);
                    }
                    if (this.a / 5 == 1) {
                        graphics.setClip(0, 0, 22, 26);
                        graphics.clipRect(0, 0, 22, 26);
                        graphics.drawImage(this.q, -55, -34, 0);
                    }
                    if (this.a / 5 == 2) {
                        graphics.setClip(0, 0, 22, 26);
                        graphics.clipRect(0, 0, 22, 26);
                        graphics.drawImage(this.q, -55, -67, 0);
                    }
                }
                if (this.c == 1) {
                    if (this.a / 5 == 0) {
                        graphics.setClip(0, 0, 29, 26);
                        graphics.clipRect(0, 0, 29, 26);
                        graphics.drawRegion(this.q, 25, 0, 29, 26, 2, 0, 0, 0);
                    }
                    if (this.a / 5 == 1) {
                        graphics.setClip(0, 0, 29, 27);
                        graphics.clipRect(0, 0, 29, 27);
                        graphics.drawRegion(this.q, 25, 32, 29, 27, 2, 0, 0, 0);
                    }
                    if (this.a / 5 == 2) {
                        graphics.setClip(0, 0, 32, 27);
                        graphics.clipRect(0, 0, 32, 27);
                        graphics.drawRegion(this.q, 22, 65, 32, 27, 2, 0, 0, 0);
                    }
                }
                if (this.c == 2) {
                    if (this.a / 5 == 0) {
                        graphics.setClip(0, 0, 22, 36);
                        graphics.clipRect(0, 0, 22, 36);
                        graphics.drawImage(this.q, 0, 0, 0);
                    }
                    if (this.a / 5 == 1) {
                        graphics.setClip(0, 0, 22, 33);
                        graphics.clipRect(0, 0, 22, 33);
                        graphics.drawImage(this.q, 0, -36, 0);
                    }
                    if (this.a / 5 == 2) {
                        graphics.setClip(0, 0, 22, 34);
                        graphics.clipRect(0, 0, 22, 34);
                        graphics.drawImage(this.q, 0, -69, 0);
                    }
                }
                if (this.c == 3) {
                    if (this.a / 5 == 0) {
                        graphics.setClip(0, 0, 29, 26);
                        graphics.clipRect(0, 0, 29, 26);
                        graphics.drawRegion(this.q, 25, 0, 29, 26, 0, 0, 0, 0);
                    }
                    if (this.a / 5 == 1) {
                        graphics.setClip(0, 0, 29, 27);
                        graphics.clipRect(0, 0, 29, 27);
                        graphics.drawRegion(this.q, 25, 32, 29, 27, 0, 0, 0, 0);
                    }
                    if (this.a / 5 == 2) {
                        graphics.setClip(0, 0, 32, 27);
                        graphics.clipRect(0, 0, 32, 27);
                        graphics.drawRegion(this.q, 22, 65, 32, 27, 0, 0, 0, 0);
                    }
                }
            }
            if (this.b == 28) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.z == 1) {
                    graphics.setClip(0, 0, 32, 25);
                    graphics.clipRect(0, 0, 32, 25);
                    if ((this.a >> 1) % 2 == 0) {
                        graphics.drawRegion(this.q, 0, 0, 32, 25, 0, 0, 0, 0);
                    } else {
                        graphics.drawRegion(this.q, 0, 0, 32, 25, 2, 0, 0, 0);
                    }
                }
                if (this.z == 2) {
                    if ((this.a >> 1) % 2 == 0) {
                        graphics.setClip(0, 0, 36, 25);
                        graphics.clipRect(0, 0, 36, 25);
                        graphics.drawImage(this.q, -32, 0, 0);
                    } else {
                        graphics.setClip(0, 0, 39, 25);
                        graphics.clipRect(0, 0, 39, 25);
                        graphics.drawImage(this.q, -68, 0, 0);
                    }
                }
                if (this.z == 3) {
                    graphics.setClip(0, 0, 18, 23);
                    graphics.clipRect(0, 0, 18, 23);
                    graphics.drawImage(this.q, -41, -27, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-10, 0);
                    graphics.setClip(0, 0, 32, 24);
                    graphics.clipRect(0, 0, 32, 24);
                    graphics.drawImage(this.q, -59, -27, 0);
                }
                if (this.z == 4) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = this.F / 5;
                    for (n5 = 0; n5 < n4; ++n5) {
                        graphics.setClip(0, 0, 18, 18);
                        graphics.clipRect(0, 0, 18, 18);
                        graphics.drawImage(this.q, -20, -33, 0);
                        if (n5 == 0) {
                            graphics.translate(-10, 0);
                            graphics.setClip(0, 0, 32, 24);
                            graphics.clipRect(0, 0, 32, 24);
                            graphics.drawImage(this.q, -59, -27, 0);
                            graphics.translate(10, 0);
                            graphics.translate(0, 10 - (this.F % 5 << 1));
                        }
                        graphics.translate(0, -10);
                    }
                    graphics.translate(0, -5);
                    graphics.setClip(0, 0, 20, 21);
                    graphics.clipRect(0, 0, 20, 21);
                    graphics.drawImage(this.q, 0, -29, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-10, 0);
                    graphics.setClip(0, 0, 32, 24);
                    graphics.clipRect(0, 0, 32, 24);
                    graphics.drawImage(this.q, -59, -27, 0);
                }
                if (this.z == 5) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    this.G += this.J;
                    if (this.G > 10) {
                        this.J = -1;
                        this.G = 10;
                    }
                    if (this.G < 0) {
                        this.J = 1;
                        this.G = 0;
                    }
                    n5 = 0;
                    for (n4 = 0; n4 < 5; ++n4) {
                        graphics.setClip(0, 0, 18, 18);
                        graphics.clipRect(0, 0, 18, 18);
                        graphics.drawImage(this.q, -20, -33, 0);
                        if (n4 == 0) {
                            graphics.translate(-10, 0);
                            graphics.setClip(0, 0, 32, 24);
                            graphics.clipRect(0, 0, 32, 24);
                            graphics.drawImage(this.q, -59, -27, 0);
                            graphics.translate(10, 0);
                        }
                        int n6 = n4;
                        this.V[n6] = this.V[n6] + this.ab[n4];
                        if (this.V[n4] > 10) {
                            this.V[n4] = 10;
                            this.ab[n4] = -1;
                        }
                        if (this.V[n4] < 0) {
                            this.V[n4] = 0;
                            this.ab[n4] = 1;
                        }
                        graphics.translate(5 - this.V[n4], -10);
                    }
                    if (this.c == 0) {
                        graphics.translate(0, -5);
                        graphics.setClip(0, 0, 20, 21);
                        graphics.clipRect(0, 0, 20, 21);
                        graphics.drawRegion(this.q, 0, 29, 20, 21, 0, 0, 0, 0);
                    }
                    if (this.c == 1) {
                        graphics.translate(0, -5);
                        graphics.setClip(0, 0, 20, 21);
                        graphics.clipRect(0, 0, 20, 21);
                        graphics.drawRegion(this.q, 0, 29, 20, 21, 6, 0, 0, 0);
                    }
                    if (this.c == 2) {
                        graphics.translate(0, -5);
                        graphics.setClip(0, 0, 20, 21);
                        graphics.clipRect(0, 0, 20, 21);
                        graphics.drawRegion(this.q, 0, 29, 20, 21, 0, 0, 0, 0);
                    }
                    if (this.c == 3) {
                        graphics.translate(0, -5);
                        graphics.setClip(0, 0, 20, 21);
                        graphics.clipRect(0, 0, 20, 21);
                        graphics.drawRegion(this.q, 0, 29, 20, 21, 5, 0, 0, 0);
                    }
                }
                if (this.z == 6) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = this.F / 5;
                    for (n5 = 0; n5 < n4; ++n5) {
                        graphics.setClip(0, 0, 18, 18);
                        graphics.clipRect(0, 0, 18, 18);
                        graphics.drawImage(this.q, -20, -33, 0);
                        if (n5 == 0) {
                            graphics.translate(-10, 0);
                            graphics.setClip(0, 0, 32, 24);
                            graphics.clipRect(0, 0, 32, 24);
                            graphics.drawImage(this.q, -59, -27, 0);
                            graphics.translate(10, 0);
                            graphics.translate(0, 10 - (this.F % 5 << 1));
                        }
                        graphics.translate(0, -10);
                    }
                    graphics.translate(0, -5);
                    graphics.setClip(0, 0, 20, 21);
                    graphics.clipRect(0, 0, 20, 21);
                    graphics.drawImage(this.q, 0, -29, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-10, 0);
                    graphics.setClip(0, 0, 32, 24);
                    graphics.clipRect(0, 0, 32, 24);
                    graphics.drawImage(this.q, -59, -27, 0);
                }
            }
            if (this.b == 29) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                if (this.z == 1) {
                    graphics.setClip(0, 0, 43, 26);
                    graphics.clipRect(0, 0, 43, 26);
                    if ((this.a >> 1) % 2 == 0) {
                        graphics.drawRegion(this.q, 0, 30, 43, 26, 0, 0, 0, 0);
                    } else {
                        graphics.drawRegion(this.q, 0, 30, 43, 26, 2, 0, 0, 0);
                    }
                }
                if (this.z == 2) {
                    if ((this.a >> 1) % 2 == 0) {
                        graphics.setClip(0, 0, 50, 30);
                        graphics.clipRect(0, 0, 50, 30);
                        graphics.drawImage(this.q, 0, -27, 0);
                    } else {
                        graphics.setClip(0, 0, 50, 30);
                        graphics.clipRect(0, 0, 50, 30);
                        graphics.drawImage(this.q, -50, -27, 0);
                    }
                }
                if (this.z == 3) {
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-6, 20);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                }
                if (this.z == 4) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = this.F / 5;
                    for (n5 = 0; n5 < n4; ++n5) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        if (n5 == 0) {
                            graphics.translate(0, 10 - (this.F % 5 << 1));
                        }
                        graphics.translate(0, -10);
                    }
                    graphics.translate(-6, -5);
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(6, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-12, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                }
                if (this.z == 5) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    this.G += this.J;
                    if (this.G > 10) {
                        this.J = -1;
                        this.G = 10;
                    }
                    if (this.G < 0) {
                        this.J = 1;
                        this.G = 0;
                    }
                    n5 = 0;
                    for (n4 = 0; n4 < 5; ++n4) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        int n7 = n4;
                        this.V[n7] = this.V[n7] + this.ab[n4];
                        if (this.V[n4] > 10) {
                            this.V[n4] = 10;
                            this.ab[n4] = -1;
                        }
                        if (this.V[n4] < 0) {
                            this.V[n4] = 0;
                            this.ab[n4] = 1;
                        }
                        graphics.translate(5 - this.V[n4], -10);
                    }
                    graphics.translate(-6, -5);
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(6, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-12, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    if (this.a >> 3 == 0) {
                        graphics.drawImage(this.q, 0, 0, 0);
                    }
                    if (this.a >> 3 == 1) {
                        graphics.drawImage(this.q, -53, 0, 0);
                    }
                    if (this.a >> 3 == 2) {
                        graphics.drawImage(this.q, -106, 0, 0);
                    }
                }
                if (this.z == 6) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = this.F / 5;
                    for (n5 = 0; n5 < n4; ++n5) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        if (n5 == 0) {
                            graphics.translate(0, 10 - (this.F % 5 << 1));
                        }
                        graphics.translate(0, -10);
                    }
                    graphics.translate(-6, -5);
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(6, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-6, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                }
                if (this.z == 7) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    for (n5 = 0; n5 < 4; ++n5) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        if (n5 == 0) {
                            graphics.translate(0, 2);
                        }
                        graphics.translate(0, -10);
                    }
                    graphics.translate(-6, -5);
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(6, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-6, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                }
                if (this.z == 8) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    for (n5 = 0; n5 < 4; ++n5) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        if (n5 == 0) {
                            graphics.translate(0, 2);
                        }
                        graphics.translate(0, -10);
                    }
                    graphics.translate(-6, -5);
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(6, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-6, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    if (this.a % 8 > 4) {
                        graphics.translate(32, this.F / 6 - 32);
                        graphics.setClip(0, 0, 20, 20);
                        graphics.clipRect(0, 0, 20, 20);
                        graphics.drawImage(this.q, -80, -62, 0);
                    } else {
                        graphics.translate(32, this.F / 6 - 32);
                        graphics.setClip(0, 0, 22, 24);
                        graphics.clipRect(0, 0, 22, 24);
                        graphics.drawImage(this.q, -80, -82, 0);
                    }
                    if (this.a % 2 == 0) {
                        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                        graphics.translate(10, 10);
                        graphics.setClip(0, 0, 44, 68);
                        graphics.clipRect(0, 0, 44, 68);
                        graphics.drawImage(this.q, -159, 0, 0);
                    }
                }
                if (this.z == 9) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-6, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    for (n5 = 0; n5 < 4; ++n5) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        if (n5 == 0) {
                            graphics.translate(0, 2);
                        }
                        graphics.translate(0 - this.F / 4, -10 + this.F / 2);
                    }
                    graphics.translate(-6, -5);
                    graphics.setClip(0, 0, 44, 48);
                    graphics.clipRect(0, 0, 44, 48);
                    graphics.drawImage(this.q, -33, -58, 0);
                    graphics.translate(6, 0);
                }
                if (this.z == 10) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.translate(-6, 16);
                    graphics.setClip(0, 0, 53, 26);
                    graphics.clipRect(0, 0, 53, 26);
                    graphics.drawImage(this.q, 0, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    n4 = 0;
                    for (n5 = 0; n5 < 4; ++n5) {
                        graphics.setClip(0, 0, 32, 35);
                        graphics.clipRect(0, 0, 32, 35);
                        graphics.drawImage(this.q, 0, -66, 0);
                        if (n5 == 0) {
                            graphics.translate(0, 2);
                        }
                        graphics.translate(-8, 6);
                    }
                    if (this.t % 3 == 1) {
                        graphics.translate(-6, -5);
                        graphics.setClip(0, 0, 44, 48);
                        graphics.clipRect(0, 0, 44, 48);
                        graphics.drawImage(this.q, -104, -59, 0);
                        graphics.translate(6, 0);
                    } else {
                        graphics.translate(-6, -5);
                        graphics.setClip(0, 0, 44, 48);
                        graphics.clipRect(0, 0, 44, 48);
                        graphics.drawImage(this.q, -33, -58, 0);
                        graphics.translate(6, 0);
                    }
                }
            }
            if (this.b == 30 || this.b == 31) {
                if (this.z == 0 || this.z == 1 || this.z == 2) {
                    if (this.z == 2 && this.a % 2 == 1) {
                        return;
                    }
                    if (this.c == 0) {
                        graphics.setClip(0, 0, 18, 27);
                        graphics.clipRect(0, 0, 18, 27);
                        graphics.drawImage(this.q, -18 * ((this.a >> 3) % 2), -54, 0);
                    }
                    if (this.c == 1) {
                        graphics.setClip(0, 0, 18, 27);
                        graphics.clipRect(0, 0, 18, 27);
                        graphics.drawImage(this.q, -18 * ((this.a >> 3) % 2), -27, 0);
                    }
                    if (this.c == 2) {
                        graphics.setClip(0, 0, 18, 27);
                        graphics.clipRect(0, 0, 18, 27);
                        graphics.drawImage(this.q, -18 * ((this.a >> 3) % 2), 0, 0);
                    }
                    if (this.c == 3) {
                        graphics.setClip(0, 0, 18, 27);
                        graphics.clipRect(0, 0, 18, 27);
                        graphics.drawRegion(this.q, 18, 27, 18, 27, 2, 0, 0, 0);
                    }
                    graphics.translate(4, 28);
                    graphics.setClip(0, 0, 12, 8);
                    graphics.clipRect(0, 0, 12, 8);
                    graphics.drawImage(this.q, 0, -148, 0);
                }
                if (this.z == 4) {
                    graphics.setClip(0, 0, 22, 25);
                    graphics.clipRect(0, 0, 22, 25);
                    graphics.drawImage(this.q, -22 * ((this.a >> 3) % 2), -105, 0);
                    graphics.setClip(0, 0, 22, 13);
                    graphics.clipRect(0, 0, 22, 13);
                    graphics.drawImage(this.q, -22 * ((this.a >> 2) % 2), -130, 0);
                    this.a = (byte)(this.a + 1);
                    if (this.a > 15) {
                        this.a = 0;
                    }
                }
            }
            if (this.b == 32) {
                if (this.z == 1) {
                    graphics.setClip(0, 0, 16, 19);
                    graphics.clipRect(0, 0, 16, 19);
                    graphics.drawImage(this.q, -16 * ((this.a >> 1) % 2), 0, 0);
                }
                if (this.z == 2) {
                    graphics.setClip(0, 0, 16, 19);
                    graphics.clipRect(0, 0, 16, 19);
                    graphics.drawImage(this.q, -16 * (this.a >> 2) - 32, 0, 0);
                    return;
                }
                if (this.z == 3) {
                    graphics.setClip(0, 0, 16, 19);
                    graphics.clipRect(0, 0, 16, 19);
                    graphics.drawImage(this.q, -16 * (this.a >> 2) - 80, 0, 0);
                    return;
                }
                if (this.z == 4) {
                    graphics.setClip(0, 0, 16, 19);
                    graphics.clipRect(0, 0, 16, 19);
                    graphics.drawImage(this.q, 16 * (this.a >> 2) - 80, 0, 0);
                    return;
                }
            }
            if (this.b == 33) {
                if (this.z == 1) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -16 * ((this.a >> 1) % 2), 0, 0);
                }
                if (this.z == 2) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -32, 0, 0);
                    return;
                }
                if (this.z == 3) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -48, 0, 0);
                    return;
                }
                if (this.z == 4) {
                    graphics.setClip(0, 0, 16, 16);
                    graphics.clipRect(0, 0, 16, 16);
                    graphics.drawImage(this.q, -16 * ((this.a >> 1) % 2), 0, 0);
                    return;
                }
            }
            if (this.b == 34) {
                graphics.translate(0, 0);
                graphics.setClip(0, 0, 24, 32);
                graphics.clipRect(0, 0, 24, 32);
                graphics.drawImage(this.q, -24 * (this.a / 3), 0, 0);
                if (this.a > 7) {
                    this.a = (byte)-1;
                }
            }
            if (this.b == 35) {
                if (this.a < 4) {
                    graphics.setClip(0, 0, 44, 36);
                    graphics.clipRect(0, 0, 44, 36);
                    graphics.drawImage(this.q, 0, this.c * -36, 0);
                    return;
                }
                if (this.a < 8) {
                    if (this.c == 1) {
                        graphics.translate(-4, 0);
                    }
                    if (this.c == 2) {
                        graphics.translate(0, -1);
                    }
                    if (this.c == 3) {
                        graphics.translate(4, 0);
                    }
                    graphics.setClip(0, 0, 44, 36);
                    graphics.clipRect(0, 0, 44, 36);
                    graphics.drawImage(this.q, -44, this.c * -36, 0);
                    return;
                }
                if (this.a < 12) {
                    graphics.setClip(0, 0, 44, 36);
                    graphics.clipRect(0, 0, 44, 36);
                    graphics.drawImage(this.q, 0, this.c * -36, 0);
                    return;
                }
                if (this.a <= 16) {
                    if (this.c == 1) {
                        graphics.translate(6, 0);
                    }
                    if (this.c == 2) {
                        graphics.translate(0, 2);
                    }
                    if (this.c == 3) {
                        graphics.translate(-6, 0);
                    }
                    graphics.setClip(0, 0, 44, 36);
                    graphics.clipRect(0, 0, 44, 36);
                    graphics.drawImage(this.q, -88, this.c * -36, 0);
                    return;
                }
            }
            if (this.b == 36) {
                if (this.z == 1) {
                    graphics.setClip(0, 0, 23, 30);
                    graphics.clipRect(0, 0, 23, 30);
                    graphics.drawImage(this.q, -23 * (this.a % 3), 0, 0);
                    --this.A;
                    if (this.A < 0) {
                        this.N = 0;
                        this.f = true;
                        this.n = true;
                    }
                }
                if (this.z == 0) {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.X - n2, this.s - n3);
                    graphics.setClip(0, 0, 23, 30);
                    graphics.clipRect(0, 0, 23, 30);
                    graphics.drawImage(this.q, -69, 0, 0);
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.i - n2, this.j - n3);
                    graphics.setClip(0, 0, 23, 30);
                    graphics.clipRect(0, 0, 23, 30);
                    graphics.drawImage(this.q, -23 * (this.a % 3), 0, 0);
                }
            }
            if (this.b == 37) {
                if (this.z == 0 || this.z == 8) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    if (this.a > 6) {
                        graphics.translate(-3, -32);
                        graphics.setClip(0, 0, 38, 38);
                        graphics.clipRect(0, 0, 38, 38);
                        graphics.drawImage(this.q, 0, 0, 0);
                    } else {
                        graphics.translate(-3, -32);
                        graphics.setClip(0, 0, 38, 38);
                        graphics.clipRect(0, 0, 38, 38);
                        graphics.drawImage(this.q, -38, 0, 0);
                    }
                }
                if (this.z == 1) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    if (this.a < 5) {
                        graphics.translate(-3, -32);
                        graphics.setClip(0, 0, 38, 38);
                        graphics.clipRect(0, 0, 38, 38);
                        graphics.drawImage(this.q, 0, 0, 0);
                    } else if (this.a < 7) {
                        graphics.translate(-3, -32);
                        graphics.setClip(0, 0, 38, 38);
                        graphics.clipRect(0, 0, 38, 38);
                        graphics.drawImage(this.q, -38, 0, 0);
                    } else {
                        graphics.translate(-9, -23);
                        graphics.setClip(0, 0, 44, 29);
                        graphics.clipRect(0, 0, 44, 29);
                        graphics.drawImage(this.q, 0, -57, 0);
                    }
                }
                if (this.z == 2) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    graphics.translate(-9, -23);
                    graphics.setClip(0, 0, 44, 29);
                    graphics.clipRect(0, 0, 44, 29);
                    graphics.drawImage(this.q, 0, -57, 0);
                }
                if (this.z == 3) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    graphics.translate(0, -39);
                    graphics.setClip(0, 0, 32, 45);
                    graphics.clipRect(0, 0, 32, 45);
                    graphics.drawImage(this.q, -44, -40, 0);
                }
                if (this.z == 4) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    graphics.translate(0, -39 - this.F * 25);
                    graphics.setClip(0, 0, 32, 45);
                    graphics.clipRect(0, 0, 32, 45);
                    graphics.drawImage(this.q, -44, -40, 0);
                }
                if (this.z == 5) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                }
                if (this.z == 6) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    if (this.a % 4 > 1) {
                        graphics.translate(-7, -23);
                        graphics.setClip(0, 0, 44, 29);
                        graphics.clipRect(0, 0, 44, 29);
                        graphics.drawImage(this.q, 0, -57, 0);
                    } else {
                        graphics.translate(-3, -32);
                        graphics.setClip(0, 0, 38, 38);
                        graphics.clipRect(0, 0, 38, 38);
                        graphics.drawImage(this.q, 0, 0, 0);
                    }
                }
                if (this.z == 7) {
                    graphics.translate(3, 32);
                    graphics.setClip(0, 0, 32, 9);
                    graphics.clipRect(0, 0, 32, 9);
                    graphics.drawImage(this.q, 0, -45, 0);
                    if (this.a % 4 > 1) {
                        graphics.translate(-7, -23);
                        graphics.setClip(0, 0, 46, 35);
                        graphics.clipRect(0, 0, 46, 35);
                        graphics.drawImage(this.q, 0, -87, 0);
                    } else {
                        graphics.translate(-7, -23);
                        graphics.setClip(0, 0, 46, 35);
                        graphics.clipRect(0, 0, 46, 35);
                        graphics.drawImage(this.q, -45, -87, 0);
                    }
                }
            }
            if (this.b == 38) {
                if (this.z == 0 || this.z == 1) {
                    graphics.setClip(0, 0, 48, 16);
                    graphics.clipRect(0, 0, 48, 16);
                    if (this.a % 5 >= 3) {
                        if (this.c == 0) {
                            graphics.drawImage(this.q, 0, 0, 0);
                        } else {
                            graphics.drawImage(this.q, 0, -16, 0);
                        }
                    } else {
                        graphics.drawImage(this.q, 0, -32, 0);
                    }
                    if (this.z == 1) {
                        graphics.translate(16, 0);
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        graphics.drawImage(this.q, 0, -48, 0);
                    }
                }
                if (this.z == 2) {
                    graphics.setClip(0, 0, 48, 16);
                    graphics.clipRect(0, 0, 48, 16);
                    graphics.drawImage(this.q, 0, -32, 0);
                    if (this.a % 5 > 2) {
                        graphics.translate(16, 0);
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        graphics.drawImage(this.q, 0, -48, 0);
                    } else {
                        graphics.translate(16, 0);
                        graphics.setClip(0, 0, 16, 16);
                        graphics.clipRect(0, 0, 16, 16);
                        graphics.drawImage(this.q, -16, -48, 0);
                    }
                }
            }
            if (this.b == 128) {
                graphics.translate(0, -6);
                graphics.setClip(0, 0, 16, 24);
                graphics.clipRect(0, 0, 16, 24);
                graphics.drawImage(this.q, 0, 0, 0);
            }
            if (this.b == 129) {
                graphics.translate(0, -6);
                graphics.setClip(0, 0, 16, 24);
                graphics.clipRect(0, 0, 16, 24);
                graphics.drawImage(this.q, -16, 0, 0);
            }
            if (this.b == 130) {
                graphics.translate(0, -6);
                graphics.setClip(0, 0, 16, 24);
                graphics.clipRect(0, 0, 16, 24);
                graphics.drawImage(this.q, -32, 0, 0);
            }
            if (this.b == 131) {
                graphics.drawImage(this.q, -16 * (this.a >> 3), -32, 0);
            }
            if (this.b == 132) {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 46, 32);
                graphics.clipRect(0, 0, 46, 32);
                graphics.drawImage(this.q, 0, 0, 0);
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 256, 176);
                graphics.clipRect(0, 0, 256, 176);
                graphics.setColor(0, 0, 0);
                graphics.drawLine(this.X - n2, this.s - n3, this.i - n2 + 45, this.j - n3 + 6);
                if (this.r == null) {
                    try {
                        this.r = Image.createImage("/Images/RingsOnWater.png");
                    }
                    catch (Exception exception) {
                        System.out.println("error?!?");
                    }
                } else {
                    graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                    graphics.translate(this.X - n2, this.s - n3);
                    graphics.translate(0, -16);
                    if (Math.abs(this.U.nextInt() % 2) == 1) {
                        graphics.translate(-7, -4);
                        graphics.setClip(0, 0, 14, 8);
                        graphics.clipRect(0, 0, 14, 8);
                        graphics.drawImage(this.r, 0, 0, 0);
                    } else {
                        graphics.translate(-6, -4);
                        graphics.setClip(0, 0, 12, 8);
                        graphics.clipRect(0, 0, 12, 8);
                        graphics.drawImage(this.r, -14, 0, 0);
                    }
                }
                this.X += this.F * Math.abs(this.U.nextInt() % 3);
                if (this.X < 150) {
                    this.F = 1;
                }
                if (this.X > 250) {
                    this.F = -1;
                }
                this.s += this.G * Math.abs(this.U.nextInt() % 2);
                if (this.s < 200) {
                    this.G = 1;
                }
                if (this.s > 230) {
                    this.G = -1;
                }
            }
            if (this.b == 133) {
                graphics.setClip(0, 0, 26, 28);
                graphics.clipRect(0, 0, 26, 28);
                graphics.drawImage(this.q, -26 * (this.a >> 1), -28 * this.c, 0);
                if (this.a > 10) {
                    this.a = 0;
                }
            }
            if (this.b == 134) {
                graphics.setClip(0, 0, 25, 33);
                graphics.clipRect(0, 0, 25, 33);
                graphics.drawImage(this.q, -25 * (this.a >> 1), 0, 0);
                if (this.a > 10) {
                    this.a = 0;
                }
            }
            if (this.b == 135) {
                n4 = 0;
                graphics.setClip(0, 0, 22, 26);
                graphics.clipRect(0, 0, 22, 26);
                n4 = this.z;
                if (this.z == 2) {
                    n4 = 0;
                }
                if (this.z == 3) {
                    n4 = 2;
                }
                graphics.drawImage(this.q, n4 * -22, 0, 0);
                return;
            }
        } else {
            if (this.b == 16) {
                graphics.translate(16, 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
            }
            graphics.drawImage(this.q, -16 * (this.e >> 1), 0, 0);
        }
    }

    private void a() {
        this.a = (byte)(this.a + 1);
        if (this.a > 15) {
            this.a = 0;
        }
        if (this.t > 0) {
            --this.t;
        }
        if (this.u > 0) {
            --this.u;
        }
    }

    final void a(Graphics graphics, int n2, int n3, h h2) {
        int n4 = this.j - 32;
        int n5 = this.i + 8;
        if (this.W == null) {
            try {
                System.out.println("Loading effects");
                this.W = Image.createImage("/Images/WindEffect.png");
                this.aa.removeAllElements();
                for (int i2 = 0; i2 < 100; ++i2) {
                    d d2 = new d();
                    new d().a = this.i + this.U.nextInt() % 50 * (1 - this.U.nextInt() % 3);
                    d2.b = this.j + this.U.nextInt() % 100;
                    d2.c = 5;
                    d2.f = ((float)n5 - (float)d2.a) / ((float)d2.b - (float)n4);
                    d2.d = d2.a;
                    d2.e = d2.b;
                    this.aa.addElement(d2);
                }
                System.out.println("setting up pointers");
            }
            catch (IOException iOException) {
                this.W = null;
                System.out.println("Loading failed");
            }
        } else {
            for (int i3 = 0; i3 < 100; ++i3) {
                d d3 = (d)this.aa.elementAt(i3);
                if (d3.b <= n4 && d3.c <= 0) {
                    d3.a = this.i + this.U.nextInt() % 50 * (1 - this.U.nextInt() % 3);
                    d3.b = this.j + 100;
                    d3.f = ((float)n5 - (float)d3.a) / ((float)d3.b - (float)n4);
                    d3.d = d3.a;
                    d3.e = d3.b;
                    d3.c = 6;
                }
                d3.a = (int)d3.d;
                d3.b = (int)d3.e;
                if (d3.b > n4) {
                    d3.d += d3.f * 7.0f;
                    d3.e -= 8.0f;
                } else {
                    d3.c = 0;
                }
                if (d3.b - n3 + 32 + 8 > 208 || d3.b - n3 + 32 + 8 < 0 || d3.a - n2 + 8 > 256 || d3.a - n2 + 8 < 0) continue;
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(d3.a - n2, d3.b - n3 + 32);
                graphics.setClip(0, 0, 8, 8);
                graphics.clipRect(0, 0, 8, 8);
                if (d3.f > 0.0f) {
                    graphics.drawImage(this.W, 0, 0, 0);
                }
                if (d3.f < 0.0f) {
                    graphics.drawImage(this.W, -16, 0, 0);
                }
                if (d3.f != 0.0f) continue;
                graphics.drawImage(this.W, -8, 0, 0);
            }
        }
        if (h2.j >= n4 + 32) {
            if (h2.j > n4 + 32) {
                h2.j -= 2;
            }
            if (h2.i > n5) {
                --h2.i;
            }
            if (h2.i < n5) {
                ++h2.i;
            }
        }
        if (h2.j < n4 + 32 + 4 && h2.j > n4 + 32 - 4 && h2.i < n5 + 10 && h2.i > n5 - 10) {
            h2.ag = true;
            this.z = 8;
            this.F = 0;
        }
    }
}

