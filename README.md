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

template = NeutralTemplate("file.ntpl", schema)
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

### Optimized rendering with render_once()

Use `render_once()` for better performance when you only need to render once:

```python
from neutraltemplate import NeutralTemplate

template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})

# Single render - use render_once() for best performance
contents = template.render_once()

# IMPORTANT: After render_once(), the template cannot be reused!
# The schema is consumed and subsequent renders will produce empty output.
# Create a new NeutralTemplate instance for the next render.
```

#### When to use render_once()

- **Single render per request**: Most web applications create a template, render it once, and discard it.
- **Large schemas**: When your schema contains thousands of keys, the performance improvement is significant.
- **Memory-constrained environments**: Avoids the memory spike of cloning large schemas.

#### When NOT to use render_once()

- **Multiple renders**: If you need to render the same template multiple times with the same schema, use `render()` instead.
- **Template reuse**: After `render_once()`, the template cannot be reused because the schema is consumed.

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

For best performance, choose the schema input method based on your use case:

| Method | Performance | Notes |
|--------|-------------|-------|
| `schema_obj` | **Best** | Python dict/list passed directly, no serialization overhead |
| `schema_msgpack` | **Near best** | Binary format, almost as fast as `schema_obj` |
| `schema_str` (JSON) | **2-3x slower** | Requires JSON parsing; difference is negligible for small schemas |

**Recommendation**: Use `schema_obj` for simplicity and best performance. For small schemas, the difference is not relevant.

```python
# Recommended: schema_obj (fastest and simplest)
template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})

# Good alternative: schema_msgpack (nearly as fast)
template = NeutralTemplate("file.ntpl", schema_msgpack=msgpack_bytes)

# Avoid for large schemas: schema_str (2-3x slower due to JSON parsing)
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

### Methods

| Method | Description |
|--------|-------------|
| `render()` | Render template (clones schema, reusable) |
| `render_once()` | Render template (consumes schema, optimized, not reusable) |
| `set_path(path)` | Set template file path |
| `set_source(source)` | Set template source code directly |
| `merge_schema(schema_str)` | Merge JSON schema from string |
| `merge_schema_msgpack(bytes)` | Merge MessagePack schema |
| `merge_schema_obj(obj)` | Merge Python dict/list as schema |
| `get_status_code()` | Get HTTP status code (e.g., "200") |
| `get_status_text()` | Get HTTP status text (e.g., "OK") |
| `get_status_param()` | Get additional error parameter |
| `has_error()` | Returns True if error occurred during render |

Links
-----

Neutral TS template engine Python Package.

- [Template docs](https://franbarinstance.github.io/neutralts-docs/docs/neutralts/doc/)
- [Repository](https://github.com/FranBarInstance/neutraltemplate)
- [Crate](https://crates.io/crates/neutralts)
- [PYPI Package](https://pypi.org/project/neutraltemplate/)
- [Examples](https://github.com/FranBarInstance/neutralts-docs/tree/master/examples/python)
