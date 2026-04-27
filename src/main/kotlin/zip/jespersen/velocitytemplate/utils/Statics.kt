package zip.jespersen.velocitytemplate.utils

import zip.jespersen.velocitytemplate.database.DatabaseManager
import zip.jespersen.velocitytemplate.Main
import zip.jespersen.velocitytemplate.config.ConfigManager
import io.github.cdimascio.dotenv.dotenv
import java.io.File

object Statics {
    
    fun load() {
        // Configs
        ConfigManager.load()
        
        try {
            Main.instance.dotEnv = dotenv(block = {
                directory = "plugins/${Main.instance.id}/"
            })
        } catch (e: Exception) {
            File("plugins/${Main.instance.id}/.env").createNewFile()
        }

        // Database
        DatabaseManager.init()
        DatabaseManager.preload()
    }
    
}