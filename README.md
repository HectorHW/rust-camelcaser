Небольшой бенчмарк, сравнивающий rust и python на примере перевода из snake_case в camelCase.

## Сборка

* создай venv (чтобы можно было легко установить модуль) через `python3 -m venv venv` и активируй его `source venv/bin/activate`

* поставь tarpaulin через `pip install maturin`

* собери и установи модуль через `maturin develop --release`

* запусти бенчмарк: `python3 bench.py`

