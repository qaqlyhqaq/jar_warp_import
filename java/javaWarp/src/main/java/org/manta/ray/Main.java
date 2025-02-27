package org.manta.ray;

import org.manta.ray.excel.XlsxParser;

public class Main {
    public static void main(String[] args) {
        String property = System.getProperty("java.library.path");
        System.out.println("java.library.path: " + property);
        XlsxParser parser = XlsxParser.build();
        System.out.println("parsing...");
        parser.call_func();
    }
}
