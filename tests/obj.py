"""Test neutral python objects."""

def main(params=None):
    """Test neutral python objects."""

    if params is None:
        params = {}

    return {
        "data": {
            "param1": params.get("param1", "_none"),
        }
    }
