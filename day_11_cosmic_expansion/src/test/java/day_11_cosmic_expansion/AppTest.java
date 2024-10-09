import day_11_cosmic_expansion.App;
import org.junit.jupiter.api.Test;

import java.io.IOException;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class AppTest {
    @Test
    public void shouldLoadTestInput() throws IOException {
        String testInput = App.loadTestInput("test_input.txt");

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

        assertEquals(testInput, expected);
    }
}
