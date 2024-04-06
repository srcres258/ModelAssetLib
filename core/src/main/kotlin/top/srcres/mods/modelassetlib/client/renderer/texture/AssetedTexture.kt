package top.srcres.mods.modelassetlib.client.renderer.texture

import com.mojang.blaze3d.platform.TextureUtil
import com.mojang.blaze3d.systems.RenderSystem
import net.minecraft.client.renderer.texture.AbstractTexture
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.ResourceManager
import top.srcres.mods.modelassetlib.ModelAssetLib
import top.srcres.mods.modelassetlib.image.Image
import top.srcres.mods.modelassetlib.image.ImageFormat
import java.util.*

class AssetedTexture(
    val location: ResourceLocation,
    rawData: ByteArray,
    format: Optional<ImageFormat>
) : AbstractTexture() {
    // Using the NativeImage#read method will straightly lead to java.lang.OutOfMemoryError: Out of stack space.
    // Hence, we are forced to implement image source decoding algorithm on our own.
    private val pixels = Image(rawData, format)

    init {
        prepareTexture()
    }

    override fun load(pResourceManager: ResourceManager) {
        ModelAssetLib.logger.info("Texture $location: pixels width is ${pixels.width}")
        ModelAssetLib.logger.info("Texture $location: pixels height is ${pixels.height}")
    }

    /**
     * Submits the native texture to OpenGL and Minecraft render system.
     */
    private fun prepareTexture() {
        if (RenderSystem.isOnRenderThread()) {
            this.prepareTexture1()
        } else {
            RenderSystem.recordRenderCall {
                this.prepareTexture1()
            }
        }
    }

    private fun prepareTexture1() {
        TextureUtil.prepareImage(this.id, this.pixels.width, this.pixels.height)
        upload()
    }

    private fun upload() {
        //TODO
    }
}