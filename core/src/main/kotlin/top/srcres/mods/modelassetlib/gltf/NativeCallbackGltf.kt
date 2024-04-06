package top.srcres.mods.modelassetlib.gltf

import top.srcres.mods.modelassetlib.jni.INativeCallback

abstract class NativeCallbackGltf(val gltf: Gltf) : INativeCallback {
    abstract fun getInitialGltfData(): ByteArray
    abstract fun loadBufferFromURI(uriStr: String): ByteArray
    abstract fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray
    abstract fun receiveImageURI(uriStr: String)
}