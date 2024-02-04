package com.github.erfur.lasso

import android.util.Log
import androidx.lifecycle.MutableLiveData

data class Application(val packageName: String, val name: String, val finder: InjectorServiceConnection) {

    private var _pid: MutableLiveData<Int> = MutableLiveData(-1)
    val pid: MutableLiveData<Int>
        get() = _pid

    fun setPid(pid: Int) {
        _pid.value?.let {
            _pid.value = pid
        }
    }

    fun triggerUpdatePid() {
        _pid.value?.let {
            finder.findProcessId(packageName) { pid: Int ->
                setPid(pid)
                Log.i("Application", "pid found: $pid")
            }
        }
    }

    fun triggerInjection() {
        Log.i("Application", "injecting code into pid: $pid")
        finder.injectCode(_pid.value!!)
    }

    override fun toString(): String {
        return "Application(name='$name', packageName='$packageName', pid=$pid)"
    }
}