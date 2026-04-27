package zip.jespersen.papertemplate.config

import kotlinx.serialization.Serializable

@Serializable
data class SettingsData(
    var template: String?
)

