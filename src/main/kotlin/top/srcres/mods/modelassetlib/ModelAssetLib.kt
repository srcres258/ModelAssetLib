package top.srcres.mods.modelassetlib

import net.minecraft.client.Minecraft
import net.neoforged.fml.common.Mod
import net.neoforged.fml.event.lifecycle.FMLConstructModEvent
import net.neoforged.fml.event.lifecycle.FMLLoadCompleteEvent
import org.slf4j.LoggerFactory
import thedarkcolour.kotlinforforge.neoforge.forge.MOD_BUS

@Mod(ModelAssetLib.MODID)
object ModelAssetLib {
    const val MODID = "modelassetlib"

    val logger = LoggerFactory.getLogger(MODID)

    init {
        MOD_BUS.addListener(::onLoadComplete)
    }

    private fun onLoadComplete(event: FMLLoadCompleteEvent) {
        val mc = Minecraft.getInstance()
        NativeLibrary.loadNative(mc.resourceManager)
        NativeLibrary.initNative()
    }
}