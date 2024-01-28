package com.github.erfur.lasso

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.github.erfur.lasso.databinding.ActivityMainBinding

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        InjectorTestClass()
        ApplicationView(this)
    }
}