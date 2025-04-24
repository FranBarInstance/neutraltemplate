"""constants"""

import os

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
TEMPLATE_ROUTER = BASE_DIR + "/neutral/tpl/cache.ntpl"
TEMPLATE_ERROR = BASE_DIR + "/neutral/tpl/cache_error.ntpl"
STATIC_FOLDER = BASE_DIR + '/static'
LANG_KEY = "lang"
THEME_KEY = "theme"
DEFAULT_SCHEMA = os.path.join(BASE_DIR, "neutral/data/schema.json")
SIMULATE_SECRET_KEY = "69bdd1e4b4047d8f4e3"
