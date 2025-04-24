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

```
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
        "snippets": {},
        "declare": {},
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
                },
                "el": {
                    "Hello nts": "Γεια σας",
                    "ref:greeting-nts": "Γεια σας"
                }
            }
        }
    },
    "data": {
        "CONTEXT": {
            "ROUTE": "",
            "HOST": "",
            "GET": {},
            "POST": {},
            "HEADERS": {},
            "FILES": {},
            "COOKIES": {},
            "SESSION": {},
            "ENV": {}
        },
        "hello": "Hello",
        "arr": {
            "hello": "Hello"
        }
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

# act accordingly at this point according to your framework

```

See: [example](https://github.com/FranBar1966/neutraltemplate/tree/master/examples)
