import java.util.ArrayList;
import java.util.stream.IntStream;

public class Day4 {
    public static void main(String[] args) {
        ArrayList<String> inputs = Helpers.getInputs("day4.txt");
        var ref = new Object() {
            int score1 = 0;
            int score2 = 0;
        };

        for (String input : inputs) {
            String[] split = input.split(",");
            ArrayList<Integer> elf1 = parsePair(split[0].split("-"));
            ArrayList<Integer> elf2 = parsePair(split[1].split("-"));
            if (elf1.containsAll(elf2) || elf2.containsAll(elf1)) {
                ref.score1++;
            }
        }
        for (String input : inputs) {
            String[] split = input.split(",");
            ArrayList<Integer> elf1 = parsePair(split[0].split("-"));
            ArrayList<Integer> elf2 = parsePair(split[1].split("-"));
            elf1.retainAll(elf2);
            elf2.retainAll(elf1);
            if (elf1.size() > 0 || elf2.size() > 0) {
                ref.score2++;
            }
        }
        System.out.println("Star 1: " + ref.score1);
        System.out.println("Star 2: " + ref.score2);
    }

    public static ArrayList<Integer> parsePair(String[] pair) {
        return IntStream.range(Integer.parseInt(pair[0]), Integer.parseInt(pair[1]) + 1).collect(ArrayList::new, ArrayList::add, ArrayList::addAll);
    }
}
