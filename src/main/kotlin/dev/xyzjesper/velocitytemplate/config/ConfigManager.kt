package dev.xyzjesper.velocitytemplate.config

import dev.xyzjesper.velocitytemplate.Main
import java.io.File

object ConfigManager {

    private val settingsFile = File("plugins/${Main.instance.id}/config.json")

    var settings = settingsFile.loadConfig(SettingsData(
        template = ""
    ))

    fun save() {
        settingsFile.writeText(json.encodeToString(settings))
    }

    fun load() {
        settings
        save()
    }

    fun reload() {
        settings = loadFromFile(settingsFile)
    }
    
}
