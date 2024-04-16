package top.srcres.mods.modelassetlib.jni

import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.ResourceManager
import top.srcres.mods.modelassetlib.ModelAssetLib
import java.io.File
import java.util.Random

object NativeLibrary {
    private fun getNativeName(): String {
        val os = OSType.detect() ?: throw UnsupportedOSException()
        return os.nativeFileName
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
        val nativeName = getNativeName()
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
        ModelAssetLib.logger.info("Successfully loaded JNI native library: ${tmpFile.absolutePath}")
    }

    fun initNative() {
        if (!initNative0()) {
            throw RuntimeException("Failed to initialise the native library.")
        }
    }

    private external fun initNative0(): Boolean
}