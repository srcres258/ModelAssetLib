package top.srcres.mods.modelassetlib

import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.ResourceManager
import java.io.File
import java.nio.file.Path
import java.util.Random

object NativeLibrary {
    private fun getNativeName(osName: String) = when (osName) {
        "Linux" -> "libmodelassetlib_native.so"
        "Windows NT" -> "modelassetlib_native.dll"
        "Mac OS" -> "libmodelassetlib_native.dylib"
        "Mac OS X" -> "libmodelassetlib_native.dylib"
        else -> throw UnsupportedOSException(osName)
    }

    private fun genRandomTmpName(): String {
        val random = Random()
        val sb = StringBuilder()
        for (i in 0 until 8) {
            val rand = random.nextInt(10)
            sb.append('0' + rand)
        }
        return sb.toString()
    }

    fun loadNative(resManager: ResourceManager) {
        val osName = System.getProperty("os.name")
        val nativeName = getNativeName(osName)
        val libRes = resManager.getResource(ResourceLocation(ModelAssetLib.MODID, "lib/$nativeName")).get()
        val tmpDir = System.getProperty("java.io.tmpdir")
        val tmpDirPath = File(tmpDir).toPath()
        val tmpFileName = "${genRandomTmpName()}-${nativeName}"
        val tmpFile = tmpDirPath.resolve(tmpFileName).toFile()
        if (!tmpFile.createNewFile())
            throw RuntimeException("Failed to create the native library file: ${tmpFile.absolutePath}")
        tmpFile.deleteOnExit()
        tmpFile.outputStream().use { tmpFileOS ->
            libRes.open().use {
                val data = it.readAllBytes()
                tmpFileOS.write(data)
            }
        }
        ModelAssetLib.logger.info("Native library was saved to ${tmpFile.absolutePath} for loading by JVM.")

        System.load(tmpFile.absolutePath)
    }

    fun initNative() {
        if (!initNative0()) {
            throw RuntimeException("Failed to initialise the native library.")
        }
    }

    external fun initNative0(): Boolean
}