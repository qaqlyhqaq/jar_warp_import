package org.manta.ray.excel;


import java.io.InputStream;

/**
 * @author lyh
 * @version 1.0.0
 * @date 2025-02-27 上午10:13:48
 */
public class XlsxParser {

    static {
        try {
            System.out.println("开始加载XlsxParser Class !");
            System.loadLibrary("jarWarpImport");
        }catch (UnsatisfiedLinkError e) {
            String property = System.getProperty("java.library.path");
            System.err.println("java.library.path: " + property);
            System.err.println("Native code library failed to load.\n" + e);
        }
    }

    static native void testFunc();

    static native void nativeParse(byte[] stream);

    public void call_func(InputStream stream) {
        testFunc();
    }

    public void nativeParseJ(byte[] stream){
        nativeParse(stream);
    }

    private XlsxParser() {}

    static public XlsxParser build() {
        return new XlsxParser();
    }
}
