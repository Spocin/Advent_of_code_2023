package day_11_cosmic_expansion;

import java.util.ArrayList;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class Galaxy {
    private final String rawInput;
    private ArrayList<ArrayList<Character>> galaxyMap;
    private int width;
    private int height;

    public Galaxy(String rawInput) {
        this.rawInput = rawInput;
        this.galaxyMap = this.parseInputToMap(this.rawInput);
        this.width = this.galaxyMap.getFirst().size();
        this.height = this.galaxyMap.size();
    }

    private ArrayList<ArrayList<Character>> parseInputToMap(String rawInput) {
        return rawInput
                .lines()
                .map(line -> line
                        .chars()
                        .mapToObj(e -> (char) e)
                        .collect(Collectors.toCollection(ArrayList::new))
                )
                .collect(Collectors.toCollection(ArrayList::new));
    }


    public void expandGalaxyMap() {
        this.expandRows();
        this.expandColumns();
    }

    private void expandRows() {
        //Get indexes to add row at
        var rowsIndexesToExpand = IntStream
                .range(0, galaxyMap.size())
                .filter(idx -> galaxyMap.get(idx).stream().allMatch(el -> el == '.'))
                .toArray();

        //Add rows at these indexes
        for (int i = 0; i < rowsIndexesToExpand.length; i++) {
            var emptyRow = new ArrayList<Character>(galaxyMap.getFirst().size());

            for (int j = 0; j < galaxyMap.getFirst().size(); j++) {
                emptyRow.add(j, '.');
            }

            galaxyMap.add(rowsIndexesToExpand[i], emptyRow);

            //Increment feature rows to hit correct position
            for (int j = i; j < rowsIndexesToExpand.length; j++) {
                rowsIndexesToExpand[j] = rowsIndexesToExpand[j] + 1;
            }
        }
    }

    private void expandColumns() {
        //Get indexes of columns to add at
        var columnsIndexesToExpand = IntStream
                .range(0, galaxyMap.getFirst().size() - 1)
                .filter(idx -> galaxyMap
                        .stream()
                        .map(line -> line.get(idx))
                        .allMatch(el -> el == '.'))
                .toArray();

        for (int i = 0; i < columnsIndexesToExpand.length; i++) {
            for (var characters : galaxyMap) {
                characters.add(columnsIndexesToExpand[i], '.');
            }

            //Increment feature columns to hit correct position
            for (int j = i; j < columnsIndexesToExpand.length; j++) {
                columnsIndexesToExpand[j] = columnsIndexesToExpand[j] + 1;
            }
        }
    }


    public String getRawInput() {
        return rawInput;
    }

    public String getMapAsString() {
        var sb = new StringBuilder();

        for (var line : galaxyMap) {
            for (var character : line) {
                sb.append(character);
            }

            sb.append("\n");
        }

        return sb.toString();
    }
}
