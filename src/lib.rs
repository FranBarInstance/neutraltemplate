//! Python bindings for Neutral TS template engine.
//!
//! This module provides Python bindings for the Neutral template engine,
//! allowing Python applications to use Neutral templates with high performance
//! thanks to the underlying Rust implementation.
//!
//! # Example
//!
//! ```python
//! from neutraltemplate import NeutralTemplate
//!
//! template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})
//! contents = template.render()
//! ```

use neutralts::utils;
use neutralts::Template;
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList, PyTuple};
use serde_json::Value;

/// Internal representation of template source type.
enum TplType {
    /// Template loaded from file path.
    FilePath(String),
    /// Template provided as raw source string.
    RawSource(String),
}

/// Internal representation of the base schema.
enum BaseSchema {
    /// No schema provided.
    None,
    /// Schema as JSON Value.
    Json(Value),
    /// Schema as MessagePack bytes.
    Msgpack(Vec<u8>),
}

/// Internal representation of schema data to merge.
enum SchemaMerge {
    /// JSON schema to merge.
    Json(Value),
    /// MessagePack schema to merge.
    Msgpack(Vec<u8>),
}

/// Python class for rendering Neutral templates.
///
/// This class provides a Python interface to the Neutral template engine,
/// supporting multiple schema input formats (JSON string, MessagePack, Python dicts).
///
/// # Example
///
/// ```python
/// from neutraltemplate import NeutralTemplate
///
/// # Using file path
/// template = NeutralTemplate("template.ntpl", schema_str='{"data": {"title": "Hello"}}')
/// output = template.render()
///
/// # Using inline source
/// template = NeutralTemplate()
/// template.set_source("{:;data.title:}")
/// template.merge_schema_obj({"data": {"title": "Hello"}})
/// output = template.render()
/// ```
#[pyclass(module = "neutraltemplate")]
struct NeutralTemplate {
    /// Template source (file path or raw string).
    tpl: TplType,
    /// Base schema provided at construction.
    base_schema: BaseSchema,
    /// Additional schemas to merge.
    schema_merges: Vec<SchemaMerge>,
    /// HTTP status code from last render.
    status_code: String,
    /// HTTP status text from last render.
    status_text: String,
    /// Additional status parameter from last render.
    status_param: String,
    /// Whether an error occurred during last render.
    has_error: bool,
}

impl NeutralTemplate {
    /// Converts a Python object to a JSON Value.
    ///
    /// Supports conversion of Python dicts, lists, tuples, strings, booleans,
    /// and numeric types to their JSON equivalents.
    ///
    /// # Arguments
    ///
    /// * `value` - A Python object to convert.
    ///
    /// # Returns
    ///
    /// A `serde_json::Value` representing the converted Python object.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if:
    /// - The value contains a non-finite float (NaN/Infinity)
    /// - The value contains an unsupported type
    fn py_to_json(value: &Bound<'_, PyAny>) -> PyResult<Value> {
        if value.is_none() {
            return Ok(Value::Null);
        }

        // Uso de `cast()` como recomienda PyO3 0.21+
        if let Ok(dict) = value.cast::<PyDict>() {
            let mut map = serde_json::Map::with_capacity(dict.len());
            for (k, v) in dict.iter() {
                // Optimizado: Intentamos extraer como &str sin asignar, si falla usamos to_string()
                let key = if let Ok(s) = k.extract::<&str>() {
                    s.to_owned()
                } else {
                    k.str()?.to_string()
                };
                map.insert(key, Self::py_to_json(&v)?);
            }
            return Ok(Value::Object(map));
        }

        if let Ok(list) = value.cast::<PyList>() {
            let mut arr = Vec::with_capacity(list.len());
            for item in list.iter() {
                arr.push(Self::py_to_json(&item)?);
            }
            return Ok(Value::Array(arr));
        }

        if let Ok(tuple) = value.cast::<PyTuple>() {
            let mut arr = Vec::with_capacity(tuple.len());
            for item in tuple.iter() {
                arr.push(Self::py_to_json(&item)?);
            }
            return Ok(Value::Array(arr));
        }

        // Tipos primitivos extraídos por referencia o copia ligera
        if let Ok(s) = value.extract::<&str>() {
            return Ok(Value::String(s.to_owned()));
        }
        if let Ok(v) = value.extract::<bool>() {
            return Ok(Value::Bool(v));
        }
        if let Ok(v) = value.extract::<i64>() {
            return Ok(Value::Number(v.into()));
        }
        if let Ok(v) = value.extract::<u64>() {
            return Ok(Value::Number(v.into()));
        }
        if let Ok(v) = value.extract::<f64>() {
            if !v.is_finite() {
                return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    "schema_obj contains non-finite float (NaN/Infinity), not valid JSON",
                ));
            }
            return serde_json::Number::from_f64(v)
                .map(Value::Number)
                .ok_or_else(|| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(
                        "schema_obj contains invalid float value",
                    )
                });
        }

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "schema_obj contains unsupported type",
        ))
    }

    /// Executes the template rendering process.
    ///
    /// This internal method handles the actual rendering logic, including:
    /// - Loading templates from file or raw source
    /// - Applying base schema and merged schemas
    /// - Capturing status information
    ///
    /// # Arguments
    ///
    /// * `py` - Python GIL token for thread safety.
    /// * `render_once` - If true, uses `render_once()` which consumes the schema
    ///   for better performance. If false, uses standard `render()` which clones
    ///   the schema and allows template reuse.
    ///
    /// # Returns
    ///
    /// The rendered template content as a string.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if template loading or rendering fails.
    fn run_render(&mut self, py: Python<'_>, render_once: bool) -> PyResult<String> {
        let (contents, status_code, status_text, status_param, has_error) = py
            .detach(|| {
                let mut template = match &self.tpl {
                    TplType::FilePath(path) => match &self.base_schema {
                        BaseSchema::None => Template::from_file_value(path, serde_json::json!({}))
                            .map_err(|e| format!("Template::from_file_value() failed: {}", e))?,
                        BaseSchema::Json(schema) => Template::from_file_value(path, schema.clone())
                            .map_err(|e| format!("Template::from_file_value() failed: {}", e))?,
                        BaseSchema::Msgpack(bytes) => Template::from_file_msgpack(path, bytes)
                            .map_err(|e| format!("Template::from_file_msgpack() failed: {}", e))?,
                    },
                    TplType::RawSource(source) => {
                        let mut template = Template::new()
                            .map_err(|e| format!("Template::new() failed: {}", e))?;
                        match &self.base_schema {
                            BaseSchema::None => {}
                            BaseSchema::Json(schema) => {
                                template.merge_schema_value(schema.clone());
                            }
                            BaseSchema::Msgpack(bytes) => {
                                template
                                    .merge_schema_msgpack(bytes)
                                    .map_err(|e| format!("merge_schema_msgpack failed: {}", e))?;
                            }
                        }
                        template.set_src_str(source);
                        template
                    }
                };

                for merge in &self.schema_merges {
                    match merge {
                        SchemaMerge::Json(schema) => {
                            template.merge_schema_value(schema.clone());
                        }
                        SchemaMerge::Msgpack(bytes) => {
                            template
                                .merge_schema_msgpack(bytes)
                                .map_err(|e| format!("merge_schema_msgpack failed: {}", e))?;
                        }
                    }
                }

                let contents = if render_once {
                    template.render_once()
                } else {
                    template.render()
                };

                Ok::<_, String>((
                    contents,
                    template.get_status_code().clone(),
                    template.get_status_text().clone(),
                    template.get_status_param().clone(),
                    template.has_error(),
                ))
            })
            .map_err(|msg| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(msg))?;

        self.status_code = status_code;
        self.status_text = status_text;
        self.status_param = status_param;
        self.has_error = has_error;

        Ok(contents)
    }
}

#[pymethods]
impl NeutralTemplate {
    /// Creates a new NeutralTemplate instance.
    ///
    /// # Arguments
    ///
    /// * `path` - Optional path to the template file. If not provided or empty,
    ///   the template source must be set later with `set_source()`.
    /// * `schema_str` - Optional JSON schema as a string.
    /// * `schema_msgpack` - Optional MessagePack schema as bytes.
    /// * `schema_obj` - Optional Python dict/list as schema.
    ///
    /// Only one of `schema_str`, `schema_msgpack`, or `schema_obj` can be provided.
    ///
    /// # Returns
    ///
    /// A new `NeutralTemplate` instance.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if:
    /// - More than one schema input is provided
    /// - The JSON schema string is invalid
    /// - The MessagePack schema is invalid
    /// - The Python object contains unsupported types
    ///
    /// # Example
    ///
    /// ```python
    /// # From file with JSON string schema
    /// template = NeutralTemplate("file.ntpl", schema_str='{"data": {}}')
    ///
    /// # From file with Python dict schema
    /// template = NeutralTemplate("file.ntpl", schema_obj={"data": {}})
    ///
    /// # Empty template (set source later)
    /// template = NeutralTemplate()
    /// template.set_source("{:;data.title:}")
    /// ```
    #[new]
    #[pyo3(signature = (path=None, schema_str=None, schema_msgpack=None, schema_obj=None))]
    #[pyo3(text_signature = "(path=None, schema_str=None, schema_msgpack=None, schema_obj=None)")]
    fn new(
        path: Option<&str>,
        schema_str: Option<&str>,
        schema_msgpack: Option<&[u8]>,
        schema_obj: Option<&Bound<'_, PyAny>>,
    ) -> PyResult<Self> {
        let has_str = schema_str.map_or(false, |s| !s.is_empty());
        let has_msgpack = schema_msgpack.map_or(false, |b| !b.is_empty());
        let has_obj = schema_obj.is_some();

        if (has_str as u8 + has_msgpack as u8 + has_obj as u8) > 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "use only one schema input: schema_str, schema_msgpack, or schema_obj",
            ));
        }

        let tpl = match path {
            Some(p) if !p.is_empty() => TplType::FilePath(p.to_owned()),
            _ => TplType::RawSource(String::new()),
        };

        let base_schema = if has_str {
            let schema: Value = serde_json::from_str(schema_str.unwrap()).map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("schema is not a valid JSON string: {}", e),
                )
            })?;
            BaseSchema::Json(schema)
        } else if has_msgpack {
            BaseSchema::Msgpack(schema_msgpack.unwrap().to_vec())
        } else if let Some(obj) = schema_obj {
            BaseSchema::Json(Self::py_to_json(obj)?)
        } else {
            BaseSchema::None
        };

        Ok(NeutralTemplate {
            tpl,
            base_schema,
            schema_merges: Vec::new(),
            status_code: String::new(),
            status_text: String::new(),
            status_param: String::new(),
            has_error: false,
        })
    }

    /// Renders the template and returns the output.
    ///
    /// This method clones the schema before rendering, allowing the template
    /// to be rendered multiple times. For single-render scenarios, consider
    /// using `render_once()` for better performance.
    ///
    /// # Returns
    ///
    /// The rendered template content as a string.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if template loading or rendering fails.
    ///
    /// # Example
    ///
    /// ```python
    /// template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})
    /// output = template.render()
    /// print(output)
    /// ```
    #[pyo3(text_signature = "(/)")]
    fn render(&mut self, py: Python<'_>) -> PyResult<String> {
        self.run_render(py, false)
    }

    /// Renders the template without cloning the schema (optimized for single use).
    ///
    /// This is an optimized version of `render()` that takes ownership of the schema
    /// instead of cloning it. Use this when you only need to render once per template
    /// instance, which is the most common use case in web applications.
    ///
    /// # When to Use
    ///
    /// - **Single render per request**: Most web applications create a template,
    ///   render it once, and discard it. This is the ideal use case for `render_once()`.
    /// - **Large schemas**: When your schema contains thousands of keys, the
    ///   performance improvement can be significant.
    /// - **Memory-constrained environments**: Avoids the memory spike of cloning
    ///   large schemas.
    ///
    /// # When NOT to Use
    ///
    /// - **Multiple renders**: If you need to render the same template multiple times
    ///   with the same schema, use `render()` instead.
    /// - **Template reuse**: After `render_once()`, the template cannot be reused
    ///   because the schema is consumed.
    ///
    /// # Post-Call Behavior
    ///
    /// After calling this method, the template's schema will be empty (`{}`) and
    /// subsequent calls to `render()` or `render_once()` will produce empty output
    /// for schema variables. The template struct itself remains valid but should
    /// be discarded after use.
    ///
    /// # Returns
    ///
    /// The rendered template content as a string.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if template loading or rendering fails.
    ///
    /// # Example
    ///
    /// ```python
    /// template = NeutralTemplate("file.ntpl", schema_obj={"data": {"title": "Hello"}})
    ///
    /// # Single render - use render_once() for best performance
    /// output = template.render_once()
    /// print(output)
    ///
    /// # Template should NOT be reused after render_once()
    /// # Create a new NeutralTemplate instance for the next render
    /// ```
    #[pyo3(text_signature = "(/)")]
    fn render_once(&mut self, py: Python<'_>) -> PyResult<String> {
        self.run_render(py, true)
    }

    /// Returns the HTTP status code from the last render.
    ///
    /// Common values include "200" for success, "404" for not found,
    /// "500" for server error, etc.
    ///
    /// # Returns
    ///
    /// The HTTP status code as a string slice.
    fn get_status_code(&self) -> &str {
        &self.status_code
    }

    /// Returns the HTTP status text from the last render.
    ///
    /// Common values include "OK" for success, "Not Found" for 404,
    /// "Internal Server Error" for 500, etc.
    ///
    /// # Returns
    ///
    /// The HTTP status text as a string slice.
    fn get_status_text(&self) -> &str {
        &self.status_text
    }

    /// Returns the additional status parameter from the last render.
    ///
    /// This is typically empty unless an error occurred with additional context.
    ///
    /// # Returns
    ///
    /// The status parameter as a string slice (empty if no error).
    fn get_status_param(&self) -> &str {
        &self.status_param
    }

    /// Returns whether an error occurred during the last render.
    ///
    /// # Returns
    ///
    /// `true` if an error occurred, `false` otherwise.
    fn has_error(&self) -> bool {
        self.has_error
    }

    /// Sets the template file path.
    ///
    /// This overrides any previously set path or source.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the template file.
    ///
    /// # Example
    ///
    /// ```python
    /// template = NeutralTemplate()
    /// template.set_path("/path/to/template.ntpl")
    /// output = template.render()
    /// ```
    fn set_path(&mut self, path: String) {
        self.tpl = TplType::FilePath(path);
    }

    /// Sets the template source code directly.
    ///
    /// This overrides any previously set path or source.
    /// Use this for inline templates instead of loading from a file.
    ///
    /// # Arguments
    ///
    /// * `source` - The template source code as a string.
    ///
    /// # Example
    ///
    /// ```python
    /// template = NeutralTemplate()
    /// template.set_source("{:;data.title:}")
    /// template.merge_schema_obj({"data": {"title": "Hello"}})
    /// output = template.render()
    /// ```
    fn set_source(&mut self, source: String) {
        self.tpl = TplType::RawSource(source);
    }

    /// Merges a JSON schema string into the existing schema.
    ///
    /// The schema is merged recursively with any existing schema data.
    ///
    /// # Arguments
    ///
    /// * `schema_str` - A valid JSON string representing the schema to merge.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if the JSON string is invalid.
    ///
    /// # Example
    ///
    /// ```python
    /// template = NeutralTemplate()
    /// template.merge_schema('{"data": {"title": "Hello"}}')
    /// ```
    #[pyo3(text_signature = "(schema_str)")]
    fn merge_schema(&mut self, schema_str: &str) -> PyResult<()> {
        let schema: Value = serde_json::from_str(schema_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "schema is not a valid JSON string: {}",
                e
            ))
        })?;
        match &mut self.base_schema {
            BaseSchema::None => self.base_schema = BaseSchema::Json(schema),
            BaseSchema::Json(base_schema) => utils::merge_schema(base_schema, &schema),
            BaseSchema::Msgpack(_) => self.schema_merges.push(SchemaMerge::Json(schema)),
        }
        Ok(())
    }

    /// Merges a MessagePack schema into the existing schema.
    ///
    /// The schema is merged recursively with any existing schema data.
    ///
    /// # Arguments
    ///
    /// * `schema_msgpack` - MessagePack bytes representing the schema to merge.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if the MessagePack bytes are invalid.
    ///
    /// # Example
    ///
    /// ```python
    /// # {"data": {"key": "value"}}
    /// schema_msgpack = bytes([129, 164, 100, 97, 116, 97, 129, 163, 107, 101, 121, 165, 118, 97, 108, 117, 101])
    /// template = NeutralTemplate()
    /// template.merge_schema_msgpack(schema_msgpack)
    /// ```
    #[pyo3(text_signature = "(schema_msgpack)")]
    fn merge_schema_msgpack(&mut self, schema_msgpack: &[u8]) -> PyResult<()> {
        self.schema_merges
            .push(SchemaMerge::Msgpack(schema_msgpack.to_vec()));
        Ok(())
    }

    /// Merges a Python dictionary or list into the existing schema.
    ///
    /// This is a convenience method that allows passing Python objects directly
    /// without JSON serialization. The object is converted to JSON internally.
    ///
    /// # Arguments
    ///
    /// * `schema_obj` - A Python dict, list, or tuple to merge as schema.
    ///
    /// # Errors
    ///
    /// Returns a `PyErr` if:
    /// - The object contains unsupported types
    /// - The object contains non-finite floats (NaN/Infinity)
    ///
    /// # Example
    ///
    /// ```python
    /// template = NeutralTemplate()
    /// template.merge_schema_obj({
    ///     "data": {
    ///         "title": "Hello World",
    ///         "items": ["one", "two", "three"]
    ///     }
    /// })
    /// output = template.render()
    /// ```
    #[pyo3(text_signature = "(schema_obj)")]
    fn merge_schema_obj(&mut self, schema_obj: &Bound<'_, PyAny>) -> PyResult<()> {
        let schema = Self::py_to_json(schema_obj)?;
        match &mut self.base_schema {
            BaseSchema::None => self.base_schema = BaseSchema::Json(schema),
            BaseSchema::Json(base_schema) => utils::merge_schema(base_schema, &schema),
            BaseSchema::Msgpack(_) => self.schema_merges.push(SchemaMerge::Json(schema)),
        }
        Ok(())
    }
}

/// Python module for the Neutral template engine.
///
/// This module exposes the `NeutralTemplate` class for use in Python.
#[pymodule]
fn neutraltemplate(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NeutralTemplate>()?;
    Ok(())
}
