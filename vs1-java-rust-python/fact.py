def factorial(n):
    resultado = 1
    for i in range(1, n + 1):
        resultado *= i
    return resultado

print(factorial(5))
print(factorial(27))

# ---------------------------------------------------------------------------
# VS // 01 · Factorial en Python.   Ejecutar:  python3 fact.py
# Los enteros de Python no tienen limite: factorial(27) = 10888869450418352160768000000 (correcto).
# El precio es rendimiento: cada multiplicacion trabaja con enteros de tamano variable.
