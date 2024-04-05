package top.srcres.mods.modelassetlib.client.renderer.texture

import com.mojang.blaze3d.platform.NativeImage
import net.minecraft.client.renderer.texture.AbstractTexture
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.ResourceManager

class AssetedTexture(
    val location: ResourceLocation,
    val rawData: ByteArray
) : AbstractTexture() {
    // TODO:
    // Using the NativeImage#read method will straightly lead to java.lang.OutOfMemoryError: Out of stack space.
    // Hence, we are forced to implement image source decoding algorithm on our own.
    val pixels = NativeImage.read(rawData)

    override fun load(pResourceManager: ResourceManager) {
    }
}