package com.example.roombasic;

import android.app.Application;
import androidx.annotation.NonNull;
import androidx.lifecycle.AndroidViewModel;
import androidx.lifecycle.LiveData;

import java.util.List;

public class UserViewModel extends AndroidViewModel {
    private UserRepository userRepository;
    public UserViewModel(@NonNull Application application) {
        super(application);
        // repository
        userRepository = new UserRepository(application);
    }
    LiveData<List<User>> getAllUserLive() {
        return userRepository.getAllUserLive();
    }
    void insertUsers(User... users) {
        userRepository.insertUsers(users);
    }
    void deleteUsers(User... users) {
        userRepository.deleteUsers(users);
    }
    void deleteAllUsers(User... users) {
        userRepository.deleteAllUsers();
    }
}
