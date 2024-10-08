package day_11_cosmic_expansion;

public class Galaxy {
    private String input;

    public Galaxy(String input) {
        this.input = input;
    }

    @Override
    public String toString() {
        return this.input;
    }

    public String getInput() {
        return input;
    }
}
