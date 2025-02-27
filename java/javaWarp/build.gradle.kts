import org.gradle.internal.classpath.Instrumented.systemProperty

plugins {
    id("java")
}

group = "org.manta.ray"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}

tasks.run {
    systemProperty("java.library.path","E:\\rust\\jarWarpImport\\java\\javaWarp\\src\\main\\resources\\lib")
}



tasks.compileJava{
    systemProperty("java.library.path","E:\\rust\\jarWarpImport\\java\\javaWarp\\src\\main\\resources\\lib")
}

tasks.test{
    systemProperty("java.library.path","E:\\rust\\jarWarpImport\\java\\javaWarp\\src\\main\\resources\\lib")
}

dependencies {
    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")
}

tasks.test {
    useJUnitPlatform()
}
