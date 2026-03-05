package dev.xyzjesper.velocitytemplate.config

interface Configurable {
    fun save()
    fun load() {}
    fun reset() {}
}