package zip.jespersen.papertemplate.utils

import zip.jespersen.papertemplate.Main
import zip.jespersen.papertemplate.config.ConfigManager
import zip.jespersen.papertemplate.database.DatabaseManager
import io.github.cdimascio.dotenv.dotenv
import java.io.File

object Statics {

    fun load() {
        // Configs
        ConfigManager.load()

        try {
            Main.instance.dotEnv = dotenv(block = {
                directory = "plugins/${Main.instance.name}/"
            })
        } catch (e: Exception) {
            File("plugins/${Main.instance.name}/.env").createNewFile()
        }

        // Database
        DatabaseManager.init()
        DatabaseManager.preload()
    }

}