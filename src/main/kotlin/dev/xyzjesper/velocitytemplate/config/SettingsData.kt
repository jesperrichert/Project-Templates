package dev.xyzjesper.velocitytemplate.config

import kotlinx.serialization.Serializable

@Serializable
data class SettingsData(
    var template: String?
)

