package com.example.roombasic;

import android.content.Context;
import android.os.AsyncTask;
import androidx.lifecycle.LiveData;

import java.util.List;

public class UserRepository {
    private LiveData<List<User>> allUserLive;
    private UserDao userDao;

    public UserRepository(Context context) {
        UserDatabase userDatabase = UserDatabase.getDatabase(context.getApplicationContext());
        userDao = userDatabase.getUserDao();
        allUserLive = userDao.getAllLive();
    }
    public LiveData<List<User>> getAllUserLive() {
        return allUserLive;
    }

    public UserDao getUserDao() {
        return userDao;
    }

    public void insertUsers(User ...users) {
        new InsertAsyncTask(userDao).execute(users);
    }
    public void deleteUsers(User ...users) {
        new DeleteAsyncTask(userDao).execute(users);
    }
    public void deleteAllUsers() {
        new DeleteAllAsyncTask(userDao).execute();
    }

    static class InsertAsyncTask extends AsyncTask<User, Void, Void> {
        private UserDao userDao;
        InsertAsyncTask(UserDao userDao) {
            this.userDao = userDao;
        }
        @Override
        protected Void doInBackground(User... users) {
            userDao.insertAll(users);
            return null;
        }
    }

    static class DeleteAsyncTask extends AsyncTask<User, Void, Void> {
        private UserDao userDao;
        DeleteAsyncTask(UserDao userDao) {
            this.userDao = userDao;
        }
        @Override
        protected Void doInBackground(User... users) {
            userDao.deleteUsers(users);
            return null;
        }
    }
    static class DeleteAllAsyncTask extends AsyncTask<Void, Void, Void> {
        private UserDao userDao;
        DeleteAllAsyncTask(UserDao userDao) {
            this.userDao = userDao;
        }
        @Override
        protected Void doInBackground(Void... voids) {
            userDao.deleteAllUsers();
            return null;
        }
    }
}
