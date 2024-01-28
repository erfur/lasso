package com.github.erfur.lasso

import android.content.Context
import android.content.Intent
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.view.ViewGroup
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.google.android.material.floatingactionbutton.FloatingActionButton
import com.topjohnwu.superuser.ipc.RootService

class ApplicationView(ctx: Context) {

    private var apps: List<ApplicationInfo> = List<ApplicationInfo>(0) { ApplicationInfo() }
    private val ctx: Context
    private val listView: RecyclerView
    private val finder: AppProcessFinderConnection

    private fun refreshApps() {
        apps = ctx.packageManager.getInstalledApplications(PackageManager.GET_META_DATA)
    }

    private fun refreshAppsList() {
        listView.adapter = ApplicationAdapter(apps.filter {
            (it.flags and ApplicationInfo.FLAG_SYSTEM) == 0
        }.map {
            Application(it.loadLabel(ctx.packageManager).toString(), it.packageName, finder)
        })
    }

    init {
        this.ctx = ctx
        finder = AppProcessFinderConnection()

        // get the root view
        val rootView = (ctx as MainActivity).findViewById<ViewGroup>(android.R.id.content)

        // start AppProcessFinderService
        val intent = Intent(ctx, AppProcessFinderService::class.java)
        RootService.bind(intent, finder)

        listView = rootView.findViewById<RecyclerView>(R.id.recyclerView)
        listView.layoutManager = LinearLayoutManager(ctx)

        refreshApps()
        refreshAppsList()

        val refreshButton = rootView.findViewById<FloatingActionButton>(R.id.floatingActionButton)
        refreshButton.setOnClickListener {
            refreshApps()
            refreshAppsList()
        }
    }
}