package com.github.erfur.lasso

import android.util.Log

data class Application(val name: String, val packageName: String, val finder: AppProcessFinderConnection) {

    override fun toString(): String {
        return "Application(name='$name', packageName='$packageName')"
    }

    fun getPid(callback: PidFoundCallback?) {
        if (callback is PidFoundCallback) {
            finder.findProcessId(packageName, callback)
        } else {
            finder.findProcessId(packageName, object: PidFoundCallback() {
                override fun onPidFound(pid: Int) {
                    Log.i("Application", "injecting code into pid: $pid")
                    finder.sendInjectCodeMessage(pid)
                }
            })
        }
    }

    open class PidFoundCallback() {
        open fun onPidFound(pid: Int) {}
    }
}