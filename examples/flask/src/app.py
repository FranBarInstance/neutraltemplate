"""Neutral TS Python package example with Flask"""

# Each route must also be added to schema.json:
#
# "validRoutes": [
#     "home",
#     ...
# ],
#
# "validRoutes" is an arbitrary name, the template verifies that there is a key to a route.

import os

from constants import STATIC_FOLDER
from flask import Flask, request, send_from_directory
from schema import Schema
from template import Template

app = Flask(__name__)

# Templates handle 404 errors, to prevent arbitrary routes from being bound to "validRoutes"
# in the "schema", to create a simple view/route you wouldn't need a function or view.
@app.route('/<path:route>', methods=['GET', 'POST'])
def catch_all(route):
    """default handle"""

    file_path = os.path.join(STATIC_FOLDER, route)
    if os.path.exists(file_path) and not os.path.isdir(file_path):
        return send_from_directory(STATIC_FOLDER, route)

    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()

@app.route('/login/<action>', defaults={'route': 'form-login'}, methods=['POST'])
def login(route):
    """route login"""
    schema = Schema(request, route)
    schema.schema["data"]["send_form_login"] = 1
    schema.schema["data"]["CONTEXT"]["SESSION"] = "69bdd1e4b4047d8f4e3"
    template = Template(schema.get())
    return template.render()

# Fake login
@app.route('/form-login', defaults={'route': 'form-login'}, methods=['POST'])
def form_login(route):
    """route form-login"""
    schema = Schema(request, route)
    schema.schema["data"]["send_form_login"] = 1

    # Fake login, any user, password: 1234
    if schema.schema["data"]["CONTEXT"]["POST"]["passwd"] == "1234":
        schema.schema["data"]["send_form_login_fails"] = None
        schema.schema["data"]["CONTEXT"]["SESSION"] = "69bdd1e4b4047d8f4e3"
    else:
        schema.schema["data"]["send_form_login_fails"] = True

    template = Template(schema.get())
    return template.render()

# Home
@app.route('/', defaults={'route': 'home'}, methods=['GET', 'POST'])
def home(route):
    """route home"""
    schema = Schema(request, route)
    template = Template(schema.get())
    return template.render()


if __name__ == '__main__':
    app.run(debug=True)
