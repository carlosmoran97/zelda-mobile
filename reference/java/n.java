/*
 * Decompiled with CFR 0.152.
 */
import java.util.Random;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

public final class n
extends f {
    int a;
    int b;
    int c;
    int d;
    boolean e;
    boolean f;
    boolean g;
    boolean h;
    boolean n;
    boolean o;
    boolean p;
    boolean q;
    byte r;
    boolean s;
    int t;
    int u;
    boolean v;
    boolean w;
    boolean x;
    boolean y;
    boolean z = false;
    int A;
    byte B;
    private byte L;
    int C = 0;
    byte D;
    byte E;
    byte F;
    byte G;
    int H;
    int I;
    private int M;
    Image J;
    Image K;

    n(int n2) {
        this.a = n2;
        this.L = 1;
        this.B = 0;
        this.D = 0;
        this.E = 0;
        this.F = 0;
        this.G = 0;
        this.F = (byte)16;
        this.G = (byte)16;
        this.D = 0;
        this.E = 0;
        this.I = 1;
        this.y = true;
        this.z = false;
        if (this.a == 12) {
            this.E = (byte)-7;
            this.D = 0;
            this.F = (byte)32;
            this.G = (byte)32;
        }
        if (this.a == 62) {
            this.E = (byte)-7;
            this.D = 0;
            this.F = (byte)32;
            this.G = (byte)32;
        }
        if (this.a == 92) {
            this.E = (byte)-100;
            this.D = 0;
            this.F = (byte)64;
            this.G = (byte)80;
        }
        if (this.a == 29) {
            this.E = 0;
            this.D = 0;
            this.F = (byte)32;
            this.G = (byte)32;
        }
        if (this.a == 30) {
            this.E = 0;
            this.D = 0;
            this.F = (byte)16;
            this.G = (byte)16;
        }
        if (this.a == 64) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 65) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 66) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 67) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 68) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 69) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 70) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 71) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 77) {
            this.E = (byte)-4;
            this.F = (byte)44;
            this.G = (byte)24;
        }
        if (this.a == 78) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 96) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 97) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 98) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 99) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 100) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 101) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 102) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 103) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 112) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 113) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 114) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 115) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 116) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 117) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 118) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 119) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 128) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 129) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 130) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 131) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 132) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 133) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 134) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 135) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 144) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 145) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 146) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 147) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 148) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a == 149) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 150) {
            this.D = (byte)-4;
            this.F = (byte)24;
            this.G = (byte)16;
        }
        if (this.a == 151) {
            this.E = (byte)-4;
            this.F = (byte)16;
            this.G = (byte)24;
        }
        if (this.a >= 192 && this.a <= 205) {
            Random random = new Random();
            this.B = (byte)Math.abs(random.nextInt() % 8);
        }
    }

    final void a(Graphics graphics, int n2, int n3, int n4, int n5) {
        if (!this.q) {
            return;
        }
        if (this.a == 0) {
            return;
        }
        if (this.J == null) {
            return;
        }
        if (n4 + n2 < this.i) {
            return;
        }
        if (this.i + 32 < n2 && this.a != 92) {
            return;
        }
        if (n5 + n3 < this.j && this.a != 222) {
            return;
        }
        if (this.j + 32 - 32 < n3) {
            return;
        }
        if (!this.w) {
            return;
        }
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(this.i - n2, this.j - n3);
        if (this.a == 3) {
            n2 = 0;
            if (this.x) {
                if (this.A >= 2) {
                    n2 = 1;
                }
                if (this.A < 2) {
                    n2 = 2;
                }
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, n2 * -16, 0, 0);
        }
        if (this.a == 4) {
            n2 = 0;
            if (this.x) {
                if (this.A >= 2) {
                    n2 = 1;
                }
                if (this.A < 2) {
                    n2 = 2;
                }
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, n2 * -16, 0, 0);
        }
        if (this.a == 5) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 6) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 7) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 8) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 9) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 10) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 11) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 12) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 32, 32);
            graphics.clipRect(0, 0, 32, 32);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 13) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 14) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, -16, 0);
        }
        if (this.a == 15) {
            n2 = 0;
            if (this.x) {
                if (this.A >= 2) {
                    n2 = 1;
                }
                if (this.A < 2) {
                    n2 = 2;
                }
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, n2 * -16, 0, 0);
        }
        if (this.a == 16) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 19) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, -16 * (this.B >> 2), 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 20) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a >= 21 && this.a <= 24) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 21), 0, 0);
        }
        if (this.a == 25) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 3), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 30) {
                this.B = 0;
            }
        }
        if (this.a >= 26 && this.a <= 27) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 26), 0, 0);
        }
        if (this.a == 28) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.B >> 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a == 29) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 32, 32);
            graphics.clipRect(0, 0, 32, 32);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 30) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * this.C, 0, 0);
            if (this.M > 0) {
                --this.M;
            }
        }
        if (this.a == 32) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a >= 34 && this.a <= 42) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 34) % 3), -16 * ((this.a - 34) / 3), 0);
        }
        if (this.a >= 43 && this.a <= 45) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 43) % 3), 0, 0);
        }
        if (this.a == 48) {
            n2 = 0;
            if (this.s) {
                n2 = 16;
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -n2, 0, 0);
        }
        if (this.a == 46) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 47) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.B >> 1) % 2), 0, 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a >= 52 && this.a <= 59) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 52), 0, 0);
        }
        if (this.a == 60) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            if (!this.s) {
                if (this.H < this.i - 30) {
                    graphics.drawImage(this.J, -16, 0, 0);
                } else if (this.H > this.i + 30) {
                    graphics.drawImage(this.J, -32, 0, 0);
                } else {
                    graphics.drawImage(this.J, 0, 0, 0);
                }
            } else {
                graphics.drawImage(this.J, -48, 0, 0);
            }
        }
        if (this.a == 61) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 62) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 32, 32);
            graphics.clipRect(0, 0, 32, 32);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 63) {
            graphics.translate(-8, 0);
            graphics.setClip(0, 0, 32, 32);
            graphics.clipRect(0, 0, 32, 32);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 64) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 65) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 66) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 67) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 68) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 69) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 70) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 71) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 72) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 73) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 74) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -32, 0, 0);
        }
        if (this.a == 75) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -48, 0, 0);
        }
        if (this.a == 76) {
            graphics.translate(-2, -1);
            graphics.setClip(0, 0, 20, 17);
            graphics.clipRect(0, 0, 20, 17);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 77) {
            graphics.translate(-2, -1);
            graphics.setClip(0, 0, 44, 15);
            graphics.clipRect(0, 0, 44, 15);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 78) {
            graphics.translate(0, -4);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 79) {
            graphics.setClip(0, 0, 64, 32);
            graphics.clipRect(0, 0, 64, 32);
            if (this.B / 2 % 2 == 0) {
                graphics.drawRegion(this.J, 0, 0, 64, 32, 0, 0, 0, 0);
            }
            if (this.B / 2 % 2 == 1) {
                graphics.drawRegion(this.J, 0, 0, 64, 32, 2, 0, 0, 0);
            }
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a >= 80 && this.a <= 83) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 80), 0, 0);
        }
        if (this.a >= 84 && this.a <= 87) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 84), 0, 0);
        }
        if (this.a >= 88 && this.a <= 91) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 88), 0, 0);
        }
        if (this.a == 92) {
            graphics.translate(0, -60);
            graphics.setClip(0, 0, 64, 74);
            graphics.clipRect(0, 0, 64, 74);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 96) {
            graphics.translate(0, -4);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 97) {
            graphics.translate(-4, 0);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 98) {
            graphics.translate(-4, 0);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 99) {
            graphics.translate(0, -4);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 100) {
            graphics.translate(0, -4);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 101) {
            graphics.translate(-4, 0);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 102) {
            graphics.translate(-4, 0);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 103) {
            graphics.translate(0, -4);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 104) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 105) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 106) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -32, 0, 0);
        }
        if (this.a == 107) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -48, 0, 0);
        }
        if (this.a >= 108 && this.a <= 111) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 108), 0, 0);
        }
        if (this.a == 112) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 113) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 114) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 115) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 116) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 117) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 118) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 119) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 120) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 121) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 122) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -32, 0, 0);
        }
        if (this.a == 123) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -48, 0, 0);
        }
        if (this.a >= 124 && this.a <= 127) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 124) % 2), -16 * ((this.a - 124) / 2), 0);
        }
        if (this.a == 128) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 129) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 130) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 131) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 132) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 133) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 134) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 135) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a >= 136 && this.a <= 139) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 136), 0, 0);
        }
        if (this.a >= 140 && this.a <= 143) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 140) % 2), -16 * ((this.a - 140) / 2), 0);
        }
        if (this.a == 144) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 145) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 146) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 147) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a == 148) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, 0, 0, 0);
        }
        if (this.a == 149) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, 0, -24, 0);
        }
        if (this.a == 150) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.J, -16, 0, 0);
        }
        if (this.a == 151) {
            graphics.translate(this.D, this.E);
            graphics.setClip(0, 0, 16, 24);
            graphics.clipRect(0, 0, 16, 24);
            graphics.drawImage(this.J, -24, -16, 0);
        }
        if (this.a >= 152 && this.a <= 156) {
            graphics.setClip(0, 0, 64, 32);
            graphics.clipRect(0, 0, 64, 32);
            if (this.B / 2 % 2 == 0) {
                graphics.drawRegion(this.J, 0, 0, 64, 32, 0, 0, 0, 0);
            }
            if (this.B / 2 % 2 == 1) {
                graphics.drawRegion(this.J, 0, 0, 64, 32, 2, 0, 0, 0);
            }
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a >= 160 && this.a <= 168) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 160) % 3), -16 * ((this.a - 160) / 3), 0);
        }
        if (this.a >= 169 && this.a <= 170) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, -16 * (this.a - 169), 0);
        }
        if (this.a >= 171 && this.a <= 174) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 171) % 2), -16 * ((this.a - 171) / 2), 0);
        }
        if (this.a >= 176 && this.a <= 184) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 176) % 3), -16 * ((this.a - 176) / 3), 0);
        }
        if (this.a >= 192 && this.a <= 203) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            if ((this.a - 192) % 4 == 0) {
                graphics.drawRegion(this.J, 16 * ((this.a - 192) / 4), 16 * (this.B / 4), 16, 16, 0, 0, 0, 0);
            }
            if ((this.a - 192) % 4 == 1) {
                graphics.drawRegion(this.J, 16 * ((this.a - 192) / 4), 16 * (this.B / 4), 16, 16, 5, 0, 0, 0);
            }
            if ((this.a - 192) % 4 == 2) {
                graphics.drawRegion(this.J, 16 * ((this.a - 192) / 4), 16 * (this.B / 4), 16, 16, 3, 0, 0, 0);
            }
            if ((this.a - 192) % 4 == 3) {
                graphics.drawRegion(this.J, 16 * ((this.a - 192) / 4), 16 * (this.B / 4), 16, 16, 6, 0, 0, 0);
            }
            if (this.B >= 11) {
                this.L = (byte)-1;
            }
            if (this.B <= 0) {
                this.L = 1;
            }
            this.B = (byte)(this.B + this.L);
        }
        if (this.a == 204) {
            if (this.z) {
                return;
            }
            graphics.translate(0, -32);
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawRegion(this.J, 16 * ((this.a - 192) / 4), 0, 16, 16, 6, 0, 0, 0);
            this.z = true;
        }
        if (this.a >= 208 && this.a <= 211) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            if ((this.a - 208) % 4 == 0) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 0, 0, 0, 0);
            }
            if ((this.a - 208) % 4 == 1) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 5, 0, 0, 0);
            }
            if ((this.a - 208) % 4 == 2) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 3, 0, 0, 0);
            }
            if ((this.a - 208) % 4 == 3) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 6, 0, 0, 0);
            }
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a >= 212 && this.a <= 215) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            if ((this.a - 212) % 4 == 0) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 0, 0, 0, 0);
            }
            if ((this.a - 212) % 4 == 1) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 5, 0, 0, 0);
            }
            if ((this.a - 212) % 4 == 2) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 3, 0, 0, 0);
            }
            if ((this.a - 212) % 4 == 3) {
                graphics.drawRegion(this.J, 16 * (this.B >> 2), 0, 16, 16, 6, 0, 0, 0);
            }
            this.B = (byte)(this.B + 1);
            if (this.B > 15) {
                this.B = 0;
            }
        }
        if (this.a >= 216 && this.a <= 220) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 216), 0, 0);
        }
        if (this.a == 185) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, -16 * (this.B >> 2), 0);
            this.B = (byte)(this.B + 1);
            if (this.B > 10) {
                this.B = 0;
            }
        }
        if (this.a == 186) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, 0, -16 * this.C, 0);
            this.e = this.C == 1;
        }
        if (this.a == 187) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16, -16 * (1 - this.C), 0);
            this.e = this.C == 0;
        }
        if (this.a >= 188 && this.a <= 191) {
            if ((this.a - 188) % 4 == 0) {
                graphics.translate(0, 4);
                graphics.setClip(0, 0, 32, 16);
                graphics.clipRect(0, 0, 32, 16);
                graphics.drawRegion(this.J, 0, 16 * (this.B >> 1), 32, 16, 0, 0, 0, 0);
            }
            if ((this.a - 188) % 4 == 1) {
                graphics.translate(-4, -8);
                graphics.setClip(0, 0, 16, 32);
                graphics.clipRect(0, 0, 16, 32);
                graphics.drawRegion(this.J, 0, 16 * (this.B >> 1), 32, 16, 5, 0, 0, 0);
            }
            if ((this.a - 188) % 4 == 2) {
                graphics.translate(0, -4);
                graphics.setClip(0, 0, 32, 16);
                graphics.clipRect(0, 0, 32, 16);
                graphics.drawRegion(this.J, 0, 16 * (this.B >> 1), 32, 16, 3, 0, 0, 0);
            }
            if ((this.a - 188) % 4 == 3) {
                graphics.translate(0, -8);
                graphics.setClip(0, 0, 16, 32);
                graphics.clipRect(0, 0, 16, 32);
                graphics.drawRegion(this.J, 0, 16 * (this.B >> 1), 32, 16, 6, 0, 0, 0);
            }
            this.B = (byte)(this.B + this.L);
            if (this.B >= 5) {
                this.L = (byte)-1;
            }
            if (this.B <= 0) {
                this.L = 1;
            }
        }
        if (this.a >= 205 && this.a <= 207 || this.a == 221) {
            if (this.a == 205) {
                graphics.translate(-16, 0);
                graphics.setClip(0, 0, 48, 32);
                graphics.clipRect(0, 0, 48, 32);
                graphics.drawRegion(this.J, 0, 0, 48, 32, 0, 0, 0, 0);
            }
            if (this.a == 206) {
                graphics.translate(-16, 0);
                graphics.setClip(0, 0, 48, 32);
                graphics.clipRect(0, 0, 48, 32);
                graphics.drawRegion(this.J, 0, 0, 48, 32, 3, 0, 0, 0);
            }
            if (this.a == 207) {
                graphics.translate(-16, -16);
                graphics.setClip(0, 0, 32, 48);
                graphics.clipRect(0, 0, 32, 48);
                graphics.drawRegion(this.J, 0, 0, 48, 32, 5, 0, 0, 0);
            }
            if (this.a == 221) {
                graphics.translate(0, -16);
                graphics.setClip(0, 0, 32, 48);
                graphics.clipRect(0, 0, 32, 48);
                graphics.drawRegion(this.J, 0, 0, 48, 32, 6, 0, 0, 0);
            }
        }
        if (this.a >= 222 && this.a <= 223 || this.a >= 237 && this.a <= 238) {
            if (this.a == 222) {
                graphics.translate(-16, -16);
                graphics.setClip(0, 0, 96, 22);
                graphics.clipRect(0, 0, 96, 22);
                graphics.drawImage(this.J, 0, -26, 0);
            }
            if (this.a == 223) {
                graphics.translate(-16, 0);
                graphics.setClip(0, 0, 96, 22);
                graphics.clipRect(0, 0, 96, 22);
                graphics.drawRegion(this.J, 0, 26, 96, 22, 3, 0, 0, 0);
            }
            if (this.a == 237) {
                graphics.translate(-16, -10);
                graphics.setClip(0, 0, 96, 26);
                graphics.clipRect(0, 0, 96, 26);
                graphics.drawImage(this.J, 0, 0, 0);
            }
            if (this.a == 238) {
                graphics.translate(-16, -10);
                graphics.setClip(0, 0, 96, 26);
                graphics.clipRect(0, 0, 96, 26);
                graphics.drawRegion(this.J, 0, 0, 96, 26, 3, 0, 0, 0);
            }
        }
        if (this.a >= 224 && this.a <= 228) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 224), 0, 0);
        }
        if (this.a >= 229 && this.a <= 232) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * (this.a - 229), 0, 0);
        }
        if (this.a >= 233 && this.a <= 236) {
            if (this.a == 233) {
                graphics.setClip(0, 0, 48, 16);
                graphics.clipRect(0, 0, 48, 16);
                graphics.drawImage(this.J, 0, 0, 0);
            }
            if (this.a == 234) {
                graphics.setClip(0, 0, 48, 16);
                graphics.clipRect(0, 0, 48, 16);
                graphics.drawImage(this.J, 0, -16, 0);
            }
            if (this.a == 235) {
                graphics.setClip(0, 0, 16, 48);
                graphics.clipRect(0, 0, 16, 48);
                graphics.drawImage(this.J, -64, 0, 0);
            }
            if (this.a == 236) {
                graphics.setClip(0, 0, 16, 48);
                graphics.clipRect(0, 0, 16, 48);
                graphics.drawImage(this.J, -48, 0, 0);
            }
        }
        if (this.a >= 240 && this.a <= 255) {
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.J, -16 * ((this.a - 240) % 4), -16 * ((this.a - 240) / 4), 0);
        }
    }
}

