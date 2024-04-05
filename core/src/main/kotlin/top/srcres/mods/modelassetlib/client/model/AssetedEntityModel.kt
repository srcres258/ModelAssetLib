package top.srcres.mods.modelassetlib.client.model

import com.mojang.blaze3d.vertex.PoseStack
import com.mojang.blaze3d.vertex.VertexConsumer
import net.minecraft.client.model.EntityModel
import net.minecraft.resources.ResourceLocation
import net.minecraft.server.packs.resources.Resource
import net.minecraft.world.entity.Entity
import top.srcres.mods.modelassetlib.ModelAssetLib
import top.srcres.mods.modelassetlib.gltf.DefaultGltf
import top.srcres.mods.modelassetlib.gltf.Gltf
import java.io.Closeable
import java.io.InputStream

class AssetedEntityModel<T : Entity?>(
    gltfData: ByteArray
) : EntityModel<T>(), Closeable {
    private val gltf: DefaultGltf

    constructor(input: InputStream)
            : this(input.use { it.readAllBytes() })

    constructor(res: Resource)
            : this(res.open())

    constructor(resLoc: ResourceLocation)
            : this(ModelAssetLib.mcInstance.resourceManager.getResource(resLoc).get())

    init {
        gltf = DefaultGltf(gltfData, ::loadBufferFromURI, ::loadImageFromURI)
        gltf.init()
    }

    override fun close() {
        gltf.close()
    }

    private fun loadBufferFromURI(uriStr: String): ByteArray
            = ModelAssetLib.mcInstance.resourceManager.getResource(ResourceLocation(uriStr))
                .get().open().readAllBytes()

    private fun loadImageFromURI(uriStr: String, mimeTypeStr: String): ByteArray
            = ModelAssetLib.mcInstance.resourceManager.getResource(ResourceLocation(uriStr))
                .get().open().readAllBytes()

    override fun renderToBuffer(
        pPoseStack: PoseStack,
        pBuffer: VertexConsumer,
        pPackedLight: Int,
        pPackedOverlay: Int,
        pRed: Float,
        pGreen: Float,
        pBlue: Float,
        pAlpha: Float
    ) {
        TODO("Not yet implemented")
    }

    override fun setupAnim(
        pEntity: T,
        pLimbSwing: Float,
        pLimbSwingAmount: Float,
        pAgeInTicks: Float,
        pNetHeadYaw: Float,
        pHeadPitch: Float
    ) {
        TODO("Not yet implemented")
    }
}