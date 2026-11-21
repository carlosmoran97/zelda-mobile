/*
 * Decompiled with CFR 0.152.
 */
import java.util.Timer;
import java.util.TimerTask;
import javax.microedition.lcdui.Command;
import javax.microedition.lcdui.CommandListener;
import javax.microedition.lcdui.Display;
import javax.microedition.lcdui.Displayable;
import javax.microedition.midlet.MIDlet;

public class Zelda
extends MIDlet
implements CommandListener {
    private k a;
    private Command b;
    private Command c;
    private Command d;
    private Command e;
    private Command f;
    private Command g;
    private Command h;
    private Command i;
    private Command j;
    private Display k;
    private m l = new m(this);
    private Timer m = new Timer();
    private a n;

    public Zelda() {
        new Timer();
        this.n = new a(this);
        this.k = Display.getDisplay(this);
        this.a = new k(this.k, this);
        this.f = new Command("Start", 1, 1);
        this.i = new Command("Play", 1, 1);
        this.c = new Command("Menu", 1, 1);
        this.g = new Command("Settings", 1, 2);
        this.b = new Command("Exit", 1, 2);
        this.d = new Command("Save & exit", 1, 2);
        this.e = new Command("Return", 1, 1);
        this.h = new Command("Default", 1, 1);
        this.j = new Command("Main Menu", 1, 1);
        this.a.addCommand(this.f);
        this.a.setCommandListener(this);
        this.m.scheduleAtFixedRate((TimerTask)this.l, 100L, 70L);
        this.m.scheduleAtFixedRate((TimerTask)this.n, 100L, 1000L);
    }

    public void startApp() {
        k k2 = ((Zelda)((Object)k2)).a;
        if (k2.c) {
            k2.c = false;
            k2.b.setCurrent(k2);
        }
    }

    public void pauseApp() {
        k k2 = ((Zelda)((Object)k2)).a;
        if (!k2.c) {
            k2.c = true;
        }
    }

    public void destroyApp(boolean bl) {
        this.a.getClass();
        b b2 = this.a.H;
        if (b2.a) {
            try {
                b2.e = true;
                Thread.sleep(100L);
                if (b2.b.getState() == 400) {
                    b2.b.stop();
                }
                if (b2.c.getState() == 400) {
                    b2.c.stop();
                }
                b2.b.close();
                b2.c.close();
            }
            catch (Exception exception) {
                System.out.println(exception.toString());
            }
        }
        this.a.a();
        this.k.setCurrent(null);
        System.gc();
        this.notifyDestroyed();
    }

    public void commandAction(Command command, Displayable displayable) {
        if (command == this.f) {
            this.a.removeCommand(this.f);
            this.a.removeCommand(this.h);
            this.a.p = 30;
            this.a.r = true;
            this.a.o = 0;
            this.a.m = true;
            this.a.n = 4;
            this.a.b();
            this.a.c();
            this.a.m = false;
            this.a.n = 0;
        }
        if (command == this.h) {
            this.a.y = 50;
            this.a.z = 56;
            this.a.A = 52;
            this.a.B = 54;
            this.a.C = 53;
            this.a.D = 55;
            this.a.E = 57;
            this.a.w = true;
        }
        if (command == this.b) {
            this.destroyApp(false);
            this.notifyDestroyed();
        }
        if (command == this.d) {
            this.a.d();
            this.destroyApp(false);
            this.notifyDestroyed();
        }
        if (command == this.g) {
            this.a.removeCommand(this.g);
            this.a.removeCommand(this.c);
            this.a.addCommand(this.e);
            this.a.removeCommand(this.h);
            this.a.q = this.a.p;
            this.a.p = 7;
        }
        if (command == this.i) {
            this.a.a(this.a.C);
            this.a.removeCommand(this.i);
        }
        if (command == this.e) {
            this.a.removeCommand(this.e);
            this.a.removeCommand(this.d);
            this.a.addCommand(this.c);
            this.a.removeCommand(this.h);
            this.a.addCommand(this.g);
            this.a.a(false);
            this.a.p = 1;
            this.a.n = 0;
            this.a.m = false;
        }
        if (command == this.c) {
            if (this.a.p == 0 || this.a.p == -1) {
                return;
            }
            this.a.removeCommand(this.c);
            this.a.removeCommand(this.g);
            this.a.addCommand(this.d);
            this.a.addCommand(this.e);
            this.a.removeCommand(this.h);
            this.a.n = 1;
            this.a.m = true;
            this.a.a(true);
        }
        if (command == this.j) {
            this.a.p = 0;
            this.a.u = false;
            this.a.v = true;
        }
    }

    static k a(Zelda zelda) {
        return zelda.a;
    }

    static Command b(Zelda zelda) {
        return zelda.c;
    }

    static Command c(Zelda zelda) {
        return zelda.e;
    }

    static Command d(Zelda zelda) {
        return zelda.d;
    }

    static Command e(Zelda zelda) {
        return zelda.h;
    }

    static Command f(Zelda zelda) {
        return zelda.f;
    }

    static Command g(Zelda zelda) {
        return zelda.g;
    }

    static Command h(Zelda zelda) {
        return zelda.j;
    }

    static Command i(Zelda zelda) {
        return zelda.i;
    }
}

