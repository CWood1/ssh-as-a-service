package com.company;

import javafx.util.Pair;

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

        for(String b : xxx.keySet()) {
            System.out.println(b + ": " + xxx.get(b));
        }

    }
}













