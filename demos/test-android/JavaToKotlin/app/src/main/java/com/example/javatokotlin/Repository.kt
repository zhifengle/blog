package com.example.javatokotlin

import java.util.*
import kotlin.collections.ArrayList

class Repository private constructor() {
    private val users = mutableListOf<User>()
    fun getUsers(): List<User?>? {
        return users
    }

    val formattedUserNames: List<String>
        get() {
            val userNames = ArrayList<String>(users.size)
            for ((firstName, lastName) in users) {
                var name = if (lastName != null) {
                    if (firstName != null) {
                        "$firstName $lastName"
                    } else {
                        lastName
                    }
                } else {
                    firstName ?: "Unknown"
                }
                userNames.add(name)
            }
            return userNames
        }

    companion object {
        private var INSTANCE: Repository? = null
        val instance: Repository?
            get() {
                if (INSTANCE == null) {
                    synchronized(Repository::class.java) { INSTANCE = Repository() }
                }
                return INSTANCE
            }
    }

    init {
        val user1 = User("Jane", "")
        val user2 = User("John", null)
        val user3 = User("Anne", "Doe")
        users.add(user1)
        users.add(user2)
        users.add(user3)
    }
}
