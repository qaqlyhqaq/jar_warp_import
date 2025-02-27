package org.manta.ray;

import cn.hutool.core.io.BufferUtil;
import cn.hutool.core.io.FileUtil;
import cn.hutool.core.io.IoUtil;
import cn.hutool.core.lang.Assert;
import cn.hutool.core.stream.StreamUtil;
import org.manta.ray.excel.XlsxParser;

import java.io.*;
import java.nio.ByteBuffer;
import java.nio.file.Path;
import java.nio.file.Paths;

public class Main {
    public static void main(String[] args) throws IOException, ClassNotFoundException {
        String thiz = "";
        InputStream in = thiz.getClass().getResourceAsStream("/lib/jarWarpImport.dll");
        Assert.isTrue(in!=null);
        Assert.isTrue(in.available() > 0);
        Path path = Paths.get("jarWarpImport.dll");
        if (FileUtil.isExistsAndNotDirectory(path, false)) {
            FileUtil.del(path);
        }
        FileUtil.writeFromStream(in, new File("jarWarpImport.dll"));
        XlsxParser parser = XlsxParser.build();
        in.close();
//        in = thiz.getClass().getResourceAsStream("/lib/jarWarpImport.dll");
        in = FileUtil.getInputStream(new File("C:\\Users\\qaqly\\Desktop\\lyh-draft.xlsx"));
        byte[] bytes = IoUtil.readBytes(in);
        System.out.println("bytes size:"+bytes.length);
//        parser.call_func(in);
        parser.nativeParseJ(bytes);
    }
}
