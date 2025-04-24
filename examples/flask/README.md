Neutral TS Python package example with Flask
============================================

Neutral is a templating engine for the web written in Rust, designed to work with any programming language (language-agnostic) via IPC/Package and natively as library/crate in Rust.

```
pip install Flask
pip install neutraltemplate
```

Navigate to the examples/flask/src directory and then:

```
export FLASK_APP=app.py && flask run
```

A server will be available on port 5000

```
http://127.0.0.1:5000/
