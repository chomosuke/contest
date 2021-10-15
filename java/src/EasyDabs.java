import java.util.Scanner;

public class EasyDabs {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int k = sc.nextInt();
        int c = sc.nextInt();
        int t = sc.nextInt();
        System.out.println(t / k * c);
    }
}
