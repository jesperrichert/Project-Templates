plugins {
    kotlin("jvm") version "2.+"
    kotlin("plugin.serialization") version "2.+"
    kotlin("kapt") version "2.2.21"
    id("com.gradleup.shadow") version "9.+"
    id("xyz.jpenilla.run-velocity") version "3.+"
    id("maven-publish")
}

val projectVersion = properties["version"] as String
val projectName = properties["name"] as String
val groupID = properties["group"] as String


val velocityVersion = properties["velocityVersion"] as String
val commandAPIVersion = properties["commandAPIVersion"] as String
val rethisVersion = properties["rethisVersion"] as String
val kormVersion = properties["kormVersion"] as String
val dotenvVersion = properties["dotenvVersion"] as String
val adventureVersion = properties["adventureVersion"] as String

group = groupID
version = projectVersion

repositories {
    mavenCentral()
    maven("https://repo.papermc.io/repository/maven-public/") {
        name = "papermc-repo"
    }
    maven("https://oss.sonatype.org/content/groups/public/") {
        name = "sonatype"
    }
    maven {
        url = uri("https://s01.oss.sonatype.org/content/repositories/snapshots")
    }
}

dependencies {

    // Velocity
    compileOnly("com.velocitypowered:velocity-api:$velocityVersion")
    kapt("com.velocitypowered:velocity-api:$velocityVersion")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")

    // Kotlin
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.9.0")
    implementation("org.jetbrains.kotlin:kotlin-stdlib-jdk8")

    // Database
    implementation("eu.vendeli:rethis:${rethisVersion}")
    implementation("org.ktorm:ktorm-core:${kormVersion}")

    // ENV
    implementation("io.github.cdimascio:dotenv-kotlin:${dotenvVersion}")
    
    // CommandAPI
    implementation("dev.jorel:commandapi-velocity-shade:$commandAPIVersion")

    // Adventure API
    implementation("net.kyori:adventure-api:4.18.0")
}

java {
    toolchain.languageVersion = JavaLanguageVersion.of(21)
}

tasks {
    runVelocity {
        velocityVersion("3.4.0-SNAPSHOT")
    }
    assemble {
        dependsOn(shadowJar)
    }
    compileJava {
        options.encoding = "UTF-8"
        options.release.set(21)
    }
}

kotlin {
    jvmToolchain(21)
}


publishing {
    repositories {
        maven {
            name = "Reposilite"
            url = uri("https://repo.xyzhub.link/releases")
            credentials {
                username = System.getenv("REPOSILITE_USER") ?: System.getProperty("REPOSILITE_USER") ?: "USERNAME"
                password = System.getenv("REPOSILITE_TOKEN") ?: System.getProperty("REPOSILITE_TOKEN") ?: "TOKEN"
            }
            authentication {
                create<BasicAuthentication>("basic")
            }
        }
    }
    publications {
        create<MavenPublication>("reposilite") {
            from(components["java"])
            artifactId = projectName
            groupId = group as String
            version = version
        }
    }
}