package top.srcres.mods.modelassetlib.gltf

class DefaultGltf(
    gltfData: ByteArray,
    val bufferFromURIFunc: (String) -> ByteArray,
    val imageFromURIFunc: (String) -> ByteArray
) : Gltf(gltfData) {
    override fun loadBufferFromURI(uriStr: String): ByteArray = bufferFromURIFunc(uriStr)

    override fun loadImageFromURI(uriStr: String): ByteArray = imageFromURIFunc(uriStr)
}