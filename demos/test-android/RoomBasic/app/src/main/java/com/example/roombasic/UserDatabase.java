package com.example.roombasic;

import android.content.Context;
import androidx.room.Database;
import androidx.room.Room;
import androidx.room.RoomDatabase;

@Database(entities = {User.class}, version = 1)
public abstract class UserDatabase extends RoomDatabase {
    private static UserDatabase INSTANCE;
    static synchronized UserDatabase getDatabase(Context ctx) {
        if (INSTANCE == null) {
            INSTANCE = Room.databaseBuilder(ctx.getApplicationContext(),
                    UserDatabase.class, "user_database").build();
        }
        return INSTANCE;
    }
    public abstract UserDao getUserDao();
}
