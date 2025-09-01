use neutralts::utils;
use neutralts::Template;
use pyo3::prelude::*;
use serde_json::Value;

enum TplType {
    FilePath(String),
    RawSource(String),
}

#[pyclass(module = "neutraltemplate")]
struct NeutralTemplate {
    tpl: TplType,
    schema: Value,
    status_code: String,
    status_text: String,
    status_param: String,
    has_error: bool,
}

#[pymethods]
impl NeutralTemplate {
    #[new]
    #[pyo3(signature = (path=None, schema_str=None))]
    #[pyo3(text_signature = "(path=None, schema_str=None)")]
    fn new(path: Option<String>, schema_str: Option<String>) -> PyResult<Self> {
        let tpl = match path {
            Some(p) if !p.is_empty() => TplType::FilePath(p),
            _ => TplType::RawSource(String::new()),
        };

        let schema = match schema_str {
            Some(s) if !s.is_empty() => {
                serde_json::from_str(&s).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("schema is not a valid JSON string: {}", e),
                    )
                })?
            }
            _ => serde_json::json!({}),
        };

        Ok(NeutralTemplate {
            tpl,
            schema,
            status_code: String::new(),
            status_text: String::new(),
            status_param: String::new(),
            has_error: false,
        })
    }

    #[pyo3(text_signature = "(/)")]
    fn render(&mut self, py: Python<'_>) -> PyResult<String> {
        let (contents, status_code, status_text, status_param, has_error) = py
            .detach(|| {
                let mut template =
                    Template::new().map_err(|e| format!("Template::new() failed: {}", e))?;

                template.merge_schema_value(self.schema.clone());

                match &self.tpl {
                    TplType::FilePath(path) => {
                        template
                            .set_src_path(path)
                            .map_err(|e| format!("set_src_path failed: {}", e))?;
                    }
                    TplType::RawSource(source) => {
                        template.set_src_str(source);
                    }
                };

                let contents = template.render();
                let status_code = template.get_status_code().clone();
                let status_text = template.get_status_text().clone();
                let status_param = template.get_status_param().clone();
                let has_error = template.has_error();

                Ok::<_, String>((contents, status_code, status_text, status_param, has_error))
            })
            .map_err(|msg| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg))?;

        self.status_code = status_code;
        self.status_text = status_text;
        self.status_param = status_param;
        self.has_error = has_error;

        Ok(contents)
    }

    fn get_status_code(&self) -> String {
        self.status_code.clone()
    }

    fn get_status_text(&self) -> String {
        self.status_text.clone()
    }

    fn get_status_param(&self) -> String {
        self.status_param.clone()
    }

    fn has_error(&self) -> bool {
        self.has_error
    }

    fn set_path(&mut self, path: String) {
        self.tpl = TplType::FilePath(path);
    }

    fn set_source(&mut self, source: String) {
        self.tpl = TplType::RawSource(source);
    }

    #[pyo3(text_signature = "(schema_str)")]
    fn merge_schema(&mut self, schema_str: String) -> PyResult<()> {
        let schema: Value = serde_json::from_str(&schema_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "schema is not a valid JSON string: {}",
                e
            ))
        })?;
        utils::merge_schema(&mut self.schema, &schema);
        Ok(())
    }
}

#[pymodule]
fn neutraltemplate(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NeutralTemplate>()?;
    Ok(())
}
