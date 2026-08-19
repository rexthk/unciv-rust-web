import org.jetbrains.kotlin.gradle.dsl.JvmTarget

plugins {
    kotlin("jvm") version "2.4.10"
    application
}

repositories {
    mavenCentral()
}

val upstreamDir = file(System.getenv("UNCIV_UPSTREAM_DIR") ?: "/Users/rexthk/repo/Unciv")
val upstreamJar = upstreamDir.resolve("desktop/build/libs/Unciv.jar")

dependencies {
    implementation(files(upstreamJar))
    implementation("com.badlogicgames.gdx:gdx-backend-headless:1.14.2")
}

kotlin {
    compilerOptions {
        jvmTarget = JvmTarget.JVM_21
    }
}

application {
    mainClass.set("com.unciv.oracle.OracleMain")
}

val prepareUpstreamJar by tasks.registering(Exec::class) {
    workingDir = upstreamDir
    environment("JAVA_HOME", System.getenv("JAVA_HOME") ?: "/opt/homebrew/opt/openjdk@21")
    commandLine(upstreamDir.resolve("gradlew"), ":desktop:dist", "--no-daemon")
    inputs.file(upstreamDir.resolve(".git/HEAD"))
    outputs.file(upstreamJar)
}

tasks.named("compileKotlin") {
    dependsOn(prepareUpstreamJar)
}

tasks.named<JavaExec>("run") {
    workingDir = upstreamDir.resolve("android/assets")
}
