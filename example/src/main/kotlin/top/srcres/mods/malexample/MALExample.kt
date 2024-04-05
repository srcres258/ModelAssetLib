package top.srcres.mods.malexample

import net.minecraft.resources.ResourceLocation
import net.minecraft.world.entity.Entity
import net.neoforged.fml.InterModComms
import net.neoforged.fml.common.Mod
import net.neoforged.fml.event.lifecycle.FMLLoadCompleteEvent
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import thedarkcolour.kotlinforforge.neoforge.forge.MOD_BUS
import top.srcres.mods.modelassetlib.ModelAssetLib
import top.srcres.mods.modelassetlib.client.model.AssetedEntityModel
import java.io.File

@Mod(MALExample.MODID)
object MALExample {
    const val MODID = "malexample"

    val logger: Logger = LoggerFactory.getLogger(MODID)

    init {
        logger.info("test message")

        ModelAssetLib.addNativeLoadedListener {
            val model = AssetedEntityModel<Entity>(ResourceLocation("malexample", "gltf/horus/horus.gltf"))
            model.close()
        }
    }
}