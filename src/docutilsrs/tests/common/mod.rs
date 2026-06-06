#![allow(dead_code)]

use docutilsrs::{parse_rst, Doctree, NodeKind, cli::CommonOptions};

pub fn coverage_rst(raw_format: &str) -> String {
    format!(
        r#"
= Title =

Subtitle
--------

* item 1
* item 2

1. enum 1
2. enum 2

.. image:: test.png
   :alt: Alt text
   :align: center
   :width: 100
   :height: 100
   :scale: 50 %
   :target: https://example.com

.. math::
   x = y

:field: value

.. note::
   This is a note

.. warning::
   This is a warning

.. raw:: {raw_format}

   <div>raw html</div>

Blockquote:

    This is a blockquote.

    -- Attribution

Text with *emphasis* and **strong** and ``literal`` and `TitleRef`.

.. [1] Footnote
.. [#] Auto footnote
.. [*] Auto symbol footnote

[1]_

.. _target:

Target text.

.. |sub| replace:: replacement
|sub|

==================
Title
==================
Subtitle
-----------

:Author: Test Author
:Version: 1.0


Definition List
  Definition

Term : classifier
  Definition 2

Term 3
  Definition 3

+-------+-------+
| Col 1         |
+-------+-------+
| 1     | 2     |
+-------+-------+

=====  =====  ======
   Inputs     Output
------------  ------
  A      B    A or B
=====  =====  ======
False  False  False
True   False  True
False  True   True
True   True   True
=====  =====  ======

.. figure:: picture.png

   This is the caption.

   And this is the legend.


.. [CIT2002] Citation.

Citation ref [CIT2002]_.

<a_problematic_bracket_that_does_not_close

Escape & < > "

.. code:: python

   def foo():
       pass

"#,
        raw_format = raw_format
    )
}

pub fn build_coverage_tree(rst: &str) -> Doctree {
    let mut tree = parse_rst(rst);
    let doc_id = tree.root();
    let prob_node = tree.append(
        doc_id,
        NodeKind::Problematic {
            refid: "prob1".into(),
            ids: "prob-id".into(),
        },
    );
    tree.append(prob_node, NodeKind::Text("problem text".into()));
    let sys_node = tree.append(
        doc_id,
        NodeKind::SystemMessage {
            level: 2,
            ty: "WARNING",
            ids: "system-message".into(),
            backrefs: "".into(),
            line: Some(0),
        },
    );
    tree.append(sys_node, NodeKind::Text("system message".into()));
    tree
}

pub fn coverage_common_options() -> CommonOptions {
    CommonOptions {
        generator: Some("true".into()),
        date: Some("true".into()),
        time: Some("true".into()),
        ..Default::default()
    }
}
