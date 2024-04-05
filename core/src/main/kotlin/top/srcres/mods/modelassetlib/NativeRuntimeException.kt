package top.srcres.mods.modelassetlib

class NativeRuntimeException(message: String) : RuntimeException(message) {
    constructor() : this("No message given")
}