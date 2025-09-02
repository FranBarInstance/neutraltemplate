Neutral TS Python package example with Flask
============================================

Neutral is a templating engine for the web written in Rust, designed to work with any programming language (language-agnostic) via IPC/Package and natively as library/crate in Rust.

```
python -m venv .venv
source .venv/bin/activate
pip install Flask
pip install neutraltemplate
```

Run:

```
source .venv/bin/activate
python src/app.py
```

A server will be available on port 5000

```
http://127.0.0.1:5000/
