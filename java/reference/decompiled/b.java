/*
 * Decompiled with CFR 0.152.
 */
import java.io.InputStream;
import javax.microedition.media.Manager;
import javax.microedition.media.Player;
import javax.microedition.media.PlayerListener;
import javax.microedition.media.control.VolumeControl;

public final class b
implements Runnable,
PlayerListener {
    private Thread f;
    boolean a = true;
    private VolumeControl g;
    private VolumeControl h;
    Player b;
    Player c;
    int d = 0;
    private int i = 0;
    private boolean j;
    private boolean k;
    private int l;
    private int m;
    private int n;
    private int o = 0;
    private long p = 0L;
    boolean e = false;
    private int q;
    private int r;

    b() {
    }

    public b(boolean bl, boolean bl2, int n2, int n3, int n4, int n5) {
        if (this.a) {
            System.out.println("Media constructor");
            this.j = false;
            this.k = true;
            this.l = -2;
            this.m = n4;
            this.n = n5;
            v0.f = new Thread(this);
            this.e = false;
            this.f.start();
            this.f.setPriority(1);
            return;
        }
        this.m = 0;
        this.n = 0;
    }

    /*
     * Enabled aggressive block sorting
     * Enabled unnecessary exception pruning
     * Enabled aggressive exception aggregation
     */
    public final void run() {
        if (this.k) {
            this.k = this.a();
        }
        this.b = null;
        this.l = -10;
        this.a(-1, 0);
        block28: while (!this.e) {
            try {
                int n2;
                b b2;
                block42: {
                    block43: {
                        Thread.sleep(10L);
                        b2 = this;
                        if (b2.o == 0) break block43;
                        long l2 = System.currentTimeMillis();
                        switch (b2.o) {
                            case 1: {
                                if (l2 - b2.p > 375L) break;
                                break block43;
                            }
                            case 2: {
                                if (l2 - b2.p > 450L) break;
                                break block43;
                            }
                            case 3: {
                                if (l2 - b2.p > 320L) break;
                                break block43;
                            }
                            case 4: {
                                if (l2 - b2.p > 500L) break;
                                break block43;
                            }
                            case 5: {
                                if (l2 - b2.p > 100L) break;
                                break block43;
                            }
                            case 6: {
                                if (l2 - b2.p > 1300L) break;
                                break block43;
                            }
                            case 7: {
                                if (l2 - b2.p > 2300L) break;
                                break block43;
                            }
                            case 8: {
                                if (l2 - b2.p > 500L) break;
                                break block43;
                            }
                            case 9: {
                                if (l2 - b2.p <= 1100L) break block43;
                            }
                        }
                        try {
                            if (b2.c.getState() == 400) {
                                b2.c.stop();
                            }
                            b2.o = 0;
                        }
                        catch (Exception exception) {
                            System.out.println(exception.toString());
                        }
                    }
                    n2 = this.d;
                    b2 = this;
                    if (b2.d == 0 && b2.i == 0) continue;
                    if (b2.d == 0 && b2.i != 0 && b2.o == 0) {
                        n2 = b2.d = b2.i;
                        b2.i = 0;
                    }
                    if (!b2.a || !b2.k || b2.n == 0) continue;
                    if (b2.o != 0) {
                        try {
                            if (b2.o == 6 || b2.o == 7 || b2.o == 9) {
                                if (b2.i == 0 && b2.d == 7 && b2.o != 9) {
                                    b2.i = b2.d;
                                }
                                b2.d = 0;
                                continue;
                            }
                            if (b2.c.getState() != 400) break block42;
                            b2.c.stop();
                            Thread.sleep(10L);
                        }
                        catch (Exception exception) {
                            System.out.println(exception.toString());
                            continue;
                        }
                    }
                }
                try {
                    switch (n2) {
                        case 1: {
                            if (b2.o != 1 && b2.o != 2) {
                                b2.o = 1;
                                b2.c.setMediaTime(39000L);
                                long l3 = 0L;
                                b2.p = System.currentTimeMillis();
                                b2.c.start();
                                break;
                            }
                            if (b2.o == 1) {
                                b2.o = 2;
                                b2.c.setMediaTime(585000L);
                                long l4 = 0L;
                                b2.p = System.currentTimeMillis();
                                b2.c.start();
                                break;
                            }
                            if (b2.o != 2) break;
                            b2.o = 3;
                            b2.c.setMediaTime(1145000L);
                            long l5 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                        case 2: {
                            b2.o = 4;
                            b2.c.setMediaTime(1511000L);
                            long l6 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                        case 3: {
                            b2.o = 5;
                            b2.c.setMediaTime(2057000L);
                            long l7 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                        case 4: {
                            if (b2.o == 6) {
                                b2.d = 0;
                                continue block28;
                            }
                            b2.o = 6;
                            b2.c.setMediaTime(2252000L);
                            long l8 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                        case 5: {
                            if (b2.o == 7) {
                                b2.d = 0;
                                continue block28;
                            }
                            b2.o = 7;
                            b2.c.setMediaTime(5000000L);
                            long l9 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                        case 6: {
                            if (b2.o == 8) {
                                b2.d = 0;
                                continue block28;
                            }
                            b2.o = 8;
                            b2.c.setMediaTime(7450000L);
                            long l10 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                        case 7: {
                            if (b2.o == 9) {
                                b2.d = 0;
                                continue block28;
                            }
                            b2.o = 9;
                            b2.c.setMediaTime(3760000L);
                            long l11 = 0L;
                            b2.p = System.currentTimeMillis();
                            b2.c.start();
                            break;
                        }
                    }
                    b2.d = 0;
                }
                catch (Exception exception) {
                    System.out.println(exception.toString());
                }
                continue;
            }
            catch (Exception exception) {
                this.a = false;
                continue;
            }
            break;
        }
        return;
    }

    public final void a(int n2, int n3) {
        System.out.println("MapChange word=" + n2 + "Map =" + n3);
        if (this.a) {
            try {
                boolean bl = false;
                if (n2 == 0 && (n3 == 58 || n3 == 68)) {
                    bl = true;
                }
                if (n2 == 9 && n3 == 15) {
                    bl = true;
                }
                if (n2 == 10 && n3 == 17) {
                    bl = true;
                }
                if (n2 == 11 && n3 == 28) {
                    bl = true;
                }
                if (this.q == 10 && this.r == 17) {
                    bl = true;
                }
                if (this.q == 11 && this.r == 28) {
                    bl = true;
                }
                if (this.q == 9 && this.r == 15) {
                    bl = true;
                }
                if (this.q == 0 && (this.r == 58 || this.r == 68)) {
                    bl = true;
                }
                if (n2 == 20) {
                    bl = true;
                }
                if (n2 != this.l || bl) {
                    this.l = n2;
                    if (n2 == -1) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda22m.mp3");
                        } else {
                            this.a("/Sound/midi/intro.mid");
                        }
                    }
                    if (n2 == 0) {
                        if (n3 != 58 && n3 != 68) {
                            if (this.j) {
                                this.a("/Sound/Mp3/xelda12m.mp3");
                            } else {
                                this.a("/Sound/midi/xelda1.mid");
                            }
                        } else {
                            this.a("/Sound/midi/Z3 Fairy.mid");
                        }
                    }
                    if (n2 == 1) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda12m.mp3");
                        } else {
                            this.a("/Sound/midi/z3cave.mid");
                        }
                    }
                    if (n2 == 3) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda62m.mp3");
                        } else {
                            this.a("/Sound/midi/Xelda2.mid");
                        }
                    }
                    if (n2 == 4) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda62m.mp3");
                        } else {
                            this.a("/Sound/midi/Xelda2.mid");
                        }
                    }
                    if (n2 == 5) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda62m.mp3");
                        } else {
                            this.a("/Sound/midi/Xelda2.mid");
                        }
                    }
                    if (n2 == 6) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda62m.mp3");
                        } else {
                            this.a("/Sound/midi/Xelda2.mid");
                        }
                    }
                    if (n2 == 7) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda62m.mp3");
                        } else {
                            this.a("/Sound/midi/Xelda2.mid");
                        }
                    }
                    if (n2 == 8) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda62m.mp3");
                        } else {
                            this.a("/Sound/midi/Xelda2.mid");
                        }
                    }
                    if (n2 == 9) {
                        if (n3 != 15) {
                            if (this.j) {
                                this.a("/Sound/Mp3/xelda62m.mp3");
                            } else {
                                this.a("/Sound/midi/Xelda2.mid");
                            }
                        } else {
                            this.a("/Sound/midi/Z3-Boss.mid");
                        }
                    }
                    if (n2 == 10) {
                        if (n3 != 17) {
                            if (this.j) {
                                this.a("/Sound/Mp3/xelda62m.mp3");
                            } else {
                                this.a("/Sound/midi/Xelda2.mid");
                            }
                        } else {
                            this.a("/Sound/midi/Z3-Boss.mid");
                        }
                    }
                    if (n2 == 11) {
                        if (n3 != 28) {
                            if (this.j) {
                                this.a("/Sound/Mp3/xelda62m.mp3");
                            } else {
                                this.a("/Sound/midi/Z4 Level 3.mid");
                            }
                        } else {
                            this.a("/Sound/midi/Z4 Mini Boss.mid");
                        }
                    }
                    if (n2 == 12) {
                        this.a("/Sound/midi/Z4 Mini Boss.mid");
                    }
                    if (n2 == 14) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda12m.mp3");
                        } else {
                            this.a("/Sound/midi/z3cave.mid");
                        }
                    }
                    if (n2 == 15) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda12m.mp3");
                        } else {
                            this.a("/Sound/midi/z3cave.mid");
                        }
                    }
                    if (n2 == 16) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda12m.mp3");
                        } else {
                            this.a("/Sound/midi/z3cave.mid");
                        }
                    }
                    if (n2 == 17) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda12m.mp3");
                        } else {
                            this.a("/Sound/midi/z3cave.mid");
                        }
                    }
                    if (n2 == 20) {
                        if (n3 == 1) {
                            this.a("/Sound/midi/z3cave.mid");
                        } else if (n3 == 3) {
                            this.a("/Sound/midi/Z4-Credits.mid");
                        } else {
                            this.a("/Sound/midi/Z3 Final Battle.mid");
                        }
                    }
                    if (n2 == 100) {
                        if (this.j) {
                            this.a("/Sound/Mp3/xelda12m.mp3");
                        } else {
                            this.a("/Sound/midi/Z3 Lost Forest.mid");
                        }
                    }
                }
                this.q = n2;
                this.r = n3;
                return;
            }
            catch (Exception exception) {}
        }
        this.q = n2;
        this.r = n3;
    }

    final boolean a(int n2) {
        System.out.println("SetFxVolume: " + n2);
        if (this.a) {
            try {
                this.n = n2;
                this.h = (VolumeControl)this.c.getControl("VolumeControl");
                if (this.h != null) {
                    this.h.setLevel(this.n);
                }
            }
            catch (Exception exception) {
                System.out.println(exception.toString());
                return false;
            }
            return true;
        }
        return false;
    }

    /*
     * Enabled force condition propagation
     * Lifted jumps to return sites
     */
    final void b(int n2) {
        System.out.println("SetMusicVolume: " + n2);
        if (!this.a) return;
        try {
            this.g = null;
            this.g = (VolumeControl)this.b.getControl("VolumeControl");
            if (this.g != null) {
                this.m = n2;
                this.g.setLevel(this.m);
                if (this.m == 0) {
                    if (this.b.getState() != 400) return;
                    this.b.stop();
                    return;
                } else {
                    this.b.start();
                }
                return;
            }
            if (this.b == null) return;
            if (this.m > 0) {
                this.b.start();
                return;
            }
            if (this.b.getState() != 400) return;
            this.b.stop();
            return;
        }
        catch (Exception exception) {
            System.out.println(exception.toString());
            return;
        }
    }

    private boolean a(String object) {
        System.out.println("playFile");
        if (this.a) {
            try {
                if (this.b != null) {
                    if (this.b.getState() == 400) {
                        this.b.stop();
                    }
                    this.b.removePlayerListener(this);
                    this.b.close();
                    this.b = null;
                    System.gc();
                }
                object = this.getClass().getResourceAsStream((String)object);
                this.b = this.j ? Manager.createPlayer((InputStream)object, "audio/mpeg") : Manager.createPlayer((InputStream)object, "audio/midi");
                this.b.realize();
                this.b.addPlayerListener(this);
                this.g = null;
                this.g = (VolumeControl)this.b.getControl("VolumeControl");
                if (this.g != null) {
                    this.g.setLevel(this.m);
                }
                this.b.prefetch();
                this.b.setLoopCount(-1);
                if (this.m > 0) {
                    this.b.start();
                }
            }
            catch (Exception exception) {
                System.out.println(exception.toString());
                this.a = false;
                return false;
            }
            return true;
        }
        return false;
    }

    public final void playerUpdate(Player player, String string, Object object) {
        System.out.println("playerUpdate");
        if (this.a) {
            if (string == "started") {
                if (player == this.b) {
                    System.out.println("Player STARTED");
                } else {
                    System.out.println("sfx STARTED");
                }
            }
            if (string == "stopped") {
                System.out.println("Player STOPPED");
            }
            if (string == "volumeChanged") {
                System.out.println("Player VOLUME_CHANGED");
            }
            return;
        }
    }

    private boolean a() {
        System.out.println("LoadSfxPlayers");
        if (this.a) {
            try {
                InputStream inputStream = this.getClass().getResourceAsStream("/Sound/Effects.wav");
                try {
                    this.c = Manager.createPlayer(inputStream, "audio/X-wav");
                }
                catch (Exception exception) {
                    System.out.println(exception.toString());
                    this.c = Manager.createPlayer(inputStream, "audio/wav");
                }
                this.c.realize();
                this.c.addPlayerListener(this);
                this.c.setLoopCount(1);
                this.c.prefetch();
                this.h = (VolumeControl)this.c.getControl("VolumeControl");
                if (this.h != null) {
                    this.h.setLevel(this.n);
                }
            }
            catch (Exception exception) {
                System.out.println(exception.toString());
                this.a = false;
                return false;
            }
            return true;
        }
        return false;
    }
}

