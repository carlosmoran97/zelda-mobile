/*
 * Decompiled with CFR 0.152.
 */
import java.util.TimerTask;

final class m
extends TimerTask {
    private final Zelda a;

    m(Zelda zelda) {
        this.a = zelda;
    }

    public final void run() {
        Zelda.a(this.a).repaint();
        if (Zelda.a((Zelda)this.a).r) {
            if (Zelda.a((Zelda)this.a).p == 0) {
                Zelda.a(this.a).removeCommand(Zelda.b(this.a));
                Zelda.a(this.a).removeCommand(Zelda.c(this.a));
                Zelda.a(this.a).removeCommand(Zelda.d(this.a));
                Zelda.a(this.a).removeCommand(Zelda.e(this.a));
                Zelda.a(this.a).addCommand(Zelda.f(this.a));
                Zelda.a(this.a).removeCommand(Zelda.g(this.a));
                Zelda.a(this.a).removeCommand(Zelda.h(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 7) {
                Zelda.a(this.a).removeCommand(Zelda.e(this.a));
                Zelda.a(this.a).removeCommand(Zelda.f(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 30) {
                Zelda.a(this.a).addCommand(Zelda.i(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 100) {
                Zelda.a(this.a).removeCommand(Zelda.b(this.a));
                Zelda.a(this.a).removeCommand(Zelda.f(this.a));
                Zelda.a(this.a).removeCommand(Zelda.c(this.a));
                Zelda.a(this.a).removeCommand(Zelda.d(this.a));
                Zelda.a(this.a).removeCommand(Zelda.e(this.a));
                Zelda.a(this.a).removeCommand(Zelda.g(this.a));
                Zelda.a(this.a).removeCommand(Zelda.i(this.a));
                Zelda.a(this.a).addCommand(Zelda.h(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 8) {
                Zelda.a(this.a).removeCommand(Zelda.f(this.a));
                Zelda.a(this.a).addCommand(Zelda.e(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 10) {
                Zelda.a(this.a).removeCommand(Zelda.b(this.a));
                Zelda.a(this.a).removeCommand(Zelda.c(this.a));
                Zelda.a(this.a).removeCommand(Zelda.d(this.a));
                Zelda.a(this.a).removeCommand(Zelda.f(this.a));
                Zelda.a(this.a).removeCommand(Zelda.g(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 1) {
                Zelda.a(this.a).removeCommand(Zelda.f(this.a));
                Zelda.a(this.a).removeCommand(Zelda.c(this.a));
                Zelda.a(this.a).removeCommand(Zelda.d(this.a));
                Zelda.a(this.a).removeCommand(Zelda.e(this.a));
                Zelda.a(this.a).addCommand(Zelda.b(this.a));
                Zelda.a(this.a).addCommand(Zelda.g(this.a));
                Zelda.a(this.a).removeCommand(Zelda.i(this.a));
            }
            if (Zelda.a((Zelda)this.a).p == 1 && Zelda.a((Zelda)this.a).m) {
                Zelda.a(this.a).removeCommand(Zelda.b(this.a));
                Zelda.a(this.a).removeCommand(Zelda.f(this.a));
                Zelda.a(this.a).removeCommand(Zelda.c(this.a));
                Zelda.a(this.a).removeCommand(Zelda.d(this.a));
                Zelda.a(this.a).removeCommand(Zelda.e(this.a));
                Zelda.a(this.a).removeCommand(Zelda.g(this.a));
                Zelda.a(this.a).addCommand(Zelda.c(this.a));
                Zelda.a(this.a).addCommand(Zelda.d(this.a));
                Zelda.a(this.a).removeCommand(Zelda.i(this.a));
            }
            Zelda.a((Zelda)this.a).r = false;
        }
        if (Zelda.a((Zelda)this.a).s) {
            this.a.destroyApp(false);
            return;
        }
    }
}

