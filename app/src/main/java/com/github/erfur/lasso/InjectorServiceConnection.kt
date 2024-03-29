package com.github.erfur.lasso

import android.content.ComponentName
import android.content.ServiceConnection
import android.os.Handler
import android.os.IBinder
import android.os.Looper
import android.os.Message
import android.os.Messenger
import android.util.Log

class InjectorServiceConnection : ServiceConnection, Handler.Callback {

    private lateinit var messenger: Messenger
    private var bound: Boolean = false

    // dictionary of callbacks
    private val callbacks = mutableMapOf<String, (Int) -> Unit>()

    override fun onServiceConnected(name: ComponentName?, service: IBinder?) {
        Log.d("AppProcessFinderConnection", "onServiceConnected")
        messenger = Messenger(service)
        bound = true
    }

    override fun onServiceDisconnected(name: ComponentName?) {
        Log.d("AppProcessFinderConnection", "onServiceDisconnected")
        bound = false
    }

    fun findProcessId(packageName: String, callback: (Int) -> Unit) {
        if (!bound) {
            Log.d("AppProcessFinderConnection", "not bound")
            return
        }

        Log.d("AppProcessFinderConnection", "findProcessId")
        val msg = Message.obtain(null, InjectorService.FIND_PID)
        msg.data.putString("packageName", packageName)

        // set up a reply messenger
        val replyHandler = Handler(Looper.getMainLooper(), this)
        val replyMessenger = Messenger(replyHandler)
        msg.replyTo = replyMessenger

        callbacks[packageName] = callback
        messenger.send(msg)
    }

    fun focusApp(packageName: String) {
        if (!bound) {
            Log.d("AppProcessFinderConnection", "not bound")
            return
        }

        Log.d("AppProcessFinderConnection", "focusApp")
        val msg = Message.obtain(null, InjectorService.FOCUS_APP)
        msg.data.putString("package_name", packageName)
        messenger.send(msg)
    }

    fun injectCode(pid: Int, filePath: String) {
        if (!bound) {
            Log.d("AppProcessFinderConnection", "not bound")
            return
        }

        Log.d("AppProcessFinderConnection", "injectCode")
        val msg = Message.obtain(null, InjectorService.INJECT_CODE)
        msg.data.putInt("pid", pid)
        msg.data.putString("file_path", filePath)
        messenger.send(msg)
    }

    override fun handleMessage(msg: Message): Boolean {
        if (!bound) {
            Log.d("AppProcessFinderConnection", "not bound")
            return false
        }

        Log.d("AppProcessFinderConnection", "handleMessage")
        when (msg.what) {
            InjectorService.FIND_PID_REPLY -> {
                Log.d("AppProcessFinderConnection", "FIND_PID_REPLY")
                val pid = msg.data.getInt("pid")
                val packageName = msg.data.getString("packageName")!!
                callbacks[packageName]?.let {
                    it(pid)
                }
            }
        }
        return true
    }
}