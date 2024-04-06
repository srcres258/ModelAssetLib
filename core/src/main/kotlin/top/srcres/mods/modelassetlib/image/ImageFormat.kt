package top.srcres.mods.modelassetlib.image

import java.util.*

enum class ImageFormat(val id: Int) {
    Png(0),
    Jpeg(1),
    Gif(2),
    WebP(3),
    Pnm(4),
    Tiff(5),
    Tga(6),
    Dds(7),
    Bmp(8),
    Ico(9),
    Hdr(10),
    OpenExr(11),
    Farbfeld(12),
    Avif(13),
    Qoi(14);

    companion object {
        fun fromExtension(ext: String): Optional<ImageFormat> {
            val extl = ext.lowercase(Locale.getDefault())
            val result: ImageFormat? = when (extl) {
                "avif" -> Avif
                "jpg", "jpeg" -> Jpeg
                "png" -> Png
                "gif" -> Gif
                "webp" -> WebP
                "tif", "tiff" -> Tiff
                "tga" -> Tga
                "dds" -> Dds
                "bmp" -> Bmp
                "ico" -> Ico
                "hdr" -> Hdr
                "exr" -> OpenExr
                "pbm", "pam", "ppm", "pgm" -> Pnm
                "ff" -> Farbfeld
                "qoi" -> Qoi
                else -> null
            }
            return if (result == null) Optional.empty() else Optional.of(result)
        }
    }
}