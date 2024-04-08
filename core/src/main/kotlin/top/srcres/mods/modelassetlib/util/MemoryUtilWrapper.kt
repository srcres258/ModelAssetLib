package top.srcres.mods.modelassetlib.util

import org.lwjgl.system.MemoryUtil
import java.nio.ByteBuffer

object MemoryUtilWrapper {
    fun memWrapByteArray(arr: ByteArray): ByteBuffer {
        val buffer = MemoryUtil.memAlloc(arr.size)
        val bufferAddr = MemoryUtil.memAddress(buffer)
        arr.forEachIndexed { i, byte ->
            MemoryUtil.memPutByte(bufferAddr + i, byte)
        }
        return buffer
    }
}