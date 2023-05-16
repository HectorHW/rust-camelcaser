import timeit
from tocamel import capitalize_rust

def to_camel_py(s):
    parts = s.split("_")
    return parts[0] + "".join(map(str.capitalize, parts[1:]))

big_string = "data" + "_data" * 100

python_speed = timeit.timeit("to_camel_py(big_string)", globals=globals())
rust_speed = timeit.timeit("capitalize_rust(big_string)", globals=globals())

print("python:", python_speed)
print("rust:", rust_speed)

speedup = (python_speed - rust_speed) / python_speed

print(f"speed reduction: {speedup*100:.1f}%")


