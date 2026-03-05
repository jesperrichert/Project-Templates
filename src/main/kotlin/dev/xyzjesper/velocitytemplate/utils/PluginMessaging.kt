package dev.xyzjesper.velocitytemplate.utils

import com.google.common.io.ByteArrayDataInput
import com.google.common.io.ByteStreams
import com.velocitypowered.api.proxy.messages.ChannelIdentifier
import com.velocitypowered.api.proxy.messages.MinecraftChannelIdentifier
import com.velocitypowered.api.proxy.server.RegisteredServer
import dev.xyzjesper.velocitytemplate.Main

object PluginMessaging {

    var MAIN_IDENTIFIER = MinecraftChannelIdentifier.from("${Main.instance.id}:main")

    fun sendPluginMessageToBackend(server: RegisteredServer, identifier: ChannelIdentifier, data: ByteArray): Boolean {
        return server.sendPluginMessage(identifier, data)
    }

    fun parseData(bytes: ByteArray): ByteArrayDataInput {
        return ByteStreams.newDataInput(bytes)
    }

}