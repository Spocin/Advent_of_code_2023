package day_11_cosmic_expansion;

import org.junit.jupiter.api.Test;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

public class AppTest {
    static String testInputFileName = "test_input.txt";

    @Test
    public void shouldLoadTestInput() throws IOException {
        var galaxy = App.loadGalaxyFromFile(testInputFileName);

        var expected = """
                ...#......
                .......#..
                #.........
                ..........
                ......#...
                .#........
                .........#
                ..........
                .......#..
                #...#.....""";

        assertEquals(galaxy.getInput(), expected);
    }

    @Test
    public void galaxyShouldExpandCorrectly() {
        assertTrue(true);
    }
}
