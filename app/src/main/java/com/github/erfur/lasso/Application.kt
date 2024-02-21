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
                Log.i("Application", "pid found: $pid")
                setPid(pid)
            }
        }
    }

    fun triggerInjection(file_path: String) {
        val pid = _pid.value!!

        if (pid == -1) {
            Log.e("Application", "pid is not found, spawning the app...")
        }

        finder.focusApp(packageName)

        _pid.value?.let {
            finder.findProcessId(packageName) { pid: Int ->
                Log.i("Application", "pid found: $pid")
                setPid(pid)

                Log.i("Application", "injecting code ($file_path) into pid: $pid")
                finder.injectCode(pid, file_path)
            }
        }
    }

    override fun toString(): String {
        return "Application(name='$name', packageName='$packageName', pid=$pid)"
    }
}