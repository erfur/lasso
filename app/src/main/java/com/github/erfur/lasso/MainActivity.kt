package com.github.erfur.lasso

import android.content.Context
import android.content.Intent
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.net.Uri
import android.os.Bundle
import android.os.ParcelFileDescriptor
import android.view.ViewGroup
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.github.erfur.lasso.databinding.ActivityMainBinding
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.topjohnwu.superuser.ipc.RootService
import java.io.File.createTempFile
import java.io.FileInputStream
import java.io.FileNotFoundException


class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    private lateinit var installedApps: List<ApplicationInfo>
    private lateinit var apps: List<Application>
    private lateinit var listView: RecyclerView
    private lateinit var finder: InjectorServiceConnection
    private lateinit var selectedApp: Application

    private fun refreshInstalledApps(){
        installedApps = packageManager.getInstalledApplications(PackageManager.GET_META_DATA)
    }

    private fun refreshAppsList(){
        apps = installedApps.filter {
            (it.flags and ApplicationInfo.FLAG_SYSTEM) == 0
        }.map {
            Application(it.packageName, it.loadLabel(packageManager).toString(), finder)
        }.sortedBy { it.name }

        listView.adapter = ApplicationAdapter(apps) {
            selectedApp = it
            selectFile()
        }
    }

    fun selectFile() {
        val intent = Intent(Intent.ACTION_GET_CONTENT).apply {
            addCategory(Intent.CATEGORY_OPENABLE)
            type = "*/*"
            addFlags(Intent.FLAG_GRANT_READ_URI_PERMISSION)
            addFlags(Intent.FLAG_GRANT_WRITE_URI_PERMISSION)
        }

        startActivityForResult(intent, 1)
    }

    override fun onActivityResult(requestCode: Int, resultCode: Int, data: Intent?) {
        super.onActivityResult(requestCode, resultCode, data)
        if (requestCode == 1 && resultCode == RESULT_OK) {
            val pfd = FileDescriptorUtil.getFileDescriptor(this, data?.data!!)!!
            val file = createTempFile("inject", ".so", cacheDir)
            FileInputStream(pfd.fileDescriptor).copyTo(file.outputStream())

            selectedApp.triggerInjection(file.absolutePath)
        }
    }

    object FileDescriptorUtil {
        fun getFileDescriptor(context: Context, uri: Uri?): ParcelFileDescriptor? {
            var pfd: ParcelFileDescriptor? = null
            try {
                pfd = context.contentResolver.openFileDescriptor(uri!!, "r")
            } catch (e: FileNotFoundException) {
                e.printStackTrace()
            }
            return pfd
        }

        fun getFd(pfd: ParcelFileDescriptor?): Int {
            return pfd?.fd ?: -1 // Invalid file descriptor
        }
    }

    private fun refreshPids() {
        apps.map {
            it.triggerUpdatePid()
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // InjectorTestClass()

        // start AppProcessFinderService
        finder = InjectorServiceConnection()
        val intent = Intent(this, InjectorService::class.java)
        RootService.bind(intent, finder)

        // get the root view
        val rootView = this.findViewById<ViewGroup>(android.R.id.content)
        listView = rootView.findViewById<RecyclerView>(R.id.recyclerView)
        listView.layoutManager = LinearLayoutManager(this)

        // init application list
        refreshInstalledApps()
        refreshAppsList()
        refreshPids()

        // set up the refresh button
        val refreshButton = rootView.findViewById<FloatingActionButton>(R.id.floatingActionButton)
        refreshButton.setOnClickListener {
            refreshPids()
        }
    }
}