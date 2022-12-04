import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class Day2 {
    public static void main(String[] args) {
        ArrayList<String> inputs = Helpers.getInputs("day2.txt");
        int star1score = 0;
        Map<String, Integer> scoreTable = new HashMap<>();

        scoreTable.put("X", 1);
        scoreTable.put("Y", 2);
        scoreTable.put("Z", 3);
        scoreTable.put("A X", 3);
        scoreTable.put("A Y", 6);
        scoreTable.put("A Z", 0);
        scoreTable.put("B X", 0);
        scoreTable.put("B Y", 3);
        scoreTable.put("B Z", 6);
        scoreTable.put("C X", 6);
        scoreTable.put("C Y", 0);
        scoreTable.put("C Z", 3);

        for (String input : inputs) {
            star1score += scoreTable.get(String.valueOf(input.charAt(input.length() - 1)));
            star1score += scoreTable.get(input);
            System.out.println(input + " " + star1score);
        }

        Map<String, Integer> star2table = new HashMap<>();
        int star2score = 0;
        // A = rock
        // B = paper
        // C = scissors
        // X = lose
        // Y = draw
        // z = win
        // Rock = 1
        // Paper = 2
        // Scissors = 3

        star2table.put("A X", 3);
        star2table.put("A Y", 4);
        star2table.put("A Z", 8);
        star2table.put("B X", 1);
        star2table.put("B Y", 5);
        star2table.put("B Z", 9);
        star2table.put("C X", 2);
        star2table.put("C Y", 6);
        star2table.put("C Z", 7);

        for (String input : inputs) {
            star2score += star2table.get(input);
            System.out.println(input + " " + star2score);
        }

        System.out.println("Star 1: " + star1score);
        System.out.println("Star 2: " + star2score);

    }


}
