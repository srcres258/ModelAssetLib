package top.srcres.mods.modelassetlib.image

import java.util.Optional

external fun nativeGetErrorMessage(): String

class Image(rawData: ByteArray, format: Optional<ImageFormat>) : AutoCloseable {
    private var rust_imageObj: Long = 0L

    constructor(rawData: ByteArray) : this(rawData, Optional.empty())

    init {
        if (format.isEmpty) {
            if (!nativeInit(rawData)) {
                val msg = nativeGetErrorMessage()
                throw RuntimeException("Failed to initialise native image: $msg")
            }
        } else {
            val fmt = format.get()
            if (!nativeInitWithFormat(rawData, fmt.id)) {
                val msg = nativeGetErrorMessage()
                throw RuntimeException("Failed to initialise native image with format $fmt: $msg")
            }
        }
    }

    private external fun nativeInit(rawData: ByteArray): Boolean

    private external fun nativeInitWithFormat(rawData: ByteArray, formatId: Int): Boolean

    private external fun nativeDestroy()

    private external fun getWidth0()

    private external fun getHeight0()

    override fun close() {
        nativeDestroy()
    }
}