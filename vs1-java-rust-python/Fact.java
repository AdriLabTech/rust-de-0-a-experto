import java.math.BigInteger;

public class Fact {
    static long factorial(int n) {
        long resultado = 1;
        for (int i = 1; i <= n; i++) {
            resultado *= i;
        }
        return resultado;
    }

    static long factorialSeguro(int n) {
        long resultado = 1;
        for (int i = 1; i <= n; i++) {
            resultado = Math.multiplyExact(resultado, i);
        }
        return resultado;
    }

    static BigInteger factorialGrande(int n) {
        BigInteger resultado = BigInteger.ONE;
        for (int i = 1; i <= n; i++) {
            resultado = resultado.multiply(BigInteger.valueOf(i));
        }
        return resultado;
    }

    public static void main(String[] args) {
        System.out.println(factorial(5));
        System.out.println(factorial(27));
        System.out.println(factorialGrande(27));
        try {
            System.out.println(factorialSeguro(27));
        } catch (ArithmeticException e) {
            System.out.println("ArithmeticException: " + e.getMessage());
        }
    }
}

// ---------------------------------------------------------------------------
// VS // 01 · Factorial en Java.   Ejecutar:  javac Fact.java && java Fact
// factorial(27) con long (64 bits) se desborda EN SILENCIO y da -5483646897237262336.
// Dos arreglos: Math.multiplyExact lanza ArithmeticException: long overflow; BigInteger no tiene limite
// (da 10888869450418352160768000000) pero es mas pesado.
