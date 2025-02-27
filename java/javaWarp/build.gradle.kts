import org.gradle.internal.classpath.Instrumented.systemProperty

plugins {
    id("java")
}

group = "org.manta.ray"
version = "1.0-SNAPSHOT"

repositories {
    mavenCentral()
}



dependencies {
    testImplementation(platform("org.junit:junit-bom:5.10.0"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    implementation("cn.hutool:hutool-all:5.8.36")
}

tasks.test {
    useJUnitPlatform()
}
