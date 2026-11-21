// Development-only sprite/cache oracle. Executes the unmodified original JAR.
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
import javax.microedition.lcdui.*;
import org.recompile.mobile.*;
import org.recompile.freej2me.Config;

public class CatalogOracle {
    static Object field(Object o, String name) throws Exception { return ReferenceHarness.field(o, name); }
    static void set(Object o, String name, Object value) throws Exception {
        Field f = o.getClass().getDeclaredField(name); f.setAccessible(true); f.set(o, value);
    }
    static Object call(Object o, String name, Class<?>[] types, Object... args) throws Exception {
        Method method = o.getClass().getDeclaredMethod(name, types); method.setAccessible(true);
        return method.invoke(o, args);
    }
    static String nonzero(Object array) {
        StringJoiner values = new StringJoiner(",", "[", "]");
        for (int i = 0; i < Array.getLength(array); i++) {
            Object raw = Array.get(array, i);
            int value = raw instanceof Boolean ? ((Boolean)raw ? 1 : 0) : (Integer)raw;
            if (value != 0) values.add("[" + i + "," + value + "]");
        }
        return values.toString();
    }
    public static void main(String[] args) throws Exception { try {
        Mobile.minLogLevel = Mobile.LOG_NONE; Mobile.config = new Config();
        MobilePlatform platform = new MobilePlatform(240, 320); Mobile.setPlatform(platform, () -> {});
        platform.dataPath = new File(args[1], "catalog-reference-saves").getAbsolutePath() + "/";
        if (!platform.load(new File(args[0]).toURI().toString())) throw new Exception("load failed");
        platform.runJar(); Thread.sleep(200);
        Object zelda = Mobile.midlet; ((java.util.Timer)field(zelda, "m")).cancel(); Thread.sleep(100);
        Object canvas = field(zelda, "a"); Image screen = Image.createImage(240, 320);
        ((CommandListener)zelda).commandAction((Command)field(zelda, "f"), (Displayable)canvas);
        ReferenceHarness.paint(canvas, screen); call(canvas, "keyPressed", new Class<?>[]{int.class}, 53);
        for (int i = 0; i < 20; i++) ReferenceHarness.paint(canvas, screen);
        Object world = field(canvas, "d"), player = field(canvas, "e");
        Class<?> resourceClass = Class.forName("o", true, canvas.getClass().getClassLoader());
        Constructor<?> constructor = resourceClass.getDeclaredConstructor(world.getClass()); constructor.setAccessible(true);
        String[][] families = {{"a","d","g","k","a"},{"b","e","i","m","c"},{"c","c","f","j","b"},{"d","a","h","l","d"}};
        StringJoiner rows = new StringJoiner(",\n", "[\n", "\n]\n");
        for (int variant = 0; variant < 2; variant++) {
            boolean classic = variant == 0; int color = variant == 0 ? 0 : 2;
            set(canvas, "a", classic); set(player, "V", (byte)color);
            for (int family = 0; family < families.length; family++) {
                String[] keys = families[family];
                for (int kind = 0; kind < 256; kind++) {
                    Object resources = constructor.newInstance(world);
                    Image image = (Image)call(resources, keys[0], new Class<?>[]{int.class}, kind);
                    int width = 0, height = 0, hash = 0, slot = -1;
                    if (image != null) {
                        width = image.getWidth(); height = image.getHeight(); hash = (int)2166136261L;
                        for (int y = 0; y < height; y++) for (int x = 0; x < width; x++) {
                            int pixel = image.getCanvas().getRGB(x, y);
                            int alpha = pixel >>> 24;
                            int[] bytes = alpha == 255 ? new int[]{(pixel >>> 16) & 255, (pixel >>> 8) & 255, pixel & 255, alpha} : new int[]{0, 0, 0, alpha};
                            for (int value : bytes) hash = (hash ^ value) * 16777619;
                        }
                    }
                    Image[] images = (Image[])field(resources, keys[1]);
                    if (image != null) for (int i = 0; i < images.length; i++) if (images[i] == image) { slot = i; break; }
                    Image again = (Image)call(resources, keys[0], new Class<?>[]{int.class}, kind);
                    String ages = nonzero(field(resources, keys[2])), used = nonzero(field(resources, keys[3]));
                    StringJoiner retained = new StringJoiner(",", "[", "]");
                    for (int i = 0; i < 3; i++) {
                        call(resources, keys[4], new Class<?>[]{});
                        retained.add(Boolean.toString(slot >= 0 && images[slot] != null));
                    }
                    rows.add("[" + classic + "," + color + "," + family + "," + kind + "," + width + "," + height + "," + Integer.toUnsignedLong(hash) + "," + slot + "," + (image == again) + "," + ages + "," + used + "," + retained + "]");
                }
            }
        }
        try (Writer writer = new FileWriter(new File(args[1], "reference-catalog.json"))) { writer.write(rows.toString()); }
        System.out.println("2048 sprite/cache cases checked against original JAR"); System.exit(0);
    } catch (Throwable error) { error.printStackTrace(); System.exit(1); } }
}
