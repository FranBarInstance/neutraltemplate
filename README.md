Python package for Neutral TS
=============================

Neutral is a templating engine for the web written in Rust, designed to work with any programming language (language-agnostic) via IPC/Package and natively as library/crate in Rust.

Install Package
---------------

```
pip install neutraltemplate
```

Usage
-----

See: [examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python)

### Basic usage with file

```python
from neutraltemplate import NeutralTemplate

schema = """
{
    "config": {
        "cache_prefix": "neutral-cache",
        "cache_dir": "",
        "cache_on_post": false,
        "cache_on_get": true,
        "cache_on_cookies": true,
        "cache_disable": false,
        "disable_js": false,
        "filter_all": false
    },
    "inherit": {
        "locale": {
            "current": "en",
            "trans": {
                "en": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "es": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                }
            }
        }
    },
    "data": {
        "hello": "Hello World"
    }
}
"""

template = NeutralTemplate("file.ntpl", schema_str=schema)
contents = template.render()

# e.g.: 200
status_code = template.get_status_code()

# e.g.: OK
status_text = template.get_status_text()

# empty if no error
status_param = template.get_status_param()

# Check for errors
if template.has_error():
    # Handle error
    pass
```

### Using Python dictionaries (schema_obj)

You can pass Python dictionaries directly without JSON serialization:

```python
from neutraltemplate import NeutralTemplate

schema_dict = {
    "data": {
        "title": "Hello World",
        "items": ["one", "two", "three"]
    }
}

# Pass dict directly via schema_obj parameter
template = NeutralTemplate("file.ntpl", schema_obj=schema_dict)
contents = template.render()
```

### Using set_source() for inline templates

```python
from neutraltemplate import NeutralTemplate

template = NeutralTemplate()
template.set_source("{:;data.title:}")
template.merge_schema_obj({"data": {"title": "Hello"}})
contents = template.render()  # "Hello"
```

### Multiple renders with the same NeutralTemplate instance

You can render the same `NeutralTemplate` instance multiple times:

```python
from neutraltemplate import NeutralTemplate

template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})

# First render
contents1 = template.render()

# You can modify the schema and render again
template.merge_schema_obj({"data": {"title": "World"}})
contents2 = template.render()
```

MessagePack schema
------------------

You can pass MessagePack bytes in the constructor or merge them later.

```python
from neutraltemplate import NeutralTemplate

# {"data": {"key": "value"}}
schema_msgpack = bytes([
    129, 164, 100, 97, 116, 97, 129, 163, 107, 101, 121, 165, 118, 97, 108, 117, 101
])

template = NeutralTemplate("file.ntpl", schema_msgpack=schema_msgpack)
template.merge_schema_msgpack(schema_msgpack)
```

Performance Notes
-----------------

For best performance, choose the schema input method based on your use case.
Current project benchmarks are in `local_bench/` and should be treated as workload-dependent:

| Method | Performance | Notes |
|--------|-------------|-------|
| `schema_obj` | **Best in current local bench** | Python dict/list converted recursively to JSON internally |
| `schema_msgpack` | **Near best in current local bench** | Binary format, validated at API boundary |
| `schema_str` (JSON) | **Usually slower for larger schemas** | Requires JSON parsing |

**Recommendation**: Use `schema_obj` for simplicity. For large schemas and hot paths, benchmark your own workload (`local_bench/bench.py`) before deciding.

```python
# Recommended: schema_obj (fastest and simplest)
template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})

# Good alternative: schema_msgpack (nearly as fast)
template = NeutralTemplate("file.ntpl", schema_msgpack=msgpack_bytes)

# Often slower for large schemas: schema_str (JSON parsing cost)
template = NeutralTemplate("file.ntpl", schema_str='{"data": {"title": "Hello"}}')
```

API Reference
-------------

### Constructor

```python
NeutralTemplate(
    path=None,           # Path to template file
    schema_str=None,     # JSON schema as string
    schema_msgpack=None, # MessagePack schema as bytes
    schema_obj=None      # Python dict/list as schema
)
```

Only one of `schema_str`, `schema_msgpack`, or `schema_obj` can be used at a time.
`schema_str` and `schema_msgpack` are validated by `neutralts` during `render()`.

### Methods

| Method | Description |
|--------|-------------|
| `render()` | Render template (optimized internally with render_once) |
| `set_path(path)` | Set template file path |
| `set_source(source)` | Set template source code directly |
| `merge_schema(schema_str)` | Merge JSON schema from string |
| `merge_schema_msgpack(bytes)` | Merge MessagePack schema |
| `merge_schema_obj(obj)` | Merge Python dict/list as schema |
| `get_status_code()` | Get HTTP status code (e.g., "200") |
| `get_status_text()` | Get HTTP status text (e.g., "OK") |
| `get_status_param()` | Get additional error parameter |
| `has_error()` | Returns True if error occurred during render |

### Advanced Usage Example

For those seeking to explore an **advanced**, production-ready implementation of this module within a full-featured Python Flask project, refer to the **[Neutral Starter Py](https://github.com/FranBarInstance/neutral-starter-py)** repository. It demonstrates complex integration scenarios, including scalable architecture patterns, comprehensive error handling, advanced dependency management, automated testing workflows.

Links
-----

Neutral TS template engine Python Package.

- [Neutral Starter Py](https://github.com/FranBarInstance/neutral-starter-py)
- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutraltemplate)
- [Crate](https://crates.io/crates/neutralts)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python)
