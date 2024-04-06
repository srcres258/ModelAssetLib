package top.srcres.mods.modelassetlib.client.renderer.texture

import net.minecraft.client.renderer.texture.AbstractTexture
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.ResourceManager
import top.srcres.mods.modelassetlib.image.Image
import top.srcres.mods.modelassetlib.image.ImageFormat
import java.util.Optional

class AssetedTexture(
    val location: ResourceLocation,
    rawData: ByteArray,
    format: Optional<ImageFormat>
) : AbstractTexture() {
    // Using the NativeImage#read method will straightly lead to java.lang.OutOfMemoryError: Out of stack space.
    // Hence, we are forced to implement image source decoding algorithm on our own.
    val pixels = Image(rawData, format)

    override fun load(pResourceManager: ResourceManager) {
    }
}