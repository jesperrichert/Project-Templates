package zip.jespersen.velocitytemplate

import com.google.inject.Inject
import com.velocitypowered.api.event.Subscribe
import com.velocitypowered.api.event.connection.PluginMessageEvent
import com.velocitypowered.api.event.proxy.ProxyInitializeEvent
import com.velocitypowered.api.plugin.Plugin
import com.velocitypowered.api.proxy.ProxyServer
import dev.jorel.commandapi.CommandAPI
import dev.jorel.commandapi.CommandAPIVelocityConfig
import zip.jespersen.velocitytemplate.utils.PluginMessaging
import io.github.cdimascio.dotenv.Dotenv
import java.util.logging.Logger


@Plugin(
    id = "velocitytemplate",
    name = "velocitytemplate",
    version = "1.0.0",
    authors = ["xyzjesper"],
    description = "velocitytemplate"
)
class velocitytemplate {
    val id = "velocitytemplate"
    var server: ProxyServer
    var logger: Logger

    companion object {
        lateinit var instance: Main
    }

    init {
        instance = this
    }

    lateinit var dotEnv: Dotenv

    @Inject
    constructor(logger: Logger, server: ProxyServer) {
        this.logger = logger
        this.server = server
        CommandAPI.onLoad(CommandAPIVelocityConfig(server, this));
    }
    
    @Subscribe
    fun onProxyInitialization(event: ProxyInitializeEvent) {
        CommandAPI.onEnable();
        server.channelRegistrar.register(PluginMessaging.MAIN_IDENTIFIER)
    }

    @Subscribe
    fun onPluginMessageFromPlayer(event: PluginMessageEvent) {
        // Check if the identifier matches first, no matter the source.
        if (!PluginMessaging.MAIN_IDENTIFIER.equals(event.identifier)) {
            return
        }
        val data = PluginMessaging.parseData(event.data)
        
    }
    
}
