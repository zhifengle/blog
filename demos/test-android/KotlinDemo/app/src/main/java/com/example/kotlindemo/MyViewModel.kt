package com.example.kotlindemo

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel

class MyViewModel : ViewModel() {
    private val _number: MutableLiveData<Int> by lazy { MutableLiveData<Int>().also { it.value = 0 } }
    val number: LiveData<Int>
    get() = _number
    fun modifyNumber(aNumber: Int) {
        _number.value = _number.value?.plus(aNumber)
    }
}
