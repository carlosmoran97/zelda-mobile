/*
 * Decompiled with CFR 0.152.
 */
import java.io.IOException;
import java.util.Random;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public final class c
extends f {
    private byte n;
    public int a;
    public int b;
    int c;
    private Image o;
    Image d;
    boolean e;
    boolean f = true;
    int g;
    private Random p;
    boolean h = false;
    private h q;

    public c(Image image, int n2, int n3, int n4, int n5, h h2) {
        this.o = image;
        this.q = h2;
        this.i = n3;
        this.j = n4 + 32;
        this.c = n5;
        this.a = n2;
        this.k = 8;
        this.l = 8;
        this.m = 0;
        this.e = false;
        this.p = new Random();
        this.n = (byte)(System.currentTimeMillis() % 31L);
        if (this.a == 4 && n5 == 1) {
            this.e = true;
            this.n = 0;
            this.b = 50;
            this.k = 8;
            this.l = 8;
        }
        if (this.a == 5) {
            this.e = false;
            this.n = 0;
            this.k = 8;
            this.l = 8;
        }
        if (this.a <= 3) {
            this.k = 4;
            this.l = 6;
            this.b = 100;
        }
        if (this.a >= 128) {
            this.k = 8;
            this.l = 8;
        }
        if (this.a == 145) {
            this.i += 9;
            this.j -= 24;
        }
    }

    public final int a(h h2, b b2, k k2, boolean bl) {
        Object object;
        ((c)object).q = h2;
        if (!((c)object).f) {
            return 0;
        }
        if (((c)object).h) {
            return 0;
        }
        switch (((c)object).a) {
            case 0: {
                if (h2.J == h2.K) {
                    return 1;
                }
                ++h2.J;
                int n2 = 2;
                object = b2;
                b2.d = n2;
                if (h2.J > h2.K) {
                    h2.J = h2.K;
                }
                return 1;
            }
            case 1: {
                if (h2.J == h2.K) {
                    return 1;
                }
                h2.J += 5;
                int n3 = 2;
                object = b2;
                b2.d = n3;
                if (h2.J > h2.K) {
                    h2.J = h2.K;
                }
                return 1;
            }
            case 2: {
                if (h2.J == h2.K) {
                    return 1;
                }
                h2.J += 10;
                int n4 = 2;
                object = b2;
                b2.d = n4;
                if (h2.J > h2.K) {
                    h2.J = h2.K;
                }
                return 1;
            }
            case 3: {
                if (h2.H == h2.I / 4 << 2) {
                    return 1;
                }
                h2.H += 4;
                int n5 = 3;
                object = b2;
                b2.d = n5;
                if (h2.H > h2.I / 4 << 2) {
                    h2.H = h2.I / 4 << 2;
                }
                return 1;
            }
            case 4: {
                if (((c)object).e) {
                    return 0;
                }
                if (h2.R == h2.Q) {
                    return 1;
                }
                h2.Q = (byte)(h2.Q + 1);
                if (h2.Q > h2.R) {
                    h2.Q = h2.R;
                }
                return 1;
            }
            case 5: {
                if (((c)object).e) {
                    return 0;
                }
                if (h2.T == h2.S) {
                    return 1;
                }
                h2.S = (byte)(h2.S + 5);
                if (h2.S > h2.T) {
                    h2.S = h2.T;
                }
                return 1;
            }
            case 128: {
                if ((h2.ak & 1 << ((c)object).c) <= 0) {
                    if (((c)object).c > 0) {
                        h2.ak |= 1 << ((c)object).c;
                    }
                    ++h2.I;
                    if (h2.I > 80) {
                        h2.I = 80;
                    }
                    h2.H = h2.I / 4 << 2;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = h2.I % 4;
                    int n6 = 5;
                    object = b2;
                    b2.d = n6;
                    return 2;
                }
                return 0;
            }
            case 129: {
                if ((h2.ak & 1 << ((c)object).c) <= 0) {
                    if (((c)object).c > 0) {
                        h2.ak |= 1 << ((c)object).c;
                    }
                    h2.I += 4;
                    if (h2.I > 80) {
                        h2.I = 80;
                    }
                    h2.H = h2.I / 4 << 2;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 24;
                    int n7 = 5;
                    object = b2;
                    b2.d = n7;
                    return 2;
                }
                return 0;
            }
            case 130: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.q = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 4;
                    int n8 = 5;
                    object = b2;
                    b2.d = n8;
                    return 2;
                }
                return 0;
            }
            case 131: {
                if ((h2.al & 1 << ((c)object).c) <= 0) {
                    h2.al |= 1 << ((c)object).c;
                    ++h2.N;
                    if (h2.N > 99) {
                        h2.N = 99;
                    }
                    return 1;
                }
                return 0;
            }
            case 132: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.r = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 7;
                    int n9 = 5;
                    object = b2;
                    b2.d = n9;
                    return 2;
                }
                return 0;
            }
            case 133: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.t = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 5;
                    int n10 = 5;
                    object = b2;
                    b2.d = n10;
                    h2.T = (byte)30;
                    h2.S = (byte)30;
                    return 2;
                }
                return 0;
            }
            case 134: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.u = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 14;
                    int n11 = 5;
                    object = b2;
                    b2.d = n11;
                    return 2;
                }
                return 0;
            }
            case 135: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    if (h2.J < 255) {
                        return 0;
                    }
                    h2.J -= 255;
                    h2.aj |= 1 << ((c)object).c;
                    if (h2.V > 1) {
                        return 1;
                    }
                    h2.V = 1;
                    h2.M = 2;
                    h2.a = null;
                    try {
                        h2.a = null;
                        h2.a = Image.createImage("/Images/LinkWalk_Blue.png");
                        h2.M = 2;
                        System.gc();
                        return 1;
                    }
                    catch (IOException iOException) {}
                }
                return 0;
            }
            case 136: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.s = true;
                    h2.R = (byte)10;
                    h2.Q = (byte)10;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 11;
                    int n12 = 5;
                    object = b2;
                    b2.d = n12;
                    return 2;
                }
                return 0;
            }
            case 137: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.K = 500;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 6;
                    int n13 = 5;
                    object = b2;
                    b2.d = n13;
                    return 2;
                }
                return 0;
            }
            case 138: {
                if (!h2.v) {
                    if (h2.J < 50) {
                        return 0;
                    }
                    h2.J -= 50;
                    h2.v = true;
                    return 1;
                }
                return 0;
            }
            case 139: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.w = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 13;
                    int n14 = 5;
                    object = b2;
                    b2.d = n14;
                    return 2;
                }
                return 0;
            }
            case 140: {
                if (h2.I >= 20) {
                    h2.aj |= 1 << ((c)object).c;
                    try {
                        if (h2.U > 1) {
                            return 1;
                        }
                        h2.b = null;
                        h2.U = 1;
                        System.gc();
                        h2.b = Image.createImage("/Images/Sword02.png");
                        h2.W = (byte)2;
                        return 1;
                    }
                    catch (IOException iOException) {
                        return 1;
                    }
                }
                return 0;
            }
            case 141: {
                h2.aj |= 1 << ((c)object).c;
                return 1;
            }
            case 142: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.x = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 12;
                    int n15 = 5;
                    object = b2;
                    b2.d = n15;
                    return 2;
                }
                return 0;
            }
            case 143: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.C = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 15;
                    int n16 = 5;
                    object = b2;
                    b2.d = n16;
                    return 2;
                }
                return 0;
            }
            case 144: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    int n2 = h2.a();
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = n2 + 15;
                    int n17 = 5;
                    object = b2;
                    b2.d = n17;
                    return 2;
                }
                return 0;
            }
            case 145: {
                if (h2.I >= 40) {
                    h2.aj |= 1 << ((c)object).c;
                    try {
                        h2.b = null;
                        h2.U = (byte)2;
                        System.gc();
                        h2.b = Image.createImage("/Images/Sword03.png");
                        h2.W = (byte)4;
                        return 1;
                    }
                    catch (IOException iOException) {
                        return 1;
                    }
                }
                return 0;
            }
            case 146: {
                if (h2.J < 99) {
                    return 0;
                }
                h2.J -= 99;
                ++h2.N;
                if (h2.N > 99) {
                    h2.N = 99;
                }
                return 1;
            }
            case 147: {
                if ((h2.am & 1 << ((c)object).c) <= 0) {
                    h2.J += 50;
                    if (((c)object).c > 0) {
                        h2.am |= 1 << ((c)object).c;
                    }
                    int n18 = 2;
                    object = b2;
                    b2.d = n18;
                    if (h2.J > h2.K) {
                        h2.J = h2.K;
                    }
                    return 1;
                }
                return 0;
            }
            case 148: {
                if ((h2.am & 1 << ((c)object).c) <= 0) {
                    h2.J += 100;
                    if (((c)object).c > 0) {
                        h2.am |= 1 << ((c)object).c;
                    }
                    int n19 = 2;
                    object = b2;
                    b2.d = n19;
                    if (h2.J > h2.K) {
                        h2.J = h2.K;
                    }
                    return 1;
                }
                return 0;
            }
            case 149: {
                if ((h2.am & 1 << ((c)object).c) <= 0) {
                    h2.J += 200;
                    if (((c)object).c > 0) {
                        h2.am |= 1 << ((c)object).c;
                    }
                    int n20 = 2;
                    object = b2;
                    b2.d = n20;
                    if (h2.J > h2.K) {
                        h2.J = h2.K;
                    }
                    return 1;
                }
                return 0;
            }
            case 150: {
                if (h2.H == h2.I / 4 << 2) {
                    return 0;
                }
                if (h2.J < 10) {
                    return 0;
                }
                h2.J -= 10;
                h2.H += 12;
                int n21 = 3;
                object = b2;
                b2.d = n21;
                if (h2.H > h2.I / 4 << 2) {
                    h2.H = h2.I / 4 << 2;
                }
                return 1;
            }
            case 151: {
                if (!h2.s) {
                    return 0;
                }
                if (h2.R == h2.Q) {
                    return 0;
                }
                if (h2.J < 25) {
                    return 0;
                }
                h2.J -= 25;
                h2.Q = (byte)(h2.Q + 10);
                if (h2.Q > h2.R) {
                    h2.Q = h2.R;
                }
                return 1;
            }
            case 152: {
                if (!h2.t) {
                    return 0;
                }
                if (h2.T == h2.S) {
                    return 0;
                }
                if (h2.J < 15) {
                    return 0;
                }
                h2.S = (byte)(h2.S + 10);
                h2.J -= 15;
                if (h2.S > h2.T) {
                    h2.S = h2.T;
                }
                return 1;
            }
            case 153: {
                if ((h2.aj & 1 << ((c)object).c) <= 0 && bl) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.g = false;
                    h2.y = true;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 8;
                    int n22 = 5;
                    object = b2;
                    b2.d = n22;
                    return 2;
                }
                return 0;
            }
            case 154: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.g = false;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 10;
                    int n23 = 5;
                    object = b2;
                    b2.d = n23;
                    return 2;
                }
            }
            case 155: {
                if ((h2.aj & 1 << ((c)object).c) <= 0) {
                    h2.aj |= 1 << ((c)object).c;
                    h2.g = false;
                    ((c)object).h = true;
                    h2.ad = 25;
                    h2.E = true;
                    k2.j = true;
                    k2.k = 26;
                    int n24 = 5;
                    object = b2;
                    b2.d = n24;
                    return 2;
                }
                return 0;
            }
        }
        return 0;
    }

    final void a(Graphics graphics, int n2, int n3, int n4, int n5) {
        if (n4 + n2 < this.i) {
            return;
        }
        if (this.i + this.k + this.k < n2) {
            return;
        }
        if (n5 + n3 < this.j) {
            return;
        }
        if (this.j + this.l + this.l - 32 < n3) {
            return;
        }
        if (!this.f) {
            return;
        }
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(this.i - n2, this.j - n3);
        if (this.a < 3) {
            graphics.setClip(0, 0, 8, 15);
            graphics.clipRect(0, 0, 8, 15);
            n4 = 0;
            if (this.n > 24) {
                ++n4;
            }
            if (this.n > 28) {
                ++n4;
            }
            graphics.drawImage(this.o, n4 * -8, -15 * this.a, 0);
        }
        if (this.a == 3) {
            graphics.setClip(0, 0, 8, 15);
            graphics.clipRect(0, 0, 8, 15);
            graphics.drawImage(this.o, 0, -45, 0);
        }
        if (this.a == 4) {
            if (!this.e) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
                return;
            }
            if (this.g == 0) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -16 * (this.n % 2), 0, 0);
            } else {
                n4 = this.p.nextInt() % 5;
                graphics.translate(0, n4 + -12);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
                graphics.translate(n4 + -12, 12 - n4);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
                graphics.translate(12 - n4, n4 + 12);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
                graphics.translate(n4 + 12, -12 - n4);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.i - n2, this.j - n3);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
            }
        }
        if (this.a == 5) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, 0, 0, 0);
        }
        if (this.a == 128) {
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            } else if (!this.h) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
        }
        if (this.a == 129) {
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -16, 0, 0);
            } else if (!this.h) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -16, 0, 0);
            }
        }
        if (this.a == 130) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
            }
        }
        if (this.a == 131 && !this.h) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, 0, 0, 0);
        }
        if (this.a == 132) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -16, 0, 0);
            }
        }
        if (this.a == 133) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
            }
        }
        if (this.a == 134) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
        }
        if (this.a == 135) {
            if (this.h) {
                return;
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, -16, 0, 0);
            graphics.translate(0, 16);
            graphics.setColor(0xFFFFFF);
            graphics.setClip(0, 0, 50, 50);
            graphics.clipRect(0, 0, 50, 50);
            graphics.drawString("255:-", 0, 0, 20);
        }
        if (this.a == 136) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
            }
        }
        if (this.a == 137) {
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 3, this.q.j - n3 - 18);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            } else if (!this.h) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
        }
        if (this.a == 138) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, -16, 0, 0);
            graphics.translate(0, 16);
            graphics.setColor(0xFFFFFF);
            graphics.setClip(0, 0, 50, 50);
            graphics.clipRect(0, 0, 50, 50);
            graphics.drawString("50:-", 0, 0, 20);
        }
        if (this.a == 139) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
            }
        }
        if (this.a == 140) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, 0, 0, 0);
        }
        if (this.a == 141) {
            if (this.h) {
                return;
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, -16, 0, 0);
        }
        if (this.a == 142) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -32, 0, 0);
            }
        }
        if (this.a == 143) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
        }
        if (this.a == 144) {
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 21);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
            if (!this.h) {
                graphics.translate(0, -16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -16 * (this.n / 3), 0, 0);
                this.n = (byte)(this.n + 1);
                if (this.n > 25) {
                    this.n = 0;
                }
                graphics.translate(0, 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -144, 0, 0);
            }
        }
        if (this.a == 145) {
            graphics.setClip(0, 0, 12, 22);
            graphics.clipRect(0, 0, 12, 22);
            graphics.drawImage(this.o, -12 * (this.n % 3), 0, 0);
        }
        if (this.a == 146) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, 0, 0, 0);
            graphics.translate(0, 16);
            graphics.setColor(0xFFFFFF);
            graphics.setClip(0, 0, 50, 50);
            graphics.clipRect(0, 0, 50, 50);
            graphics.drawString("99:-", 0, 0, 20);
        }
        if (this.a >= 147 && this.a <= 149) {
            if (this.h) {
                return;
            }
            graphics.setClip(0, 0, 8, 15);
            graphics.clipRect(0, 0, 8, 15);
            n4 = 0;
            if (this.n > 24) {
                ++n4;
            }
            if (this.n > 28) {
                ++n4;
            }
            graphics.drawImage(this.o, n4 * -8, -15 * (this.a - 147), 0);
        }
        if (this.a == 150) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, 0, 0, 0);
            graphics.translate(0, 16);
            graphics.setColor(0xFFFFFF);
            graphics.setClip(0, 0, 50, 50);
            graphics.clipRect(0, 0, 50, 50);
            graphics.drawString("10:-", 0, 0, 20);
        }
        if (this.a == 151) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, -16, 0, 0);
            graphics.translate(0, 16);
            graphics.setColor(0xFFFFFF);
            graphics.setClip(0, 0, 50, 50);
            graphics.clipRect(0, 0, 50, 50);
            graphics.drawString("25:-", 0, 0, 20);
        }
        if (this.a == 152) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.o, -32, 0, 0);
            graphics.translate(0, 16);
            graphics.setColor(0xFFFFFF);
            graphics.setClip(0, 0, 50, 50);
            graphics.clipRect(0, 0, 50, 50);
            graphics.drawString("15:-", 0, 0, 20);
        }
        if (this.a == 153) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, -16, 0, 0);
            }
        }
        if (this.a == 154) {
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
            if (!this.h) {
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
        }
        if (this.a == 155) {
            if (!this.h) {
                graphics.setClip(0, 0, 32, 24);
                graphics.clipRect(0, 0, 32, 24);
                graphics.drawImage(this.d, 0, 0, 0);
            } else {
                graphics.translate(0, -12);
                graphics.setClip(0, 0, 32, 38);
                graphics.clipRect(0, 0, 32, 38);
                graphics.drawImage(this.d, 0, -24, 0);
            }
            if (this.q.E) {
                graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
                graphics.translate(this.q.i - n2 + 4, this.q.j - n3 - 16);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.o, 0, 0, 0);
            }
        }
        this.n = (byte)(this.n + 1);
        if (this.n > 31) {
            this.n = 0;
        }
    }
}

