package top.srcres.mods.modelassetlib.jni

class NativeRuntimeException(message: String) : RuntimeException(message) {
    constructor() : this("No message given")
}