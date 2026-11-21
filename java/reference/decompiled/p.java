/*
 * Decompiled with CFR 0.152.
 */
import java.util.Random;
import javax.microedition.lcdui.Graphics;
import javax.microedition.lcdui.Image;

final class p
extends f {
    byte a;
    int b;
    int c;
    private Image s;
    Image d;
    int e;
    int f;
    int g;
    int h;
    int n;
    int o;
    private int t;
    private int u;
    private int v;
    boolean p;
    int q;
    boolean r;

    p(Image image, int n2, int n3, int n4) {
        this.s = image;
        this.h = n2;
        this.b = n3;
        this.c = n4;
        if (this.h == 0) {
            this.u = 4;
            this.v = 4;
            this.f = 8;
            this.g = 8;
            this.n = 4;
            this.o = 1;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 1) {
            this.u = 4;
            this.v = 4;
            this.f = 8;
            this.g = 8;
            this.n = 4;
            this.o = 3;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 2) {
            this.u = 5;
            this.v = 5;
            this.f = 10;
            this.g = 10;
            this.n = 5;
            this.o = 1;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 3) {
            this.u = 0;
            this.v = 0;
            this.n = 5;
            this.f = 16;
            this.g = 5;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
            this.o = 3;
        }
        if (this.h == 5) {
            this.u = 8;
            this.v = 8;
            this.f = 15;
            this.g = 15;
            this.n = 5;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
            this.o = 5;
        }
        if (this.h == 6) {
            this.u = 8;
            this.v = 8;
            this.n = 5;
            this.o = 10;
            this.f = 16;
            this.g = 16;
            this.t = 0;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 7) {
            this.u = 12;
            this.v = 12;
            this.f = 25;
            this.g = 25;
            this.n = 5;
            this.o = 10;
            this.t = 0;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 8) {
            this.u = 5;
            this.v = 5;
            this.f = 10;
            this.g = 10;
            this.n = 4;
            this.o = 1;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 9) {
            this.u = 5;
            this.v = 5;
            this.f = 10;
            this.g = 10;
            this.n = 4;
            this.o = 1;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 10) {
            this.u = 5;
            this.v = 5;
            this.f = 10;
            this.g = 10;
            this.n = 4;
            this.o = 1;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
        }
        if (this.h == 11) {
            this.u = 0;
            this.v = 0;
            this.n = 5;
            this.f = 17;
            this.g = 7;
            if (this.c > 0) {
                this.e = 2;
            }
            if (this.c < 0) {
                this.e = 0;
            }
            if (this.b > 0) {
                this.e = 1;
            }
            if (this.b < 0) {
                this.e = 3;
            }
            this.o = 3;
        }
    }

    final void a(Graphics graphics, int n2, int n3, int n4, int n5) {
        if (n4 + n2 < this.i) {
            this.a();
            return;
        }
        if (this.i + this.u + this.u < n2) {
            this.a();
            return;
        }
        if (n5 + n3 < this.j) {
            this.a();
            return;
        }
        if (this.j + this.v + this.v - 32 < n3) {
            this.a();
            return;
        }
        graphics.translate(-graphics.getTranslateX(), -graphics.getTranslateY());
        graphics.translate(this.i - n2, this.j - n3);
        if (this.h == 0) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 8, 8);
            graphics.clipRect(0, 0, 8, 8);
            graphics.drawImage(this.s, -8 * (this.a % 2), 0, 0);
        }
        if (this.h == 1) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 8, 8);
            graphics.clipRect(0, 0, 8, 8);
            graphics.drawImage(this.s, -8 * (this.a % 3), 0, 0);
        }
        if (this.h == 2) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 10, 10);
            graphics.clipRect(0, 0, 10, 10);
            graphics.drawImage(this.s, -10 * (this.a / 2), 0, 0);
        }
        if (this.h == 3) {
            if (this.e == 0) {
                this.f = 5;
                this.g = 16;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 5, 16);
                graphics.clipRect(0, 0, 5, 16);
                graphics.drawImage(this.s, -21, 0, 0);
            }
            if (this.e == 1) {
                this.f = 16;
                this.g = 5;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 16, 5);
                graphics.clipRect(0, 0, 16, 5);
                graphics.drawImage(this.s, 0, 0, 0);
            }
            if (this.e == 2) {
                this.f = 5;
                this.g = 16;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 5, 16);
                graphics.clipRect(0, 0, 5, 16);
                graphics.drawImage(this.s, -16, 0, 0);
            }
            if (this.e == 3) {
                this.f = 16;
                this.g = 5;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 16, 5);
                graphics.clipRect(0, 0, 16, 5);
                graphics.drawImage(this.s, 0, -5, 0);
            }
        }
        if (this.h == 5) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.s, -16 * (this.a % 3), -16 * this.e, 0);
        }
        if (this.h == 6) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            Random random = new Random();
            random.setSeed(this.i * this.j + random.nextInt());
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.s, -16 * (this.a % 2), 0, 0);
            if (this.t > 12) {
                graphics.translate(12, 0);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.s, -16 * (this.a % 3), 0, 0);
            }
            if (this.t > 24) {
                graphics.translate(12, 0);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.s, -32, 0, 0);
            }
            if (this.t > 36) {
                graphics.translate(12, 0);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.s, -48, 0, 0);
            }
            if (this.t > 48) {
                graphics.translate(12, 0);
                graphics.setClip(0, 0, 16, 16);
                graphics.clipRect(0, 0, 16, 16);
                graphics.drawImage(this.s, -64, 0, 0);
            }
        }
        if (this.h == 7) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 25, 25);
            graphics.clipRect(0, 0, 25, 25);
            graphics.drawImage(this.s, -25 * (this.a % 2), 0, 0);
        }
        if (this.h == 8) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 16, 16);
            graphics.clipRect(0, 0, 16, 16);
            graphics.drawImage(this.s, -16 * (this.a % 4), 0, 0);
        }
        if (this.h == 9) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.s, -24 * ((this.a >> 1) % 3), 0, 0);
        }
        if (this.h == 10) {
            if (this.i - n2 + this.f > 256) {
                return;
            }
            if (this.j - n3 + this.g > 208) {
                return;
            }
            graphics.setClip(0, 0, 24, 16);
            graphics.clipRect(0, 0, 24, 16);
            graphics.drawImage(this.s, -24 * ((this.a >> 1) % 3), 0, 0);
        }
        if (this.h == 11) {
            if (this.e == 0) {
                this.f = 7;
                this.g = 17;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 7, 17);
                graphics.clipRect(0, 0, 7, 17);
                graphics.drawImage(this.s, -24, 0, 0);
            }
            if (this.e == 1) {
                this.f = 17;
                this.g = 7;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 17, 7);
                graphics.clipRect(0, 0, 17, 7);
                graphics.drawImage(this.s, 0, 0, 0);
            }
            if (this.e == 2) {
                this.f = 7;
                this.g = 17;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 7, 17);
                graphics.clipRect(0, 0, 7, 17);
                graphics.drawImage(this.s, -17, 0, 0);
            }
            if (this.e == 3) {
                this.f = 17;
                this.g = 7;
                if (this.i - n2 + this.f > 256) {
                    return;
                }
                if (this.j - n3 + this.g > 208) {
                    return;
                }
                graphics.setClip(0, 0, 17, 7);
                graphics.clipRect(0, 0, 17, 7);
                graphics.drawImage(this.s, 0, -7, 0);
            }
        }
        this.a();
    }

    private void a() {
        this.i += this.b * this.n;
        this.j += this.c * this.n;
        this.a = (byte)(this.a + 1);
        this.t += this.n;
        if (this.a >= 7) {
            this.a = 0;
        }
        if (this.c > 0) {
            this.e = 2;
        }
        if (this.c < 0) {
            this.e = 0;
        }
        if (this.b > 0) {
            this.e = 1;
        }
        if (this.b < 0) {
            this.e = 3;
        }
        if (this.h == 11) {
            this.e = 0;
        }
    }
}

