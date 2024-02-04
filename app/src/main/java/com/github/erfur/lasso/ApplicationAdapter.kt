package com.github.erfur.lasso

import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.ImageView
import android.widget.TextView
import androidx.recyclerview.widget.RecyclerView

class ApplicationAdapter(private val apps: List<Application>): RecyclerView.Adapter<ApplicationAdapter.ViewHolder>() {

    inner class ViewHolder(itemView: View): RecyclerView.ViewHolder(itemView) {
        val applicationIcon: ImageView = itemView.findViewById(R.id.imageView2)
        val applicationName: TextView = itemView.findViewById(R.id.textView)
        val applicationPackageName: TextView = itemView.findViewById(R.id.textView2)
        val applicationPid: TextView = itemView.findViewById(R.id.textView3)

        init {
            itemView.setOnClickListener {
                apps[adapterPosition].triggerInjection()
            }
        }

        fun bind(application: Application) {
            applicationIcon.setImageDrawable(itemView.context.packageManager.getApplicationIcon(application.packageName))
            applicationName.text = application.name
            applicationPackageName.text = application.packageName
            applicationPid.text = ""

            application.pid.observe(itemView.context as MainActivity) {
                if (it != -1)
                    applicationPid.text = it.toString()
            }
        }
    }

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): ViewHolder {
        val v = LayoutInflater.from(parent.context).inflate(R.layout.recycler_view_item, parent, false)
        return ViewHolder(v)
    }

    override fun getItemCount() = apps.size

    override fun onBindViewHolder(holder: ViewHolder, position: Int) {
        holder.bind(apps.get(position))
    }
}