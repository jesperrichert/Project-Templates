package dev.xyzjesper.velocitytemplate.utils

import com.velocitypowered.api.proxy.messages.ChannelIdentifier
import com.velocitypowered.api.proxy.server.RegisteredServer
import dev.xyzjesper.velocitytemplate.Main
import java.util.Optional


object Utils {

    fun getServerFromName(name: String): Optional<RegisteredServer?>? {
        return Main.instance.server.getServer(name)
    }
    
}