for i in range(1, 16):
    if i % 15 == 0:
        print("FizzBuzz")
    elif i % 3 == 0:
        print("Fizz")
    elif i % 5 == 0:
        print("Buzz")
    else:
        print(i)

# ---------------------------------------------------------------------------
# VS // 01 · FizzBuzz en Python.   Ejecutar:  python3 fizzbuzz.py
# Se lee de arriba abajo y el truco es preguntar primero por el caso doble (i % 15).
# Salida: 1 2 Fizz 4 Buzz Fizz 7 8 Fizz Buzz 11 Fizz 13 14 FizzBuzz (uno por linea).
