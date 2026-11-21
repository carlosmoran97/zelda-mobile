// Capture the reference platform's glyphs/metrics once; the Rust app uses PNGs.
import java.io.*;
import javax.imageio.ImageIO;
import javax.microedition.lcdui.*;
import org.recompile.mobile.*;
import org.recompile.freej2me.Config;

public class ExportFonts {
    public static void main(String[] args)throws Exception{
        try{
            Mobile.minLogLevel=Mobile.LOG_NONE;Mobile.config=new Config();
            MobilePlatform platform=new MobilePlatform(240,320);Mobile.setPlatform(platform,()->{});
            if(!platform.load(new File(args[0]).toURI().toString()))throw new Exception("Cannot load reference");
            platform.runJar();File out=new File(args[1]);out.mkdirs();StringBuilder json=new StringBuilder("[\n");boolean first=true;
            for(int size:new int[]{8,0,16})for(int style:new int[]{0,1}){
                Font f=Font.getFont(0,style,size);Image image=Image.createImage(512,512);Graphics g=image.getGraphics();g.setColor(0);g.fillRect(0,0,512,512);g.setColor(0xffffff);g.setFont(f);
                for(int c=32;c<256;c++){if(c==127)continue;int x=(c%16)*32,y=(c/16)*32;g.setClip(x,y,32,32);g.drawString(String.valueOf((char)c),x,y,20);}
                java.awt.image.BufferedImage source=image.getCanvas();java.awt.image.BufferedImage b=new java.awt.image.BufferedImage(512,512,java.awt.image.BufferedImage.TYPE_INT_ARGB);for(int y=0;y<512;y++)for(int x=0;x<512;x++){int alpha=(source.getRGB(x,y)>>16)&255;b.setRGB(x,y,(alpha<<24)|0xffffff);}
                String filename="midp-"+size+"-"+style+".png";ImageIO.write(b,"png",new File(out,filename));
                if(!first)json.append(",\n");first=false;
                json.append("{\"point_size\":").append(f.getPointSize()).append(",\"style\":").append(style).append(",\"height\":").append(f.getHeight()).append(",\"baseline\":").append(f.getBaselinePosition()).append(",\"image\":\"").append(filename).append("\",\"widths\":[");
                for(int c=0;c<256;c++){if(c!=0)json.append(',');json.append(f.charWidth((char)c));}json.append("]}");
                System.out.println("Font "+size+" style "+style+" -> "+f.getPointSize()+" px, height "+f.getHeight());
            }
            json.append("\n]\n");try(Writer writer=new FileWriter(new File(out,"metrics.json"))){writer.write(json.toString());}System.exit(0);
        }catch(Throwable e){e.printStackTrace();System.exit(1);}
    }
}
