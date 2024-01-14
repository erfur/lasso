package com.github.erfur.lasso

import android.content.Intent
import android.content.pm.ApplicationInfo
import android.content.pm.PackageManager
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import android.view.Menu
import android.view.MenuItem
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.RecyclerView
import com.github.erfur.lasso.databinding.ActivityMainBinding
import com.topjohnwu.superuser.ipc.RootService

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    private external fun testFunction()

    companion object {
      init {
         System.loadLibrary("lasso")
      }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        val apps = packageManager.getInstalledApplications(PackageManager.GET_META_DATA)

        val finder = AppProcessFinderConnection()

        // start AppProcessFinderService
        val intent = Intent(this, AppProcessFinderService::class.java)
        RootService.bind(intent, finder)

        val recyclerView = findViewById<RecyclerView>(R.id.recyclerView)
        recyclerView.layoutManager = LinearLayoutManager(this)
        recyclerView.adapter = ApplicationAdapter(apps.filter {
            (it.flags and ApplicationInfo.FLAG_SYSTEM) == 0
        }.map {
            Application(it.loadLabel(packageManager).toString(), it.packageName, finder)
        })

        // create a thread that runs testFunction periodically
        Thread {
            while (true) {
                Thread.sleep(1000)
                testFunction()
            }
        }.start()
    }

    override fun onCreateOptionsMenu(menu: Menu): Boolean {
        // Inflate the menu; this adds items to the action bar if it is present.
        menuInflater.inflate(R.menu.menu_main, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        // Handle action bar item clicks here. The action bar will
        // automatically handle clicks on the Home/Up button, so long
        // as you specify a parent activity in AndroidManifest.xml.
        return when (item.itemId) {
            R.id.action_settings -> true
            else -> super.onOptionsItemSelected(item)
        }
    }
}