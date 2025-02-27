package org.manta.ray.excel;


/**
 * @author lyh
 * @version 1.0.0
 * @date 2025-02-27 上午10:13:48
 */
public class XlsxParser {

    static {
        try {
            System.loadLibrary("jarWarpImport");
        }catch (UnsatisfiedLinkError e) {
            String property = System.getProperty("java.library.path");
            System.err.println("java.library.path: " + property);
            System.err.println("Native code library failed to load.\n" + e);
        }
    }

    static native void testFunc();

    public void call_func() {
        System.out.println("call_func");
        testFunc();
    }

    private XlsxParser() {}

    static public XlsxParser build() {
        return new XlsxParser();
    }
}
