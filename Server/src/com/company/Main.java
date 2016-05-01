package com.company;

import java.util.HashMap;

public class Main {


    public static void main(String[] args) {
        Endpoint end = new Endpoint(); // listen for endpoint
        end.FollowHost("google.com");
        try {
            Thread.sleep(10000);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        HashMap<String, Boolean> xxx = end.GetStatus();
        System.out.println("Google.com: " + (xxx.get("google.com") ? "active" : "inactive"));

    }
}













