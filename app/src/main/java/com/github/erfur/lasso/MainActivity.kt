package com.github.erfur.lasso

import android.content.Intent
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.os.Bundle
import android.view.ViewGroup
import androidx.appcompat.app.AppCompatActivity
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.github.erfur.lasso.databinding.ActivityMainBinding
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.topjohnwu.superuser.ipc.RootService

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    private lateinit var installedApps: List<ApplicationInfo>
    private lateinit var apps: List<Application>
    private lateinit var listView: RecyclerView
    private lateinit var finder: InjectorServiceConnection

    private fun refreshInstalledApps(){
        installedApps = packageManager.getInstalledApplications(PackageManager.GET_META_DATA)
    }

    private fun refreshAppsList(){
        apps = installedApps.filter {
            (it.flags and ApplicationInfo.FLAG_SYSTEM) == 0
        }.map {
            Application(it.packageName, it.loadLabel(packageManager).toString(), finder)
        }

        listView.adapter = ApplicationAdapter(apps)
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