package day_11_cosmic_expansion;

import org.junit.jupiter.api.Test;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class AppTest {
    @Test
    public void shouldLoadTestInput() throws IOException {
        Galaxy testGalaxy = App.loadGalaxyFromFile("test_input.txt");

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

        assertEquals(testGalaxy.getRawInput(), expected);
    }
}
