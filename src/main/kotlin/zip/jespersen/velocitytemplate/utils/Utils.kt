package zip.jespersen.velocitytemplate.utils

import com.velocitypowered.api.proxy.messages.ChannelIdentifier
import com.velocitypowered.api.proxy.server.RegisteredServer
import zip.jespersen.velocitytemplate.Main
import java.util.Optional


object Utils {

    fun getServerFromName(name: String): Optional<RegisteredServer?>? {
        return Main.instance.server.getServer(name)
    }
    
}