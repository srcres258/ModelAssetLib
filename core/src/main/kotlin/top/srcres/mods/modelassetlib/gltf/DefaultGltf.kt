package top.srcres.mods.modelassetlib.gltf

class DefaultGltf(
    gltfData: ByteArray,
    val bufferFromURIFunc: (String) -> ByteArray,
    val imageFromURIFunc: (String, String) -> ByteArray
) : Gltf(gltfData) {
    override fun loadBufferFromURI(uriStr: String): ByteArray = bufferFromURIFunc(uriStr)

    override fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray = imageFromURIFunc(uriStr, mimeTypeStr)
}