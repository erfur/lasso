package com.github.erfur.lasso

import android.content.ComponentName
import android.content.ServiceConnection
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.Message
import android.os.Messenger
import android.util.Log

class AppProcessFinderConnection: ServiceConnection, Handler.Callback {

    private lateinit var messenger: Messenger
    // dictionary of callbacks
    private val callbacks = mutableMapOf<String, Application.PidFoundCallback>()

    override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
        Log.d("AppProcessFinderConnection", "onServiceConnected")
        messenger = Messenger(service)
    }

    fun findProcessId(packageName: String, callback: Application.PidFoundCallback) {
        Log.d("AppProcessFinderConnection", "findProcessId")
        val msg = Message.obtain(null, AppProcessFinderService.FIND_PID)
        msg.data.putString("packageName", packageName)

        // set up a reply messenger
        val replyHandler = Handler(Looper.getMainLooper(), this)
        val replyMessenger = Messenger(replyHandler)
        msg.replyTo = replyMessenger

        callbacks[packageName] = callback
        messenger.send(msg)
    }

    fun getProcessMaps(pid: Int) {
        Log.d("AppProcessFinderConnection", "getProcessMaps")
        val msg = Message.obtain(null, AppProcessFinderService.GET_MAPS)
        msg.data.putInt("pid", pid)

        // set up a reply messenger
        val replyHandler = Handler(Looper.getMainLooper(), this)
        val replyMessenger = Messenger(replyHandler)
        msg.replyTo = replyMessenger

        messenger.send(msg)
    }

    fun sendInjectCodeMessage(pid: Int) {
        Log.d("AppProcessFinderConnection", "injectCode")
        val msg = Message.obtain(null, AppProcessFinderService.INJECT_CODE)
        msg.data.putInt("pid", pid)
        messenger.send(msg)
    }

    override fun onServiceDisconnected(name: ComponentName?) {
        Log.d("AppProcessFinderConnection", "onServiceDisconnected")
    }

    override fun handleMessage(msg: Message): Boolean {
        Log.d("AppProcessFinderConnection", "handleMessage")
        when (msg.what) {
            AppProcessFinderService.FIND_PID_REPLY -> {
                Log.d("AppProcessFinderConnection", "FIND_PID_REPLY")
                val pid = msg.data.getInt("pid")
                val packageName = msg.data.getString("packageName")!!
                callbacks[packageName]?.onPidFound(pid)
            }
        }
        return true
    }
}