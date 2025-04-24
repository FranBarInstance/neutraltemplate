"""template and response"""

import json
from flask import redirect, make_response
from constants import TEMPLATE_ROUTER, TEMPLATE_ERROR
from neutraltemplate import NeutralTemplate

class Template:
    """NeutralIpcTemplate"""
    def __init__(self, schema):
        self.schema = schema

    def render(self):
        """render template and return response"""
        template = NeutralTemplate(TEMPLATE_ROUTER, json.dumps(self.schema))
        contents = template.render()
        status_code = int(template.get_status_code())
        status_text = template.get_status_text()
        status_param = template.get_status_param()

        # The template may generate redirects.
        if status_code in [301, 302, 307, 308]:
            response = make_response(redirect(status_param, code=status_code))
            return response

        # The template may generate HTTP errors.
        if status_code >= 400:
            error = {
                "data": {
                    "CONTEXT": {
                        "ROUTE": "error"
                    },
                    "error": {
                        "code": status_code,
                        "text": status_text,
                        "param": status_param
                    }
                }
            }
            template.set_path(TEMPLATE_ERROR)
            template.merge_schema(json.dumps(error))
            contents = template.render()

        response = make_response(contents, status_code)
        response.headers['Content-Type'] = 'text/html'
        return response
