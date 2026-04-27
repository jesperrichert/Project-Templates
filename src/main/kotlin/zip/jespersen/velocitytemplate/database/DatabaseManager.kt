package zip.jespersen.velocitytemplate.database

import zip.jespersen.velocitytemplate.Main
import zip.jespersen.velocitytemplate.utils.Log
import org.ktorm.database.Database
import java.io.File
import kotlin.system.exitProcess


object DatabaseManager {

    var database: Database? = null
    var databaseFile = File("plugins/${Main.instance.id}/plugin.db")

    fun init() {
        try {
            if (!databaseFile.exists())
                databaseFile.createNewFile()
            database = Database.connect("jdbc:sqlite:plugins/${Main.instance.id}/plugin.db")
            Log.info("Loaded database connection")
            if (!Main.instance.server.pluginManager.isLoaded(Main.instance.id)) {
                exitProcess(0)
            }
        } catch (ex: Exception) {
            Log.error("Connection to database failed")
        }
    }

    fun preload() {
        if (database == null) {
            Log.error("No Database connection found.")
            return
        }

        // SQLUtils.command()...

    }

}
