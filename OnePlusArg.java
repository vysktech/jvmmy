public class OnePlusArg {
  public int sum(int a, int b) {
    return a + b;
  }

  public static void main(String[] args) {
    int input = Integer.valueOf(args[0]);
    int result = new OnePlusArg().sum(10, input);
  }
}
