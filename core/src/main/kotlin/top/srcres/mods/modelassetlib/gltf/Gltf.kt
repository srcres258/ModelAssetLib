package top.srcres.mods.modelassetlib.gltf

import net.minecraft.resources.ResourceLocation
import top.srcres.mods.modelassetlib.ModelAssetLib
import java.io.Closeable

abstract class Gltf(
    gltfData: ByteArray
) : Closeable {
    class NativeCallback(gltf: Gltf, val gltfData: ByteArray) : NativeCallbackGltf(gltf) {
        override fun getInitialGltfData() = gltfData

        override fun loadBufferFromURI(uriStr: String): ByteArray = gltf.loadBufferFromURI(uriStr)

        override fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray = gltf.loadImageFromURI(uriStr, mimeTypeStr)
    }

    private val nativeCallback = NativeCallback(this, gltfData)
    private var rust_gltfObj: Long = 0L
    private var rust_loadedGltfObj: Long = 0L

    private external fun nativeInit()

    private external fun nativeDestroy()

    open fun init() {
        nativeInit()
    }

    abstract fun loadBufferFromURI(uriStr: String): ByteArray

    abstract fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray

    override fun close() {
        nativeDestroy()
    }
}