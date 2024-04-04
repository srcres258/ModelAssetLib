package top.srcres.mods.modelassetlib

class UnsupportedOSException(osName: String)
    : RuntimeException("The operating system $osName is not supported.")
