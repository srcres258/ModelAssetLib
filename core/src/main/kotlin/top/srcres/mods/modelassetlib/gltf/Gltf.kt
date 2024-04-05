package top.srcres.mods.modelassetlib.gltf

abstract class Gltf(
    gltfData: ByteArray
) : AutoCloseable {
    class NativeCallback(gltf: Gltf, val gltfData: ByteArray) : NativeCallbackGltf(gltf) {
        override fun getInitialGltfData() = gltfData

        override fun loadBufferFromURI(uriStr: String): ByteArray = gltf.loadBufferFromURI(uriStr)

        override fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray = gltf.loadImageFromURI(uriStr, mimeTypeStr)

        override fun receiveImageURI(uriStr: String) = gltf.receiveImageURI(uriStr)
    }

    private val nativeCallback = NativeCallback(this, gltfData)
    private var rust_gltfObj: Long = 0L
    private var rust_loadedGltfObj: Long = 0L

    val imageURIList = ArrayList<String>()

    private external fun nativeInit()

    private external fun nativeDestroy()

    open fun init() {
        nativeInit()
    }

    abstract fun loadBufferFromURI(uriStr: String): ByteArray

    abstract fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray

    private fun receiveImageURI(uriStr: String) {
        imageURIList.add(uriStr)
    }

    external fun getImageDataByURI(uriStr: String): ByteArray

    override fun close() {
        nativeDestroy()
    }
}