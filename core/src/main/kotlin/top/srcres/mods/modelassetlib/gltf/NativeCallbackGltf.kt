package top.srcres.mods.modelassetlib.gltf

abstract class NativeCallbackGltf(val gltf: Gltf) {
    abstract fun getInitialGltfData(): ByteArray
    abstract fun loadBufferFromURI(uriStr: String): ByteArray
    abstract fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray
}