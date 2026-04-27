package zip.jespersen.papertemplate

import dev.jorel.commandapi.CommandAPI
import dev.jorel.commandapi.CommandAPIPaperConfig
import zip.jespersen.papertemplate.config.ConfigManager
import zip.jespersen.papertemplate.database.DatabaseManager
import zip.jespersen.papertemplate.utils.Statics
import io.github.cdimascio.dotenv.Dotenv
import io.github.cdimascio.dotenv.dotenv
import net.crystopia.crystalshard.utils.Log
import org.bukkit.plugin.java.JavaPlugin

class papertemplatePlugin : JavaPlugin() {

    companion object {
        lateinit var instance: papertemplatePlugin
    }

    init {
        instance = this
    }

    lateinit var dotEnv: Dotenv

    override fun onLoad() {
        CommandAPI.onLoad(CommandAPIPaperConfig(this).silentLogs(true))

        Statics.load()

        Log.info("Loading papertemplate...")
    }

    override fun onEnable() {
        CommandAPI.onEnable()

        Log.info("Plugin papertemplate enabled!")
    }

    override fun onDisable() {
        CommandAPI.onDisable()

        Log.info("Plugin papertemplate disabled!")
    }

}