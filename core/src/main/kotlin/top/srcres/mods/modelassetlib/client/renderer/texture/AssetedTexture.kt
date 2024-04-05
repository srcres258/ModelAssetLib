package top.srcres.mods.modelassetlib.client.renderer.texture

import net.minecraft.client.renderer.texture.AbstractTexture
import net.minecraft.server.packs.resources.ResourceManager

class AssetedTexture(val rawData: ByteArray) : AbstractTexture() {
    override fun load(pResourceManager: ResourceManager) {
    }
}