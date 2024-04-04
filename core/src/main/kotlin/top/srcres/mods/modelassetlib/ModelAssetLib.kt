package top.srcres.mods.modelassetlib

import net.minecraft.client.Minecraft
import net.minecraft.world.entity.Entity
import net.neoforged.fml.common.Mod
import net.neoforged.fml.event.lifecycle.FMLLoadCompleteEvent
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import thedarkcolour.kotlinforforge.neoforge.forge.MOD_BUS
import top.srcres.mods.modelassetlib.client.model.AssetedEntityModel
import java.io.File

@Mod(ModelAssetLib.MODID)
object ModelAssetLib {
    const val MODID = "modelassetlib"

    val logger: Logger = LoggerFactory.getLogger(MODID)
    val mcInstance: Minecraft
        get() = Minecraft.getInstance()

    init {
        MOD_BUS.addListener(::onLoadComplete)
    }

    private fun onLoadComplete(event: FMLLoadCompleteEvent) {
        val mc = Minecraft.getInstance()
        NativeLibrary.loadNative(mc.resourceManager)
        NativeLibrary.initNative()

        val data = File("/home/srcres/App/blender-4.0.2-linux-x64/models/Horus/export/Horus.gltf").inputStream().readAllBytes()
        val model = AssetedEntityModel<Entity>(data)
        logger.info("model.getGltfMeshCount(): ${model.getGltfMeshCount()}")
        model.close()
    }
}