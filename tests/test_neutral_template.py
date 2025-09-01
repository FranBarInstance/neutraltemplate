"""Test module for NeutralTemplate Rust class exposed to Python via PyO3."""
import unittest
import os
import time
from neutraltemplate import NeutralTemplate

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
TEMPLATE1 = BASE_DIR + "/template1.ntpl"
TEMPLATE_ERROR = BASE_DIR + "/non_existent.ntpl"
TEMPLATE_CACHE = """
    {:^include; {:flg; require :} >> tests/snippets.ntpl :}
    {:^locale; tests/locale.{:lang;:}.json :}
    {:^;:}<div1></div1>
    {:^;:}<div2></div2>
    {:^;:}<div3></div3>
    {:^;:}::--::{:^date; %S :}::--::
    {:^;:}{:sum; /{:;one:}/{:;one:}/ :}
    {:^;:}{:fetch; '/url' >> loading... :}
    {:^;:}{:join; /__test-arr-nts/|/ :}
    {:^;:}{:;__hello-nts:}
    {:^;:}{:allow; _test-nts >> {:;__hello-nts:} :}
    {:^;:}{:!allow; _test-nts >> {:;__hello-nts:} :}
    {:^;:}{:array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
    {:^;:}{:!array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
    {:^;:}{:bool; true >> true :}
    {:^;:}{:!bool; true >> true :}
    {:^;:}{:coalesce; {:;empty:}{:;__hello-nts:} :}
    {:^;:}{:code; {:param; {:;__hello-nts:} >> {:;__hello-nts:} :} {:coalesce; {:;empty:}{:param; {:;__hello-nts:} :} :} :}
    {:^;:}{:contains; /haystack/st/ >> contains :}
    {:^;:}{:defined; __test-nts >> is defined :}
    {:^;:}{:!defined; __test-nts >> is defined :}
    {:^;:}{:code;  :}{:else; else :}
    {:^;:}{:eval; {:;__test-nts:} >> {:;__eval__:} :}
    {:^;:}{:filled; __test-nts >> is filled :}
    {:^;:}{:!filled; __test-nts >> is filled :}
    {:^;:}{:for; n 0 9 >> {:;n:} :}
    {:^;:}{:hash; {:;__test-nts:} :}
    {:^;:}{:lang; :}
    {:^;:}{:moveto; <div1 >> 1{:;__test-nts:} :}
    {:^;:}{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}
    {:^;:}{:rand; 1..1 :}
    {:^;:}{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}
    {:^;:}{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}
    {:^;:}{:trans; {:trans; Hello nts :} :}
    {:^;:}{:obj; tests/obj.json :}
    {:^cache; /3/ >>
        {:^;:}::--::{:^date; %S :}::--::
        {:^;:}{:sum; /{:;one:}/{:;one:}/ :}
        {:^;:}{:fetch; '/url' >> loading... :}
        {:^;:}{:join; /__test-arr-nts/|/ :}
        {:^;:}{:;__hello-nts:}
        {:^;:}{:allow; _test-nts >> {:;__hello-nts:} :}
        {:^;:}{:!allow; _test-nts >> {:;__hello-nts:} :}
        {:^;:}{:array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
        {:^;:}{:!array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
        {:^;:}{:bool; true >> true :}
        {:^;:}{:!bool; true >> true :}
        {:^;:}{:coalesce; {:;empty:}{:;__hello-nts:} :}
        {:^;:}{:code; {:param; {:;__hello-nts:} >> {:;__hello-nts:} :} {:coalesce; {:;empty:}{:param; {:;__hello-nts:} :} :} :}
        {:^;:}{:contains; /haystack/st/ >> contains :}
        {:^;:}{:defined; __test-nts >> is defined :}
        {:^;:}{:!defined; __test-nts >> is defined :}
        {:^;:}{:code;  :}{:else; else :}
        {:^;:}{:eval; {:;__test-nts:} >> {:;__eval__:} :}
        {:^;:}{:filled; __test-nts >> is filled :}
        {:^;:}{:!filled; __test-nts >> is filled :}
        {:^;:}{:for; n 0 9 >> {:;n:} :}
        {:^;:}{:hash; {:;__test-nts:} :}
        {:^;:}{:lang; :}
        {:^;:}{:moveto; <div2 >> 2{:;__test-nts:} :}
        {:^;:}{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}
        {:^;:}{:rand; 1..1 :}
        {:^;:}{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}
        {:^;:}{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}
        {:^;:}{:trans; {:trans; Hello nts :} :}
        {:^;:}{:obj; tests/obj.json :}
        {:!cache;
            {:^;:}::--::{:^date; %S :}::--::
            {:^;:}{:sum; /{:;one:}/{:;one:}/ :}
            {:^;:}{:fetch; '/url' >> loading... :}
            {:^;:}{:join; /__test-arr-nts/|/ :}
            {:^;:}{:;__hello-nts:}
            {:^;:}{:allow; _test-nts >> {:;__hello-nts:} :}
            {:^;:}{:!allow; _test-nts >> {:;__hello-nts:} :}
            {:^;:}{:array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
            {:^;:}{:!array; __test-arr-nts >> {:each; __test-arr-nts k v >> {:;k:}{:;v:} :} :}
            {:^;:}{:bool; true >> true :}
            {:^;:}{:!bool; true >> true :}
            {:^;:}{:coalesce; {:;empty:}{:;__hello-nts:} :}
            {:^;:}{:code; {:param; {:;__hello-nts:} >> {:;__hello-nts:} :} {:coalesce; {:;empty:}{:param; {:;__hello-nts:} :} :} :}
            {:^;:}{:contains; /haystack/st/ >> contains :}
            {:^;:}{:defined; __test-nts >> is defined :}
            {:^;:}{:!defined; __test-nts >> is defined :}
            {:^;:}{:code;  :}{:else; else :}
            {:^;:}{:eval; {:;__test-nts:} >> {:;__eval__:} :}
            {:^;:}{:filled; __test-nts >> is filled :}
            {:^;:}{:!filled; __test-nts >> is filled :}
            {:^;:}{:for; n 0 9 >> {:;n:} :}
            {:^;:}{:hash; {:;__test-nts:} :}
            {:^;:}{:lang; :}
            {:^;:}{:moveto; <div3 >> 3{:;__test-nts:} :}
            {:^;:}{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}
            {:^;:}{:rand; 1..1 :}
            {:^;:}{:replace; /{:;__test-nts:}/{:;__test-arr-nts->0:}/ >> {:;__hello-nts:} :}
            {:^;:}{:same; /{:;__test-nts:}/{:;__test-nts:}/ >> {:;__test-nts:} :}
            {:^;:}{:trans; {:trans; Hello nts :} :}
            {:^;:}{:obj; tests/obj.json :}
        :}
    :}
""".strip()
SCHEMA1 = """
{
    "data": {
        "__hello-nts": "Overwritten __hello-nts"
    }
}
"""
SCHEMA2 = """
{
    "config": {
        "infinite_loop_max_bifs": 555000,
        "comments": "remove",
        "errors": "hide",
        "comments": "remove",
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
        "snippets": {
            "__hello-nts": "<div>{:trans; ref:greeting-nts :}</div>",
            "inject": "{:;inject:}"
        },
        "declare": {
            "any": "*",
            "_test-nts": "en es fr de nts",
            "_test-nts-empty": "~ nts en es fr de",
            "_test-nts-asterisk": "*en* nts es fr de",
            "_test-nts-question": "en?nts nts es fr de",
            "_test-nts-dot": "en.nts es fr de"
        },
        "params": {},
        "locale": {
            "current": "en",
            "trans": {
                "en": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "en-US": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "en-UK": {
                    "Hello nts": "Hello",
                    "ref:greeting-nts": "Hello"
                },
                "es": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "es-ES": {
                    "Hello nts": "Hola",
                    "ref:greeting-nts": "Hola"
                },
                "de": {
                    "Hello nts": "Hallo",
                    "ref:greeting-nts": "Hallo"
                },
                "fr": {
                    "Hello nts": "Bonjour",
                    "ref:greeting-nts": "Bonjour"
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
            "GET": {
                "escape": "<>&\\"'/{}"
            }
        },
        "__hello-nts": "Hello nts",
        "__ref-hello-nts": "__hello-nts",
        "__test-local": "local",
        "__test-nts": "nts",
        "__test-empty-nts": "",
        "__test-null-nts": null,
        "__test-zero-nts": 0,
        "__test-bool-true-string-nts": true,
        "__test-bool-true-num-nts": 1,
        "__test-bool-false-string-nts": false,
        "__test-bool-false-num-nts": 0,
        "__test-bool-false-empty-nts": "",
        "__test-arr-nts": [
            "one",
            "two",
            "three"
        ],
        "__test-arr-empty-nts": [],
        "__test-obj-empty-nts": {},
        "__test-obj-nts": {
            "level1": "Ok",
            "level1-obj": {
                "level1": "Ok",
                "level2-obj": {
                    "level2": "Ok",
                    "level3-arr": [
                        "one",
                        "two",
                        "three"
                    ]
                }
            }
        },
        "mailfotmated": "{::}",
        "inject": "{:exit; 403 :}",
        "escape": "<>&\\"'/{}",
        "double_escape": "&lt;&gt;&amp;&quot;&#x27;&#x2F;&#123;&#125;",
        "true": true,
        "false": false,
        "text": "text",
        "zero": "0",
        "one": "1",
        "spaces": "  ",
        "empty": "",
        "null": null,
        "emptyarr": [],
        "array": {
            "true": true,
            "false": false,
            "text": "text",
            "zero": "0",
            "one": "1",
            "spaces": "  ",
            "empty": "",
            "null": null
        }
    }
}
"""

class TestNeutralTemplate(unittest.TestCase):
    """Test cases for NeutralTemplate class."""

    def test_initialization_with_no_params(self):
        """Test initialization no params."""

        template = NeutralTemplate()
        contents = template.render()

        self.assertEqual(contents, "")
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_initialization_with_file(self):
        """Test initialization with file."""

        template = NeutralTemplate(TEMPLATE1)
        contents = template.render()

        self.assertEqual(contents, "Hello nts")
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_initialization_with_file_and_schema(self):
        """Test initialization with file and schema."""

        template = NeutralTemplate(TEMPLATE1, SCHEMA1)
        contents = template.render()

        self.assertEqual(contents, "Overwritten __hello-nts")
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_initialization_set_file_and_schema(self):
        """Test initialization set file and schema."""

        template = NeutralTemplate()
        template.set_path(TEMPLATE1)
        template.merge_schema(SCHEMA1)
        contents = template.render()

        self.assertEqual(contents, "Overwritten __hello-nts")
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_initialization_set_source_and_schema(self):
        """Test initialization set source and schema."""

        template = NeutralTemplate()
        template.set_source("{:;__hello-nts:}")
        template.merge_schema(SCHEMA1)
        contents = template.render()

        self.assertEqual(contents, "Overwritten __hello-nts")
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_has_error_parse_false(self):
        """Test get error parse false"""

        template = NeutralTemplate()
        template.set_source("{:;__hello-nts:}")
        contents = template.render()

        self.assertEqual(contents, "Hello nts")
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_has_error_parse_true(self):
        """Test get error parse true"""

        template = NeutralTemplate()
        template.set_source("{:force-error;__hello-nts:}")
        contents = template.render()

        self.assertEqual(contents, "")
        self.assertEqual(template.has_error(), True)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

    def test_initialization_with_invalid_file(self):
        """Test initialization fails with invalid file."""

        template = NeutralTemplate(TEMPLATE_ERROR)

        with self.assertRaises(RuntimeError) as context:
            template.render()

        error_message = str(context.exception)
        self.assertIn("No such file or directory", error_message)
        self.assertIn("os error 2", error_message)

    def test_initialization_with_invalid_schema(self):
        """Test initialization fails with invalid JSON schema."""

        with self.assertRaises(ValueError):
            NeutralTemplate(TEMPLATE1, "{")

    def test_initialization_with_invalid_json_merge_schema(self):
        """Test initialization fails with invalid JSON in merge_schema."""

        template = NeutralTemplate(TEMPLATE_ERROR)

        with self.assertRaises(ValueError):
            template.merge_schema("{")

    def test_bif_cache_complete(self):
        """Test some bif with cache."""

        template = NeutralTemplate()
        template.set_source(TEMPLATE_CACHE)
        template.merge_schema(SCHEMA2)
        contents = template.render()
        expected = "2<div id=\"\" class=\"neutral-fetch-auto \" data-url=\"/url\" data-wrap=\"\">\n    loading...\n</div>one|two|threeHello ntsHello nts0one1two2threetrueHello ntsHello ntscontainsis definedelsentsis filled01234567895c96e4f24ce6e234e6bd4df066748030en{:neutral; {:;__test-nts:} >> {:;__test-nts:} :}1Hello onentsHelloPython Obj"

        result_write_parts = contents.split("::--::")
        self.assertEqual(result_write_parts[0], "<div1>1nts</div1><div2>2nts</div2><div3>3nts</div3>")
        self.assertEqual(result_write_parts[2], expected)
        self.assertEqual(result_write_parts[4], expected)
        self.assertEqual(result_write_parts[6], expected)
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

        # we give 1 second for “date” to show a different result in !cache
        time.sleep(1.1)

        contents = template.render()

        result_read_parts = contents.split("::--::")
        self.assertEqual(result_write_parts[0], result_read_parts[0])

        # if the dates are not different, it has not been read from the cache.
        self.assertNotEqual (result_write_parts[1], result_read_parts[1])
        self.assertEqual(result_write_parts[2], result_read_parts[2])
        self.assertEqual(result_write_parts[3], result_read_parts[3])
        self.assertEqual(result_write_parts[4], result_read_parts[4])

        # if the dates are not different, it has not been read from the cache.
        self.assertNotEqual(result_write_parts[5], result_read_parts[5])
        self.assertEqual(result_write_parts[6], result_read_parts[6])
        self.assertEqual(template.has_error(), False)
        self.assertEqual(template.get_status_code(), "200")
        self.assertEqual(template.get_status_text(), "OK")
        self.assertEqual(template.get_status_param(), "")

        # we give 2 second for cache reset to repeat tests because: {:^cache; /3/ >>
        time.sleep(2)


if __name__ == '__main__':
    unittest.main()
