package top.srcres.mods.modelassetlib.image

import java.nio.ByteBuffer
import java.util.Optional

external fun nativeIsErrorOccurred(): Boolean
external fun nativeGetErrorMessage(): String
external fun nativeClearError()

fun newExceptionFromNativeErrorMessage(prefixMessage: String): RuntimeException {
    val msg = nativeGetErrorMessage()
    return RuntimeException("$prefixMessage: $msg")
}

class Image(rawData: ByteArray, format: Optional<ImageFormat>) : AutoCloseable {
    constructor(rawData: ByteArray) : this(rawData, Optional.empty())

    init {
        if (format.isEmpty) {
            if (!nativeInit(rawData)) {
                throw newExceptionFromNativeErrorMessage("Failed to initialise native image").let {
                    nativeClearError()
                    it
                }
            }
        } else {
            val fmt = format.get()
            if (!nativeInitWithFormat(rawData, fmt.id)) {
                throw newExceptionFromNativeErrorMessage("Failed to initialise native image with format $fmt").let {
                    nativeClearError()
                    it
                }
                nativeClearError()
            }
        }
    }

    private var rust_imageObj: Long = 0L

    val width: Int
        get() {
            val n = getWidth0()
            if (n < 0) {
                throw newExceptionFromNativeErrorMessage("Failed to get image width")
            } else {
                return n
            }
        }

    val height: Int
        get() {
            val n = getHeight0()
            if (n < 0) {
                throw newExceptionFromNativeErrorMessage("Failed to get image height")
            } else {
                return n
            }
        }

    val rgbaData: ByteArray
        get() {
            val result = getRgbaData0()
            if (nativeIsErrorOccurred()) {
                throw newExceptionFromNativeErrorMessage("Failed to get image height").let {
                    nativeClearError()
                    it
                }
            }
            return result
        }

    private external fun nativeInit(rawData: ByteArray): Boolean

    private external fun nativeInitWithFormat(rawData: ByteArray, formatId: Int): Boolean

    private external fun nativeDestroy()

    private external fun getWidth0(): Int

    private external fun getHeight0(): Int

    private external fun getRgbaData0(): ByteArray

    override fun close() {
        nativeDestroy()
    }
}