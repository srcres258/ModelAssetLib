package top.srcres.mods.modelassetlib

import net.minecraft.client.Minecraft
import net.neoforged.fml.common.Mod
import net.neoforged.fml.event.lifecycle.FMLLoadCompleteEvent
import org.slf4j.Logger
import org.slf4j.LoggerFactory
import thedarkcolour.kotlinforforge.neoforge.forge.MOD_BUS
import java.util.ArrayList

@Mod(ModelAssetLib.MODID)
object ModelAssetLib {
    const val MODID = "modelassetlib"

    val logger: Logger = LoggerFactory.getLogger(MODID)
    val mcInstance: Minecraft
        get() = Minecraft.getInstance()
    private val nativeLoadedListenerList = ArrayList<() -> Unit>();

    init {
        MOD_BUS.addListener(::onLoadComplete)
    }

    fun addNativeLoadedListener(listener: () -> Unit) {
        nativeLoadedListenerList.add(listener)
    }

    private fun onLoadComplete(event: FMLLoadCompleteEvent) {
        val mc = Minecraft.getInstance()
        NativeLibrary.loadNative(mc.resourceManager)
        NativeLibrary.initNative()

        for (listener in nativeLoadedListenerList) {
            listener()
        }
    }
}