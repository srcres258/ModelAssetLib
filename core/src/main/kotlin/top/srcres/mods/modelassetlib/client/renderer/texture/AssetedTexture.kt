package top.srcres.mods.modelassetlib.client.renderer.texture

import com.mojang.blaze3d.platform.TextureUtil
import com.mojang.blaze3d.systems.RenderSystem
import net.minecraft.client.renderer.texture.AbstractTexture
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.ResourceManager
import org.lwjgl.opengl.GL11
import org.lwjgl.system.MemoryUtil
import top.srcres.mods.modelassetlib.ModelAssetLib
import top.srcres.mods.modelassetlib.image.Image
import top.srcres.mods.modelassetlib.image.ImageFormat
import top.srcres.mods.modelassetlib.util.MemoryUtilWrapper
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
            RenderSystem.recordRenderCall(this::prepareTexture1)
        }
    }

    private fun prepareTexture1() {
        // Ensure we're on the render thread at first in order
        // to keep away from unexpected behaviours.
        RenderSystem.assertOnRenderThreadOrInit()

        TextureUtil.prepareImage(this.id, this.pixels.width, this.pixels.height)
        upload()
    }

    private fun upload() {
        this.bind()

        val rgbaData = pixels.rgbaData
        val rgbaDataBuf = MemoryUtilWrapper.memWrapByteArray(rgbaData)
        val rgbaDataBufAddr = MemoryUtil.memAddress(rgbaDataBuf)
        val width = pixels.width
        val height = pixels.height
        GL11.glTexImage2D(GL11.GL_TEXTURE_2D, 0, GL11.GL_RGBA, width, height, 0, GL11.GL_RGBA, GL11.GL_UNSIGNED_BYTE, rgbaDataBufAddr)

        /*
        TODO: Set up texture parameters by invoking OpenGL's glTexParameteri
         with the data from glTF file's "sampler" section referred by the
         "textures" section.
         */
    }
}