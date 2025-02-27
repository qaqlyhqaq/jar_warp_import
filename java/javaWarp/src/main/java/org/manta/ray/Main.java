package org.manta.ray;

import cn.hutool.core.io.FileUtil;
import cn.hutool.core.lang.Assert;
import org.manta.ray.excel.XlsxParser;

import java.io.*;
import java.nio.file.Path;
import java.nio.file.Paths;

public class Main {
    public static void main(String[] args) throws IOException, ClassNotFoundException {
        String thiz = "";
        InputStream in = thiz.getClass().getResourceAsStream("/lib/jarWarpImport.dll");
        if(in!=null) {
            Path path = Paths.get("jarWarpImport.dll");
            if(FileUtil.isExistsAndNotDirectory(path,false)){
                FileUtil.del(path);
            }
            System.out.println("/lib/jarWarpImport.dll:+"+in.available());
            Assert.isTrue(in!=null);
            Assert.isTrue(in.available()>0);
            FileUtil.writeFromStream(in,new File("jarWarpImport.dll"));
        }
        ClassLoader classLoader = ClassLoader.getSystemClassLoader();
        XlsxParser parser = XlsxParser.build();
        InputStream stream = classLoader.getResourceAsStream("/lib/jarWarpImport.dll");
        parser.call_func(stream);
        parser.nativeParseJ(stream);
    }
}
