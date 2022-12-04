import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Set;
import java.util.stream.Collectors;

public class Day3 {
    public static void main(String[] args) {
        ArrayList<String> inputs = Helpers.getInputs("day3.txt");
        HashMap<String, Integer> priorities = new HashMap<>();
        priorities.put("a", 1);
        priorities.put("b", 2);
        priorities.put("c", 3);
        priorities.put("d", 4);
        priorities.put("e", 5);
        priorities.put("f", 6);
        priorities.put("g", 7);
        priorities.put("h", 8);
        priorities.put("i", 9);
        priorities.put("j", 10);
        priorities.put("k", 11);
        priorities.put("l", 12);
        priorities.put("m", 13);
        priorities.put("n", 14);
        priorities.put("o", 15);
        priorities.put("p", 16);
        priorities.put("q", 17);
        priorities.put("r", 18);
        priorities.put("s", 19);
        priorities.put("t", 20);
        priorities.put("u", 21);
        priorities.put("v", 22);
        priorities.put("w", 23);
        priorities.put("x", 24);
        priorities.put("y", 25);
        priorities.put("z", 26);
        priorities.put("A", 27);
        priorities.put("B", 28);
        priorities.put("C", 29);
        priorities.put("D", 30);
        priorities.put("E", 31);
        priorities.put("F", 32);
        priorities.put("G", 33);
        priorities.put("H", 34);
        priorities.put("I", 35);
        priorities.put("J", 36);
        priorities.put("K", 37);
        priorities.put("L", 38);
        priorities.put("M", 39);
        priorities.put("N", 40);
        priorities.put("O", 41);
        priorities.put("P", 42);
        priorities.put("Q", 43);
        priorities.put("R", 44);
        priorities.put("S", 45);
        priorities.put("T", 46);
        priorities.put("U", 47);
        priorities.put("V", 48);
        priorities.put("W", 49);
        priorities.put("X", 50);
        priorities.put("Y", 51);
        priorities.put("Z", 52);

        var ref = new Object() {
            int score1 = 0;
            int score2 = 0;
        };
        for (String input : inputs) {
            // System.out.println(input);
            HashSet<Character> compartment1 = input.substring(0, input.length() / 2).chars()
                    .mapToObj(e -> (char) e)
                    .collect(Collectors.toCollection(HashSet::new));
            HashSet<Character> compartment2 = input.substring(input.length() / 2).chars()
                    .mapToObj(e -> (char) e)
                    .collect(Collectors.toCollection(HashSet::new));
            compartment1.retainAll(compartment2);
            compartment1.forEach(x -> ref.score1 += priorities.get(x.toString()));

        }

        ArrayList<HashSet<Character>> elves = new ArrayList<>();
        for (String input : inputs) {
            // System.out.println(input);

            HashSet<Character> elf = input
                    .chars()
                    .mapToObj(e -> (char) e)
                    .collect(Collectors.toCollection(HashSet::new));

            elves.add(elf);
            if (elves.size() == 3) {
                elves.get(0).retainAll(elves.get(1));
                elves.get(0).retainAll(elves.get(2));
                System.out.print(elves.get(0));
                elves.get(0).forEach(x -> ref.score2 += priorities.get(x.toString()));
                elves.clear();
            }
        }

        System.out.println(ref.score1);
        System.out.println(ref.score2);
    }
}
