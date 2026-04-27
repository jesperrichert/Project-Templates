package zip.jespersen.velocitytemplate.config

interface Configurable {
    fun save()
    fun load() {}
    fun reset() {}
}