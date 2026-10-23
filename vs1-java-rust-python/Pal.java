public class Pal {
    static boolean esPalindromo(String palabra) {
        String limpia = palabra.toLowerCase().replace(" ", "");
        return limpia.equals(new StringBuilder(limpia).reverse().toString());
    }
    public static void main(String[] a) {
        for (int i = 1; i < 16; i++) {
            if (i % 15 == 0) System.out.println(i + ": FizzBuzz");
            else if (i % 3 == 0) System.out.println(i + ": Fizz");
            else if (i % 5 == 0) System.out.println(i + ": Buzz");
        }
        System.out.println(esPalindromo("Somos o no somos") + " " + esPalindromo("hola"));
    }
}

// ---------------------------------------------------------------------------
// VS // 01 · Palindromos en Java.   Ejecutar:  javac Pal.java && java Pal   -> true false
// Compara con la cadena invertida usando StringBuilder.reverse().
// Nota: esta version imprime ademas las lineas "3: Fizz", "5: Buzz"... antes del resultado (pal.py no lo hace).
