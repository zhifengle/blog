package com.example.roombasic;

import android.os.Bundle;
import android.view.View;
import android.widget.TextView;
import androidx.appcompat.app.AppCompatActivity;
import androidx.lifecycle.Observer;
import androidx.lifecycle.ViewModelProvider;
import androidx.recyclerview.widget.LinearLayoutManager;
import androidx.recyclerview.widget.RecyclerView;

import java.util.ArrayList;
import java.util.List;

public class MainActivity extends AppCompatActivity {
    UserViewModel userViewModel;
    RecyclerView recyclerView;
    MyAdapter myAdapter;

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);
        recyclerView = findViewById(R.id.recyclerView);
        myAdapter = new MyAdapter();
        recyclerView.setLayoutManager(new LinearLayoutManager(this));
        recyclerView.setAdapter(myAdapter);
        userViewModel = new ViewModelProvider(this).get(UserViewModel.class);
        userViewModel.getAllUserLive().observe(this, new Observer<List<User>>() {
            @Override
            public void onChanged(List<User> users) {
                myAdapter.setAllUsers(users);
                myAdapter.notifyDataSetChanged();
            }
        });
        findViewById(R.id.buttonInsert).setOnClickListener(new View.OnClickListener() {
            @Override
            public void onClick(View view) {
                String[] words = {
                        "Called",
                        "with",
                        "the",
                        "data",
                        "in",
                        "the",
                        "database",
                        "to",
                        "decide",
                        "whether",
                        "to",
                        "fetch"

                };
                List<User> users = new ArrayList<>();
                for (String w: words) {
                    User user1 = new User();
                    user1.firstName = w;
                    user1.lastName = new StringBuilder(w).reverse().toString();
                    users.add(user1);
                }
                userViewModel.insertUsers(users.toArray(new User[0]));
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
