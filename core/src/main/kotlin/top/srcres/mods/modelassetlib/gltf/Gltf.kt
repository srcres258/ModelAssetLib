package top.srcres.mods.modelassetlib.gltf

import top.srcres.mods.modelassetlib.ModelAssetLib
import java.io.Closeable

class Gltf(
    gltfData: ByteArray
) : Closeable {
    class NativeCallback(gltf: Gltf, val gltfData: ByteArray) : NativeCallbackGltf(gltf) {
        override fun getInitialGltfData() = gltfData

        override fun loadBufferFromURI(uriStr: String): ByteArray {
            TODO("Not yet implemented")
        }

        override fun loadImageFromURI(uriStr: String): ByteArray {
            TODO("Not yet implemented")
        }
    }

    private val nativeCallback = NativeCallback(this, gltfData)
    private var rust_gltfObj: Long = 0L

    init {
        nativeInit()
    }

    private external fun nativeInit()

    private external fun nativeDestroy()

    override fun close() {
        nativeDestroy()
    }
}