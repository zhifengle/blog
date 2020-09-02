package com.example.roombasic;

import android.view.View;
import android.widget.Button;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;
import android.os.Bundle;
import androidx.lifecycle.LiveData;
import androidx.lifecycle.Observer;
import androidx.lifecycle.ViewModel;
import androidx.lifecycle.ViewModelProvider;
import androidx.room.Room;

import java.util.List;

public class MainActivity extends AppCompatActivity {
    TextView textView;
    Button btnInsert, btnClear;
    UserViewModel userViewModel;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        textView = findViewById(R.id.textView);
        userViewModel = new ViewModelProvider(this).get(UserViewModel.class);
        userViewModel.getAllUserLive().observe(this, new Observer<List<User>>() {
            @Override
            public void onChanged(List<User> users) {
                StringBuilder text = new StringBuilder();
                for (int i = 0; i < users.size(); i++) {
                    User user = users.get(i);
                    text.append(user.uid)
                            .append(":").append(user.firstName)
                            .append("=").append(user.lastName)
                            .append("\n");
                }
                textView.setText(text);
            }
        });
        findViewById(R.id.buttonInsert).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                User user1 = new User();
                user1.firstName = "Jack";
                user1.lastName = "Lee";
                User user2 = new User();
                user2.firstName = "Alan";
                user2.lastName = "Wood";
                userViewModel.insertUsers(user1, user2);
            }
        });
        findViewById(R.id.buttonDelete).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                userViewModel.deleteAllUsers();
            }
        });
    }
}
