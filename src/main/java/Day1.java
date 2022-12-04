import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.*;

class CustomIntegerComparator implements Comparator<Integer> {

    @Override
    public int compare(Integer o1, Integer o2) {
        return o2.compareTo(o1);
    }
}

public class Day1 {
    public static void main(String[] args) {
        try {
            PriorityQueue<Integer> sums = new PriorityQueue<>(new CustomIntegerComparator());
            ArrayList<ArrayList<Integer>> inputs = getDayOneInputs();
            for (ArrayList<Integer> i : inputs) {
                int sum = i.stream().mapToInt(Integer::intValue).sum();
                sums.add(sum);
            }
            while (!sums.isEmpty()) {
                System.out.println(sums.poll());
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static ArrayList<ArrayList<Integer>> getDayOneInputs() throws IOException {
        ArrayList<ArrayList<Integer>> inputs = new ArrayList<>();
        String fileName = "src/main/resources/day1.txt";
        Path path = Paths.get(fileName);
        List<String> allLines = Files.readAllLines(path, StandardCharsets.UTF_8);
        ArrayList<Integer> elf = new ArrayList<>();
        for (String line : allLines) {
            if (Objects.equals(line, "")) {
                inputs.add((ArrayList<Integer>) elf.clone());
                elf.clear();
            } else {
                elf.add(Integer.parseInt(line));
            }

        }
        return inputs;
    }
}
