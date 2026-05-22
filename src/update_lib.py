import re
def update_file(path, replacements):
    with open(path, 'r') as f:
        content = f.read()
    for old, new in replacements:
        content = content.replace(old, new)
    with open(path, 'w') as f:
        f.write(content)

update_file('docutilsrs/src/lib.rs', [
    ('fn py_main() {}', 'fn py_main(py: Python) -> PyResult<()> { py.import_bound("docutils.__main__")?.getattr("main")?.call0()?; Ok(()) }'),
    ('fn py_rst2html() {}', 'fn py_rst2html(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2html")?.call0()?; Ok(()) }'),
    ('fn py_rst2html4() {}', 'fn py_rst2html4(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2html4")?.call0()?; Ok(()) }'),
    ('fn py_rst2html5() {}', 'fn py_rst2html5(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2html5")?.call0()?; Ok(()) }'),
    ('fn py_rst2latex() {}', 'fn py_rst2latex(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2latex")?.call0()?; Ok(()) }'),
    ('fn py_rst2man() {}', 'fn py_rst2man(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2man")?.call0()?; Ok(()) }'),
    ('fn py_rst2odt() {}', 'fn py_rst2odt(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2odt")?.call0()?; Ok(()) }'),
    ('fn py_rst2pseudoxml() {}', 'fn py_rst2pseudoxml(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2pseudoxml")?.call0()?; Ok(()) }'),
    ('fn py_rst2s5() {}', 'fn py_rst2s5(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2s5")?.call0()?; Ok(()) }'),
    ('fn py_rst2xetex() {}', 'fn py_rst2xetex(py: Python) -> PyResult<()> { py.import_bound("docutils.core")?.getattr("rst2xetex")?.call0()?; Ok(()) }')
])

update_file('pygmentsrs/src/lib.rs', [
    ('fn py_main() {}', 'fn py_main(py: Python) -> PyResult<()> { py.import_bound("pygments.cmdline")?.getattr("main")?.call0()?; Ok(()) }')
])
