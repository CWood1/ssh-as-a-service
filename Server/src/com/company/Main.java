package com.company;

import org.omg.SendingContext.RunTime;

import javax.xml.crypto.Data;
import java.io.DataInput;
import java.io.DataInputStream;
import java.io.DataOutputStream;
import java.io.IOException;
import java.net.ServerSocket;
import java.net.Socket;
import java.util.HashMap;


public class Main {
    DataInputStream input;
    DataOutputStream output;


    public void main(String[] args) {
        //wait for connection
        getConnetion();
    }







    public void getConnetion() {
        try {

            ServerSocket socket = new ServerSocket(5432);
            System.out.println("Conncetion received from: " + socket.getInetAddress());
            Socket connection = socket.accept();
            input = new DataInputStream(connection.getInputStream());
            output = new DataOutputStream(connection.getOutputStream());
        } catch (IOException ex) {
            throw new RuntimeException("Socket error");
        }
    }


    public void FollowHost(String hostname) {
        byte[] buffer = new byte[4 + hostname.length()];
        if (hostname.length() > 0xFFFF) { //continuation flag
            buffer[3] = (byte) 0;
        }
        else {
            buffer[3] = (byte) 0;
        }
        short length = (short) hostname.length();
        buffer[0] = (byte) (length & 0xFF); //length
        buffer[1] = (byte) (length & 0xFF00);
        buffer[2] = 1; //message type

        for (int i = 3; i < buffer.length; i++)
            buffer[i] = (byte) hostname.charAt(i-3);

    }

    public  HashMap<String, Boolean> GetStatus() {
        byte[] message = {0, 0, 2, 0};
        try {
            output.write(message);
        }
        catch (IOException ex) {
            throw new RuntimeException("Fuck");

        }
        byte[] reply = Receive();

        HashMap<String, Boolean> list = new HashMap<>();

        int i = 0;

        while (i < reply.length) {
            int counter = 0;
            for (; reply[i] > 2; i++) {
                counter++;

            }
            byte[] name = new byte[counter];
            int y = i - counter;
            for (int x = 0; x < counter; x++)
                name[x] = reply[y++];

            String ascii = name.toString();
            boolean status = reply[i] == 1;
            list.put(ascii, status);
        }
        return list;


    }










    //support methods

    //support methods
    public byte[] Receive() {
        byte buffer[] = null;
        try {
            short len = input.readShort();

            if (len > 0) {
                buffer = new byte[len + 4];
                input.readFully(buffer);
            }
        }
        catch(IOException ex) {
            throw new RuntimeException("Shit's fucked mate");
        }
        return buffer;
    }


}






