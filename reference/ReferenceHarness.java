// Development-only oracle: execute the unmodified JAR using FreeJ2ME's services.
// Neither this harness nor FreeJ2ME is included in the native application.
import java.io.File;
import java.lang.reflect.*;
import javax.imageio.ImageIO;
import javax.microedition.lcdui.*;
import org.recompile.mobile.*;
import org.recompile.freej2me.Config;

public class ReferenceHarness {
    static Object field(Object o,String name)throws Exception{Class<?> c=o.getClass();while(c!=null){try{Field f=c.getDeclaredField(name);f.setAccessible(true);return f.get(o);}catch(NoSuchFieldException e){c=c.getSuperclass();}}throw new NoSuchFieldException(name);}
    static void invoke(Object o,String name,Class<?>[] types,Object... args)throws Exception{Method m=o.getClass().getDeclaredMethod(name,types);m.setAccessible(true);m.invoke(o,args);}
    static void paint(Object canvas,Image image)throws Exception{Graphics g=image.getGraphics();g.translate(-g.getTranslateX(),-g.getTranslateY());g.setClip(0,0,240,320);invoke(canvas,"paint",new Class<?>[]{Graphics.class},g);}
    public static void main(String[] args)throws Exception{
        try{
            Mobile.minLogLevel=Mobile.LOG_ERROR;
            Mobile.config=new Config();
            MobilePlatform platform=new MobilePlatform(240,320);
            Mobile.setPlatform(platform,()->{});
            platform.dataPath=new File(args[1],"reference-saves").getAbsolutePath()+"/";
            if(!platform.load(new File(args[0]).toURI().toString()))throw new Exception("Cannot load original JAR");
            platform.runJar();
            Font small=Font.getFont(0,0,8);
            System.out.println("REFERENCE FONT: "+small.awtFont+" height="+small.getHeight()+" width="+small.stringWidth("Here rests the sword of evils bane.."));
            Thread.sleep(200);
            Object midlet=Mobile.midlet;
            ((java.util.Timer)field(midlet,"m")).cancel();
            Object canvas=field(midlet,"a");
            Image image=Image.createImage(240,320);
            paint(canvas,image);
            ImageIO.write(image.getCanvas(),"png",new File(args[1],"reference-title.png"));
            ((CommandListener)midlet).commandAction((Command)field(midlet,"f"),(Displayable)canvas);
            paint(canvas,image);
            ImageIO.write(image.getCanvas(),"png",new File(args[1],"reference-slots.png"));
            invoke(canvas,"keyPressed",new Class<?>[]{int.class},53);
            for(int i=0;i<20;i++)paint(canvas,image);
            ImageIO.write(image.getCanvas(),"png",new File(args[1],"reference-game.png"));
            Object player=field(canvas,"e");
            System.out.println("REFERENCE state="+field(canvas,"p")+" position="+field(player,"i")+","+field(player,"j"));
            File nativeFrame=new File(args[1],"opening.png");
            if(nativeFrame.exists()){
                java.awt.image.BufferedImage nativeImage=ImageIO.read(nativeFrame);int different=0;
                for(int y=0;y<208;y++)for(int x=0;x<240;x++)if(image.getCanvas().getRGB(x,y)!=nativeImage.getRGB(x,y))different++;
                System.out.println("OPENING PIXEL DIFFERENCES: "+different+" / 49920");
            }
            System.exit(0);
        }catch(Throwable e){e.printStackTrace();System.exit(1);}
    }
}
