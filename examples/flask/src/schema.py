"""fill schema"""

import json
from http.cookies import SimpleCookie
from flask import request
from constants import DEFAULT_SCHEMA, LANG_KEY, THEME_KEY

# It is important to distinguish between data coming from the user and data
# coming from the application. "CONTEXT" has some security measures such as escaping.
class Schema:
    """Schema"""

    def __init__(self, req, route):
        self.req = req
        self.route = route.strip('/\\')
        self.schema = {}
        self._default()
        self._populate_context()
        self._negotiate_language()
        self._set_theme()

    def _default(self):
        with open(DEFAULT_SCHEMA, "r", encoding="utf-8") as file:
            schema_json = file.read()
        self.schema = json.loads(schema_json)
        self.schema.setdefault("data", {})
        self.schema["data"].setdefault("CONTEXT", {})
        self.schema["data"]["CONTEXT"].setdefault("GET", {})
        self.schema["data"]["CONTEXT"].setdefault("POST", {})
        self.schema["data"]["CONTEXT"].setdefault("COOKIES", {})
        self.schema["data"]["CONTEXT"].setdefault("HEADERS", {})

    def _populate_context(self):
        self.schema["data"]["CONTEXT"]["ROUTE"] = self.route
        self.schema["data"]["CONTEXT"]["HEADERS"]["HOST"] = request.headers.get(
            "Host", None
        )

        for key, value in self.req.args.items():
            self.schema["data"]["CONTEXT"]["GET"][key] = value

        if self.req.method == "POST":
            for key, value in self.req.form.items():
                self.schema["data"]["CONTEXT"]["POST"][key] = value

        for key, value in request.headers.items():
            self.schema["data"]["CONTEXT"]["HEADERS"][key] = value

        if self.req.headers.get('Cookie'):
            cookie = SimpleCookie(self.req.headers.get('Cookie'))
            for key, morsel in cookie.items():
                self.schema["data"]["CONTEXT"]["COOKIES"][key] = morsel.value

        # Fake session
        self.schema["data"]["CONTEXT"]["SESSION"] = self.schema["data"]["CONTEXT"]["COOKIES"].get(
            "SESSION", None)

    def _negotiate_language(self):
        languages = self.schema["data"]["site"]["validLanguages"]

        self.schema["inherit"]["locale"]["current"] = (
            self.schema["data"]["CONTEXT"]["GET"].get(LANG_KEY) or
            self.schema["data"]["CONTEXT"]["COOKIES"].get(LANG_KEY) or
            self.req.accept_languages.best_match(languages) or
            ""
        )

        if self.schema["inherit"]["locale"]["current"] not in languages:
            self.schema["inherit"]["locale"]["current"] = languages[0]

    def _set_theme(self):
        """theme"""
        self.schema["data"]["site"]["theme"] = (
            self.schema["data"]["CONTEXT"]["GET"].get(THEME_KEY) or
            self.schema["data"]["CONTEXT"]["COOKIES"].get(THEME_KEY) or
            self.schema["data"]["site"]["validThemes"][0]
        )

    def get(self):
        """get schema"""
        return self.schema
