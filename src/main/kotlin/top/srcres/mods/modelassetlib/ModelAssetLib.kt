package top.srcres.mods.modelassetlib

import net.neoforged.fml.common.Mod
import org.slf4j.LoggerFactory

@Mod(ModelAssetLib.MODID)
object ModelAssetLib {
    const val MODID = "modelassetlib"

    val logger = LoggerFactory.getLogger(MODID)

    init {
        logger.info("test message")
    }
}