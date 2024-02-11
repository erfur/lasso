package com.github.erfur.lasso

import android.app.ActivityManager
import android.content.Intent
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.Message
import android.os.Messenger
import android.util.Log
import com.topjohnwu.superuser.ipc.RootService

class InjectorService: RootService(), Handler.Callback {

    private external fun initLasso()
    private external fun getMaps(pid: Int)
    private external fun injectCode(pid: Int, filePath: String)

    init {
        System.loadLibrary("linjector_jni")
    }

    companion object {
        const val FIND_PID: Int = 0
        const val FIND_PID_REPLY: Int = 1
        const val GET_MAPS: Int = 2
        const val GET_MAPS_REPLY: Int = 3
        const val FOCUS_APP: Int = 4
        const val INJECT_CODE: Int = 5
    }

    override fun onBind(intent: Intent): IBinder {
        Log.d("AppProcessFinderService", "onBind")
        initLasso()
        return Messenger(Handler(Looper.getMainLooper(), this)).binder
    }

    // method that finds the process id of a given package name
    private fun findProcessId(packageName: String): Int {
        Log.d("AppProcessFinderService", "findProcessId")
        val manager = getSystemService(ACTIVITY_SERVICE) as ActivityManager
        val runningApps = manager.runningAppProcesses
        var pid = -1

        for (processInfo in runningApps) {
            if (processInfo.processName == packageName) {
                pid = processInfo.pid
            }
        }

        Log.d("AppProcessFinderService", "pid: ${pid}")
        return pid
    }

    override fun handleMessage(msg: Message): Boolean {
        Log.d("AppProcessFinderService", "handleMessage")
        return when (msg.what) {
            FIND_PID -> {
                Log.d("AppProcessFinderService", "FIND_PID")
                val packageName = msg.data.getString("packageName")!!
                val reply = Message.obtain(null, FIND_PID_REPLY)
                reply.data.putInt("pid", findProcessId(packageName))
                reply.data.putString("packageName", packageName)
                msg.replyTo.send(reply)
                true
            }

            GET_MAPS -> {
                Log.d("AppProcessFinderService", "GET_MAPS")
                val pid = msg.data.getInt("pid")
                val reply = Message.obtain(null, GET_MAPS_REPLY)
                reply.data.putInt("pid", pid)
                getMaps(pid)
                reply.data.putInt("maps", 31337)
                msg.replyTo.send(reply)
                true
            }

            FOCUS_APP -> {
                Log.d("AppProcessFinderService", "FOCUS_APP")
                val packageName = msg.data.getString("package_name")!!

                // bring the application forward
                Log.d("AppProcessFinderService", "bringing application to the foreground")
                Runtime.getRuntime().exec(arrayOf("monkey", "-p", packageName, "1"))
                true
            }

            INJECT_CODE -> {
                Log.d("AppProcessFinderService", "INJECT_CODE")
                val pid = msg.data.getInt("pid")
                val filePath = msg.data.getString("file_path")!!

                Log.d("AppProcessFinderService", "injecting code into pid: $pid from file: $filePath")
                injectCode(pid, filePath)
                true
            }

            else -> {
                Log.d("AppProcessFinderService", "invalid message: ${msg.what}")
                false
            }
        }
    }
}

