package day_11_cosmic_expansion;

import java.io.IOException;
import java.nio.charset.StandardCharsets;

public class App {
    static String inputFileName = "input.txt";

    public static void main(String[] args) throws IOException {
        System.out.println(loadGalaxyFromFile(inputFileName));
    }

    public static Galaxy loadGalaxyFromFile(String fileName) throws IOException {
        try (var inputStream = App.class.getClassLoader().getResourceAsStream(fileName)) {
            if (inputStream == null) {
                throw new IllegalArgumentException(String.format("File with name %s not found", fileName));
            }

            var input = new String(inputStream.readAllBytes(), StandardCharsets.UTF_8);

            return new Galaxy(input);
        } catch (IOException e) {
            throw new IOException("Error while trying to open stream to input.");
        }
    }
}
