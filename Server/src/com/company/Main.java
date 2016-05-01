package com.company;
public class Main {


    public static void main(String[] args) {
        Endpoint end = new Endpoint(); // listen for endpoint
        end.FollowHost("google.com");
        try {
            Thread.sleep(10000);
        } catch (InterruptedException e) {
            e.printStackTrace();
        }
        end.GetStatus();

    }
}













