package zip.jespersen.papertemplate.config

interface Configurable {
    fun save()
    fun load() {}
    fun reset() {}
}