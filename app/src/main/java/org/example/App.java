package org.example;

import de.gurkenlabs.litiengine.Game;

public class App {
    public String getGreeting() {
        return "Hello World!";
    }

    public static void main(String[] args) {
        Game.init();
        Game.start();
    }
}
