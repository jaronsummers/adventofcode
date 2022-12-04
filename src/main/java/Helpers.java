import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

public class Helpers {
    public static ArrayList<String> getInputs(String fileName) {
        String fileRoot = "src/main/resources/";
        Path path = Paths.get(fileRoot, fileName);
        List<String> allLines = null;
        try {
            allLines = Files.readAllLines(path, StandardCharsets.UTF_8);
        } catch (IOException e) {
            e.printStackTrace();
            System.exit(1);
        }
        return new ArrayList<>(allLines);
    }
}
