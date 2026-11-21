/*
 * Decompiled with CFR 0.152.
 */
import java.io.IOException;
import javax.microedition.lcdui.Image;

public final class o {
    private j b;
    private Image[] c;
    private Image[] d;
    Image[] a;
    private Image[] e;
    private int[] f;
    private int[] g;
    private int[] h;
    private int[] i;
    private boolean[] j;
    private boolean[] k;
    private boolean[] l;
    private boolean[] m;

    public o(j j2) {
        Runtime.getRuntime().totalMemory();
        this.c = new Image[256];
        this.d = new Image[256];
        this.a = new Image[256];
        this.e = new Image[16];
        this.f = new int[256];
        this.g = new int[256];
        this.h = new int[256];
        this.i = new int[16];
        this.j = new boolean[256];
        this.k = new boolean[256];
        this.l = new boolean[256];
        this.m = new boolean[16];
        this.b = j2;
    }

    public final void a() {
        for (int i2 = 0; i2 < 256; ++i2) {
            if (this.d[i2] != null) {
                int n2 = i2;
                this.g[n2] = this.g[n2] - 1;
                if (this.g[i2] <= 0) {
                    this.g[i2] = 0;
                    this.d[i2] = null;
                }
            }
            this.k[i2] = false;
        }
        System.gc();
    }

    public final void b() {
        for (int i2 = 0; i2 < 256; ++i2) {
            if (this.c[i2] != null) {
                int n2 = i2;
                this.f[n2] = this.f[n2] - 1;
                if (this.f[i2] <= 0) {
                    this.f[i2] = 0;
                    this.c[i2] = null;
                    System.out.println("removed event " + i2);
                }
            }
            this.j[i2] = false;
        }
        System.gc();
    }

    public final void c() {
        for (int i2 = 0; i2 < 16; ++i2) {
            if (this.e[i2] != null) {
                int n2 = i2;
                this.i[n2] = this.i[n2] - 1;
                if (this.i[i2] <= 0) {
                    this.i[i2] = 0;
                    this.e[i2] = null;
                }
            }
            this.m[i2] = false;
        }
        System.gc();
    }

    public final void d() {
        for (int i2 = 0; i2 < 256; ++i2) {
            if (this.a[i2] != null) {
                int n2 = i2;
                this.h[n2] = this.h[n2] - 1;
                if (this.h[i2] <= 0) {
                    this.h[i2] = 0;
                    this.a[i2] = null;
                }
            }
            this.l[i2] = false;
        }
        System.gc();
    }

    public final Image a(int n2) {
        block81: {
            try {
                if (n2 <= 3 && n2 >= 0) {
                    if (!this.k[0]) {
                        this.g[0] = this.g[0] + 3;
                    }
                    this.k[0] = true;
                    if (this.d[0] == null) {
                        try {
                            this.d[0] = Image.createImage("/Images/item_Rupies.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[0];
                }
                if (n2 == 4) {
                    if (!this.k[n2]) {
                        int n3 = n2;
                        this.g[n3] = this.g[n3] + 3;
                    }
                    this.k[n2] = true;
                    if (this.d[n2] == null) {
                        try {
                            this.d[n2] = Image.createImage("/Images/Bomb.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[n2];
                }
                if (n2 >= 5 && n2 <= 7) {
                    if (!this.k[5]) {
                        this.g[5] = this.g[5] + 3;
                    }
                    this.k[5] = true;
                    if (this.d[5] == null) {
                        try {
                            this.d[5] = Image.createImage("/Images/Item5-7.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[5];
                }
                if (n2 >= 128 && n2 <= 130) {
                    if (!this.k[128]) {
                        this.g[128] = this.g[128] + 3;
                    }
                    this.k[128] = true;
                    if (this.d[128] == null) {
                        try {
                            this.d[128] = Image.createImage("/Images/Item128-131.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[128];
                }
                if (n2 >= 131 && n2 <= 133) {
                    if (!this.k[131]) {
                        this.g[131] = this.g[131] + 3;
                    }
                    this.k[131] = true;
                    if (this.d[131] == null) {
                        try {
                            this.d[131] = Image.createImage("/Images/Item131-133.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[131];
                }
                if (n2 >= 134 && n2 <= 136) {
                    if (!this.k[134]) {
                        this.g[134] = this.g[134] + 3;
                    }
                    this.k[134] = true;
                    if (this.d[134] == null) {
                        try {
                            this.d[134] = Image.createImage("/Images/Item134-136.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[134];
                }
                if (n2 >= 137 && n2 <= 139) {
                    if (!this.k[137]) {
                        this.g[137] = this.g[137] + 3;
                    }
                    this.k[137] = true;
                    if (this.d[137] == null) {
                        try {
                            this.d[137] = Image.createImage("/Images/Item137-139.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[137];
                }
                if (n2 >= 140 && n2 <= 142) {
                    if (!this.k[140]) {
                        this.g[140] = this.g[140] + 3;
                    }
                    this.k[140] = true;
                    if (this.d[140] == null) {
                        try {
                            this.d[140] = Image.createImage("/Images/Item140-142.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[140];
                }
                if (n2 == 143 || n2 == 153) {
                    if (!this.k[143]) {
                        this.g[143] = this.g[143] + 3;
                    }
                    this.k[143] = true;
                    if (this.d[143] == null) {
                        try {
                            this.d[143] = Image.createImage("/Images/Item153-155.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[143];
                }
                if (n2 == 144) {
                    if (!this.k[144]) {
                        this.g[144] = this.g[144] + 3;
                    }
                    this.k[144] = true;
                    if (this.d[144] == null) {
                        try {
                            this.d[144] = Image.createImage("/Images/Triforce2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[144];
                }
                if (n2 == 145) {
                    if (!this.k[145]) {
                        this.g[145] = this.g[145] + 3;
                    }
                    this.k[145] = true;
                    if (this.d[145] == null) {
                        try {
                            this.d[145] = Image.createImage("/Images/Mastersword.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[145];
                }
                if (n2 == 146) {
                    if (!this.k[n2]) {
                        int n4 = n2;
                        this.g[n4] = this.g[n4] + 3;
                    }
                    this.k[n2] = true;
                    if (this.d[146] == null) {
                        try {
                            this.d[146] = Image.createImage("/Images/Item131-133.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[146];
                }
                if (n2 >= 147 && n2 <= 149) {
                    if (!this.k[147]) {
                        this.g[147] = this.g[147] + 3;
                    }
                    this.k[147] = true;
                    if (this.d[147] == null) {
                        try {
                            this.d[147] = Image.createImage("/Images/Rupies2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[147];
                }
                if (n2 >= 150 && n2 <= 152) {
                    if (!this.k[150]) {
                        this.g[150] = this.g[150] + 3;
                    }
                    this.k[150] = true;
                    if (this.d[150] == null) {
                        try {
                            this.d[150] = Image.createImage("/Images/Item150-152.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[150];
                }
                if (n2 == 154) {
                    if (!this.k[n2]) {
                        int n5 = n2;
                        this.g[n5] = this.g[n5] + 3;
                    }
                    this.k[n2] = true;
                    if (this.d[154] == null) {
                        try {
                            this.d[154] = Image.createImage("/Images/BigKey.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.d[154];
                }
                if (n2 != 155) break block81;
                if (!this.k[n2]) {
                    int n6 = n2;
                    this.g[n6] = this.g[n6] + 3;
                }
                this.k[n2] = true;
                if (this.d[155] == null) {
                    try {
                        this.d[155] = Image.createImage("/Images/Outfit.png");
                    }
                    catch (IOException iOException) {
                        System.out.println("error loadin item img");
                        return null;
                    }
                }
                return this.d[155];
            }
            catch (OutOfMemoryError outOfMemoryError) {
                System.out.println("Error... out of memory");
                System.gc();
                return this.a(n2);
            }
        }
        return null;
    }

    public final Image b(int n2) {
        block61: {
            try {
                if (n2 == 0) {
                    if (!this.m[n2]) {
                        int n3 = n2;
                        this.i[n3] = this.i[n3] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/ProjStone.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 1) {
                    if (!this.m[n2]) {
                        int n4 = n2;
                        this.i[n4] = this.i[n4] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/ProjBall.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 2) {
                    if (!this.m[n2]) {
                        int n5 = n2;
                        this.i[n5] = this.i[n5] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/Bomerang1.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 3) {
                    if (!this.m[n2]) {
                        int n6 = n2;
                        this.i[n6] = this.i[n6] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/Arrows.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 4) {
                    if (!this.m[n2]) {
                        int n7 = n2;
                        this.i[n7] = this.i[n7] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/HookShot.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 5) {
                    if (!this.m[n2]) {
                        int n8 = n2;
                        this.i[n8] = this.i[n8] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/WandShot01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 6) {
                    if (!this.m[n2]) {
                        int n9 = n2;
                        this.i[n9] = this.i[n9] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/Smoke01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 7) {
                    if (!this.m[n2]) {
                        int n10 = n2;
                        this.i[n10] = this.i[n10] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/Shot7.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 8) {
                    if (!this.m[n2]) {
                        int n11 = n2;
                        this.i[n11] = this.i[n11] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/Boomerang01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 9) {
                    if (!this.m[n2]) {
                        int n12 = n2;
                        this.i[n12] = this.i[n12] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/ProjFire.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 == 10) {
                    if (!this.m[n2]) {
                        int n13 = n2;
                        this.i[n13] = this.i[n13] + 3;
                    }
                    this.m[n2] = true;
                    if (this.e[n2] == null) {
                        try {
                            this.e[n2] = Image.createImage("/Images/ProjIce.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin item img");
                            return null;
                        }
                    }
                    return this.e[n2];
                }
                if (n2 != 11) break block61;
                if (!this.m[n2]) {
                    int n14 = n2;
                    this.i[n14] = this.i[n14] + 3;
                }
                this.m[n2] = true;
                if (this.e[n2] == null) {
                    try {
                        this.e[n2] = Image.createImage("/Images/LightArrow.png");
                    }
                    catch (IOException iOException) {
                        System.out.println("error loadin item img");
                        return null;
                    }
                }
                return this.e[n2];
            }
            catch (OutOfMemoryError outOfMemoryError) {
                System.out.println("Error... out of memory");
                System.gc();
                return this.b(n2);
            }
        }
        return null;
    }

    final Image c(int n2) {
        block391: {
            try {
                if (n2 == 3) {
                    if (!this.j[n2]) {
                        int n3 = n2;
                        this.f[n3] = this.f[n3] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Bush.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 4) {
                    if (!this.j[n2]) {
                        int n4 = n2;
                        this.f[n4] = this.f[n4] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/TealBush.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 5) {
                    if (!this.j[n2]) {
                        int n5 = n2;
                        this.f[n5] = this.f[n5] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/const1.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 6) {
                    if (!this.j[n2]) {
                        int n6 = n2;
                        this.f[n6] = this.f[n6] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/GreenFlowers.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 7) {
                    if (!this.j[n2]) {
                        int n7 = n2;
                        this.f[n7] = this.f[n7] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/TealFlowers.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 8) {
                    if (!this.j[n2]) {
                        int n8 = n2;
                        this.f[n8] = this.f[n8] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Candle1.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 9) {
                    if (!this.j[n2]) {
                        int n9 = n2;
                        this.f[n9] = this.f[n9] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Waterfall02.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 10) {
                    if (!this.j[n2]) {
                        int n10 = n2;
                        this.f[n10] = this.f[n10] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Waterfall01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 11) {
                    if (!this.j[n2]) {
                        int n11 = n2;
                        this.f[n11] = this.f[n11] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Candle2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 12) {
                    if (!this.j[n2]) {
                        int n12 = n2;
                        this.f[n12] = this.f[n12] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Gravestone.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 13 || n2 == 14) {
                    if (!this.j[13]) {
                        this.f[13] = this.f[13] + 3;
                    }
                    this.j[13] = true;
                    if (this.c[13] == null) {
                        try {
                            this.c[13] = Image.createImage("/Images/Fence.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[13];
                }
                if (n2 == 15) {
                    if (!this.j[n2]) {
                        int n13 = n2;
                        this.f[n13] = this.f[n13] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/BrownBush.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 16) {
                    if (!this.j[n2]) {
                        int n14 = n2;
                        this.f[n14] = this.f[n14] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Sign.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 19) {
                    if (!this.j[n2]) {
                        int n15 = n2;
                        this.f[n15] = this.f[n15] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/HappyFlowers.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 20) {
                    if (!this.j[n2]) {
                        int n16 = n2;
                        this.f[n16] = this.f[n16] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/HookTarget.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 >= 21 && n2 <= 24) {
                    if (!this.j[21]) {
                        this.f[21] = this.f[21] + 3;
                    }
                    this.j[21] = true;
                    if (this.c[21] == null) {
                        try {
                            this.c[21] = Image.createImage("/Images/Brygga.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[21];
                }
                if (n2 == 25) {
                    if (!this.j[n2]) {
                        int n17 = n2;
                        this.f[n17] = this.f[n17] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/GreenWaterTest01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 >= 26 && n2 <= 27) {
                    if (!this.j[n2]) {
                        int n18 = n2;
                        this.f[n18] = this.f[n18] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[26] == null) {
                        try {
                            this.c[26] = Image.createImage("/Images/Bridge.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[26];
                }
                if (n2 == 28) {
                    if (!this.j[n2]) {
                        int n19 = n2;
                        this.f[n19] = this.f[n19] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/WaterTest01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 29) {
                    if (!this.j[n2]) {
                        int n20 = n2;
                        this.f[n20] = this.f[n20] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Pedestal.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 30) {
                    if (!this.j[n2]) {
                        int n21 = n2;
                        this.f[n21] = this.f[n21] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Globes.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 32) {
                    if (!this.j[n2]) {
                        int n22 = n2;
                        this.f[n22] = this.f[n22] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Stone.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 >= 34 && n2 <= 42) {
                    if (!this.j[34]) {
                        this.f[34] = this.f[34] + 3;
                    }
                    this.j[34] = true;
                    if (this.c[34] == null) {
                        try {
                            this.c[34] = Image.createImage("/Images/Net.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[34];
                }
                if (n2 >= 43 && n2 <= 45) {
                    if (!this.j[43]) {
                        this.f[43] = this.f[43] + 3;
                    }
                    this.j[43] = true;
                    if (this.c[43] == null) {
                        try {
                            this.c[43] = Image.createImage("/Images/Stairs.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[43];
                }
                if (n2 == 46) {
                    if (!this.j[n2]) {
                        int n23 = n2;
                        this.f[n23] = this.f[n23] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Stone.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 47) {
                    if (!this.j[n2]) {
                        int n24 = n2;
                        this.f[n24] = this.f[n24] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Warp.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 48) {
                    if (!this.j[n2]) {
                        int n25 = n2;
                        this.f[n25] = this.f[n25] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Trigger01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 >= 52 && n2 <= 59) {
                    if (!this.j[52]) {
                        this.f[52] = this.f[52] + 3;
                    }
                    this.j[52] = true;
                    if (this.c[52] == null) {
                        try {
                            this.c[52] = Image.createImage("/Images/Claws.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[52];
                }
                if (n2 == 60) {
                    if (!this.j[n2]) {
                        int n26 = n2;
                        this.f[n26] = this.f[n26] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/WallEye.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 61) {
                    if (!this.j[n2]) {
                        int n27 = n2;
                        this.f[n27] = this.f[n27] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Monster10.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 62) {
                    if (!this.j[n2]) {
                        int n28 = n2;
                        this.f[n28] = this.f[n28] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Gravestone2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 63) {
                    if (!this.j[n2]) {
                        int n29 = n2;
                        this.f[n29] = this.f[n29] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Ball.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 >= 64 && n2 <= 67) {
                    if (!this.j[64]) {
                        this.f[64] = this.f[64] + 3;
                    }
                    this.j[64] = true;
                    if (this.c[64] == null) {
                        try {
                            this.c[64] = Image.createImage("/Images/Dungeon Doors.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[64];
                }
                if (n2 >= 68 && n2 <= 71) {
                    if (!this.j[68]) {
                        this.f[68] = this.f[68] + 3;
                    }
                    this.j[68] = true;
                    if (this.c[68] == null) {
                        try {
                            this.c[68] = Image.createImage("/Images/Dungeon Doors 2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[68];
                }
                if (n2 >= 72 && n2 <= 75) {
                    if (!this.j[72]) {
                        this.f[72] = this.f[72] + 3;
                    }
                    this.j[72] = true;
                    if (this.c[72] == null) {
                        try {
                            this.c[72] = Image.createImage("/Images/Wall.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[72];
                }
                if (n2 >= 104 && n2 <= 107) {
                    if (!this.j[104]) {
                        this.f[104] = this.f[104] + 3;
                    }
                    this.j[104] = true;
                    if (this.c[104] == null) {
                        try {
                            this.c[104] = Image.createImage("/Images/Wall2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[104];
                }
                if (n2 >= 108 && n2 <= 111) {
                    if (!this.j[108]) {
                        this.f[108] = this.f[108] + 3;
                    }
                    this.j[108] = true;
                    if (this.c[108] == null) {
                        try {
                            this.c[108] = Image.createImage("/Images/BehindTree02.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[108];
                }
                if (n2 >= 120 && n2 <= 123) {
                    if (!this.j[120]) {
                        this.f[120] = this.f[120] + 3;
                    }
                    this.j[120] = true;
                    if (this.c[120] == null) {
                        try {
                            this.c[120] = Image.createImage("/Images/Wall3.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[120];
                }
                if (n2 == 76) {
                    if (!this.j[n2]) {
                        int n30 = n2;
                        this.f[n30] = this.f[n30] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/BombWall.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 77) {
                    if (!this.j[n2]) {
                        int n31 = n2;
                        this.f[n31] = this.f[n31] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/BossDoor01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 78) {
                    if (!this.j[n2]) {
                        int n32 = n2;
                        this.f[n32] = this.f[n32] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/BossDoor02.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 == 79) {
                    if (!this.j[n2]) {
                        int n33 = n2;
                        this.f[n33] = this.f[n33] + 3;
                    }
                    this.j[n2] = true;
                    if (this.c[n2] == null) {
                        try {
                            this.c[n2] = Image.createImage("/Images/Lightning05.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[n2];
                }
                if (n2 >= 96 && n2 <= 99) {
                    if (!this.j[96]) {
                        this.f[96] = this.f[96] + 3;
                    }
                    this.j[96] = true;
                    if (this.c[96] == null) {
                        try {
                            this.c[96] = Image.createImage("/Images/Dungeon Doors 3.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[96];
                }
                if (n2 >= 100 && n2 <= 103) {
                    if (!this.j[100]) {
                        this.f[100] = this.f[100] + 3;
                    }
                    this.j[100] = true;
                    if (this.c[100] == null) {
                        try {
                            this.c[100] = Image.createImage("/Images/Dungeon Doors 4.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[100];
                }
                if (n2 >= 112 && n2 <= 115) {
                    if (!this.j[112]) {
                        this.f[112] = this.f[112] + 3;
                    }
                    this.j[112] = true;
                    if (this.c[112] == null) {
                        try {
                            this.c[112] = Image.createImage("/Images/Dungeon Doors 5.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[112];
                }
                if (n2 >= 116 && n2 <= 119) {
                    if (!this.j[116]) {
                        this.f[116] = this.f[116] + 3;
                    }
                    this.j[116] = true;
                    if (this.c[116] == null) {
                        try {
                            this.c[116] = Image.createImage("/Images/Dungeon Doors 6.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[116];
                }
                if (n2 >= 124 && n2 <= 127) {
                    if (!this.j[124]) {
                        this.f[124] = this.f[124] + 3;
                    }
                    this.j[124] = true;
                    if (this.c[124] == null) {
                        try {
                            this.c[124] = Image.createImage("/Images/FairyStatue.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[124];
                }
                if (n2 >= 128 && n2 <= 131) {
                    if (!this.j[128]) {
                        this.f[128] = this.f[128] + 3;
                    }
                    this.j[128] = true;
                    if (this.c[128] == null) {
                        try {
                            this.c[128] = Image.createImage("/Images/Dungeon Doors 7.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[128];
                }
                if (n2 >= 132 && n2 <= 135) {
                    if (!this.j[132]) {
                        this.f[132] = this.f[132] + 3;
                    }
                    this.j[132] = true;
                    if (this.c[132] == null) {
                        try {
                            this.c[132] = Image.createImage("/Images/Dungeon Doors 8.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[132];
                }
                if (n2 >= 136 && n2 <= 139) {
                    if (!this.j[136]) {
                        this.f[136] = this.f[136] + 3;
                    }
                    this.j[136] = true;
                    if (this.c[136] == null) {
                        try {
                            this.c[136] = Image.createImage("/Images/Wall4.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[136];
                }
                if (n2 >= 140 && n2 <= 143) {
                    if (!this.j[140]) {
                        this.f[140] = this.f[140] + 3;
                    }
                    this.j[140] = true;
                    if (this.c[140] == null) {
                        try {
                            this.c[140] = Image.createImage("/Images/Statue02.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[140];
                }
                if (n2 >= 144 && n2 <= 147) {
                    if (!this.j[144]) {
                        this.f[144] = this.f[144] + 3;
                    }
                    this.j[144] = true;
                    if (this.c[144] == null) {
                        try {
                            this.c[144] = Image.createImage("/Images/Dungeon Doors 10.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[144];
                }
                if (n2 >= 148 && n2 <= 151) {
                    if (!this.j[148]) {
                        this.f[148] = this.f[148] + 3;
                    }
                    this.j[148] = true;
                    if (this.c[148] == null) {
                        try {
                            this.c[148] = Image.createImage("/Images/Dungeon Doors 9.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[148];
                }
                if (n2 == 152) {
                    if (!this.j[152]) {
                        this.f[152] = this.f[152] + 3;
                    }
                    this.j[152] = true;
                    if (this.c[152] == null) {
                        try {
                            this.c[152] = Image.createImage("/Images/Lightning01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[152];
                }
                if (n2 == 153) {
                    if (!this.j[153]) {
                        this.f[153] = this.f[153] + 3;
                    }
                    this.j[153] = true;
                    if (this.c[153] == null) {
                        try {
                            this.c[153] = Image.createImage("/Images/Lightning02.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[153];
                }
                if (n2 == 154) {
                    if (!this.j[154]) {
                        this.f[154] = this.f[154] + 3;
                    }
                    this.j[154] = true;
                    if (this.c[154] == null) {
                        try {
                            this.c[154] = Image.createImage("/Images/Lightning03.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[154];
                }
                if (n2 == 155) {
                    if (!this.j[155]) {
                        this.f[155] = this.f[155] + 3;
                    }
                    this.j[155] = true;
                    if (this.c[155] == null) {
                        try {
                            this.c[155] = Image.createImage("/Images/Lightning04.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[155];
                }
                if (n2 >= 80 && n2 <= 83) {
                    if (!this.j[80]) {
                        this.f[80] = this.f[80] + 3;
                    }
                    this.j[80] = true;
                    if (this.c[80] == null) {
                        try {
                            this.c[80] = Image.createImage("/Images/GreenBombTree.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[80];
                }
                if (n2 >= 84 && n2 <= 87) {
                    if (!this.j[84]) {
                        this.f[84] = this.f[84] + 3;
                    }
                    this.j[84] = true;
                    if (this.c[84] == null) {
                        try {
                            this.c[84] = Image.createImage("/Images/TealBombTree.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[84];
                }
                if (n2 >= 88 && n2 <= 91) {
                    if (!this.j[88]) {
                        this.f[88] = this.f[88] + 3;
                    }
                    this.j[88] = true;
                    if (this.c[88] == null) {
                        try {
                            this.c[88] = Image.createImage("/Images/BehindTree01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[88];
                }
                if (n2 == 92) {
                    if (!this.j[92]) {
                        this.f[92] = this.f[92] + 3;
                    }
                    this.j[92] = true;
                    if (this.c[92] == null) {
                        try {
                            this.c[92] = Image.createImage("/Images/PalmTree.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[92];
                }
                if (n2 >= 160 && n2 <= 168) {
                    if (!this.j[160]) {
                        this.f[160] = this.f[160] + 3;
                    }
                    this.j[160] = true;
                    if (this.c[160] == null) {
                        try {
                            this.c[160] = Image.createImage("/Images/Carpet2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[160];
                }
                if (n2 >= 169 && n2 <= 170) {
                    if (!this.j[169]) {
                        this.f[169] = this.f[169] + 3;
                    }
                    this.j[169] = true;
                    if (this.c[169] == null) {
                        try {
                            this.c[169] = Image.createImage("/Images/Pillar.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[169];
                }
                if (n2 >= 171 && n2 <= 174) {
                    if (!this.j[171]) {
                        this.f[171] = this.f[171] + 3;
                    }
                    this.j[171] = true;
                    if (this.c[171] == null) {
                        try {
                            this.c[171] = Image.createImage("/Images/Statue.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[171];
                }
                if (n2 >= 176 && n2 <= 184) {
                    if (!this.j[176]) {
                        this.f[176] = this.f[176] + 3;
                    }
                    this.j[176] = true;
                    if (this.c[176] == null) {
                        try {
                            this.c[176] = Image.createImage("/Images/Carpet1.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[176];
                }
                if (n2 == 185) {
                    if (!this.j[185]) {
                        this.f[185] = this.f[185] + 3;
                    }
                    this.j[185] = true;
                    if (this.c[185] == null) {
                        try {
                            this.c[185] = Image.createImage("/Images/Spikes.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[185];
                }
                if (n2 >= 186 && n2 <= 187) {
                    if (!this.j[186]) {
                        this.f[186] = this.f[186] + 3;
                    }
                    this.j[186] = true;
                    if (this.c[186] == null) {
                        try {
                            this.c[186] = Image.createImage("/Images/FloorThingy.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[186];
                }
                if (n2 >= 188 && n2 <= 191) {
                    if (!this.j[188]) {
                        this.f[188] = this.f[188] + 3;
                    }
                    this.j[188] = true;
                    if (this.c[188] == null) {
                        try {
                            this.c[188] = Image.createImage("/Images/WallLamps.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[188];
                }
                if (n2 >= 192 && n2 <= 204) {
                    if (!this.j[192]) {
                        this.f[192] = this.f[192] + 3;
                    }
                    this.j[192] = true;
                    if (this.c[192] == null) {
                        try {
                            this.c[192] = Image.createImage("/Images/AnimatedWater01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[192];
                }
                if (n2 >= 205 && n2 <= 207 || n2 == 221) {
                    if (!this.j[205]) {
                        this.f[205] = this.f[205] + 3;
                    }
                    this.j[205] = true;
                    if (this.c[205] == null) {
                        try {
                            this.c[205] = Image.createImage("/Images/Stairs01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[205];
                }
                if (n2 >= 208 && n2 <= 211) {
                    if (!this.j[208]) {
                        this.f[208] = this.f[208] + 3;
                    }
                    this.j[208] = true;
                    if (this.c[208] == null) {
                        try {
                            this.c[208] = Image.createImage("/Images/Waterfall02.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[208];
                }
                if (n2 >= 212 && n2 <= 215) {
                    if (!this.j[212]) {
                        this.f[212] = this.f[212] + 3;
                    }
                    this.j[212] = true;
                    if (this.c[212] == null) {
                        try {
                            this.c[212] = Image.createImage("/Images/Waterfall01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[212];
                }
                if (n2 >= 216 && n2 <= 220) {
                    if (!this.j[216]) {
                        this.f[216] = this.f[216] + 3;
                    }
                    this.j[216] = true;
                    if (this.c[216] == null) {
                        try {
                            this.c[216] = Image.createImage("/Images/FloorObjects.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[216];
                }
                if (n2 >= 222 && n2 <= 223 || n2 >= 237 && n2 <= 238) {
                    if (!this.j[222]) {
                        this.f[222] = this.f[222] + 3;
                    }
                    this.j[222] = true;
                    if (this.c[222] == null) {
                        try {
                            this.c[222] = Image.createImage("/Images/BigDoor01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[222];
                }
                if (n2 >= 224 && n2 <= 228) {
                    if (!this.j[224]) {
                        this.f[224] = this.f[224] + 3;
                    }
                    this.j[224] = true;
                    if (this.c[224] == null) {
                        try {
                            this.c[224] = Image.createImage("/Images/HouseTop.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[224];
                }
                if (n2 >= 229 && n2 <= 232) {
                    if (!this.j[229]) {
                        this.f[229] = this.f[229] + 3;
                    }
                    this.j[229] = true;
                    if (this.c[229] == null) {
                        try {
                            this.c[229] = Image.createImage("/Images/BehindTree03.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[229];
                }
                if (n2 >= 233 && n2 <= 236) {
                    if (!this.j[233]) {
                        this.f[233] = this.f[233] + 3;
                    }
                    this.j[233] = true;
                    if (this.c[233] == null) {
                        try {
                            this.c[233] = Image.createImage("/Images/AboveDoor01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Event img");
                            return null;
                        }
                    }
                    return this.c[233];
                }
                if (n2 < 240 || n2 > 255) break block391;
                if (!this.j[240]) {
                    this.f[240] = this.f[240] + 3;
                }
                this.j[240] = true;
                if (this.c[240] == null) {
                    try {
                        this.c[240] = Image.createImage("/Images/Entrance.png");
                    }
                    catch (IOException iOException) {
                        System.out.println("error loadin Event img");
                        return null;
                    }
                }
                return this.c[240];
            }
            catch (OutOfMemoryError outOfMemoryError) {
                System.out.println("Error... out of memory");
                System.gc();
                return this.c(n2);
            }
        }
        return null;
    }

    final Image d(int n2) {
        block226: {
            try {
                if (n2 == 0) {
                    if (!this.l[n2]) {
                        int n3 = n2;
                        this.h[n3] = this.h[n3] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Octo2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 1) {
                    if (!this.l[n2]) {
                        int n4 = n2;
                        this.h[n4] = this.h[n4] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Fire.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 2) {
                    if (!this.l[n2]) {
                        int n5 = n2;
                        this.h[n5] = this.h[n5] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster3.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 3) {
                    if (!this.l[n2]) {
                        int n6 = n2;
                        this.h[n6] = this.h[n6] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster4.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 4) {
                    if (!this.l[n2]) {
                        int n7 = n2;
                        this.h[n7] = this.h[n7] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster5.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 5) {
                    if (!this.l[n2]) {
                        int n8 = n2;
                        this.h[n8] = this.h[n8] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster6.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 6) {
                    if (!this.l[n2]) {
                        int n9 = n2;
                        this.h[n9] = this.h[n9] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster7.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 7) {
                    if (!this.l[n2]) {
                        int n10 = n2;
                        this.h[n10] = this.h[n10] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster8.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 8) {
                    if (!this.l[n2]) {
                        int n11 = n2;
                        this.h[n11] = this.h[n11] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Moldorm.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 9) {
                    if (!this.l[n2]) {
                        int n12 = n2;
                        this.h[n12] = this.h[n12] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster10.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 10) {
                    if (!this.l[n2]) {
                        int n13 = n2;
                        this.h[n13] = this.h[n13] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster11.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 11) {
                    if (!this.l[n2]) {
                        int n14 = n2;
                        this.h[n14] = this.h[n14] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster12.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 12) {
                    if (!this.l[n2]) {
                        int n15 = n2;
                        this.h[n15] = this.h[n15] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster13.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 13) {
                    if (!this.l[n2]) {
                        int n16 = n2;
                        this.h[n16] = this.h[n16] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster16.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 14) {
                    if (!this.l[n2]) {
                        int n17 = n2;
                        this.h[n17] = this.h[n17] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = this.b.a.a ? Image.createImage("/Images/Ghost2.png") : Image.createImage("/Images/Ghost2-2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 15) {
                    if (!this.l[n2]) {
                        int n18 = n2;
                        this.h[n18] = this.h[n18] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster19.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 16) {
                    if (!this.l[n2]) {
                        int n19 = n2;
                        this.h[n19] = this.h[n19] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Boss1.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 17) {
                    if (!this.l[n2]) {
                        int n20 = n2;
                        this.h[n20] = this.h[n20] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Boss2-2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 18) {
                    if (!this.l[n2]) {
                        int n21 = n2;
                        this.h[n21] = this.h[n21] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Boss3.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 19) {
                    if (!this.l[n2]) {
                        int n22 = n2;
                        this.h[n22] = this.h[n22] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster18.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 20) {
                    if (!this.l[n2]) {
                        int n23 = n2;
                        this.h[n23] = this.h[n23] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Boss4-2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 21) {
                    if (!this.l[n2]) {
                        int n24 = n2;
                        this.h[n24] = this.h[n24] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = this.b.a.a ? Image.createImage("/Images/Boss5-2.png") : Image.createImage("/Images/Boss5-3.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 22) {
                    if (!this.l[n2]) {
                        int n25 = n2;
                        this.h[n25] = this.h[n25] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Boss6.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 23) {
                    if (!this.l[n2]) {
                        int n26 = n2;
                        this.h[n26] = this.h[n26] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Ganondorf.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 24) {
                    if (!this.l[n2]) {
                        int n27 = n2;
                        this.h[n27] = this.h[n27] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Ganon.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 25) {
                    if (!this.l[n2]) {
                        int n28 = n2;
                        this.h[n28] = this.h[n28] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/BigOcto.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 26) {
                    if (!this.l[n2]) {
                        int n29 = n2;
                        this.h[n29] = this.h[n29] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/waterfly.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 27) {
                    if (!this.l[n2]) {
                        int n30 = n2;
                        this.h[n30] = this.h[n30] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Knight2.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 28) {
                    if (!this.l[n2]) {
                        int n31 = n2;
                        this.h[n31] = this.h[n31] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/wormbabies.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 29) {
                    if (!this.l[n2]) {
                        int n32 = n2;
                        this.h[n32] = this.h[n32] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/WormBoss.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 30) {
                    if (!this.l[n2]) {
                        int n33 = n2;
                        this.h[n33] = this.h[n33] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Komune.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 31) {
                    if (!this.l[n2]) {
                        int n34 = n2;
                        this.h[n34] = this.h[n34] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Kotakune.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 32) {
                    if (!this.l[n2]) {
                        int n35 = n2;
                        this.h[n35] = this.h[n35] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster32.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 33) {
                    if (!this.l[n2]) {
                        int n36 = n2;
                        this.h[n36] = this.h[n36] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster33.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 34) {
                    if (!this.l[n2]) {
                        int n37 = n2;
                        this.h[n37] = this.h[n37] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Fairy.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 35) {
                    if (!this.l[n2]) {
                        int n38 = n2;
                        this.h[n38] = this.h[n38] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Moblin01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 36) {
                    if (!this.l[n2]) {
                        int n39 = n2;
                        this.h[n39] = this.h[n39] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/GreenFlames.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 37) {
                    if (!this.l[n2]) {
                        int n40 = n2;
                        this.h[n40] = this.h[n40] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Digdogger.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 38) {
                    if (!this.l[n2]) {
                        int n41 = n2;
                        this.h[n41] = this.h[n41] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Gooma.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 >= 128 && n2 <= 130) {
                    if (!this.l[128]) {
                        this.h[128] = this.h[128] + 3;
                    }
                    this.l[128] = true;
                    if (this.a[128] == null) {
                        try {
                            this.a[128] = Image.createImage("/Images/Monster128-130.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[128];
                }
                if (n2 == 131) {
                    if (!this.l[n2]) {
                        int n42 = n2;
                        this.h[n42] = this.h[n42] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Monster13.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 132) {
                    if (!this.l[n2]) {
                        int n43 = n2;
                        this.h[n43] = this.h[n43] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Fisherman01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 133) {
                    if (!this.l[n2]) {
                        int n44 = n2;
                        this.h[n44] = this.h[n44] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/Zelda01.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 == 134) {
                    if (!this.l[n2]) {
                        int n45 = n2;
                        this.h[n45] = this.h[n45] + 3;
                    }
                    this.l[n2] = true;
                    if (this.a[n2] == null) {
                        try {
                            this.a[n2] = Image.createImage("/Images/ZeldaBow.png");
                        }
                        catch (IOException iOException) {
                            System.out.println("error loadin Map img");
                            return null;
                        }
                    }
                    return this.a[n2];
                }
                if (n2 != 135) break block226;
                if (!this.l[n2]) {
                    int n46 = n2;
                    this.h[n46] = this.h[n46] + 3;
                }
                this.l[n2] = true;
                if (this.a[n2] == null) {
                    try {
                        this.a[n2] = this.b.a.e.V == 2 ? Image.createImage("/Images/GreenTingle.png") : Image.createImage("/Images/RedTingle.png");
                    }
                    catch (IOException iOException) {
                        System.out.println("error loadin Map img");
                        return null;
                    }
                }
                return this.a[n2];
            }
            catch (OutOfMemoryError outOfMemoryError) {
                System.out.println("Error... out of memory");
                System.gc();
                return this.d(n2);
            }
        }
        return null;
    }
}

