package com.example.kotlindemo

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import androidx.lifecycle.Observer
import androidx.lifecycle.ViewModelProvider
import kotlinx.android.synthetic.main.activity_main.*

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
        textView.text = "0"
        val viewModel = ViewModelProvider(this).get(MyViewModel::class.java)

        viewModel.number.observe(this, Observer { textView.text = it.toString() })
        buttonPlus.setOnClickListener {
            viewModel.modifyNumber(1)
        }

        buttonMinus.setOnClickListener {
            viewModel.modifyNumber(-1)
        }
    }
}
