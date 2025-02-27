package org.manta.ray.excel;

import java.io.InputStream;

/**
 * @author lyh
 * @version 1.0.0
 * @date 2025-02-27 上午10:13:48
 */
public class XlsxParser {

    static {
        System.loadLibrary("rust_java_demo");
    }

    private XlsxParser() {}

    public XlsxParser build(InputStream stream) {
        return new XlsxParser();
    }
}
