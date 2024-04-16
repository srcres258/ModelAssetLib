package top.srcres.mods.modelassetlib.jni

import net.minecraft.Util

/**
 * Enum of all kinds of operating systems supported by the native library.
 */
enum class OSType(
    val nativeFileName: String,
    val mcUtilOS: Util.OS,
    val describableName: String
) {
    WINDOWS("modelassetlib_native.dll",
        Util.OS.WINDOWS,
        "Windows"),
    LINUX("libmodelassetlib_native.so",
        Util.OS.LINUX,
        "Linux");

    companion object {
        /**
         * @return The operating system type which JVM is running on and is supported by the native.
         *         Null will be returned if not support by the native or unknown by JVM.
         */
        fun detect(): OSType? {
            val platform = Util.getPlatform()
            return when (platform) {
                Util.OS.WINDOWS -> WINDOWS
                Util.OS.LINUX -> LINUX
                else -> null
            }
        }
    }
}