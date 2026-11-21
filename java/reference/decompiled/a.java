/*
 * Decompiled with CFR 0.152.
 */
import java.util.TimerTask;

final class a
extends TimerTask {
    private final Zelda a;

    a(Zelda zelda) {
        this.a = zelda;
    }

    public final void run() {
        Zelda.a((Zelda)this.a).h = true;
        Zelda.a((Zelda)this.a).f = true;
    }
}

