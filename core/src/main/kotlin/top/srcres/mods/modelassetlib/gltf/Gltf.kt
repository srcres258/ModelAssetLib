package top.srcres.mods.modelassetlib.gltf

import java.io.Closeable

class Gltf(
    gltfData: ByteArray
) : Closeable {
    class NativeCallback(gltf: Gltf, val gltfData: ByteArray) : NativeCallbackGltf(gltf) {
        override fun getInitialGltfData() = gltfData

        override fun loadBufferFromURI(uriStr: String): ByteArray {
            println("loadBufferFromURI called")
            TODO()
        }

        override fun loadImageFromURI(uriStr: String): ByteArray {
            println("loadImageFromURI called")
            TODO()
        }
    }

    private val nativeCallback = NativeCallback(this, gltfData)
    private var rust_gltfObj: Long = 0L
    private var rust_loadedGltfObj: Long = 0L

    init {
        nativeInit()
    }

    private external fun nativeInit()

    private external fun nativeDestroy()

    override fun close() {
        nativeDestroy()
    }
}