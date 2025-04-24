use neutralts::utils;
use neutralts::Template;
use pyo3::prelude::*;
use serde_json::Value;

enum TplType {
    FilePath(String),
    RawSource(String),
}

#[pyclass]
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
    fn new(path: Option<String>, schema_str: Option<String>) -> PyResult<Self> {
        let tpl = match path {
            Some(p) if !p.is_empty() => TplType::FilePath(p),
            _ => TplType::RawSource(String::new()),
        };
        let schema = match schema_str {
            Some(s) if !s.is_empty() => {
                serde_json::from_str(&s)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        format!("schema is not a valid JSON string: {}", e)
                    ))?
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

    fn render(&mut self) -> PyResult<String> {
        let mut template =
            Template::new().map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
        template.merge_schema_value(self.schema.clone());
        match &self.tpl {
            TplType::FilePath(path) => {
                template.set_src_path(path)
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
            }
            TplType::RawSource(source) => {
                template.set_src_str(source);
            }
        };
        let contents = template.render();
        self.status_code = template.get_status_code().clone();
        self.status_text = template.get_status_text().clone();
        self.status_param = template.get_status_param().clone();

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
