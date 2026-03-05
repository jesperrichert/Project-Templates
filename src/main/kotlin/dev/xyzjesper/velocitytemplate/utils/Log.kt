package dev.xyzjesper.velocitytemplate.utils

import java.text.SimpleDateFormat
import java.util.*

object Log {

    val RESET: String
        get() = "\u001B[0m"

    val GRAY: String
        get() = "\\033[1;30m"

    val WHITE: String
        get() = "\\033[1;37m"

    val RED: String
        get() = "\u001B[31m"

    val GREEN: String
        get() = "\u001B[32m"

    val LIGHTGREEN: String
        get() = "\\033[1;32m"

    val YELLOW: String
        get() = "\u001B[33m"
    val BLUE: String
        get() = "\u001B[34m"
    val MAGENTA: String
        get() = "\u001B[35m"

    fun getTimestamp(): String {
        val format = SimpleDateFormat("yyyy-MM-dd HH:mm:ss")
        return format.format(Date())
    }

    fun warn(message: String) {
        println("${YELLOW}[WARN] $message$RESET")
    }

    fun error(message: String) {
        println("${RED}[ERROR] $message$RESET")
    }

    fun info(message: String) {
        println("${BLUE}[INFO] $message$RESET")
    }

    fun log(message: String) {
        println("[Log] $message$RESET")
    }

    fun debug(message: String) {
        println("${MAGENTA}[DEBUG] $message$RESET")
    }
}