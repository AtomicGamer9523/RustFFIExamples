public class Main {
    private static native String greet(String name);
    static {
        System.loadLibrary("rustlib");
    }

    public static void main(String[] args) {
        String output = Main.greet("World");
        System.out.println(output);
    }
}
