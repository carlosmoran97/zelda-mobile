import java.io.*;
import java.lang.reflect.*;
import javax.microedition.lcdui.*;
import org.recompile.mobile.*;
import org.recompile.freej2me.Config;

public class BehaviorOracle {
    static Object field(Object o,String n)throws Exception{return ReferenceHarness.field(o,n);}
    static Object call(Object o,String n,Class<?>[]t,Object...a)throws Exception{Method m=o.getClass().getDeclaredMethod(n,t);m.setAccessible(true);return m.invoke(o,a);}
    static String snapshot(Object c,Object p,String label)throws Exception{
        byte[]b=(byte[])call(p,"b",new Class<?>[]{});StringBuilder hex=new StringBuilder();for(byte v:b)hex.append(String.format("%02x",v&255));
        return "{\"label\":\""+label+"\",\"state\":"+field(c,"p")+",\"x\":"+field(p,"i")+",\"y\":"+field(p,"j")+",\"health\":"+field(p,"H")+",\"sword\":"+field(p,"q")+",\"inventory\":"+field(c,"m")+",\"save\":\""+hex+"\"}";
    }
    public static void main(String[]args)throws Exception{try{
        Mobile.minLogLevel=Mobile.LOG_NONE;Mobile.config=new Config();MobilePlatform platform=new MobilePlatform(240,320);Mobile.setPlatform(platform,()->{});
        platform.dataPath=new File(args[1],"behavior-reference-saves").getAbsolutePath()+"/";
        if(!platform.load(new File(args[0]).toURI().toString()))throw new Exception("load failed");platform.runJar();Thread.sleep(200);
        Object z=Mobile.midlet;((java.util.Timer)field(z,"m")).cancel();Thread.sleep(100);Object c=field(z,"a");Image image=Image.createImage(240,320);
        ((CommandListener)z).commandAction((Command)field(z,"f"),(Displayable)c);ReferenceHarness.paint(c,image);call(c,"keyPressed",new Class<?>[]{int.class},53);for(int i=0;i<20;i++)ReferenceHarness.paint(c,image);
        Object p=field(c,"e"),sound=field(c,"H");StringBuilder out=new StringBuilder("[\n");out.append(snapshot(c,p,"opening"));
        int[]keys={56,54};String[]labels={"down","right"};for(int n=0;n<2;n++){call(c,"keyPressed",new Class<?>[]{int.class},keys[n]);for(int i=0;i<8;i++)ReferenceHarness.paint(c,image);call(c,"keyReleased",new Class<?>[]{int.class},keys[n]);out.append(",\n").append(snapshot(c,p,labels[n]));}
        Class<?> itemClass=Class.forName("c",true,c.getClass().getClassLoader()),hero=p.getClass(),snd=sound.getClass(),canvasClass=c.getClass();
        Constructor<?> ctor=itemClass.getDeclaredConstructor(Image.class,int.class,int.class,int.class,int.class,hero);ctor.setAccessible(true);
        for(int kind:new int[]{130,3}){if(kind==3){Field h=hero.getDeclaredField("H");h.setAccessible(true);h.setInt(p,4);}Object item=ctor.newInstance(null,kind,0,0,kind==130?1:0,p);call(item,"a",new Class<?>[]{hero,snd,canvasClass,boolean.class},p,sound,c,false);for(int i=0;i<30;i++)ReferenceHarness.paint(c,image);out.append(",\n").append(snapshot(c,p,kind==130?"sword pickup":"heart pickup"));}
        call(c,"keyPressed",new Class<?>[]{int.class},53);call(c,"keyReleased",new Class<?>[]{int.class},53);call(c,"keyPressed",new Class<?>[]{int.class},53);for(int i=0;i<8;i++)ReferenceHarness.paint(c,image);call(c,"keyReleased",new Class<?>[]{int.class},53);out.append(",\n").append(snapshot(c,p,"sword action"));
        call(c,"keyPressed",new Class<?>[]{int.class},57);for(int i=0;i<2;i++)ReferenceHarness.paint(c,image);call(c,"keyReleased",new Class<?>[]{int.class},57);out.append(",\n").append(snapshot(c,p,"inventory")).append("\n]\n");
        try(Writer w=new FileWriter(new File(args[1],"reference-behavior.json"))){w.write(out.toString());}System.out.println("Behavior oracle complete");System.exit(0);
    }catch(Throwable e){e.printStackTrace();System.exit(1);}}
}
